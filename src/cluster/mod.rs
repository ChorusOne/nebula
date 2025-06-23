mod storage;

use crate::cluster::storage::RaftStorage;
use crate::config::RaftConfig;
use crate::error::SignerError;
use crate::types::ConsensusData;
use log::{info, warn};
use protobuf::Message as ProtobufMessage;
use raft::prelude::{ConfState, EntryType, Message as RaftProtoMessage, Snapshot};
use raft::{Config as RaftCoreConfig, RawNode, StateRole, Storage};
use slog::{Drain, o};
use std::collections::{HashMap, VecDeque};
use std::io::{BufReader, BufWriter, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::mpsc::{self, RecvTimeoutError, Sender};
use std::sync::{Arc, RwLock};
use std::thread;
use std::time::{Duration, Instant};

use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};

enum RaftMessage {
    Propose(ConsensusData, Sender<Result<(), SignerError>>),
    Msg(RaftProtoMessage),
    TransferLeadership(u64),
}

pub struct SignerRaftNode {
    node_id: u64,
    pub signer_state: Arc<RwLock<ConsensusData>>,
    proposal_sender: Sender<RaftMessage>,
    raft_state: Arc<RwLock<(StateRole, u64)>>,
}

impl SignerRaftNode {
    pub fn new(config: RaftConfig) -> Arc<Self> {
        let drain = slog_stdlog::StdLog.fuse();
        let drain = slog_async::Async::new(drain)
            .chan_size(4096)
            .overflow_strategy(slog_async::OverflowStrategy::Block)
            .build()
            .fuse();
        let logger = slog::Logger::root(drain, o!("tag" => format!("[{}]", config.node_id)));

        let path = format!("{}_{}", config.data_path, config.node_id);
        info!("[init] storage at {}", path);
        let mut storage = RaftStorage::new(&path);

        let peer_ids: Vec<u64> = config.peers.iter().map(|p| p.id).collect();
        let init_state = storage.initial_state().unwrap();
        let initial_sm_state =
            if init_state.hard_state.commit == 0 && init_state.hard_state.term == 0 {
                info!(
                    "[init] fresh store, bootstrapping with peers: {:?}",
                    peer_ids
                );
                let mut snap = Snapshot::default();
                snap.mut_metadata().set_index(1);
                snap.mut_metadata().set_term(1);
                let mut cs = ConfState::default();
                cs.set_voters(peer_ids);
                snap.mut_metadata().set_conf_state(cs);
                storage.apply_snapshot(snap).unwrap();

                let state_file_path = std::path::Path::new(&config.initial_state_path);
                if let Some(bootstrap_state) = ConsensusData::load_from_file(state_file_path) {
                    info!(
                        "[init] loaded bootstrap state from state.json: {}",
                        bootstrap_state
                    );
                    storage.write_signer_state(&bootstrap_state).unwrap();
                    bootstrap_state
                } else {
                    info!("[init] no state.json found, using default state.");
                    let default_state = ConsensusData::default();
                    storage.write_signer_state(&default_state).unwrap();
                    default_state
                }
            } else {
                info!("[init] found existing state, loading from DB");
                storage.read_signer_state().unwrap()
            };

        let raft_cfg = RaftCoreConfig {
            id: config.node_id,
            election_tick: 10,
            heartbeat_tick: 3,
            ..Default::default()
        };
        raft_cfg.validate().unwrap();

        let (in_tx, in_rx) = mpsc::channel::<RaftMessage>();
        let (out_tx, out_rx) = mpsc::channel::<RaftProtoMessage>();
        let raft_state = Arc::new(RwLock::new((StateRole::Follower, 0)));

        {
            let bind = config.bind_addr.clone();
            let in_tx_clone = in_tx.clone();
            thread::spawn(move || {
                let listener =
                    TcpListener::bind(&bind).unwrap_or_else(|_| panic!("bind failed on {}", bind));
                info!("[net] listening on {}", bind);
                for conn in listener.incoming() {
                    if let Ok(stream) = conn {
                        let in_tx = in_tx_clone.clone();
                        thread::spawn(move || {
                            let mut reader = BufReader::new(stream);
                            loop {
                                let len = match reader.read_u32::<BigEndian>() {
                                    Ok(l) => l as usize,
                                    Err(_) => break,
                                };
                                let mut buf = vec![0; len];
                                if reader.read_exact(&mut buf).is_err() {
                                    break;
                                }
                                match RaftProtoMessage::parse_from_bytes(&buf) {
                                    Ok(msg) => {
                                        if in_tx.send(RaftMessage::Msg(msg)).is_err() {
                                            break;
                                        }
                                    }
                                    Err(e) => warn!("[net] parse error: {:?}", e),
                                }
                            }
                        });
                    }
                }
            });
        }

        {
            let peers_config = config.peers.clone();
            let me = config.node_id;

            thread::spawn(move || {
                let mut peer_writers: HashMap<u64, (String, Option<BufWriter<TcpStream>>)> =
                    peers_config
                        .into_iter()
                        .filter(|p| p.id != me)
                        .map(|p| (p.id, (p.addr, None)))
                        .collect();

                while let Ok(msg) = out_rx.recv() {
                    let to_id = msg.to;

                    let (addr, writer_opt) = match peer_writers.get_mut(&to_id) {
                        Some(info) => info,
                        None => {
                            warn!("[net] trying to send message to unknown peer {}", to_id);
                            continue;
                        }
                    };

                    if writer_opt.is_none() {
                        match TcpStream::connect(&*addr) {
                            Ok(stream) => {
                                info!("[net] connected to {} ({})", addr, to_id);
                                *writer_opt = Some(BufWriter::new(stream));
                            }
                            Err(_) => {
                                warn!(
                                    "[net] failed to connect to {} ({}); will retry on next message",
                                    addr, to_id
                                );
                                continue;
                            }
                        }
                    }

                    if let Some(w) = writer_opt {
                        let bytes = msg.write_to_bytes().unwrap();
                        if w.write_u32::<BigEndian>(bytes.len() as u32).is_err()
                            || w.write_all(&bytes).is_err()
                            || w.flush().is_err()
                        {
                            warn!(
                                "[net] failed to send message to {}; connection broken. will reconnect.",
                                to_id
                            );
                            *writer_opt = None;
                        }
                    }
                }
            });
        }

        let mut raft_node = RawNode::new(&raft_cfg, storage, &logger).unwrap();

        let signer_state = Arc::new(RwLock::new(initial_sm_state));
        let sm_clone = Arc::clone(&signer_state);
        let state_clone = Arc::clone(&raft_state);

        thread::spawn(move || {
            let mut last_tick = Instant::now();
            let mut timeout = Duration::from_millis(100);

            let mut proposal_callbacks: VecDeque<Sender<Result<(), SignerError>>> = VecDeque::new();

            loop {
                match in_rx.recv_timeout(timeout) {
                    Ok(RaftMessage::Propose(data, callback)) => {
                        let _ = raft_node.propose(vec![], data.to_bytes());
                        proposal_callbacks.push_back(callback);
                    }
                    Ok(RaftMessage::Msg(m)) => {
                        let _ = raft_node.step(m);
                    }
                    Ok(RaftMessage::TransferLeadership(transferee_id)) => {
                        raft_node.transfer_leader(transferee_id);
                    }
                    Err(RecvTimeoutError::Timeout) => {}
                    Err(RecvTimeoutError::Disconnected) => break,
                }

                let elapsed = last_tick.elapsed();
                if elapsed >= timeout {
                    raft_node.tick();
                    last_tick = Instant::now();
                    timeout = Duration::from_millis(100);
                } else {
                    timeout -= elapsed;
                }

                on_ready(
                    &mut raft_node,
                    &sm_clone,
                    &out_tx,
                    &state_clone,
                    &mut proposal_callbacks,
                );
            }
        });

        Arc::new(SignerRaftNode {
            signer_state: signer_state,
            proposal_sender: in_tx,
            raft_state,
            node_id: config.node_id,
        })
    }

