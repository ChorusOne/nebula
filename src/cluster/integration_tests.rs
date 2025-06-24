use crate::backend::{Ed25519Signer, SigningBackend};
use crate::cluster::SignerRaftNode;
use crate::config::{PeerConfig, RaftConfig};
use crate::error::SignerError;
use crate::handler::SigningHandler;
use crate::proto::v0_38;
use crate::signer::Signer;
use crate::signer::mock_connection::{MockCometBFTConnection, MockConnectionHandle};
use crate::types::SignedMsgType;
use crate::versions::VersionV0_38;
use log::info;
use prost::Message;
use rand::Rng;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};
use tempfile::TempDir;

struct TestHarness {
    _temp_dir: TempDir,
    nodes: Vec<Arc<SignerRaftNode>>,
    signing_lock: Arc<Mutex<()>>,
}

impl TestHarness {
    fn new(num_nodes: usize) -> Self {
        setup();
        let temp_dir = TempDir::new().unwrap();
        let port_prefix = rand::rng().random_range(30000..60000);

        let peers: Vec<PeerConfig> = (1..=num_nodes)
            .map(|i| PeerConfig {
                id: i as u64,
                addr: format!("127.0.0.1:{}", port_prefix + i as u64),
            })
            .collect();

        let nodes: Vec<Arc<SignerRaftNode>> = (1..=num_nodes)
            .map(|i| create_test_node(port_prefix as u64, i as u64, peers.clone(), &temp_dir))
            .collect();

        Self {
            _temp_dir: temp_dir,
            nodes,
            signing_lock: Arc::new(Mutex::new(())),
        }
    }

    fn wait_for_leader(&self, timeout: Duration) -> Option<Arc<SignerRaftNode>> {
        wait_for_leader(&self.nodes, timeout)
    }

    fn followers(&self) -> Vec<Arc<SignerRaftNode>> {
        self.nodes
            .iter()
            .filter(|n| !n.is_leader())
            .cloned()
            .collect()
    }

    pub fn shutdown_node(&mut self, node_id: u64) -> Result<(), SignerError> {
        if let Some(node) = self.nodes.iter().find(|n| n.node_id == node_id) {
            node.shutdown()?;
        }
        self.nodes.retain(|n| n.node_id != node_id);
        Ok(())
    }

    pub fn shutdown_followers(&mut self) -> Result<(), SignerError> {
        let leader_id = self
            .wait_for_leader(Duration::from_secs(5))
            .ok_or_else(|| SignerError::Other("No leader found".to_string()))?
            .node_id;

        let follower_ids: Vec<u64> = self
            .nodes
            .iter()
            .filter(|n| n.node_id != leader_id)
            .map(|n| n.node_id)
            .collect();

        for id in follower_ids {
            self.shutdown_node(id)?;
        }
        Ok(())
    }

    fn handle_request(
        &self,
        signer: &mut Signer<Box<dyn SigningBackend>, VersionV0_38, MockCometBFTConnection>,
        node: &Arc<SignerRaftNode>,
    ) -> Result<(), SignerError> {
        SigningHandler::<VersionV0_38>::handle_single_request(signer, node, &self.signing_lock)
    }
}

fn create_signer_with_mock_conn() -> (
    Signer<Box<dyn SigningBackend>, VersionV0_38, MockCometBFTConnection>,
    MockConnectionHandle,
) {
    let (mock_conn, handle) = MockCometBFTConnection::new();
    let signing_backend = Ed25519Signer::from_key_file("./keys/privkey").unwrap();
    let backend_trait_object: Box<dyn SigningBackend> = Box::new(signing_backend);
    let signer =
        Signer::<_, VersionV0_38, _>::new(backend_trait_object, mock_conn, "test-chain".into());
    (signer, handle)
}

pub fn setup() {
    let _ = env_logger::Builder::new()
        .filter_level(log::LevelFilter::Trace)
        .try_init();
}

fn create_test_node(
    port_prefix: u64,
    node_id: u64,
    peers: Vec<PeerConfig>,
    temp_dir: &TempDir,
) -> Arc<SignerRaftNode> {
    let config = RaftConfig {
        node_id,
        bind_addr: format!("127.0.0.1:{}", port_prefix + node_id),
        data_path: temp_dir
            .path()
            .join(format!("node_{}", node_id))
            .to_str()
            .unwrap()
            .to_string(),
        peers,
        initial_state_path: "./non_existent_initial_state.json".to_string(),
    };
    SignerRaftNode::new(config)
}

