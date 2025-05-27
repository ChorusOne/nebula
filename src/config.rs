use nebula::SignerError;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub private_key_path: PathBuf,
    pub chain_id: String,
    pub version: ProtocolVersionConfig,
    pub connection: ConnectionConfig,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ProtocolVersionConfig {
    V0_34,
    V0_37,
    V0_38,
    V1_0,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ConnectionConfig {
    pub host: String,
    pub port: u16,
    pub identity_key_path: PathBuf,
    pub secret_connection_version: SecretConnectionVersion,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SecretConnectionVersion {
    V0_33,
    V0_34,
}

impl Config {
    pub fn from_file(path: &str) -> Result<Self, SignerError> {
        let contents = fs::read_to_string(path)?;
        let config: Config = toml::from_str(&contents)?;
        Ok(config)
    }
}

// TODO this will always be v0_34
impl ConnectionConfig {
    pub fn to_tendermint_version(&self) -> tendermint_p2p::secret_connection::Version {
        match self.secret_connection_version {
            SecretConnectionVersion::V0_33 => tendermint_p2p::secret_connection::Version::V0_33,
            SecretConnectionVersion::V0_34 => tendermint_p2p::secret_connection::Version::V0_34,
        }
    }
}
