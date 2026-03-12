mod backend;
mod cluster;
mod config;
mod connection;
mod error;
mod keygen;
#[allow(clippy::all)]
mod proto;
mod protocol;
mod signer;
mod types;
mod versions;

use crate::backend::SigningBackend;
use crate::cluster::RaftEvent;
use crate::error::SignerError;
use crate::protocol::Response;
use crate::types::Vote;
use clap::{Parser as _, Subcommand};
use cluster::SignerRaftNode;
use config::{Config, ProtocolVersionConfig};
use log::{LevelFilter, error, info, trace, warn};
use protocol::{CheckedRequest, Request};
use signer::Signer;
use std::io::{Read, Write};
use std::str::FromStr;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex, mpsc};
use std::thread;
use std::time::Duration;
use types::{ConsensusData, KeyType};
use versions::{ProtocolVersion, VersionV0_34, VersionV0_37, VersionV0_38, VersionV1_0};

#[derive(clap::Parser)]
#[command(name = "signer")]
#[command(about = "A distributed CometBFT remote signer")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Start {
        #[arg(short, long = "config")]
        config_path: String,
    },
    Init {
        #[arg(short, long)]
        output_path: String,

        #[arg(short, long)]
        backend: config::SigningMode,
    },
    Keys {
        #[command(subcommand)]
        command: KeysCommands,
    },
}

#[derive(Subcommand)]
enum KeysCommands {
    Generate {
        #[arg(long, value_enum)]
        key_type: types::KeyType,
    },
}

fn main() -> Result<(), SignerError> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Start { config_path } => {
            let config = Config::from_file(&config_path)?;
            env_logger::Builder::new()
                .filter_level(LevelFilter::from_str(&config.log_level).unwrap_or(LevelFilter::Info))
                .init();
            start_signer(config)
        }
        Commands::Init {
            output_path,
            backend,
        } => {
            let default_config = Config::default_config(backend);
            default_config.write_to_file(&output_path)?;
            println!("Generated default configuration at '{}'", output_path);
            Ok(())
        }
        Commands::Keys { command } => {
            match command {
                KeysCommands::Generate { key_type } => {
                    let key_type_str = match key_type {
                        KeyType::Ed25519 => "ed25519",
                        KeyType::Secp256k1 => "secp256k1",
                        KeyType::Bls12381 => "bls12381",
                    };

                    if let Err(e) = keygen::generate_keys(key_type_str) {
                        eprintln!("Key generation failed: {}", e);
                        std::process::exit(1);
                    }
                }
            }
            Ok(())
        }
    }
}

/// Creates a Raft node and listens on events emitted by the Nebula Raft machinery.
///
/// There are two events emitted:
/// - Leadership changed -> drives the leader loop
/// - State applied -> currently, only used for tests
fn start_signer(config: Config) -> Result<(), SignerError> {
    info!("Chain ID: {}", config.chain_id);
    info!("Protocol version: {:?}", config.version);
    let (tx, rx) = mpsc::channel::<RaftEvent>();
    info!("Node ID: {}", config.raft.node_id);
    let node_id = config.raft.node_id;
    let raft: Arc<Mutex<SignerRaftNode>> = Arc::new(Mutex::new(SignerRaftNode::new(
        config.raft.clone(),
        tx.clone(),
    )));

    let mut leader_loop: Option<LeaderLoopHandle> = None;

    if let Ok(guard) = raft.lock() {
        if guard.is_leader() {
            leader_loop = Some(start_leader_loop(config.clone(), Arc::clone(&raft)));
        }
    }

    loop {
        match rx.recv() {
            Ok(RaftEvent::LeadershipChanged(from, to)) => {
                info!("Leadership changed from: {}, to: {}", from, to);
                if to == node_id {
                    if leader_loop.is_none() {
                        leader_loop = Some(start_leader_loop(config.clone(), Arc::clone(&raft)));
                    }
                } else if leader_loop.is_some() {
                    stop_leader_loop(&mut leader_loop);
                }
            }
            Ok(RaftEvent::StateApplied { .. }) => {}
            Err(_) => {
                warn!("Raft event channel closed; shutting down leader loop");
                stop_leader_loop(&mut leader_loop);
                break;
            }
        }
    }

    Ok(())
}

struct LeaderLoopHandle {
    stop: Arc<AtomicBool>,
    join: thread::JoinHandle<()>,
}