fn wait_for_leader(
    nodes: &[Arc<SignerRaftNode>],
    timeout: Duration,
) -> Option<Arc<SignerRaftNode>> {
    let start = Instant::now();
    while start.elapsed() < timeout {
        for node in nodes {
            if node.is_leader() {
                return Some(Arc::clone(node));
            }
        }
        thread::sleep(Duration::from_millis(30));
    }
    None
}

fn create_proposal_request_bytes(height: i64, round: i64) -> Vec<u8> {
    let proposal_req = v0_38::privval::SignProposalRequest {
        proposal: Some(v0_38::types::Proposal {
            r#type: SignedMsgType::Proposal as i32,
            height,
            round: round as i32,
            ..Default::default()
        }),
        chain_id: "test-chain".to_string(),
    };
    let msg = v0_38::privval::Message {
        sum: Some(v0_38::privval::message::Sum::SignProposalRequest(
            proposal_req,
        )),
    };
    let mut req_bytes = Vec::new();
    msg.encode_length_delimited(&mut req_bytes).unwrap();
    req_bytes
}

fn create_vote_request_bytes(height: i64, round: i64, vote_type: SignedMsgType) -> Vec<u8> {
    let vote_req = v0_38::privval::SignVoteRequest {
        vote: Some(v0_38::types::Vote {
            r#type: vote_type as i32,
            height,
            round: round as i32,
            ..Default::default()
        }),
        chain_id: "test-chain".to_string(),
    };
    let msg = v0_38::privval::Message {
        sum: Some(v0_38::privval::message::Sum::SignVoteRequest(vote_req)),
    };
    let mut req_bytes = Vec::new();
    msg.encode_length_delimited(&mut req_bytes).unwrap();
    req_bytes
}

#[test]
fn happy_path_signing_on_stable_cluster() {
    let harness = TestHarness::new(3);
    let leader_node = harness
        .wait_for_leader(Duration::from_secs(10))
        .expect("Failed to elect a leader");

    println!(
        "Leader is node {}",
        leader_node.raft_state.read().unwrap().1
    );

    let (mut signer, handle) = create_signer_with_mock_conn();

    let req_bytes = create_proposal_request_bytes(100, 0);
    handle.request_sender.send(req_bytes).unwrap();

    harness
        .handle_request(&mut signer, &leader_node)
        .expect("Failed to handle request");

    let response_bytes = handle.response_receiver.recv().unwrap();
    let response_msg =
        v0_38::privval::Message::decode_length_delimited(response_bytes.as_slice()).unwrap();

    match response_msg.sum {
        Some(v0_38::privval::message::Sum::SignedProposalResponse(res)) => {
            assert!(res.error.is_none());
            assert!(!res.proposal.unwrap().signature.is_empty());
        }
        _ => panic!("Wrong response type"),
    }

    thread::sleep(Duration::from_millis(500));
    for node in &harness.nodes {
        let state = node.signer_state.read().unwrap();
        assert_eq!(state.height, 100);
        assert_eq!(state.round, 0);
        assert_eq!(state.step, SignedMsgType::Proposal as u8);
    }
}

// In reality this will not happen, because it's the leader who initiates the connection.
// However maybe the test will be useful to check for leadership transfers and
// maybe when simplifying the is_leader checks (which are at every other point)
#[test]
fn signing_rejected_if_not_leader() {
    let harness = TestHarness::new(3);
    let leader_node = harness
        .wait_for_leader(Duration::from_secs(10))
        .expect("Failed to elect a leader");

    let follower_node = harness.followers().pop().unwrap();

    println!(
        "Leader: {}, Follower: {}",
        leader_node.node_id(),
        follower_node.node_id()
    );

    let (mut signer, handle) = create_signer_with_mock_conn();

    let req_bytes = create_proposal_request_bytes(100, 0);
    handle.request_sender.send(req_bytes).unwrap();

    harness
        .handle_request(&mut signer, &follower_node)
        .expect("Failed to handle request");

    let response_bytes = handle.response_receiver.recv().unwrap();
    let response_msg =
        v0_38::privval::Message::decode_length_delimited(response_bytes.as_slice()).unwrap();

    match response_msg.sum {
        Some(v0_38::privval::message::Sum::SignedProposalResponse(res)) => {
            assert!(res.error.is_some());
            assert!(res.error.unwrap().description.contains("Not the leader"));
        }
        _ => panic!("Wrong response type"),
    }
}

