mod backend;
mod cluster;
mod config;
mod connection;
mod error;
mod keygen;
mod persist;
mod proto;
mod protocol;
mod signer;
mod types;
mod versions;

use crate::backend::SigningBackend;
use crate::cluster::RaftEvent;
use crate::error::SignerError;
use crate::protocol::Response;
use clap::{Parser as _, Subcommand};
use cluster::SignerRaftNode;
use config::{Config, PersistConfig, ProtocolVersionConfig};
use log::{LevelFilter, debug, error, info, warn};
use persist::{Persist, PersistVariants};
use protocol::{CheckedProposalRequest, CheckedVoteRequest, Request, ValidRequest};
use signer::Signer;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::str::FromStr;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex, mpsc};
use std::thread;
use std::time::Duration;
use tendermint_p2p::secret_connection::SecretConnection;
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

fn start_signer(config: Config) -> Result<(), SignerError> {
    info!("Chain ID: {}", config.chain_id);
    info!("Protocol version: {:?}", config.version);
    let signing_backend = crate::backend::create_backend(&config)?;
    let pub_key = signing_backend.public_key()?;
    info!("Public key: {}", pub_key);

    let (tx, rx) = mpsc::channel::<RaftEvent>();
    let mut raft_node_id: Option<u64> = None;
    let state_persist: Arc<Mutex<PersistVariants>> = match &config.persist {
        PersistConfig::Raft { raft } => {
            info!("Node ID: {}", raft.node_id);
            raft_node_id = Some(raft.node_id);
            Arc::new(Mutex::new(PersistVariants::Raft(SignerRaftNode::new(
                raft.clone(),
                tx.clone(),
            ))))
        }
        PersistConfig::Local { local } => {
            info!("Local persistence path: {:?}", local.path);
            Arc::new(Mutex::new(PersistVariants::Local(
                persist::LocalState::from_file(&local.path).expect("Failed to read local state"),
            )))
        }
    };

    if let Some(node_id) = raft_node_id {
        let mut leader_loop: Option<LeaderLoopHandle> = None;

        if let Ok(guard) = state_persist.lock() {
            if let PersistVariants::Raft(node) = &*guard {
                if node.is_leader() {
                    leader_loop = Some(start_leader_loop(
                        config.clone(),
                        Arc::clone(&state_persist),
                    ));
                }
            }
        }

        loop {
            match rx.recv() {
                Ok(RaftEvent::LeadershipChanged(from, to)) => {
                    info!("Leadership changed from: {}, to: {}", from, to);
                    if to == node_id {
                        if leader_loop.is_none() {
                            leader_loop = Some(start_leader_loop(
                                config.clone(),
                                Arc::clone(&state_persist),
                            ));
                        }
                    } else if leader_loop.is_some() {
                        stop_leader_loop(&mut leader_loop);
                    }
                }
                Err(_) => {
                    warn!("Raft event channel closed; shutting down leader loop");
                    stop_leader_loop(&mut leader_loop);
                    break;
                }
            }
        }

        return Ok(());
    }

    loop {
        let stop = Arc::new(AtomicBool::new(false));
        let result = match config.version {
            ProtocolVersionConfig::V0_34 => {
                run_leader::<VersionV0_34>(&config, &state_persist, &stop)
            }
            ProtocolVersionConfig::V0_37 => {
                run_leader::<VersionV0_37>(&config, &state_persist, &stop)
            }
            ProtocolVersionConfig::V0_38 => {
                run_leader::<VersionV0_38>(&config, &state_persist, &stop)
            }
            ProtocolVersionConfig::V1_0 => {
                run_leader::<VersionV1_0>(&config, &state_persist, &stop)
            }
        };

        match result {
            Ok(()) => warn!("Leader loop exited normally"),
            Err(e) => error!("Leader loop error: {}", e),
        }
    }
}

struct LeaderLoopHandle {
    stop: Arc<AtomicBool>,
    join: thread::JoinHandle<()>,
}

