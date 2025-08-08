mod backend;
mod cluster;
mod config;
mod connection;
mod error;
mod persist;
mod proto;
mod protocol;
mod signer;
mod types;
mod versions;

use crate::backend::vault_signer_plugin::PluginVaultSigner;
use crate::backend::{
    Bls12381Signer, Ed25519Signer, Secp256k1Signer, SigningBackend,
    vault_transit::TransitVaultSigner,
};
use crate::config::SigningMode;
use crate::error::SignerError;
use crate::protocol::Response;
use cluster::SignerRaftNode;
use config::{Config, PersistConfig, ProtocolVersionConfig};
use connection::open_secret_connection;
use log::{debug, error, info, warn};
use persist::{Persist, PersistVariants};
use protocol::{CheckedRequest, Request, ValidRequest};
use signer::Signer;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::str::FromStr;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use tendermint_p2p::secret_connection::SecretConnection;
use types::{ConsensusData, KeyType};
use versions::{ProtocolVersion, VersionV0_34, VersionV0_37, VersionV0_38, VersionV1_0};

fn main() -> Result<(), SignerError> {
    let config_path = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "config.toml".to_string());
    let config = Config::from_file(&config_path)?;

    env_logger::Builder::new()
        .filter_level(
            log::LevelFilter::from_str(&config.log_level).unwrap_or(log::LevelFilter::Info),
        )
        .init();

    info!("Loading config from: {}", config_path);
    info!("Chain ID: {}", config.chain_id);
    info!("Protocol version: {:?}", config.version);
    let state_persist: Arc<Mutex<PersistVariants>> = match &config.persist {
        PersistConfig::Raft { raft } => {
            info!("Node ID: {}", raft.node_id);
            Arc::new(Mutex::new(PersistVariants::Raft(SignerRaftNode::new(
                raft.clone(),
            ))))
        }
        PersistConfig::Local { local } => {
            info!("Local persistence path: {:?}", local.path);
            Arc::new(Mutex::new(PersistVariants::Local(
                persist::LocalState::new(&ConsensusData {
                    height: 0,
                    round: 0,
                    step: 0,
                }),
            )))
        }
    };

    loop {
        let result = match config.version {
            ProtocolVersionConfig::V0_34 => run_leader::<VersionV0_34>(&config, &state_persist),
            ProtocolVersionConfig::V0_37 => run_leader::<VersionV0_37>(&config, &state_persist),
            ProtocolVersionConfig::V0_38 => run_leader::<VersionV0_38>(&config, &state_persist),
            ProtocolVersionConfig::V1_0 => run_leader::<VersionV1_0>(&config, &state_persist),
        };

        match result {
            Ok(()) => warn!("Leader loop exited normally"),
            Err(e) => error!("Leader loop error: {}", e),
        }
    }
}

// TODO: bring this back?
fn wait_for_leader(raft_node: &Arc<SignerRaftNode>) {
    while raft_node.leader_id().is_none() {
        thread::sleep(Duration::from_millis(200));
    }
    info!("Current leader: {}", raft_node.leader_id().unwrap());
}

// TODO: bring this back?
fn wait_as_follower(raft_node: &Arc<SignerRaftNode>) {
    info!("This node is a follower, standing by…");
    while !raft_node.is_leader() {
        thread::sleep(Duration::from_secs(1));
        if let Some(leader) = raft_node.leader_id() {
            info!("Leader is: {}", leader);
        }
    }
}

fn run_leader<V: ProtocolVersion + Send + 'static>(
    config: &Config,
    persist: &Arc<Mutex<PersistVariants>>,
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

            info!("connecting to {host}:{port}");
            thread::spawn(move || handle_connection::<V>(host, port, config, p))
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
) -> Result<(), SignerError> {
    let mut retry_count = 0;
    let identity_key = ed25519_consensus::SigningKey::new(rand_core::OsRng);

    let mut signer = create_signer::<V>(&host, port, &identity_key, &config)?;

    loop {
        let response = handle_single_request(&mut signer, &persist);
        if let Err(ref e) = response {
            error!("Error handling request from {}:{} - {}", host, port, e);
            match reconnect::<V>(&host, port, &identity_key, &config, &mut retry_count) {
                Ok(new_signer) => signer = new_signer,
                Err(_) => continue,
            }
        } else {
            retry_count = 0;
        }
    }
}