fn start_leader_loop(config: Config, persist: Arc<Mutex<SignerRaftNode>>) -> LeaderLoopHandle {
    let stop = Arc::new(AtomicBool::new(false));
    let stop_for_thread = Arc::clone(&stop);
    let join = thread::spawn(move || {
        let result = match config.version {
            ProtocolVersionConfig::V0_34 => {
                run_leader::<VersionV0_34>(&config, &persist, &stop_for_thread)
            }
            ProtocolVersionConfig::V0_37 => {
                run_leader::<VersionV0_37>(&config, &persist, &stop_for_thread)
            }
            ProtocolVersionConfig::V0_38 => {
                run_leader::<VersionV0_38>(&config, &persist, &stop_for_thread)
            }
            ProtocolVersionConfig::V1_0 => {
                run_leader::<VersionV1_0>(&config, &persist, &stop_for_thread)
            }
        };

        match result {
            Ok(()) => warn!("Leader loop exited normally"),
            Err(e) => error!("Leader loop error: {}", e),
        }
    });

    LeaderLoopHandle { stop, join }
}

fn stop_leader_loop(handle: &mut Option<LeaderLoopHandle>) {
    if let Some(handle) = handle.take() {
        handle.stop.store(true, Ordering::SeqCst);
        if let Err(e) = handle.join.join() {
            warn!("Leader loop thread panicked: {:?}", e);
        }
    }
}

fn run_leader<V: ProtocolVersion + Send + 'static>(
    config: &Config,
    persist: &Arc<Mutex<SignerRaftNode>>,
    stop: &Arc<AtomicBool>,
) -> Result<(), SignerError> {
    info!(
        "Running leader loop for {} connections",
        config.connections.len()
    );

    let config = Arc::new(config.clone());

    let handles: Vec<_> = config
        .connections
        .iter()
        .map(|conn| {
            let config = Arc::clone(&config);
            let p = Arc::clone(persist);
            let host = conn.host.clone();
            let port = conn.port;
            let stop = Arc::clone(stop);

            info!("connecting to {host}:{port}");
            thread::spawn(move || handle_connection::<V>(host, port, config, p, stop))
        })
        .collect();

    for handle in handles {
        if let Err(e) = handle.join().expect("Handler thread panicked") {
            error!("Connection handler error: {}", e);
        }
    }

    Ok(())
}

fn handle_connection<V: ProtocolVersion + Send + 'static>(
    host: String,
    port: u16,
    config: Arc<Config>,
    persist: Arc<Mutex<SignerRaftNode>>,
    stop: Arc<AtomicBool>,
) -> Result<(), SignerError> {
    let mut retry_count = 0;
    let identity_key = ed25519_consensus::SigningKey::new(rand_core::OsRng);

    let mut signer =
        crate::signer::create_signer::<V>(&host, port, &identity_key, &config, Some(&stop))?;

    loop {
        if stop.load(Ordering::SeqCst) {
            break;
        }

        let response = handle_single_request(&mut signer, &persist);
        if let Err(ref e) = response {
            if let SignerError::IoError(io) = e {
                if io.kind() == std::io::ErrorKind::TimedOut
                    || io.kind() == std::io::ErrorKind::WouldBlock
                {
                    if stop.load(Ordering::SeqCst) {
                        break;
                    }
                    continue;
                }
            }

            error!("Error handling request from {}:{} - {}", host, port, e);
            match reconnect::<V>(&host, port, &identity_key, &config, &mut retry_count, &stop) {
                Ok(new_signer) => signer = new_signer,
                Err(_) => continue,
            }
        } else {
            retry_count = 0;
        }
    }
    Ok(())
}

enum RequestProcessingAction<V: ProtocolVersion> {
    SignAndPersist {
        request: CheckedRequest,
        request_state: ConsensusData,
    },
    ReplayFromCache {
        request: CheckedRequest,
        cached: ConsensusData,
    },
    ReplyWith(Response<V::ProposalResponse, V::VoteResponse, V::PubKeyResponse, V::PingResponse>),
    // TODO: remove this and use ReplyWith
    ShowPublicKey,
}

fn proposal_response_from_signature<V: ProtocolVersion>(
    proposal: &types::Proposal,
    signature: Vec<u8>,
) -> Response<V::ProposalResponse, V::VoteResponse, V::PubKeyResponse, V::PingResponse> {
    Response::Proposal(V::create_proposal_response(proposal, signature))
}

fn vote_response_from_signature<V: ProtocolVersion>(
    vote: &types::Vote,
    signature: Vec<u8>,
    extension_signature: Option<Vec<u8>>,
) -> Response<V::ProposalResponse, V::VoteResponse, V::PubKeyResponse, V::PingResponse> {
    Response::Vote(V::create_vote_response(
        vote,
        signature,
        extension_signature,
    ))
}