    pub fn replicate_state(&self, new_state: ConsensusData) -> Result<(), SignerError> {
        info!("[api] replicating state: {:?}", new_state);
        if !self.is_leader() {
            return Err(SignerError::Other(
                "This node is not the leader".to_string(),
            ));
        }

        let (tx, rx) = mpsc::channel();
        self.proposal_sender
            .send(RaftMessage::Propose(new_state, tx))
            .map_err(|e| {
                SignerError::Other(format!("Failed to send proposal to raft thread: {}", e))
            })?;

        match rx.recv_timeout(Duration::from_secs(5)) {
            Ok(Ok(())) => {
                info!(
                    "[api] replication successful, propagated state: {}",
                    new_state
                );
                Ok(())
            }
            Ok(Err(e)) => {
                warn!("[api] replication failed: {:?}", e);
                Err(e)
            }
            Err(_) => {
                warn!("[api] replication timed out");
                Err(SignerError::Other(
                    "State replication timed out".to_string(),
                ))
            }
        }
    }

    pub fn is_leader(&self) -> bool {
        self.raft_state.read().unwrap().0 == StateRole::Leader
    }

    pub fn leader_id(&self) -> Option<u64> {
        let state = self.raft_state.read().unwrap();
        if state.1 == 0 { None } else { Some(state.1) }
    }

    #[allow(dead_code)] // TODO
    pub fn node_id(&self) -> u64 {
        self.node_id
    }

