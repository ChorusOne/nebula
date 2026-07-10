use super::mock_connection::MockCometBFTConnection;
use crate::backend::Ed25519Signer;
use crate::cluster::SignerRaftNode;
use crate::config::{PeerConfig, RaftConfig};
use crate::handle_single_request;
use crate::proto::v0_38;
use crate::signer::Signer;
use crate::types::SignedMsgType;
use crate::versions::VersionV0_38;
use prost::Message;
use rand::Rng;
use std::sync::{Arc, Mutex, mpsc};
use std::thread;
use std::time::{Duration, Instant};
use tempfile::TempDir;

fn create_single_node_raft(temp_dir: &TempDir) -> SignerRaftNode {
    let port = rand::rng().random_range(30000..60000);
    let bind_addr = format!("127.0.0.1:{port}");
    let config = RaftConfig {
        node_id: 1,
        bind_addr: bind_addr.clone(),
        data_path: temp_dir.path().join("node_1").to_string_lossy().to_string(),
        peers: vec![PeerConfig {
            id: 1,
            addr: bind_addr,
        }],
        initial_state_path: "./non_existent_initial_state.json".to_string(),
    };
    let (events_tx, _events_rx) = mpsc::channel();
    SignerRaftNode::new(config, events_tx)
}

fn wait_for_leader(node: &SignerRaftNode, timeout: Duration) {
    let start = Instant::now();
    while start.elapsed() < timeout {
        if node.is_leader() {
            return;
        }
        thread::sleep(Duration::from_millis(30));
    }
    panic!("Single-node raft did not become leader within timeout");
}

fn make_prevote_request(
    height: i64,
    round: i64,
    timestamp_secs: i64,
    block_hash: Option<Vec<u8>>,
) -> Vec<u8> {
    let block_id = block_hash.map(|hash| v0_38::types::BlockId {
        hash: hash.into(),
        part_set_header: Some(v0_38::types::PartSetHeader {
            total: 1,
            hash: vec![7, 7, 7].into(),
        }),
    });

    let req = v0_38::privval::SignVoteRequest {
        vote: Some(v0_38::types::Vote {
            r#type: SignedMsgType::Prevote as i32,
            height,
            round: round as i32,
            block_id,
            timestamp: Some(prost_types::Timestamp {
                seconds: timestamp_secs,
                nanos: 0,
            }),
            ..Default::default()
        }),
        chain_id: "test-chain".to_string(),
    };

    let msg = v0_38::privval::Message {
        sum: Some(v0_38::privval::message::Sum::SignVoteRequest(req)),
    };

    let mut bytes = Vec::new();
    msg.encode_length_delimited(&mut bytes).unwrap();
    bytes
}

fn recv_signed_vote_response(
    handle: &super::mock_connection::MockConnectionHandle,
) -> v0_38::privval::SignedVoteResponse {
    let response_bytes = handle
        .response_receiver
        .recv_timeout(Duration::from_secs(1))
        .unwrap();
    let response_msg =
        v0_38::privval::Message::decode_length_delimited(response_bytes.as_slice()).unwrap();
    match response_msg.sum {
        Some(v0_38::privval::message::Sum::SignedVoteResponse(res)) => res,
        _ => panic!("Expected SignedVoteResponse"),
    }
}

#[test]
fn signer_with_mock_connection() {
    let (mock_conn, handle) = MockCometBFTConnection::new();

    let backend = Ed25519Signer::from_key_file("./keys/privkey").unwrap();

    let mut signer = Signer::<_, VersionV0_38, _>::new(backend, mock_conn, "test-chain".into());

    let proposal_req = v0_38::privval::SignProposalRequest {
        proposal: Some(v0_38::types::Proposal {
            r#type: SignedMsgType::Proposal as i32,
            height: 1,
            round: 1,
            ..Default::default()
        }),
        chain_id: "test-chain".to_string(),
    };
    let msg = v0_38::privval::Message {
        sum: Some(v0_38::privval::message::Sum::SignProposalRequest(
            proposal_req,
        )),
    };

    let temp_dir = TempDir::new().unwrap();
    let raft_node = create_single_node_raft(&temp_dir);
    wait_for_leader(&raft_node, Duration::from_secs(5));
    let state_persist = Arc::new(Mutex::new(raft_node));

    let mut req_bytes = Vec::new();
    msg.encode_length_delimited(&mut req_bytes).unwrap();

    handle.request_sender.send(req_bytes).unwrap();

    handle_single_request(&mut signer, &state_persist).unwrap();

    let response_bytes = handle
        .response_receiver
        .recv_timeout(Duration::from_secs(1))
        .unwrap();
    let response_msg =
        v0_38::privval::Message::decode_length_delimited(response_bytes.as_slice()).unwrap();

    match response_msg.sum {
        Some(v0_38::privval::message::Sum::SignedProposalResponse(res)) => {
            assert!(res.error.is_none());
            let signed_proposal = res.proposal.unwrap();
            assert_eq!(signed_proposal.height, 1);
            assert_eq!(signed_proposal.round, 1);
            assert!(
                !signed_proposal.signature.is_empty(),
                "Signature should not be empty"
            );
            println!("Got signature: {}", hex::encode(&signed_proposal.signature));
        }
        _ => panic!("Expected a SignedProposalResponse"),
    }
}

#[test]
fn prevote_same_hrs_only_same_block_id_is_allowed() {
    let (mock_conn, handle) = MockCometBFTConnection::new();
    let backend = Ed25519Signer::from_key_file("./keys/privkey").unwrap();
    let mut signer = Signer::<_, VersionV0_38, _>::new(backend, mock_conn, "test-chain".into());

    let temp_dir = TempDir::new().unwrap();
    let raft_node = create_single_node_raft(&temp_dir);
    wait_for_leader(&raft_node, Duration::from_secs(5));
    let state_persist = Arc::new(Mutex::new(raft_node));

    // 1) block A => should sign
    handle
        .request_sender
        .send(make_prevote_request(1, 0, 1_700_000_001, Some(vec![0xAA])))
        .unwrap();
    handle_single_request(&mut signer, &state_persist).unwrap();
    let res1 = recv_signed_vote_response(&handle);
    assert!(res1.error.is_none());
    assert!(
        res1.vote
            .as_ref()
            .is_some_and(|v| !v.signature.is_empty() && v.block_id.is_some())
    );

    // 2) nil block => same HRS, different block id semantics, should be rejected
    handle
        .request_sender
        .send(make_prevote_request(1, 0, 1_700_000_002, None))
        .unwrap();
    handle_single_request(&mut signer, &state_persist).unwrap();
    let res2 = recv_signed_vote_response(&handle);
    assert!(res2.error.is_some());

    // 3) block A again => should work (same block id as #1)
    handle
        .request_sender
        .send(make_prevote_request(1, 0, 1_700_000_003, Some(vec![0xAA])))
        .unwrap();
    handle_single_request(&mut signer, &state_persist).unwrap();
    let res3 = recv_signed_vote_response(&handle);
    assert!(res3.error.is_none());
    assert!(
        res3.vote
            .as_ref()
            .and_then(|v| v.block_id.as_ref())
            .is_some_and(|id| id.hash.as_ref() == [0xAA])
    );

    // 4) block B => same HRS, different non-nil block id, should be rejected
    handle
        .request_sender
        .send(make_prevote_request(1, 0, 1_700_000_004, Some(vec![0xBB])))
        .unwrap();
    handle_single_request(&mut signer, &state_persist).unwrap();
    let res4 = recv_signed_vote_response(&handle);
    assert!(res4.error.is_some());
}
