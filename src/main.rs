mod config;
mod connection;
mod protocol;
mod signer;
mod tcp;
mod types;
mod versions;

use config::{Config, ProtocolVersionConfig};
use connection::open_secret_connection;
use signer::{NativeSigner, SigningBackend};
use tcp::TcpSigner;
use versions::{ProtocolVersion, VersionV0_34, VersionV0_37, VersionV0_38, VersionV1_0};

trait Signer {
    fn run(&mut self) -> Result<(), Box<dyn std::error::Error>>;
}

impl<T: SigningBackend, V: ProtocolVersion> Signer for TcpSigner<T, V> {
    fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.run()
    }
}

fn create_signer(config: Config) -> Result<Box<dyn Signer>, Box<dyn std::error::Error>> {
    let signer = NativeSigner::from_key_file(&config.private_key_path)?;

    let identity_key = ed25519_consensus::SigningKey::try_from(
        &std::fs::read(&config.connection.identity_key_path)?[..32],
    )?;

    let connection = open_secret_connection(
        &config.connection.host,
        config.connection.port,
        identity_key,
        config.connection.to_tendermint_version(),
    )?;

    match config.version {
        ProtocolVersionConfig::V0_34 => Ok(Box::new(TcpSigner::<_, VersionV0_34>::new(
            signer,
            connection,
            config.chain_id,
        ))),
        ProtocolVersionConfig::V0_37 => Ok(Box::new(TcpSigner::<_, VersionV0_37>::new(
            signer,
            connection,
            config.chain_id,
        ))),
        ProtocolVersionConfig::V0_38 => Ok(Box::new(TcpSigner::<_, VersionV0_38>::new(
            signer,
            connection,
            config.chain_id,
        ))),
        ProtocolVersionConfig::V1_0 => Ok(Box::new(TcpSigner::<_, VersionV1_0>::new(
            signer,
            connection,
            config.chain_id,
        ))),
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config_path = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "config.toml".to_string());

    let config = Config::from_file(&config_path)?;

    println!("Loading config from: {}", config_path);
    println!("Chain ID: {}", config.chain_id);
    println!("Protocol version: {:?}", config.version);
    println!(
        "Connecting to {}:{}",
        config.connection.host, config.connection.port
    );

    let mut signer = create_signer(config)?;
    signer.run()?;

    Ok(())
}
