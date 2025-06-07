mod backend;
mod cluster;
mod config;
mod connection;
mod protocol;
mod signer;
mod types;
mod versions;

use crate::backend::{Bls12381Signer, Secp256k1Signer};
use crate::backend::{Ed25519Signer, SigningBackend, vault::VaultSigner};
use crate::config::SigningMode;
use cluster::Cluster;
use config::{Config, ProtocolVersionConfig};
use connection::open_secret_connection;
use log::{error, info, warn};
use nebula::SignerError;
use protocol::{Request, Response};
use signer::Signer;
use std::cmp::Ordering;
use std::sync::Arc;
use std::thread;
use std::thread::sleep;
use std::time::{Duration, Instant};
use types::ConsensusData;
use types::KeyType;
use types::SignedMsgType;
use versions::{ProtocolVersion, VersionV0_34, VersionV0_37, VersionV0_38, VersionV1_0};

use crate::types::{Proposal, Vote};

fn main() -> Result<(), SignerError> {
    let config_path = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "config.toml".to_string());
    let config: Config = Config::from_file(&config_path).expect("failed to parse config");

    env_logger::Builder::new()
        .filter_level(log::LevelFilter::Info)
        .init();

    info!("Loading config from: {}", config_path);
    info!("Chain ID: {}", config.chain_id);
    info!("Protocol version: {:?}", config.version);
    info!("Node ID: {}", config.raft.node_id);

    // let cluster = Cluster::new(config.raft.clone()).expect("failed to start cluster");
    let cluster = Cluster::new(config.raft.clone());

    loop {
        loop {
            if let Some(leader_id) = cluster.leader_id() {
                info!("Current leader: {}", leader_id);
                break;
            }
            sleep(Duration::from_millis(200));
        }

        if cluster.is_leader() {
            info!("This node ({}) is now the leader!", config.raft.node_id);

            let run_result = match config.version {
                ProtocolVersionConfig::V0_34 => {
                    run_leader_loop_for_version::<VersionV0_34>(&config, &cluster)
                }
                ProtocolVersionConfig::V0_37 => {
                    run_leader_loop_for_version::<VersionV0_37>(&config, &cluster)
                }
                ProtocolVersionConfig::V0_38 => {
                    run_leader_loop_for_version::<VersionV0_38>(&config, &cluster)
                }
                ProtocolVersionConfig::V1_0 => {
                    run_leader_loop_for_version::<VersionV1_0>(&config, &cluster)
                }
            };

            match run_result {
                Ok(()) => {
                    warn!("Leader loop exited normally (leadership lost?)");
                }
                Err(e) => {
                    error!("Leader loop error: {}", e);
                }
            }

            info!("No longer the leader -> back to follower mode");
        } else {
            info!("This node is a follower, standing by…");
            loop {
                sleep(Duration::from_secs(1));
                if cluster.is_leader() {
                    break;
                }
                if let Some(ldr) = cluster.leader_id() {
                    info!("Leader is: {}", ldr);
                }
            }
        }
    }
}