#[test]
fn double_sign_prevention() {
    let harness = TestHarness::new(3);
    let leader_node = harness
        .wait_for_leader(Duration::from_secs(5))
        .expect("Failed to elect leader");

    let (mut signer, handle) = create_signer_with_mock_conn();

    let req_bytes = create_proposal_request_bytes(100, 0);
    handle.request_sender.send(req_bytes.clone()).unwrap();

    harness
        .handle_request(&mut signer, &leader_node)
        .expect("Failed to handle first request");

    let response_bytes = handle.response_receiver.recv().unwrap();
    let response_msg =
        v0_38::privval::Message::decode_length_delimited(response_bytes.as_slice()).unwrap();

    match response_msg.sum {
        Some(v0_38::privval::message::Sum::SignedProposalResponse(res)) => {
            assert!(res.error.is_none());
            assert!(!res.proposal.unwrap().signature.is_empty());
        }
        _ => panic!("Wrong response type"),
    }

    handle.request_sender.send(req_bytes).unwrap();
    leader_node
        .transfer_leadership(leader_node.node_id() % 3 + 1)
        .unwrap();

    let leader_node = harness
        .wait_for_leader(Duration::from_secs(5))
        .expect("Failed to elect leader");

    harness
        .handle_request(&mut signer, &leader_node)
        .expect("Failed to handle second request");

    let response_bytes = handle.response_receiver.recv().unwrap();
    let response_msg =
        v0_38::privval::Message::decode_length_delimited(response_bytes.as_slice()).unwrap();

    match response_msg.sum {
        Some(v0_38::privval::message::Sum::SignedProposalResponse(res)) => {
            assert!(res.error.is_some());
            assert!(res.error.unwrap().description.contains("double-sign"));
        }
        _ => panic!("Wrong response type"),
    }
}

#[test]
fn leader_election_during_signing() {
    let harness = TestHarness::new(3);
    let initial_leader = harness
        .wait_for_leader(Duration::from_secs(10))
        .expect("Failed to elect a leader");
    let initial_leader_id = initial_leader.raft_state.read().unwrap().1;
    info!("initial_leader_id: {}", initial_leader_id);

    let success_count = Arc::new(AtomicU64::new(0));
    let error_count = Arc::new(AtomicU64::new(0));

    let mut handles = Vec::new();
    for _i in 0..3 {
        let leader = Arc::clone(&initial_leader);
        let success_counter = Arc::clone(&success_count);
        let error_counter = Arc::clone(&error_count);
        let lock_clone = Arc::clone(&harness.signing_lock);

        let handle = thread::spawn(move || {
            let (mut signer, handle) = create_signer_with_mock_conn();

            for height in 100..110 {
                let req_bytes = create_proposal_request_bytes(height, 0);
                if handle.request_sender.send(req_bytes).is_err() {
                    break;
                }

                match SigningHandler::<VersionV0_38>::handle_single_request(
                    &mut signer,
                    &leader,
                    &lock_clone,
                ) {
                    Ok(()) => {
                        if let Ok(response_bytes) = handle.response_receiver.try_recv() {
                            let response_msg = v0_38::privval::Message::decode_length_delimited(
                                response_bytes.as_slice(),
                            )
                            .unwrap();
                            if let Some(v0_38::privval::message::Sum::SignedProposalResponse(res)) =
                                response_msg.sum
                            {
                                if res.error.is_none() {
                                    success_counter.fetch_add(1, Ordering::SeqCst);
                                } else {
                                    error_counter.fetch_add(1, Ordering::SeqCst);
                                }
                            }
                        }
                    }
                    Err(_) => {
                        error_counter.fetch_add(1, Ordering::SeqCst);
                    }
                }

                thread::sleep(Duration::from_millis(15));
            }
        });
        handles.push(handle);
    }

    thread::sleep(Duration::from_millis(500));

    println!("initial leader id: {}", initial_leader_id);
    let transferee_id = (initial_leader_id % 3) + 1;
    info!(
        "Transferring leadership from {} to {}",
        initial_leader_id, transferee_id
    );
    initial_leader.transfer_leadership(transferee_id).unwrap();

    thread::sleep(Duration::from_millis(2000));
    let new_leader = harness.wait_for_leader(Duration::from_secs(15));
    assert!(new_leader.is_some(), "New leader should be elected");
    let new_leader_id = new_leader.unwrap().raft_state.read().unwrap().1;
    assert_ne!(
        new_leader_id, initial_leader_id,
        "Leadership should have changed"
    );

    for handle in handles {
        handle.join().unwrap();
    }
    let total_success = success_count.load(Ordering::SeqCst);
    let total_errors = error_count.load(Ordering::SeqCst);

    println!(
        "Successful signings: {}, Errors: {}",
        total_success, total_errors
    );
    assert!(
        total_success > 0,
        "Some requests should have succeeded before leadership change"
    );
    assert!(
        total_errors > 0,
        "Some requests should have failed after leadership change"
    );
}

