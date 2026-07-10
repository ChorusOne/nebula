use super::*;
use crate::config::{PeerConfig, RaftConfig};
use crate::types::{ConsensusData, SignedMsgType};
use raft::prelude::{Entry, EntryType, Message as RaftProtoMessage, MessageType};
use raft::{Config as RaftCoreConfig, RawNode, StateRole};
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};
use tempfile::TempDir;

fn create_test_config(
    port_prefix: u64,
    node_id: u64,
    temp_dir: &TempDir,
    peers: Vec<PeerConfig>,
) -> RaftConfig {
    RaftConfig {
        node_id,
        bind_addr: format!("127.0.0.1:{}", port_prefix + node_id),
        data_path: temp_dir
            .path()
            .join(format!("node_{}", node_id))
            .to_str()
            .unwrap()
            .to_string(),
        peers,
        initial_state_path: "./test_consensus_state.json".to_string(),
    }
}

fn wait_for_leader(clusters: &[Arc<SignerRaftNode>], timeout: Duration) -> Option<u64> {
    let start = Instant::now();
    while start.elapsed() < timeout {
        for cluster in clusters {
            if cluster.is_leader() {
                return Some(cluster.leader_id().unwrap());
            }
        }
        thread::sleep(Duration::from_millis(100));
    }
    None
}

fn get_leader_cluster(clusters: &[Arc<SignerRaftNode>]) -> Option<Arc<SignerRaftNode>> {
    for cluster in clusters {
        if cluster.is_leader() {
            return Some(Arc::clone(cluster));
        }
    }
    None
}

#[test]
fn single_node_cluster() {
    let temp_dir = TempDir::new().unwrap();
    let peers = vec![PeerConfig {
        id: 1,
        addr: "127.0.0.1:8001".to_string(),
    }];
    let config = create_test_config(8000, 1, &temp_dir, peers);

    let (events_tx, _events_rx) = mpsc::channel();
    let cluster = Arc::new(SignerRaftNode::new(config, events_tx));

    let leader_id = wait_for_leader(&[Arc::clone(&cluster)], Duration::from_secs(5));
    assert_eq!(leader_id, Some(1));
    assert!(cluster.is_leader());

    let new_state = ConsensusData {
        height: 100,
        round: 1,
        step: SignedMsgType::Proposal,
        ..Default::default()
    };

    let result = cluster.replicate_state(&new_state);
    assert!(result.is_ok());

    let current_state = cluster.signer_state.read().unwrap().clone();
    assert_eq!(current_state, new_state);
}

#[test]
fn three_node_cluster_basic() {
    let temp_dir = TempDir::new().unwrap();
    let peers = vec![
        PeerConfig {
            id: 1,
            addr: "127.0.0.1:9001".to_string(),
        },
        PeerConfig {
            id: 2,
            addr: "127.0.0.1:9002".to_string(),
        },
        PeerConfig {
            id: 3,
            addr: "127.0.0.1:9003".to_string(),
        },
    ];

    let (events_tx, _events_rx) = mpsc::channel();
    let cluster1 = Arc::new(SignerRaftNode::new(
        create_test_config(9000, 1, &temp_dir, peers.clone()),
        events_tx.clone(),
    ));
    let cluster2 = Arc::new(SignerRaftNode::new(
        create_test_config(9000, 2, &temp_dir, peers.clone()),
        events_tx.clone(),
    ));
    let cluster3 = Arc::new(SignerRaftNode::new(
        create_test_config(9000, 3, &temp_dir, peers.clone()),
        events_tx,
    ));

    let clusters = vec![cluster1, cluster2, cluster3];

    let leader_id = wait_for_leader(&clusters, Duration::from_secs(10));
    assert!(leader_id.is_some());

    let leader = get_leader_cluster(&clusters).unwrap();
    let new_state = ConsensusData {
        height: 200,
        round: 2,
        step: SignedMsgType::Prevote,
        ..Default::default()
    };

    let result = leader.replicate_state(&new_state);
    assert!(result.is_ok());

    thread::sleep(Duration::from_secs(2));

    for cluster in &clusters {
        let state = cluster.signer_state.read().unwrap().clone();
        assert_eq!(state, new_state);
    }
}

