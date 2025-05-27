mod backend;
mod config;
mod connection;
mod protocol;
mod signer;
mod types;
mod versions;

use backend::{NativeSigner, SigningBackend};
use config::{Config, ProtocolVersionConfig};
use connection::open_secret_connection;
use log::{error, info};
use signer::Signer;
use std::net::TcpStream;
use std::thread::sleep;
use std::time::Duration;
use tendermint_p2p::secret_connection::SecretConnection;
use versions::{VersionV0_34, VersionV0_37, VersionV0_38, VersionV1_0};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config_path = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "config.toml".to_string());
    let config = Config::from_file(&config_path)?;

    info!("Loading config from: {}", config_path);
    info!("Chain ID: {}", config.chain_id);
    info!("Protocol version: {:?}", config.version);
    info!(
        "Connecting to {}:{}",
        config.connection.host, config.connection.port
    );

    let backend = NativeSigner::from_key_file(&config.private_key_path)?;

    let identity_key = ed25519_consensus::SigningKey::try_from(
        &std::fs::read(&config.connection.identity_key_path)?[..32],
    )?;

    let conn = open_secret_connection(
        &config.connection.host,
        config.connection.port,
        identity_key,
        config.connection.to_tendermint_version(),
    )?;

    info!("Starting request loop for chain: {}", config.chain_id);

    match config.version {
        ProtocolVersionConfig::V0_34 => {
            let mut signer = Signer::<NativeSigner, VersionV0_34, SecretConnection<TcpStream>>::new(
                backend,
                conn,
                config.chain_id,
            );
            run_signer_loop(&mut signer)?;
        }
        ProtocolVersionConfig::V0_37 => {
            let mut signer = Signer::<NativeSigner, VersionV0_37, SecretConnection<TcpStream>>::new(
                backend,
                conn,
                config.chain_id,
            );
            run_signer_loop(&mut signer)?;
        }
        ProtocolVersionConfig::V0_38 => {
            let mut signer = Signer::<NativeSigner, VersionV0_38, SecretConnection<TcpStream>>::new(
                backend,
                conn,
                config.chain_id,
            );
            run_signer_loop(&mut signer)?;
        }
        ProtocolVersionConfig::V1_0 => {
            let mut signer = Signer::<NativeSigner, VersionV1_0, SecretConnection<TcpStream>>::new(
                backend,
                conn,
                config.chain_id,
            );
            run_signer_loop(&mut signer)?;
        }
    }

    Ok(())
}

fn run_signer_loop<
    T: SigningBackend,
    V: versions::ProtocolVersion,
    C: std::io::Read + std::io::Write,
>(
    signer: &mut Signer<T, V, C>,
) -> Result<(), Box<dyn std::error::Error>> {
    loop {
        match handle_single_request(signer) {
            Ok(()) => {}
            Err(e) => {
                error!("Error handling request: {}. Continuing...", e);
                sleep(Duration::from_millis(100));
            }
        }
    }
}

pub fn handle_single_request<
    T: SigningBackend,
    V: versions::ProtocolVersion,
    C: std::io::Read + std::io::Write,
>(
    signer: &mut Signer<T, V, C>,
) -> Result<(), Box<dyn std::error::Error>> {
    let req = signer.read_request()?;
    info!("Received request: {:?}", req);

    let resp = signer.process_request(req)?;
    signer.send_response(resp)?;

    Ok(())
}