#[test]
fn signing_old_blocks_after_state_advancement() {
    let harness = TestHarness::new(1);
    let leader = harness.wait_for_leader(Duration::from_secs(5)).unwrap();

    let (mut signer, handle) = create_signer_with_mock_conn();

    let req_bytes = create_proposal_request_bytes(100, 0);
    handle.request_sender.send(req_bytes).unwrap();
    harness.handle_request(&mut signer, &leader).unwrap();
    handle.response_receiver.recv().unwrap(); // consume response

    for height in 101..120 {
        let req_bytes = create_proposal_request_bytes(height, 0);
        handle.request_sender.send(req_bytes).unwrap();
        harness.handle_request(&mut signer, &leader).unwrap();
        handle.response_receiver.recv().unwrap();
    }

    let old_req_bytes = create_proposal_request_bytes(50, 0);
    handle.request_sender.send(old_req_bytes).unwrap();
    harness.handle_request(&mut signer, &leader).unwrap();

    let response_bytes = handle.response_receiver.recv().unwrap();
    let response_msg =
        v0_38::privval::Message::decode_length_delimited(response_bytes.as_slice()).unwrap();

    match response_msg.sum {
        Some(v0_38::privval::message::Sum::SignedProposalResponse(res)) => {
            assert!(res.error.is_some(), "Should reject old block");
            assert!(res.error.unwrap().description.contains("double-sign"));
        }
        _ => panic!("Expected SignedProposalResponse with error"),
    }
}

#[test]
fn mixed_vote_types_with_state_transitions() {
    let harness = TestHarness::new(1);
    let leader = harness.wait_for_leader(Duration::from_secs(5)).unwrap();
    let (mut signer, handle) = create_signer_with_mock_conn();

    let height = 300;
    let round = 0;

    let proposal_bytes = create_proposal_request_bytes(height, round);
    handle.request_sender.send(proposal_bytes).unwrap();
    assert!(harness.handle_request(&mut signer, &leader).is_ok());
    handle.response_receiver.recv().unwrap();

    let prevote_bytes = create_vote_request_bytes(height, round, SignedMsgType::Prevote);
    handle.request_sender.send(prevote_bytes).unwrap();
    assert!(harness.handle_request(&mut signer, &leader).is_ok());
    handle.response_receiver.recv().unwrap();

    let precommit_bytes = create_vote_request_bytes(height, round, SignedMsgType::Precommit);
    handle.request_sender.send(precommit_bytes).unwrap();
    assert!(harness.handle_request(&mut signer, &leader).is_ok());
    handle.response_receiver.recv().unwrap();

    let duplicate_prevote_bytes = create_vote_request_bytes(height, round, SignedMsgType::Prevote);
    handle.request_sender.send(duplicate_prevote_bytes).unwrap();
    harness.handle_request(&mut signer, &leader).unwrap();

    let response_bytes = handle.response_receiver.recv().unwrap();
    let response_msg =
        v0_38::privval::Message::decode_length_delimited(response_bytes.as_slice()).unwrap();

    match response_msg.sum {
        Some(v0_38::privval::message::Sum::SignedVoteResponse(res)) => {
            assert!(res.error.is_some(), "Should reject duplicate prevote");
        }
        _ => panic!("Expected SignedVoteResponse with error"),
    }
}