fn raw_test_node(temp_dir: &TempDir, voters: Vec<u64>) -> RawNode<RocksDBStorage> {
    let config = RaftConfig {
        node_id: 1,
        bind_addr: "127.0.0.1:0".to_string(),
        data_path: temp_dir
            .path()
            .join("raw-node")
            .to_string_lossy()
            .into_owned(),
        peers: voters
            .iter()
            .map(|id| PeerConfig {
                id: *id,
                addr: format!("127.0.0.1:{}", 20_000 + id),
            })
            .collect(),
        initial_state_path: "./non_existent_initial_state.json".to_string(),
    };
    let storage = create_storage(&config);
    let raft_config = RaftCoreConfig {
        id: 1,
        election_tick: 10,
        heartbeat_tick: 3,
        applied: 1,
        ..Default::default()
    };
    RawNode::new(&raft_config, storage, &stdlog_to_slog()).unwrap()
}

#[test]
fn new_leader_is_not_activated_before_current_term_entry_is_committed() {
    let temp_dir = TempDir::new().unwrap();
    let mut raft = raw_test_node(&temp_dir, vec![1, 2, 3]);
    let signer_state = Arc::new(RwLock::new(ConsensusData::default()));
    let raft_state = Arc::new(RwLock::new((StateRole::Follower, 0)));
    let signing_ready = Arc::new(RwLock::new(false));
    let (net_tx, _net_rx) = mpsc::channel();
    let mut callbacks = HashMap::new();

    raft.campaign().unwrap();
    let mut vote = RaftProtoMessage::default();
    vote.set_msg_type(MessageType::MsgRequestVoteResponse);
    vote.set_from(2);
    vote.set_to(1);
    vote.set_term(raft.raft.term);
    raft.step(vote).unwrap();
    assert!(raft.raft.state == StateRole::Leader);

    let election_events = on_ready(
        &mut raft,
        &signer_state,
        &net_tx,
        &raft_state,
        &signing_ready,
        &mut callbacks,
    );
    assert!(
        !election_events
            .iter()
            .any(|event| matches!(event, RaftEvent::LeadershipChanged(_, 1))),
        "a leader must not serve CometBFT until its current-term barrier is committed and applied"
    );

    let mut ack = RaftProtoMessage::default();
    ack.set_msg_type(MessageType::MsgAppendResponse);
    ack.set_from(2);
    ack.set_to(1);
    ack.set_term(raft.raft.term);
    ack.set_index(raft.raft.raft_log.last_index());
    raft.step(ack).unwrap();

    let ready_events = on_ready(
        &mut raft,
        &signer_state,
        &net_tx,
        &raft_state,
        &signing_ready,
        &mut callbacks,
    );
    assert!(
        ready_events
            .iter()
            .any(|event| matches!(event, RaftEvent::LeadershipChanged(_, 1))),
        "the leader should become externally active after its current-term barrier is applied"
    );
}

#[test]
fn unrelated_committed_entry_does_not_acknowledge_pending_proposal() {
    let temp_dir = TempDir::new().unwrap();
    let mut raft = raw_test_node(&temp_dir, vec![1]);
    let signer_state = Arc::new(RwLock::new(ConsensusData::default()));
    let (callback_tx, callback_rx) = mpsc::channel();
    let mut callbacks = HashMap::from([(3, callback_tx)]);
    let mut events = Vec::new();

    let pending = ConsensusData {
        height: 12,
        round: 0,
        step: SignedMsgType::Prevote,
        sign_data: b"pending block".to_vec(),
        ..Default::default()
    };
    let unrelated = ConsensusData {
        height: 11,
        round: 0,
        step: SignedMsgType::Prevote,
        sign_data: b"unrelated block".to_vec(),
        ..Default::default()
    };
    assert_ne!(pending, unrelated);

    let mut entry = Entry::default();
    entry.set_index(2);
    entry.set_term(2);
    entry.set_entry_type(EntryType::EntryNormal);
    entry.set_data(unrelated.to_bytes().into());
    handle_committed_entries(
        &mut raft,
        vec![entry],
        &signer_state,
        &mut callbacks,
        &mut events,
    );

    assert!(
        callback_rx.try_recv().is_err(),
        "committing unrelated state must not report that the pending proposal committed"
    );

    let mut pending_entry = Entry::default();
    pending_entry.set_index(3);
    pending_entry.set_term(2);
    pending_entry.set_entry_type(EntryType::EntryNormal);
    pending_entry.set_data(pending.to_bytes().into());
    handle_committed_entries(
        &mut raft,
        vec![pending_entry],
        &signer_state,
        &mut callbacks,
        &mut events,
    );
    assert!(
        callback_rx
            .recv_timeout(Duration::from_millis(50))
            .unwrap()
            .is_ok(),
        "the callback should succeed once its exact log index is durably applied"
    );
}

