mod storage;

use crate::cluster::storage::RocksDBStorage;
use crate::config::RaftConfig;
use crate::error::SignerError;
use crate::types::ConsensusData;
use log::{info, warn};
use protobuf::Message as ProtobufMessage;
use raft::prelude::{ConfState, EntryType, Message as RaftProtoMessage, Snapshot};
use raft::{Config as RaftCoreConfig, RawNode, StateRole, Storage};
use slog::{Drain, o};
use std::collections::HashMap;
use std::io::{BufReader, BufWriter, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::atomic::{AtomicBool, Ordering as AtomicOrdering};
use std::sync::mpsc::{self, RecvTimeoutError, Sender};
use std::sync::{Arc, RwLock};
use std::thread;
use std::time::{Duration, Instant};

use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};

enum RaftMessage {
    Propose(ConsensusData, Sender<Result<(), SignerError>>),
    Msg(RaftProtoMessage),
    TransferLeadership(u64),
    #[cfg(test)]
    Shutdown,
}

type ProposalCallbacks = HashMap<u64, Sender<Result<(), SignerError>>>;
const MAX_RAFT_MESSAGE_SIZE: u32 = 16 * 1024 * 1024;

pub enum RaftEvent {
    LeadershipChanged(u64, u64),
    StateApplied { node_id: u64, data: ConsensusData },
}

pub struct SignerRaftNode {
    node_id: u64,
    pub signer_state: Arc<RwLock<ConsensusData>>,
    proposal_sender: Sender<RaftMessage>,
    raft_state: Arc<RwLock<(StateRole, u64)>>,
    signing_ready: Arc<RwLock<bool>>,
    #[cfg(test)]
    shutdown_handle: Arc<RwLock<Option<thread::JoinHandle<()>>>>,
    #[cfg(test)]
    transport_shutdown: Arc<AtomicBool>,
    #[cfg(test)]
    transport_handles: Arc<RwLock<Vec<thread::JoinHandle<()>>>>,
}

impl SignerRaftNode {
    #[cfg(test)]
    pub fn shutdown(&self) -> Result<(), SignerError> {
        info!("Shutting down node {}", self.node_id);

        let had_raft_thread = self.shutdown_handle.read().unwrap().is_some();
        if had_raft_thread {
            let _ = self.proposal_sender.send(RaftMessage::Shutdown);
        }

        if let Some(handle) = self.shutdown_handle.write().unwrap().take() {
            handle
                .join()
                .map_err(|_| SignerError::Other("Failed to join raft thread".to_string()))?;
        }

        self.transport_shutdown.store(true, AtomicOrdering::Release);
        for handle in self.transport_handles.write().unwrap().drain(..) {
            handle
                .join()
                .map_err(|_| SignerError::Other("Failed to join Raft transport thread".into()))?;
        }

        Ok(())
    }

    pub fn new(config: RaftConfig, events_tx: mpsc::Sender<RaftEvent>) -> Self {
        let logger = stdlog_to_slog();
        let storage = create_storage(&config);
        let signer_state = Arc::new(RwLock::new(storage.read_signer_state().unwrap()));

        let (in_tx, in_rx) = mpsc::channel::<RaftMessage>();
        let (out_tx, out_rx) = mpsc::channel::<RaftProtoMessage>();
        let raft_state = Arc::new(RwLock::new((StateRole::Follower, 0)));
        let signing_ready = Arc::new(RwLock::new(false));
        let transport_shutdown = Arc::new(AtomicBool::new(false));

        let inbound_handle = start_inbound_handler(
            config.bind_addr.clone(),
            in_tx.clone(),
            config.node_id,
            config.peers.iter().map(|peer| peer.id).collect(),
            Arc::clone(&transport_shutdown),
        );
        let outbound_handle = start_outbound_handler(out_rx, config.peers.clone(), config.node_id);

        #[cfg(not(test))]
        {
            drop(inbound_handle);
            drop(outbound_handle);
        }

        let _handle = start_raft_thread(RaftThreadConfig {
            node_id: config.node_id,
            storage,
            logger,
            in_rx,
            out_tx,
            events_tx,
            signer_state: Arc::clone(&signer_state),
            raft_state: Arc::clone(&raft_state),
            signing_ready: Arc::clone(&signing_ready),
        });

        SignerRaftNode {
            signer_state,
            proposal_sender: in_tx,
            raft_state,
            signing_ready,
            node_id: config.node_id,
            #[cfg(test)]
            shutdown_handle: Arc::new(RwLock::new(Some(_handle))),
            #[cfg(test)]
            transport_shutdown,
            #[cfg(test)]
            transport_handles: Arc::new(RwLock::new(vec![inbound_handle, outbound_handle])),
        }
    }