fn run_leader_loop_for_version<V: ProtocolVersion + Send + 'static>(
    config: &Config,
    cluster: &Arc<Cluster>,
) -> Result<(), SignerError> {
    info!(
        "Running leader loop (version = {:?}); spawning {} handler threads",
        config.version,
        config.connections.len()
    );

    let config = Arc::new(config.clone());
    let signing_lock = Arc::new(std::sync::Mutex::new(()));

    let mut thread_handles = Vec::with_capacity(config.connections.len());

    for conn_cfg in config.connections.iter() {
        let identity_key = ed25519_consensus::SigningKey::new(rand_core::OsRng);

        let cluster_clone = Arc::clone(cluster);
        let config_clone = Arc::clone(&config);
        let signing_lock_clone = Arc::clone(&signing_lock);
        let chain_id = config_clone.chain_id.clone();
        let host_str = conn_cfg.host.clone();
        let port_num = conn_cfg.port;

        let handle = thread::spawn(move || -> Result<(), SignerError> {
            let mut retry_count = 0;
            const MAX_RETRY_DELAY: Duration = Duration::from_secs(30);

            let establish_connection =
                || -> Result<Signer<Box<dyn SigningBackend>, V, _>, SignerError> {
                    info!("Connecting to CometBFT node at {}:{} …", host_str, port_num);

                    let raw_conn = open_secret_connection(
                        &host_str,
                        port_num,
                        identity_key.clone(),
                        tendermint_p2p::secret_connection::Version::V0_34,
                    )?;
                    info!(" -> Connected to CometBFT at {}:{}", host_str, port_num);

                    let backend: Box<dyn SigningBackend> = match config_clone.signing_mode {
                        SigningMode::Native => {
                            let native_cfg = config_clone.signing.native.as_ref().unwrap();

                            let priv_key_path = native_cfg.private_key_path.clone();
                            match native_cfg.key_type {
                                KeyType::Ed25519 => {
                                    let ed = Ed25519Signer::from_key_file(&priv_key_path)?;
                                    Box::new(ed)
                                }
                                KeyType::Secp256k1 => {
                                    let sk = Secp256k1Signer::from_key_file(&priv_key_path)?;
                                    Box::new(sk)
                                }
                                KeyType::Bls12_381 => {
                                    let bls = Bls12381Signer::from_key_file(&priv_key_path)?;
                                    Box::new(bls)
                                }
                            }
                        }
                        SigningMode::Vault => {
                            let vault_cfg = config_clone.signing.vault.as_ref().unwrap();

                            let vs = VaultSigner::new(vault_cfg.clone())?;
                            Box::new(vs)
                        }
                    };

                    let signer = Signer::<Box<dyn SigningBackend>, V, _>::new(
                        backend,
                        raw_conn,
                        chain_id.clone(),
                    );
                    Ok(signer)
                };

            let mut signer_instance = establish_connection()?;

            loop {
                let start = Instant::now();
                if !cluster_clone.is_leader() {
                    warn!(
                        "Thread for {}:{} sees leadership lost -> exiting",
                        host_str, port_num
                    );
                    break;
                }

                let req = match signer_instance.read_request() {
                    Ok(r) => {
                        retry_count = 0;
                        r
                    }
                    Err(e) => {
                        if !cluster_clone.is_leader() {
                            break;
                        }

                        error!(
                            "I/O error reading from {}:{} -> {}. Attempting reconnection...",
                            host_str, port_num, e
                        );

                        loop {
                            if !cluster_clone.is_leader() {
                                break;
                            }

                            retry_count += 1;
                            let delay = std::cmp::min(
                                Duration::from_millis(100 * 2_u64.pow(retry_count.min(10))),
                                MAX_RETRY_DELAY,
                            );

                            warn!(
                                "Reconnection attempt {} for {}:{} in {:?}",
                                retry_count, host_str, port_num, delay
                            );

                            thread::sleep(delay);

                            match establish_connection() {
                                Ok(new_signer) => {
                                    info!("Successfully reconnected to {}:{}", host_str, port_num);
                                    signer_instance = new_signer;
                                    retry_count = 0;
                                    break;
                                }
                                Err(reconnect_err) => {
                                    error!(
                                        "Reconnection failed for {}:{} -> {}",
                                        host_str, port_num, reconnect_err
                                    );
                                    continue;
                                }
                            }
                        }

                        continue;
                    }
                };

                info!(
                    "Received request after {:?} from {}:{}: {:?}",
                    start.elapsed(),
                    host_str,
                    port_num,
                    req
                );
                let start = Instant::now(); // we

                let response = match req {
                    Request::SignProposal(proposal) => {
                        let _guard = signing_lock_clone.lock().unwrap();

                        if !cluster_clone.is_leader() {
                            break;
                        }

                        let current_state = cluster_clone.state_machine.read().unwrap().clone();
                        if !should_sign_proposal(&current_state, &proposal) {
                            info!(
                                "prevented us from double signing a proposal at: {:?}",
                                proposal
                            );
                            Response::SignedProposal(
                                <V as ProtocolVersion>::create_proposal_response(
                                    None,
                                    Vec::new(),
                                    Some("would double-sign proposal at same height/round".into()),
                                ),
                            )
                        } else {
                            let new_state = ConsensusData {
                                height: proposal.height,
                                round: proposal.round,
                                step: SignedMsgType::Proposal as u8,
                            };

                            if let Err(e) = cluster_clone.replicate_state(new_state) {
                                error!("CRITICAL: State replication failed: {}. Not signing.", e);
                                Response::SignedProposal(
                                    <V as ProtocolVersion>::create_proposal_response(
                                        None,
                                        Vec::new(),
                                        Some(format!("Raft replication failed: {}", e)),
                                    ),
                                )
                            } else {
                                signer_instance.process_request(Request::SignProposal(proposal))?
                            }
                        }
                    }

                    Request::SignVote(vote) => {
                        let _guard = signing_lock_clone.lock().unwrap();
                        if !cluster_clone.is_leader() {
                            break;
                        }

                        let current_state = cluster_clone.state_machine.read().unwrap().clone();
                        if !should_sign_vote(&current_state, &vote) {
                            info!("prevented us from double signing a vote at: {:?}", vote);
                            Response::SignedVote(<V as ProtocolVersion>::create_vote_response(
                                None,
                                Vec::new(),
                                None,
                                Some("would double-sign vote at same height/round".into()),
                            ))
                        } else {
                            let new_state = ConsensusData {
                                height: vote.height,
                                round: vote.round,
                                step: vote.step.into(),
                            };

                            if let Err(e) = cluster_clone.replicate_state(new_state) {
                                error!("CRITICAL: State replication failed: {}. Not signing.", e);
                                Response::SignedVote(<V as ProtocolVersion>::create_vote_response(
                                    None,
                                    Vec::new(),
                                    None,
                                    Some(format!("Raft replication failed: {}", e)),
                                ))
                            } else {
                                signer_instance.process_request(Request::SignVote(vote))?
                            }
                        }
                    }

                    other => signer_instance.process_request(other)?,
                };
                info!("Request processed. Took: {:?}", start.elapsed());

                match signer_instance.send_response(response) {
                    Ok(()) => {}
                    Err(e) => {
                        error!(
                            "Error sending response to {}:{} -> {}. Will reconnect on next iteration.",
                            host_str, port_num, e
                        );
                    }
                }
                info!(
                    "Sent response! Sending the response took: {:?}",
                    start.elapsed()
                );
            }

            Ok(())
        });

        thread_handles.push(handle);
    }

    for handle in thread_handles {
        if let Err(e) = handle.join().expect("A handler thread panicked") {
            error!("Connection handler returned Err: {}", e);
        }
    }

    Ok(())
}