    #[allow(dead_code)] // TODO
    pub fn transfer_leadership(&self, transferee_id: u64) -> Result<(), SignerError> {
        info!("[api] transferring leadership to node {}", transferee_id);
        if !self.is_leader() {
            return Err(SignerError::Other(
                "This node is not the leader, cannot transfer leadership".to_string(),
            ));
        }

        self.proposal_sender
            .send(RaftMessage::TransferLeadership(transferee_id))
            .map_err(|e| {
                SignerError::Other(format!(
                    "Failed to send leadership transfer request to raft thread: {}",
                    e
                ))
            })
    }
}

fn on_ready(
    raft_group: &mut RawNode<RaftStorage>,
    signer_state: &Arc<RwLock<ConsensusData>>,
    net_tx: &Sender<RaftProtoMessage>,
    raft_state: &Arc<RwLock<(StateRole, u64)>>,
    proposal_callbacks: &mut VecDeque<Sender<Result<(), SignerError>>>,
) {
    if !raft_group.has_ready() {
        return;
    }

    let mut ready = raft_group.ready();

    if let Some(ss) = ready.ss() {
        let was_leader = raft_state.read().unwrap().0 == StateRole::Leader;
        let is_leader = ss.raft_state == StateRole::Leader;

        if was_leader && !is_leader {
            warn!(
                "[on_ready] leadership lost, failing {} pending proposals",
                proposal_callbacks.len()
            );
            for callback in proposal_callbacks.drain(..) {
                let _ = callback.send(Err(SignerError::Other(
                    "Lost leadership during replication".into(),
                )));
            }
        }

        let mut state = raft_state.write().unwrap();
        state.0 = ss.raft_state;
        state.1 = ss.leader_id;
    }

    for msg in ready.take_messages() {
        let _ = net_tx.send(msg);
    }

    if !ready.snapshot().is_empty() {
        let snap = ready.snapshot().clone();
        raft_group.mut_store().apply_snapshot(snap.clone()).unwrap();

        if let Some(sm_data) = ConsensusData::from_bytes(snap.get_data()) {
            info!(
                "[on_ready] loaded state machine from snapshot: {:?}",
                sm_data
            );
            *signer_state.write().unwrap() = sm_data;
        }
    }

    if !ready.entries().is_empty() {
        raft_group
            .mut_store()
            .append_entries(ready.entries())
            .unwrap();
    }

    if let Some(hs) = ready.hs() {
        raft_group.mut_store().set_hard_state(hs.clone()).unwrap();
    }

    for msg in ready.take_persisted_messages() {
        let _ = net_tx.send(msg);
    }

    if !ready.committed_entries().is_empty() {
        handle_committed_entries(
            raft_group,
            ready.take_committed_entries(),
            signer_state,
            proposal_callbacks,
        );
    }

    let mut light_rd = raft_group.advance(ready);

    for msg in light_rd.take_messages() {
        let _ = net_tx.send(msg);
    }

    if !light_rd.committed_entries().is_empty() {
        handle_committed_entries(
            raft_group,
            light_rd.take_committed_entries(),
            signer_state,
            proposal_callbacks,
        );
    }

    raft_group.advance_apply();
}

fn handle_committed_entries(
    raft_group: &mut RawNode<RaftStorage>,
    committed_entries: Vec<raft_proto::eraftpb::Entry>,
    signer_state: &Arc<RwLock<ConsensusData>>,
    proposal_callbacks: &mut VecDeque<Sender<Result<(), SignerError>>>,
) {
    for ent in committed_entries {
        match ent.get_entry_type() {
            EntryType::EntryNormal => {
                if !ent.get_data().is_empty() {
                    if let Some(ns) = ConsensusData::from_bytes(ent.get_data()) {
                        info!("[on_ready] applying normal entry: {}", ns);
                        *signer_state.write().unwrap() = ns;
                        raft_group.mut_store().write_signer_state(&ns).unwrap();

                        if let Some(callback) = proposal_callbacks.pop_front() {
                            if let Err(e) = callback.send(Ok(())) {
                                warn!("[on_ready] failed to send commit confirmation: {:?}", e);
                            }
                        }
                    }
                }
            }
            EntryType::EntryConfChange => {
                info!("[on_ready] applying conf change entry");
                let cc: raft::prelude::ConfChange =
                    protobuf::Message::parse_from_bytes(ent.get_data()).unwrap();
                let cs = raft_group.apply_conf_change(&cc).unwrap();
                raft_group.mut_store().set_conf_state(cs).unwrap();
            }
            EntryType::EntryConfChangeV2 => {
                warn!("unhandled EntryConfChangeV2");
            }
        }
    }
}

#[cfg(test)]
mod integration_tests;

#[cfg(test)]
mod tests;