    pub fn replicate_state(&self, new_state: &ConsensusData) -> Result<(), SignerError> {
        let leader = self
            .leader_id()
            .map(|id| id.to_string())
            .unwrap_or_else(|| "none".to_string());
        info!("replicating state: {}, leader_id: {}", new_state, leader,);
        if !self.is_leader() {
            return Err(SignerError::NotLeader(format!(
                "node {}, current leader {}",
                self.node_id, leader
            )));
        }

        let (tx, rx) = mpsc::channel();
        self.proposal_sender
            .send(RaftMessage::Propose(new_state.clone(), tx))
            .map_err(|e| {
                SignerError::Other(format!("Failed to send proposal to raft thread: {}", e))
            })?;

        match rx.recv_timeout(Duration::from_secs(5)) {
            Ok(Ok(())) => {
                info!("replication successful, propagated state: {}", new_state);
                Ok(())
            }
            Ok(Err(e)) => {
                warn!("replication failed: {:?}", e);
                Err(e)
            }
            Err(_) => {
                warn!("replication timed out");
                Err(SignerError::StateReplication(
                    "State replication timed out".to_string(),
                ))
            }
        }
    }

    pub fn is_leader(&self) -> bool {
        self.raft_state.read().unwrap().0 == StateRole::Leader
            && *self.signing_ready.read().unwrap()
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
        info!("transferring leadership to node {}", transferee_id);
        if !self.is_leader() {
            return Err(SignerError::NotLeader(
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

#[cfg(test)]
impl Drop for SignerRaftNode {
    fn drop(&mut self) {
        let _ = self.shutdown();
    }
}

fn stdlog_to_slog() -> slog::Logger {
    let drain = slog_stdlog::StdLog.fuse();
    let drain = slog_async::Async::new(drain)
        .chan_size(4096)
        .overflow_strategy(slog_async::OverflowStrategy::Block)
        .build()
        .fuse();
    slog::Logger::root(drain, o!())
}

fn create_storage(config: &RaftConfig) -> RocksDBStorage {
    info!("storage path: {}", config.data_path);
    let mut storage = RocksDBStorage::new(&config.data_path);

    let peer_ids: Vec<u64> = config.peers.iter().map(|p| p.id).collect();
    let init_state = storage.initial_state().unwrap();

    if init_state.hard_state.commit == 0 && init_state.hard_state.term == 0 {
        info!("fresh store, bootstrapping with peers: {:?}", peer_ids);
        bootstrap_storage(&mut storage, peer_ids, &config.initial_state_path);
    } else {
        info!("found existing state, loading from DB");
    }

    validate_storage(&storage);

    storage
}

fn validate_storage(storage: &RocksDBStorage) {
    let raft_state = storage
        .initial_state()
        .expect("invalid persisted Raft hard/configuration state");
    let signer_state = storage
        .read_signer_state()
        .expect("invalid or missing persisted signer state");
    let applied = storage
        .applied_index()
        .expect("invalid or missing persisted applied index");
    let first = storage
        .first_index()
        .expect("invalid or missing persisted Raft truncation metadata");
    let last = storage
        .last_index()
        .expect("invalid or missing persisted Raft last index");
    let boundary = first.saturating_sub(1);
    storage
        .term(boundary)
        .expect("invalid persisted Raft truncation boundary term");

    assert!(
        boundary <= applied && applied <= last,
        "persisted applied index {applied} is outside Raft log bounds {boundary}..={last}"
    );
    assert!(
        raft_state.hard_state.get_commit() <= last,
        "persisted commit index {} exceeds last log index {last}",
        raft_state.hard_state.get_commit()
    );
    info!("validated persisted signer state: {}", signer_state);
}

fn bootstrap_storage(storage: &mut RocksDBStorage, peer_ids: Vec<u64>, initial_state_path: &str) {
    let state_file_path = std::path::Path::new(initial_state_path);
    let initial_state = match std::fs::read(state_file_path) {
        Ok(bytes) => {
            let bootstrap_state = ConsensusData::from_bytes(&bytes).unwrap_or_else(|| {
                panic!(
                    "bootstrap signer state at {} is malformed or incomplete",
                    state_file_path.display()
                )
            });
            info!(
                "loaded bootstrap state from state.json: {}",
                bootstrap_state
            );
            bootstrap_state
        }
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            info!(
                "no state file found at {}, using default state.",
                initial_state_path
            );
            ConsensusData::default()
        }
        Err(error) => panic!(
            "failed to read bootstrap signer state at {}: {}",
            state_file_path.display(),
            error
        ),
    };

    let mut snap = Snapshot::default();
    snap.mut_metadata().set_index(1);
    snap.mut_metadata().set_term(1);
    let mut cs = ConfState::default();
    cs.set_voters(peer_ids);
    snap.mut_metadata().set_conf_state(cs);
    snap.set_data(initial_state.to_bytes().into());
    storage.apply_snapshot(snap).unwrap();
}

fn start_inbound_handler(
    bind_addr: String,
    in_tx: Sender<RaftMessage>,
    node_id: u64,
    peer_ids: Vec<u64>,
    shutdown: Arc<AtomicBool>,
) -> thread::JoinHandle<()> {
    thread::spawn(move || {
        let listener = TcpListener::bind(&bind_addr)
            .unwrap_or_else(|_| panic!("bind failed on {}", bind_addr));
        listener
            .set_nonblocking(true)
            .expect("failed to make Raft listener nonblocking");
        info!("listening on {}", bind_addr);
        #[cfg(test)]
        let mut connection_handles = Vec::new();
        while !shutdown.load(AtomicOrdering::Acquire) {
            match listener.accept() {
                Ok((stream, _)) => {
                    if let Err(error) = stream.set_nonblocking(false) {
                        warn!("failed to make accepted Raft stream blocking: {}", error);
                        continue;
                    }
                    #[cfg(test)]
                    stream
                        .set_read_timeout(Some(Duration::from_millis(250)))
                        .expect("failed to set test Raft stream timeout");
                    let in_tx = in_tx.clone();
                    let peer_ids = peer_ids.clone();
                    let connection_shutdown = Arc::clone(&shutdown);
                    let connection_handle = thread::spawn(move || {
                        let mut reader = BufReader::new(stream);
                        loop {
                            if connection_shutdown.load(AtomicOrdering::Acquire) {
                                break;
                            }
                            let len = match reader.read_u32::<BigEndian>() {
                                Ok(len) => len,
                                #[cfg(test)]
                                Err(error)
                                    if matches!(
                                        error.kind(),
                                        std::io::ErrorKind::WouldBlock
                                            | std::io::ErrorKind::TimedOut
                                    ) =>
                                {
                                    continue;
                                }
                                Err(_) => break,
                            };
                            if len > MAX_RAFT_MESSAGE_SIZE {
                                warn!(
                                    "closing Raft connection after oversized frame: {} bytes",
                                    len
                                );
                                break;
                            }
                            let mut buf = vec![0; len as usize];
                            if reader.read_exact(&mut buf).is_err() {
                                break;
                            }
                            match RaftProtoMessage::parse_from_bytes(&buf) {
                                Ok(msg) => {
                                    if let Err(reason) =
                                        validate_network_message(&msg, node_id, &peer_ids)
                                    {
                                        warn!("rejected inbound Raft message: {}", reason);
                                        continue;
                                    }
                                    if in_tx.send(RaftMessage::Msg(msg)).is_err() {
                                        break;
                                    }
                                }
                                Err(e) => warn!("parse error: {:?}", e),
                            }
                        }
                    });
                    #[cfg(test)]
                    connection_handles.push(connection_handle);
                    #[cfg(not(test))]
                    drop(connection_handle);
                }
                Err(error) if error.kind() == std::io::ErrorKind::WouldBlock => {
                    thread::sleep(Duration::from_millis(10));
                }
                Err(error) => {
                    warn!("Raft listener accept failed: {}", error);
                    thread::sleep(Duration::from_millis(50));
                }
            }
        }
        #[cfg(test)]
        for handle in connection_handles {
            let _ = handle.join();
        }
    })
}

fn validate_network_message(
    message: &RaftProtoMessage,
    node_id: u64,
    peer_ids: &[u64],
) -> Result<(), String> {
    // This rejects accidental/cross-cluster traffic, but it is not authentication: a client can
    // still forge a configured `from` ID. Multi-node deployments must protect this transport with
    // an authenticated private network until Nebula supports authenticated Raft framing itself.
    if message.get_msg_type() == raft::prelude::MessageType::MsgPropose {
        return Err("MsgPropose is local-only in Nebula".into());
    }
    if message.get_to() != node_id {
        return Err(format!(
            "message addressed to node {}, received by node {node_id}",
            message.get_to()
        ));
    }
    if message.get_from() == node_id || !peer_ids.contains(&message.get_from()) {
        return Err(format!("unknown peer id {}", message.get_from()));
    }
    Ok(())
}

fn start_outbound_handler(
    out_rx: mpsc::Receiver<RaftProtoMessage>,
    peers: Vec<crate::config::PeerConfig>,
    node_id: u64,
) -> thread::JoinHandle<()> {
    thread::spawn(move || {
        let mut peer_writers: HashMap<u64, (String, Option<BufWriter<TcpStream>>)> = peers
            .into_iter()
            .filter(|p| p.id != node_id)
            .map(|p| (p.id, (p.addr, None)))
            .collect();

        while let Ok(msg) = out_rx.recv() {
            let to_id = msg.to;

            let (addr, writer_opt) = match peer_writers.get_mut(&to_id) {
                Some(info) => info,
                None => {
                    warn!("trying to send message to unknown peer {}", to_id,);
                    continue;
                }
            };

            if writer_opt.is_none() {
                match TcpStream::connect(&*addr) {
                    Ok(stream) => {
                        // stream
                        //     .set_write_timeout(Some(Duration::from_secs(1)))
                        //     .expect("failed to set write timeout on raft stream");
                        info!("connected to {} ({})", addr, to_id);
                        *writer_opt = Some(BufWriter::new(stream));
                    }
                    Err(_) => {
                        warn!(
                            "failed to connect to {} ({}); will retry on next message",
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
                        "failed to send message to {}; connection broken. will reconnect.",
                        to_id
                    );
                    *writer_opt = None;
                }
            }
        }
    })
}

struct RaftThreadConfig {
    node_id: u64,
    storage: RocksDBStorage,
    logger: slog::Logger,
    in_rx: mpsc::Receiver<RaftMessage>,
    out_tx: Sender<RaftProtoMessage>,
    events_tx: Sender<RaftEvent>,
    signer_state: Arc<RwLock<ConsensusData>>,
    raft_state: Arc<RwLock<(StateRole, u64)>>,
    signing_ready: Arc<RwLock<bool>>,
}

fn start_raft_thread(cfg: RaftThreadConfig) -> thread::JoinHandle<()> {
    thread::spawn(move || {
        let RaftThreadConfig {
            node_id,
            storage,
            logger,
            in_rx,
            out_tx,
            events_tx,
            signer_state,
            raft_state,
            signing_ready,
        } = cfg;

        // The highest log position that is known to be in stable storage
        // on a quorum of nodes.
        //
        // Invariant: applied <= committed
        // pub committed: u64,

        // The highest log position that is known to be persisted in stable
        // storage. It's used for limiting the upper bound of committed and
        // persisted entries.
        //
        // Invariant: persisted < unstable.offset && applied <= persisted
        // pub persisted: u64,

        // The highest log position that the application has been instructed
        // to apply to its state machine.
        //
        // Invariant: applied <= min(committed, persisted)
        // pub applied: u64,
        let initial_state = storage.initial_state().unwrap();
        let persisted_commit = initial_state.hard_state.get_commit();
        let persisted_applied = storage.applied_index().unwrap();
        let applied_index = persisted_applied.min(persisted_commit);
        if persisted_applied > persisted_commit {
            warn!(
                "persisted applied index {} is ahead of persisted commit {}; clamping restart applied to {}",
                persisted_applied, persisted_commit, applied_index
            );
        }

        let raft_cfg = RaftCoreConfig {
            id: node_id,
            election_tick: 10,
            check_quorum: true,
            pre_vote: true,
            heartbeat_tick: 3,
            applied: applied_index,
            ..Default::default()
        };
        raft_cfg.validate().unwrap();

        let mut raft_node = RawNode::new(&raft_cfg, storage, &logger).unwrap();
        let mut last_tick = Instant::now();
        let mut timeout = Duration::from_millis(100);
        let mut proposal_callbacks = ProposalCallbacks::new();

        loop {
            match in_rx.recv_timeout(timeout) {
                Ok(RaftMessage::Propose(data, callback)) => {
                    handle_local_proposal(
                        &mut raft_node,
                        &signing_ready,
                        data,
                        callback,
                        &mut proposal_callbacks,
                    );
                }
                Ok(RaftMessage::Msg(m)) => {
                    let _ = raft_node.step(m);
                }
                Ok(RaftMessage::TransferLeadership(transferee_id)) => {
                    raft_node.transfer_leader(transferee_id);
                }
                #[cfg(test)]
                Ok(RaftMessage::Shutdown) => {
                    info!("Raft thread received shutdown signal");
                    for (_, callback) in proposal_callbacks.drain() {
                        let _ = callback
                            .send(Err(SignerError::Other("Node shutting down".to_string())));
                    }
                    break;
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
            let events = on_ready(
                &mut raft_node,
                &signer_state,
                &out_tx,
                &raft_state,
                &signing_ready,
                &mut proposal_callbacks,
            );
            for event in events {
                let _ = events_tx.send(event);
            }
        }
    })
}

fn handle_local_proposal(
    raft_node: &mut RawNode<RocksDBStorage>,
    signing_ready: &Arc<RwLock<bool>>,
    data: ConsensusData,
    callback: Sender<Result<(), SignerError>>,
    proposal_callbacks: &mut ProposalCallbacks,
) {
    if raft_node.raft.state != StateRole::Leader || !*signing_ready.read().unwrap() {
        let _ = callback.send(Err(SignerError::NotLeader(
            "Raft leader has not applied its current-term barrier".into(),
        )));
        return;
    }

    let previous_last_index = raft_node.raft.raft_log.last_index();
    if let Err(error) = raft_node.propose(vec![], data.to_bytes()) {
        let _ = callback.send(Err(SignerError::StateReplication(format!(
            "Raft rejected proposal: {error}"
        ))));
        return;
    }

    let proposal_index = raft_node.raft.raft_log.last_index();
    if previous_last_index.checked_add(1) != Some(proposal_index) {
        let _ = callback.send(Err(SignerError::NotLeader(
            "Raft did not append the proposal locally".into(),
        )));
        return;
    }

    if let Some(replaced) = proposal_callbacks.insert(proposal_index, callback) {
        let _ = replaced.send(Err(SignerError::StateReplication(format!(
            "duplicate local proposal index {proposal_index}"
        ))));
    }
}

fn on_ready(
    raft_group: &mut RawNode<RocksDBStorage>,
    signer_state: &Arc<RwLock<ConsensusData>>,
    net_tx: &Sender<RaftProtoMessage>,
    raft_state: &Arc<RwLock<(StateRole, u64)>>,
    signing_ready: &Arc<RwLock<bool>>,
    proposal_callbacks: &mut ProposalCallbacks,
) -> Vec<RaftEvent> {
    let mut events: Vec<RaftEvent> = vec![];
    if !raft_group.has_ready() {
        return events;
    }

    let mut ready = raft_group.ready();

    if let Some(ss) = ready.ss() {
        let state = raft_state.read().unwrap();
        let was_leader = state.0 == StateRole::Leader;
        let old_leader_id = state.1;
        let is_leader = ss.raft_state == StateRole::Leader;
        drop(state);

        if was_leader && !is_leader {
            warn!(
                "leadership lost, failing {} pending proposals",
                proposal_callbacks.len()
            );
            *signing_ready.write().unwrap() = false;
            events.push(RaftEvent::LeadershipChanged(old_leader_id, ss.leader_id));
            for (_, callback) in proposal_callbacks.drain() {
                let _ = callback.send(Err(SignerError::NotLeader(
                    "Lost leadership during replication".into(),
                )));
            }
        }

        let mut state = raft_state.write().unwrap();
        state.0 = ss.raft_state;
        state.1 = ss.leader_id;

        if !is_leader {
            *signing_ready.write().unwrap() = false;
        }
    }

    for msg in ready.take_messages() {
        let _ = net_tx.send(msg);
    }

    if !ready.snapshot().is_empty() {
        let snap = ready.snapshot().clone();
        raft_group.mut_store().apply_snapshot(snap).unwrap();
        let persisted = raft_group
            .store()
            .read_signer_state()
            .expect("snapshot signer state was not durably installed");
        info!(
            "loaded normalized state machine from snapshot: {}",
            persisted
        );
        *signer_state.write().unwrap() = persisted;
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
            &mut events,
        );
    }

    let mut light_rd = raft_group.advance(ready);

    if let Some(commit_index) = light_rd.commit_index() {
        info!("updating commit index");
        raft_group
            .mut_store()
            .set_commit_index(commit_index)
            .unwrap();
    }

    for msg in light_rd.take_messages() {
        let _ = net_tx.send(msg);
    }

    if !light_rd.committed_entries().is_empty() {
        handle_committed_entries(
            raft_group,
            light_rd.take_committed_entries(),
            signer_state,
            proposal_callbacks,
            &mut events,
        );
    }

    raft_group.advance_apply();

    let mut ready_for_signing = signing_ready.write().unwrap();
    if raft_group.raft.state == StateRole::Leader && !*ready_for_signing {
        let applied = raft_group.store().applied_index().unwrap();
        if applied > 0 && raft_group.store().term(applied).ok() == Some(raft_group.raft.term) {
            info!(
                "leader {} is signing-ready after applying current-term entry {}",
                raft_group.raft.id, applied
            );
            *ready_for_signing = true;
            events.push(RaftEvent::LeadershipChanged(0, raft_group.raft.id));
        }
    }
    events
}

fn handle_committed_entries(
    raft_group: &mut RawNode<RocksDBStorage>,
    committed_entries: Vec<raft_proto::eraftpb::Entry>,
    signer_state: &Arc<RwLock<ConsensusData>>,
    proposal_callbacks: &mut ProposalCallbacks,
    events: &mut Vec<RaftEvent>,
) {
    let mut last_applied = None;
    for ent in committed_entries {
        last_applied = Some(ent.get_index());
        match ent.get_entry_type() {
            EntryType::EntryNormal => {
                if ent.get_data().is_empty() {
                    let current = signer_state.read().unwrap().clone();
                    raft_group
                        .mut_store()
                        .write_signer_state_and_applied(&current, ent.get_index())
                        .expect("failed to durably apply Raft barrier entry");
                } else {
                    let result = ConsensusData::from_bytes(ent.get_data())
                        .ok_or_else(|| {
                            SignerError::StateReplication(format!(
                                "malformed consensus record at Raft index {}",
                                ent.get_index()
                            ))
                        })
                        .and_then(|ns| {
                            let applied = apply_consensus_record(
                                raft_group,
                                signer_state,
                                ns,
                                ent.get_index(),
                            )?;
                            events.push(RaftEvent::StateApplied {
                                node_id: raft_group.raft.id,
                                data: applied,
                            });
                            Ok(())
                        });

                    if result.is_err() {
                        let current = signer_state.read().unwrap().clone();
                        raft_group
                            .mut_store()
                            .write_signer_state_and_applied(&current, ent.get_index())
                            .expect("failed to durably reject unsafe Raft command");
                    }

                    if let Some(callback) = proposal_callbacks.remove(&ent.get_index()) {
                        if let Err(e) = callback.send(result) {
                            warn!("failed to send commit confirmation: {:?}", e);
                        }
                    } else if let Err(error) = result {
                        warn!(
                            "rejected committed consensus record at index {}: {}",
                            ent.get_index(),
                            error
                        );
                    }
                }
            }
            EntryType::EntryConfChange => {
                info!("applying conf change entry");
                let cc: raft::prelude::ConfChange =
                    protobuf::Message::parse_from_bytes(ent.get_data()).unwrap();
                let cs = raft_group.apply_conf_change(&cc).unwrap();
                raft_group.mut_store().set_conf_state(cs).unwrap();
                let current = signer_state.read().unwrap().clone();
                raft_group
                    .mut_store()
                    .write_signer_state_and_applied(&current, ent.get_index())
                    .expect("failed to durably apply Raft configuration entry");
            }
            EntryType::EntryConfChangeV2 => {
                panic!("unhandled committed EntryConfChangeV2");
            }
        }
    }

    debug_assert_eq!(
        last_applied,
        raft_group.store().applied_index().ok(),
        "every committed entry must atomically advance the durable applied index"
    );
}

// TODO: more graceful errors than unwrap
fn apply_consensus_record(
    raft_group: &mut RawNode<RocksDBStorage>,
    signer_state: &Arc<RwLock<ConsensusData>>,
    next: ConsensusData,
    applied_index: u64,
) -> Result<ConsensusData, SignerError> {
    let current = signer_state.read().unwrap().clone();

    info!(
        "applying normal entry: {}, current node state: {}, node_id: {}",
        next, current, raft_group.raft.id,
    );
    let persisted = current.validated_update(&next)?;

    raft_group
        .mut_store()
        .write_signer_state_and_applied(&persisted, applied_index)
        .expect("failed to durably apply signer state");
    *signer_state.write().unwrap() = persisted.clone();
    Ok(persisted)
}

#[cfg(test)]
mod integration_tests;

#[cfg(test)]
mod tests;

// #[cfg(test)]
// mod partition_tests;