enum RequestProcessingAction<V: ProtocolVersion> {
    PersistAndSign {
        request: ValidRequest,
        new_state: ConsensusData,
    },
    ReplyWith(Response<V::ProposalResponse, V::VoteResponse, V::PubKeyResponse, V::PingResponse>),
    ShowPublicKey,
}

fn process_request<T: SigningBackend, V: ProtocolVersion>(
    request: Request,
    consensus_state: &ConsensusData,
) -> RequestProcessingAction<V> {
    match request {
        Request::Proposal(proposal) => match proposal.check(consensus_state) {
            CheckedRequest::ValidRequest {
                request,
                cd: new_state,
            } => RequestProcessingAction::PersistAndSign { request, new_state },
            CheckedRequest::DoubleSignProposal(cd) => RequestProcessingAction::ReplyWith(
                Response::SignedProposal(V::create_double_sign_prop_response(&cd)),
            ),
            CheckedRequest::DoubleSignVote(cd) => RequestProcessingAction::ReplyWith(
                Response::SignedVote(V::create_double_sign_vote_response(&cd)),
            ),
        },
        Request::Vote(vote) => match vote.check(consensus_state) {
            CheckedRequest::ValidRequest {
                request,
                cd: new_state,
            } => RequestProcessingAction::PersistAndSign { request, new_state },
            CheckedRequest::DoubleSignProposal(cd) => RequestProcessingAction::ReplyWith(
                Response::SignedProposal(V::create_double_sign_prop_response(&cd)),
            ),
            CheckedRequest::DoubleSignVote(cd) => RequestProcessingAction::ReplyWith(
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
        RequestProcessingAction::PersistAndSign { request, new_state } => {
            if let Err(e) = guard.persist(&new_state) {
                error!("Could not persist state: {e:?}");
                V::create_error_response(&format!("Cannot persist new consensus state: {e:?}"))
            } else {
                // TODO: signer should only be able to sign with a PersistedValue or similar
                signer.sign(request)?
            }
        }
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

fn create_signer<V: ProtocolVersion>(
    host: &str,
    port: u16,
    identity_key: &ed25519_consensus::SigningKey,
    config: &Config,
) -> Result<Signer<Box<dyn SigningBackend>, V, SecretConnection<TcpStream>>, SignerError> {
    info!("Connecting to CometBFT at {}:{}", host, port);

    let conn = open_secret_connection(
        host,
        port,
        identity_key.clone(),
        tendermint_p2p::secret_connection::Version::V0_34,
    )?;

    let backend = create_backend(config)?;

    Ok(Signer::new(backend, conn, config.chain_id.clone()))
}

fn create_backend(config: &Config) -> Result<Box<dyn SigningBackend>, SignerError> {
    match config.signing_mode {
        SigningMode::Native => {
            let native = config.signing.native.as_ref().unwrap();
            let path = &native.private_key_path;

            match native.key_type {
                KeyType::Ed25519 => Ok(Box::new(Ed25519Signer::from_key_file(path)?)),
                KeyType::Secp256k1 => Ok(Box::new(Secp256k1Signer::from_key_file(path)?)),
                KeyType::Bls12381 => Ok(Box::new(Bls12381Signer::from_key_file(path)?)),
            }
        }
        SigningMode::VaultTransit => {
            let vault = config.signing.vault.as_ref().unwrap();
            Ok(Box::new(TransitVaultSigner::new(vault.clone())?))
        }
        SigningMode::VaultSignerPlugin => {
            let cfg = config.signing.vault.as_ref().unwrap();
            Ok(Box::new(PluginVaultSigner::new(cfg.clone())?))
        }
    }
}

fn reconnect<V: ProtocolVersion>(
    host: &str,
    port: u16,
    identity_key: &ed25519_consensus::SigningKey,
    config: &Config,
    retry_count: &mut u32,
) -> Result<Signer<Box<dyn SigningBackend>, V, SecretConnection<TcpStream>>, SignerError> {
    const MAX_RETRY_DELAY: Duration = Duration::from_secs(30);

    loop {
        *retry_count += 1;
        let delay =
            Duration::from_millis(100 * 2_u64.pow((*retry_count).min(10))).min(MAX_RETRY_DELAY);

        warn!(
            "Reconnection attempt {} for {}:{} in {:?}",
            retry_count, host, port, delay
        );
        thread::sleep(delay);

        match create_signer::<V>(host, port, identity_key, config) {
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