fn response_from_checked_request_signature<V: ProtocolVersion>(
    request: &CheckedRequest,
    signature: Vec<u8>,
    extension_signature: Option<Vec<u8>>,
) -> Response<V::ProposalResponse, V::VoteResponse, V::PubKeyResponse, V::PingResponse> {
    match request {
        CheckedRequest::Proposal(proposal) => {
            proposal_response_from_signature::<V>(proposal, signature)
        }
        CheckedRequest::Vote(vote) => {
            vote_response_from_signature::<V>(vote, signature, extension_signature)
        }
    }
}

fn vote_sign_data<V: ProtocolVersion>(
    vote: &Vote,
    chain_id: &str,
) -> Result<(Vec<u8>, Vec<u8>), SignerError> {
    let sign_data = V::vote_to_bytes(vote, chain_id)?;
    let ext_sign_data = if vote.step == types::SignedMsgType::Precommit
        && vote.block_id.as_ref().is_some_and(|id| !id.hash.is_empty())
    {
        V::vote_extension_to_bytes(vote, chain_id)?
    } else {
        vec![]
    };
    Ok((sign_data, ext_sign_data))
}

/// Determines the course of action for a request.
///
/// For non-consensus requests (Show public key or Ping), return a command with a ready to send reply.
/// For consensus requests (Proposal or Vote), check the request against the signer's state and:
/// - for proposals, check if the height/round has already been signed. If yes, reply with a "would double sign error". If no, return a command to persist the state and sign over the proposal.
/// - for votes, build the corresponding consensus state (height/round/step and sign bytes) and:
///   - if the request matches the current signer state at the same height/round/step:
///     - if the stored vote and incoming vote are identical besides timestamp differences allowed by the protocol version, return a command to sign again and persist.
///     - if the stored vote is identical, return a command to replay the stored signature.
///     - if the vote conflicts with the stored state, reply with a "would double sign error".
///   - otherwise, run the standard vote validation against the signer state and either return a double sign error or a command to persist the state and sign the vote.
fn process_request<V: ProtocolVersion>(
    request: Request,
    raft_node: &SignerRaftNode,
    chain_id: &str,
) -> Result<RequestProcessingAction<V>, SignerError> {
    match request {
        Request::Proposal(proposal) => {
            trace!("checking proposal: {:?}", proposal);
            match proposal.check(&raft_node.signer_state.read().unwrap().clone()) {
                protocol::CheckedProposalRequest::DoubleSignProposal(consensus_data) => {
                    Ok(RequestProcessingAction::ReplyWith(Response::Proposal(
                        V::create_double_sign_prop_response(&consensus_data),
                    )))
                }
                protocol::CheckedProposalRequest::ValidRequest(checked_request) => {
                    Ok(RequestProcessingAction::SignAndPersist {
                        request: checked_request.clone(),
                        request_state: ConsensusData::from(&checked_request),
                    })
                }
            }
        }
        Request::Vote(vote) => {
            trace!("checking vote: {}", vote);
            let (sign_data, ext_sign_data) = vote_sign_data::<V>(&vote, chain_id)?;

            let request_state = ConsensusData {
                height: vote.height,
                round: vote.round,
                step: vote.step,
                sign_data,
                ext_sign_data,
                ..Default::default()
            };

            let current_state = raft_node.signer_state.read().unwrap().clone();
            let is_same_hrs = current_state.height == request_state.height
                && current_state.round == request_state.round
                && current_state.step == request_state.step;

            if is_same_hrs && !current_state.sign_data.is_empty() {
                let ext_matches = if request_state.ext_sign_data.is_empty() {
                    current_state.ext_sign_data.is_empty() && current_state.ext_signature.is_empty()
                } else {
                    current_state.ext_sign_data == request_state.ext_sign_data
                };

                if current_state.sign_data == request_state.sign_data
                    && !current_state.signature.is_empty()
                    && ext_matches
                {
                    info!("replaying stored vote signature for vote: {}", vote);
                    return Ok(RequestProcessingAction::ReplayFromCache {
                        request: CheckedRequest::Vote(vote),
                        cached: current_state,
                    });
                }

                let only_ts = V::vote_sign_bytes_only_differ_by_timestamp(
                    &current_state.sign_data,
                    &request_state.sign_data,
                )?;

                if only_ts {
                    // Vote extensions are non-deterministic
                    // if !ext_matches {
                    //     info!("only ts differs, but ext data does not match. not signing");
                    //     return Ok(RequestProcessingAction::ReplyWith(Response::Vote(
                    //         V::create_double_sign_vote_response(&request_state),
                    //     )));
                    // }
                    info!(
                        "only ts differs, issuing a sign command. current state: {}",
                        current_state
                    );
                    return Ok(RequestProcessingAction::SignAndPersist {
                        request: CheckedRequest::Vote(vote),
                        request_state,
                    });
                }

                return Ok(RequestProcessingAction::ReplyWith(Response::Vote(
                    V::create_double_sign_vote_response(&request_state),
                )));
            }

            info!("checking vote, current state: {}", current_state);
            match vote.check(&current_state) {
                protocol::CheckedVoteRequest::DoubleSignVote(consensus_data) => {
                    Ok(RequestProcessingAction::ReplyWith(Response::Vote(
                        V::create_double_sign_vote_response(&consensus_data),
                    )))
                }
                protocol::CheckedVoteRequest::ValidRequest(checked_request) => {
                    info!("valid vote, issuing a sign and persist command");
                    Ok(RequestProcessingAction::SignAndPersist {
                        request: checked_request,
                        request_state,
                    })
                }
            }
        }
        Request::ShowPublicKey => Ok(RequestProcessingAction::ShowPublicKey),
        Request::Ping => Ok(RequestProcessingAction::ReplyWith(Response::Ping(
            V::create_ping_response(),
        ))),
    }
}