fn start_leader_loop(config: Config, persist: Arc<Mutex<PersistVariants>>) -> LeaderLoopHandle {
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
    persist: &Arc<Mutex<PersistVariants>>,
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
    persist: Arc<Mutex<PersistVariants>>,
    stop: Arc<AtomicBool>,
) -> Result<(), SignerError> {
    let mut retry_count = 0;
    let identity_key = ed25519_consensus::SigningKey::new(rand_core::OsRng);

    let mut signer = crate::signer::connect_to_cometbft_node::<V>(
        &host,
        port,
        &identity_key,
        &config,
        Some(&stop),
    )?;

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
    PersistAndSign { request: ValidRequest },
    ReplyWith(Response<V::ProposalResponse, V::VoteResponse, V::PubKeyResponse, V::PingResponse>),
    ShowPublicKey,
}

fn process_request<T: SigningBackend, V: ProtocolVersion>(
    request: Request,
    consensus_state: &ConsensusData,
) -> RequestProcessingAction<V> {
    match request {
        Request::Proposal(proposal) => match proposal.check(consensus_state) {
            CheckedProposalRequest::ValidRequest(request) => {
                RequestProcessingAction::PersistAndSign { request }
            }
            CheckedProposalRequest::DoubleSignProposal(cd) => RequestProcessingAction::ReplyWith(
                Response::SignedProposal(V::create_double_sign_prop_response(&cd)),
            ),
        },
        Request::Vote(vote) => match vote.check(consensus_state) {
            CheckedVoteRequest::ValidRequest(request) => {
                RequestProcessingAction::PersistAndSign { request }
            }
            CheckedVoteRequest::DoubleSignVote(cd) => RequestProcessingAction::ReplyWith(
                Response::SignedVote(V::create_double_sign_vote_response(&cd)),
            ),
        },
        Request::ShowPublicKey => RequestProcessingAction::ShowPublicKey,
        Request::Ping => {
            RequestProcessingAction::ReplyWith(Response::Ping(V::create_ping_response()))
        }
    }
}

pub fn handle_single_request<T: SigningBackend, V: ProtocolVersion, C: Read + Write>(
    signer: &mut Signer<T, V, C>,
    persist: &Arc<Mutex<PersistVariants>>,
) -> Result<(), SignerError> {
    let start = std::time::Instant::now();
    let request = signer.read_request()?;

    info!("Received request after {:?}", start.elapsed());
    debug!("Request: {request:?}");
    let start = std::time::Instant::now();
    let mut guard = persist.lock().unwrap();
    let consensus_state = guard.state();

    let action = process_request::<T, V>(request, &consensus_state);
    let response = match action {
        RequestProcessingAction::PersistAndSign { request } => match guard.persist(request) {
            Err(e) => {
                error!("Could not persist state: {e:?}");
                V::create_error_response(&format!("Cannot persist new consensus state: {e:?}"))
            }
            Ok(persisted) => signer.sign(persisted)?,
        },
        RequestProcessingAction::ReplyWith(response) => response,
        RequestProcessingAction::ShowPublicKey => {
            let public_key = signer.public_key()?;
            Response::PublicKey(V::create_pub_key_response(&public_key))
        }
    };

    info!("Processing request took: {:?}", start.elapsed());
    let start = std::time::Instant::now();

    debug!("Sending response to validator");
    signer.send_response(response)?;
    drop(guard);
    info!("Sending the response took: {:?}", start.elapsed());
    Ok(())
}

fn reconnect<V: ProtocolVersion>(
    host: &str,
    port: u16,
    identity_key: &ed25519_consensus::SigningKey,
    config: &Config,
    retry_count: &mut u32,
    stop: &AtomicBool,
) -> Result<Signer<Box<dyn SigningBackend>, V, SecretConnection<TcpStream>>, SignerError> {
    const MAX_RETRY_DELAY: Duration = Duration::from_secs(30);

    loop {
        if stop.load(Ordering::SeqCst) {
            return Err(SignerError::Other(
                "Connection attempt cancelled".to_string(),
            ));
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
            return Err(SignerError::Other(
                "Connection attempt cancelled".to_string(),
            ));
        }

        match crate::signer::connect_to_cometbft_node::<V>(
            host,
            port,
            identity_key,
            config,
            Some(stop),
        ) {
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
