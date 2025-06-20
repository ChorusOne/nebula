use super::*;
use crate::config::{PeerConfig, RaftConfig};
use crate::types::{ConsensusData, SignedMsgType};
use std::sync::Arc;
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

    let cluster = SignerRaftNode::new(config);

    let leader_id = wait_for_leader(&[Arc::clone(&cluster)], Duration::from_secs(5));
    assert_eq!(leader_id, Some(1));
    assert!(cluster.is_leader());

    let new_state = ConsensusData {
        height: 100,
        round: 1,
        step: SignedMsgType::Proposal as u8,
    };

    let result = cluster.replicate_state(new_state.clone());
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

    let cluster1 = SignerRaftNode::new(create_test_config(9000, 1, &temp_dir, peers.clone()));
    let cluster2 = SignerRaftNode::new(create_test_config(9000, 2, &temp_dir, peers.clone()));
    let cluster3 = SignerRaftNode::new(create_test_config(9000, 3, &temp_dir, peers.clone()));

    let clusters = vec![cluster1, cluster2, cluster3];

    let leader_id = wait_for_leader(&clusters, Duration::from_secs(10));
    assert!(leader_id.is_some());

    let leader = get_leader_cluster(&clusters).unwrap();
    let new_state = ConsensusData {
        height: 200,
        round: 2,
        step: SignedMsgType::Prevote as u8,
    };

    let result = leader.replicate_state(new_state.clone());
    assert!(result.is_ok());

    thread::sleep(Duration::from_secs(2));

    for cluster in &clusters {
        let state = cluster.signer_state.read().unwrap().clone();
        assert_eq!(state, new_state);
    }
}