pub fn handle_single_request<T: SigningBackend, V: ProtocolVersion, C: Read + Write>(
    signer: &mut Signer<T, V, C>,
    raft: &Arc<Mutex<SignerRaftNode>>,
) -> Result<(), SignerError> {
    let start = std::time::Instant::now();
    let request = signer.read_request()?;

    info!(
        "Received request after {:?}. Request: {:?}",
        start.elapsed(),
        request
    );
    let start = std::time::Instant::now();
    let raft = raft.lock().unwrap();
    let action = process_request::<V>(request, &raft, signer.chain_id())?;

    let response = match action {
        RequestProcessingAction::SignAndPersist {
            request,
            mut request_state,
        } => {
            let (signature, extension_signature) = signer.sign_request(&request)?;
            request_state.signature = signature.clone();
            request_state.ext_signature = extension_signature.clone().unwrap_or_default();

            if let Err(e) = raft.replicate_state(&request_state) {
                error!("Could not persist state: {e}");
                V::create_error_response(&format!("Cannot persist new consensus state: {e}"))
            } else {
                info!("responding with a fresh signature");

                response_from_checked_request_signature::<V>(
                    &request,
                    signature,
                    extension_signature,
                )
            }
        }
        RequestProcessingAction::ReplayFromCache { request, cached } => {
            info!("responding with a cached response");
            response_from_checked_request_signature::<V>(
                &request,
                cached.signature.clone(),
                (!cached.ext_signature.is_empty()).then_some(cached.ext_signature.clone()),
            )
        }
        RequestProcessingAction::ReplyWith(response) => response,
        RequestProcessingAction::ShowPublicKey => {
            let public_key = signer.public_key()?;
            Response::PublicKey(V::create_pub_key_response(&public_key))
        }
    };

    info!(
        "Processing request took: {:?}, sending response to validator",
        start.elapsed()
    );
    signer.send_response(response)?;
    drop(raft);
    Ok(())
}

fn reconnect<V: ProtocolVersion>(
    host: &str,
    port: u16,
    identity_key: &ed25519_consensus::SigningKey,
    config: &Config,
    retry_count: &mut u32,
    stop: &AtomicBool,
) -> Result<crate::signer::NetworkSigner<V>, SignerError> {
    const MAX_RETRY_DELAY: Duration = Duration::from_secs(30);

    loop {
        if stop.load(Ordering::SeqCst) {
            return Err(SignerError::NotLeader(config.raft.node_id.to_string()));
        }

        *retry_count += 1;
        let delay =
            Duration::from_millis(100 * 2_u64.pow((*retry_count).min(10))).min(MAX_RETRY_DELAY);

        warn!(
            "Reconnection attempt {} for {}:{} in {:?}",
            retry_count, host, port, delay
        );
        thread::sleep(delay);

        if stop.load(Ordering::SeqCst) {
            return Err(SignerError::NotLeader(config.raft.node_id.to_string()));
        }

        match crate::signer::create_signer::<V>(host, port, identity_key, config, Some(stop)) {
            Ok(signer) => {
                info!("Successfully reconnected to {}:{}", host, port);
                *retry_count = 0;
                return Ok(signer);
            }
            Err(e) => {
                error!("Reconnection failed for {}:{} - {}", host, port, e);
            }
        }
    }
}
