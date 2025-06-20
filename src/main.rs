mod backend;
mod cluster;
mod config;
mod connection;
mod error;
mod handler;
mod proto;
mod protocol;
mod safeguards;
mod signer;
mod types;
mod versions;

use crate::backend::{
    Bls12381Signer, Ed25519Signer, Secp256k1Signer, SigningBackend, vault::VaultSigner,
};
use crate::config::SigningMode;
use crate::error::SignerError;
use crate::handler::SigningHandler;
use cluster::SignerRaftNode;
use config::{Config, ProtocolVersionConfig};
use connection::open_secret_connection;
use log::{error, info, warn};
use signer::Signer;
use std::net::TcpStream;
use std::str::FromStr;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use tendermint_p2p::secret_connection::SecretConnection;
use types::KeyType;
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
    info!("Node ID: {}", config.raft.node_id);

    let raft_node = SignerRaftNode::new(config.raft.clone());

    loop {
        wait_for_leader(&raft_node);

        if raft_node.is_leader() {
            info!("This node ({}) is now the leader!", config.raft.node_id);

            let result = match config.version {
                ProtocolVersionConfig::V0_34 => run_leader::<VersionV0_34>(&config, &raft_node),
                ProtocolVersionConfig::V0_37 => run_leader::<VersionV0_37>(&config, &raft_node),
                ProtocolVersionConfig::V0_38 => run_leader::<VersionV0_38>(&config, &raft_node),
                ProtocolVersionConfig::V1_0 => run_leader::<VersionV1_0>(&config, &raft_node),
            };

            match result {
                Ok(()) => warn!("Leader loop exited normally"),
                Err(e) => error!("Leader loop error: {}", e),
            }
        } else {
            wait_as_follower(&raft_node);
        }
    }
}

fn wait_for_leader(raft_node: &Arc<SignerRaftNode>) {
    while raft_node.leader_id().is_none() {
        thread::sleep(Duration::from_millis(200));
    }
    info!("Current leader: {}", raft_node.leader_id().unwrap());
}

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
    raft_node: &Arc<SignerRaftNode>,
) -> Result<(), SignerError> {
    info!(
        "Running leader loop for {} connections",
        config.connections.len()
    );

    let config = Arc::new(config.clone());
    let signing_lock = Arc::new(Mutex::new(())); // TODO: a lot of stuff depends on this single lock

    let handles: Vec<_> = config
        .connections
        .iter()
        .map(|conn| {
            let raft_node = Arc::clone(raft_node);
            let config = Arc::clone(&config);
            let signing_lock = Arc::clone(&signing_lock);
            let host = conn.host.clone();
            let port = conn.port;

            thread::spawn(move || {
                handle_connection::<V>(host, port, config, raft_node, signing_lock)
            })
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
    raft_node: Arc<SignerRaftNode>,
    signing_lock: Arc<Mutex<()>>,
) -> Result<(), SignerError> {
    let mut retry_count = 0;
    let identity_key = ed25519_consensus::SigningKey::new(rand_core::OsRng);

    let mut signer = create_signer::<V>(&host, port, &identity_key, &config)?;

    loop {
        if !raft_node.is_leader() {
            warn!("Leadership lost for {}:{}", host, port);
            break;
        }

        match SigningHandler::<V>::handle_single_request(&mut signer, &raft_node, &signing_lock) {
            Ok(()) => {
                retry_count = 0;
            }
            Err(e) => {
                if !raft_node.is_leader() {
                    break;
                }

                error!("Error handling request from {}:{} - {}", host, port, e);

                match reconnect::<V>(
                    &host,
                    port,
                    &identity_key,
                    &config,
                    &raft_node,
                    &mut retry_count,
                ) {
                    Ok(new_signer) => signer = new_signer,
                    Err(_) => continue,
                }
            }
        }
    }

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
                KeyType::Bls12_381 => Ok(Box::new(Bls12381Signer::from_key_file(path)?)),
            }
        }
        SigningMode::Vault => {
            let vault = config.signing.vault.as_ref().unwrap();
            Ok(Box::new(VaultSigner::new(vault.clone())?))
        }
    }
}

fn reconnect<V: ProtocolVersion>(
    host: &str,
    port: u16,
    identity_key: &ed25519_consensus::SigningKey,
    config: &Config,
    raft_node: &Arc<SignerRaftNode>,
    retry_count: &mut u32,
) -> Result<Signer<Box<dyn SigningBackend>, V, SecretConnection<TcpStream>>, SignerError> {
    const MAX_RETRY_DELAY: Duration = Duration::from_secs(30);

    loop {
        if !raft_node.is_leader() {
            return Err(SignerError::Other("Leadership lost".into()));
        }

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