#[test]
fn local_proposal_is_rejected_after_stepdown_even_if_readiness_is_stale() {
    let temp_dir = TempDir::new().unwrap();
    let mut raft = raw_test_node(&temp_dir, vec![1]);
    assert_eq!(raft.raft.state, StateRole::Follower);

    let signing_ready = Arc::new(RwLock::new(true));
    let (callback_tx, callback_rx) = mpsc::channel();
    let mut callbacks = HashMap::new();
    let state = ConsensusData {
        height: 30,
        round: 0,
        step: SignedMsgType::Prevote,
        sign_data: b"block-a".to_vec(),
        ..Default::default()
    };

    handle_local_proposal(
        &mut raft,
        &signing_ready,
        state,
        callback_tx,
        &mut callbacks,
    );

    assert!(callbacks.is_empty());
    assert!(matches!(
        callback_rx.recv_timeout(Duration::from_millis(50)),
        Ok(Err(SignerError::NotLeader(_)))
    ));
    assert!(
        !raft.has_ready(),
        "a follower must not forward a local signing proposal to another leader"
    );
}

#[test]
fn network_rejects_direct_msg_propose_injection() {
    let mut message = RaftProtoMessage::default();
    message.set_msg_type(MessageType::MsgPropose);
    message.set_from(2);
    message.set_to(1);
    let mut entry = Entry::default();
    entry.set_data(
        ConsensusData {
            height: 100,
            round: 0,
            step: SignedMsgType::Precommit,
            sign_data: b"forged block".to_vec(),
            signature: b"forged signature".to_vec(),
            ..Default::default()
        }
        .to_bytes()
        .into(),
    );
    message.set_entries(vec![entry].into());

    assert!(validate_network_message(&message, 1, &[1, 2, 3]).is_err());
}

#[test]
fn network_rejects_unknown_senders_and_wrong_recipients() {
    let mut message = RaftProtoMessage::default();
    message.set_msg_type(MessageType::MsgHeartbeat);
    message.set_from(99);
    message.set_to(1);
    assert!(validate_network_message(&message, 1, &[1, 2, 3]).is_err());

    message.set_from(2);
    message.set_to(3);
    assert!(validate_network_message(&message, 1, &[1, 2, 3]).is_err());
}

#[test]
#[should_panic(expected = "bootstrap signer state")]
fn malformed_existing_bootstrap_state_fails_closed() {
    let temp_dir = TempDir::new().unwrap();
    let state_path = temp_dir.path().join("state.json");
    std::fs::write(&state_path, b"{}").unwrap();
    let mut storage = RocksDBStorage::new(temp_dir.path().join("raft-db"));

    bootstrap_storage(&mut storage, vec![1, 2, 3], state_path.to_str().unwrap());
}

#[test]
fn state_machine_rejects_conflicting_sign_bytes_at_the_same_hrs() {
    let temp_dir = TempDir::new().unwrap();
    let mut raft = raw_test_node(&temp_dir, vec![1]);
    let signer_state = Arc::new(RwLock::new(ConsensusData::default()));
    let signed_a = ConsensusData {
        height: 25,
        round: 3,
        step: SignedMsgType::Precommit,
        sign_data: b"block A".to_vec(),
        signature: b"signature A".to_vec(),
        ..Default::default()
    };
    let signed_b = ConsensusData {
        sign_data: b"block B".to_vec(),
        signature: b"signature B".to_vec(),
        ..signed_a.clone()
    };

    apply_consensus_record(&mut raft, &signer_state, signed_a.clone(), 2).unwrap();
    assert!(apply_consensus_record(&mut raft, &signer_state, signed_b, 3).is_err());

    assert_eq!(
        *signer_state.read().unwrap(),
        signed_a,
        "a committed command must not overwrite an existing vote with conflicting sign bytes at the same HRS"
    );
}