#[test]
fn leadership_handoff() {
    let harness = TestHarness::new(3);
    let leader = harness.wait_for_leader(Duration::from_secs(10)).unwrap();
    let leader_id = leader.raft_state.read().unwrap().1;

    let (mut signer, handle) = create_signer_with_mock_conn();

    let req_bytes = create_proposal_request_bytes(400, 0);
    handle.request_sender.send(req_bytes).unwrap();

    let leader_clone = Arc::clone(&leader);
    leader_clone
        .transfer_leadership(leader_clone.node_id % 3 + 1)
        .unwrap();
    thread::spawn(move || {
        thread::sleep(Duration::from_millis(100));
        drop(leader_clone);
    });

    thread::sleep(Duration::from_millis(200));
    let result = harness.handle_request(&mut signer, &leader);

    match result {
        Err(_) => println!("Request correctly failed due to leadership loss"),
        Ok(()) => {
            if let Ok(response_bytes) = handle.response_receiver.try_recv() {
                let response_msg =
                    v0_38::privval::Message::decode_length_delimited(response_bytes.as_slice())
                        .unwrap();
                match response_msg.sum {
                    Some(v0_38::privval::message::Sum::SignedProposalResponse(res)) => {
                        if res.error.is_some() {
                            println!("Request correctly returned error due to leadership loss");
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    let remaining_nodes: Vec<_> = harness
        .nodes
        .iter()
        .cloned()
        .filter(|n| n.raft_state.read().unwrap().1 != leader_id)
        .collect();
    let new_leader = wait_for_leader(&remaining_nodes, Duration::from_secs(30));
    assert!(new_leader.is_some(), "New leader should be elected");
}

#[test]
fn rapid_round_advancement() {
    let harness = TestHarness::new(1);
    let leader = harness.wait_for_leader(Duration::from_secs(5)).unwrap();
    let (mut signer, handle) = create_signer_with_mock_conn();

    let height = 500;

    for round in 0..10 {
        let proposal_bytes = create_proposal_request_bytes(height, round);
        handle.request_sender.send(proposal_bytes).unwrap();

        let result = harness.handle_request(&mut signer, &leader);
        assert!(
            result.is_ok(),
            "Should be able to sign proposal at round {}",
            round
        );
        handle.response_receiver.recv().unwrap();

        if round > 0 {
            let old_round_bytes = create_proposal_request_bytes(height, round - 1);
            handle.request_sender.send(old_round_bytes).unwrap();
            harness.handle_request(&mut signer, &leader).unwrap();

            let response_bytes = handle.response_receiver.recv().unwrap();
            let response_msg =
                v0_38::privval::Message::decode_length_delimited(response_bytes.as_slice())
                    .unwrap();

            match response_msg.sum {
                Some(v0_38::privval::message::Sum::SignedProposalResponse(res)) => {
                    assert!(
                        res.error.is_some(),
                        "Should reject old round at round {}",
                        round
                    );
                }
                _ => panic!("Expected error response for old round"),
            }
        }
    }
}

// sign block, leader failover, get request to sign old block, what happens
#[test]
fn double_sign_prevention_after_leadership_change() {
    let harness = TestHarness::new(3);

    let initial_leader = harness.wait_for_leader(Duration::from_secs(10)).unwrap();
    let initial_leader_id = initial_leader.raft_state.read().unwrap().1;

    let (mut signer1, handle1) = create_signer_with_mock_conn();

    let req_bytes = create_proposal_request_bytes(100, 0);
    handle1.request_sender.send(req_bytes).unwrap();

    harness
        .handle_request(&mut signer1, &initial_leader)
        .unwrap();

    let response_bytes = handle1.response_receiver.recv().unwrap();
    let response_msg =
        v0_38::privval::Message::decode_length_delimited(response_bytes.as_slice()).unwrap();

    match response_msg.sum {
        Some(v0_38::privval::message::Sum::SignedProposalResponse(res)) => {
            assert!(res.error.is_none(), "Initial signing should succeed");
        }
        _ => panic!("Expected SignedProposalResponse"),
    }

    initial_leader
        .transfer_leadership(initial_leader_id % 3 + 1)
        .unwrap();
    thread::sleep(Duration::from_millis(2000));

    let new_leader = harness.wait_for_leader(Duration::from_secs(15)).unwrap();

    let (mut signer2, handle2) = create_signer_with_mock_conn();

    let duplicate_req_bytes = create_proposal_request_bytes(100, 0);
    handle2.request_sender.send(duplicate_req_bytes).unwrap();

    harness.handle_request(&mut signer2, &new_leader).unwrap();

    let response_bytes2 = handle2.response_receiver.recv().unwrap();
    let response_msg2 =
        v0_38::privval::Message::decode_length_delimited(response_bytes2.as_slice()).unwrap();

    match response_msg2.sum {
        Some(v0_38::privval::message::Sum::SignedProposalResponse(res)) => {
            assert!(res.error.is_some(), "Duplicate signing should be prevented");
            let error_desc = res.error.unwrap().description;
            assert!(
                error_desc.contains("double-sign") || error_desc.contains("Would double-sign"),
                "Error should mention double signing, got: {}",
                error_desc
            );
        }
        _ => panic!("Expected SignedProposalResponse with error"),
    }

    let new_req_bytes = create_proposal_request_bytes(101, 0);
    handle2.request_sender.send(new_req_bytes).unwrap();

    harness.handle_request(&mut signer2, &new_leader).unwrap();

    let response_bytes3 = handle2.response_receiver.recv().unwrap();
    let response_msg3 =
        v0_38::privval::Message::decode_length_delimited(response_bytes3.as_slice()).unwrap();

    match response_msg3.sum {
        Some(v0_38::privval::message::Sum::SignedProposalResponse(res)) => {
            assert!(res.error.is_none(), "New block signing should succeed");
        }
        _ => panic!("Expected successful SignedProposalResponse"),
    }
}

#[test]
fn no_replicate_acks() {
    let mut harness = TestHarness::new(3);

    let initial_leader = harness.wait_for_leader(Duration::from_secs(10)).unwrap();

    let (mut signer1, handle1) = create_signer_with_mock_conn();

    let req_bytes = create_proposal_request_bytes(100, 0);
    handle1.request_sender.send(req_bytes).unwrap();

    harness.shutdown_followers().unwrap();

    thread::sleep(Duration::from_millis(500));

    harness
        .handle_request(&mut signer1, &initial_leader)
        .unwrap();

    let response_bytes = handle1.response_receiver.recv().unwrap();
    let response_msg =
        v0_38::privval::Message::decode_length_delimited(response_bytes.as_slice()).unwrap();

    match response_msg.sum {
        Some(v0_38::privval::message::Sum::SignedProposalResponse(res)) => {
            assert!(
                res.error.is_some(),
                "Replication should fail without followers"
            );
        }
        _ => panic!("Expected SignedProposalResponse"),
    }
}

#[test]
fn new_leader_signing() {
    let mut harness = TestHarness::new(3);

    let initial_leader = harness.wait_for_leader(Duration::from_secs(10)).unwrap();

    let (mut signer1, handle1) = create_signer_with_mock_conn();

    let req_bytes = create_proposal_request_bytes(100, 0);
    handle1.request_sender.send(req_bytes).unwrap();

    harness.shutdown_node(initial_leader.node_id()).unwrap();

    thread::sleep(Duration::from_millis(500));

    let new_leader = harness.wait_for_leader(Duration::from_secs(1)).unwrap();

    harness.handle_request(&mut signer1, &new_leader).unwrap();

    let response_bytes = handle1.response_receiver.recv().unwrap();
    let response_msg =
        v0_38::privval::Message::decode_length_delimited(response_bytes.as_slice()).unwrap();

    match response_msg.sum {
        Some(v0_38::privval::message::Sum::SignedProposalResponse(res)) => {
            assert!(res.error.is_none(), "2-node cluster should still work");
        }
        _ => panic!("Expected SignedProposalResponse"),
    }
}

#[test]
fn some_turbulence() {
    let mut harness = TestHarness::new(3);

    let initial_leader = harness.wait_for_leader(Duration::from_secs(10)).unwrap();

    let (mut signer1, handle1) = create_signer_with_mock_conn();

    let req_bytes = create_proposal_request_bytes(100, 0);
    handle1.request_sender.send(req_bytes).unwrap();

    harness.shutdown_node(initial_leader.node_id()).unwrap(); // kill the leader

    thread::sleep(Duration::from_millis(500));

    let new_leader = harness.wait_for_leader(Duration::from_secs(1)).unwrap();

    harness.handle_request(&mut signer1, &new_leader).unwrap();

    let response_bytes = handle1.response_receiver.recv().unwrap();
    let response_msg =
        v0_38::privval::Message::decode_length_delimited(response_bytes.as_slice()).unwrap();

    match response_msg.sum {
        Some(v0_38::privval::message::Sum::SignedProposalResponse(res)) => {
            assert!(res.error.is_none(), "2-node cluster should still work");
        }
        _ => panic!("Expected SignedProposalResponse"),
    }
}