/*
A signer should only sign a proposal p if any of the following lines are true:

    p.Height > s.Height (1)
    p.Height == s.Height && p.Round > s.Round (2)

In other words, a proposal should only be signed if it’s at a higher height, or a higher round for the same height. Once a proposal or vote has been signed for a given height and round, a proposal should never be signed for the same height and round.
*/
fn should_sign_proposal(state: &ConsensusData, proposal: &Proposal) -> bool {
    let msg_ty = SignedMsgType::from(proposal.step);
    if msg_ty != SignedMsgType::Proposal {
        return false;
    }

    match (
        proposal.height.cmp(&state.height),
        proposal.round.cmp(&state.round),
    ) {
        // (1)
        (Ordering::Greater, _) => true,

        // (2)
        (Ordering::Equal, Ordering::Greater) => true,

        _ => false,
    }
}

/*
A signer should only sign a vote v if any of the following lines are true:

    v.Height > s.Height (1)
    v.Height == s.Height && v.Round > s.Round (2)
    v.Height == s.Height && v.Round == s.Round && v.Step == 0x1 && s.Step == 0x20 (3)
    v.Height == s.Height && v.Round == s.Round && v.Step == 0x2 && s.Step != 0x2 (4)

In other words, a vote should only be signed if it’s:
  - at a higher height
  - at a higher round for the same height
  - a prevote for the same height and round where we haven’t signed a prevote or precommit (but have signed a proposal)
  - a precommit for the same height and round where we haven’t signed a precommit (but have signed a proposal and/or a prevote)
*/
fn should_sign_vote(state: &ConsensusData, vote: &Vote) -> bool {
    info!("checking consensus state: {} against vote: {}", state, vote);
    let vote_step = SignedMsgType::from(vote.step);
    match (
        vote.height.cmp(&state.height),
        vote.round.cmp(&state.round),
        vote_step,
        state.step.into(),
    ) {
        // (1)
        (Ordering::Greater, _, _, _) => true,

        // (2)
        (Ordering::Equal, Ordering::Greater, _, _) => true,

        // (3)
        (Ordering::Equal, Ordering::Equal, SignedMsgType::Prevote, SignedMsgType::Proposal) => true,

        // (4)
        (Ordering::Equal, Ordering::Equal, SignedMsgType::Precommit, stp)
            if stp != SignedMsgType::Precommit =>
        {
            true
        }

        // everything else: don't sign
        _ => false,
    }
}

/*
* func shouldSignVoteExtension(chainID string, signBz, extSignBz []byte) (bool, error) {
   var vote cmtypes.CanonicalVote
   if err := protoio.UnmarshalDelimited(signBz, &vote); err != nil {
       return false, nil
   }

   if vote.Type == cmtypes.PrecommitType && vote.BlockID != nil && len(extSignBz) > 0 {
       var ext cmtypes.CanonicalVoteExtension
       if err := protoio.UnmarshalDelimited(extSignBz, &ext); err != nil {
           return false, fmt.Errorf("failed to unmarshal vote extension: %w", err)
       }

       switch {
       case ext.ChainId != chainID:
           return false, fmt.Errorf("extension chain ID %s does not match expected %s", ext.ChainId, chainID)
       case ext.Height != vote.Height:
           return false, fmt.Errorf("extension height %d does not match vote height %d", ext.Height, vote.Height)
       case ext.Round != vote.Round:
           return false, fmt.Errorf("extension round %d does not match vote round %d", ext.Round, vote.Round)
       }

       return true, nil
   }

   return false, nil
}
*/
