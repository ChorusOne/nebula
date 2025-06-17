use crate::error::SignerError;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct PeerConfig {
    pub id: u64,
    pub addr: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct RaftConfig {
    pub node_id: u64,
    pub bind_addr: String,
    pub data_path: String,
    pub peers: Vec<PeerConfig>,
    pub initial_state_path: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Config {
    pub chain_id: String,
    pub version: ProtocolVersionConfig,
    pub connections: Vec<ConnectionConfig>,
    pub signing_mode: SigningMode,

    pub raft: RaftConfig,

    #[serde(default)]
    pub signing: SigningConfigs,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum ProtocolVersionConfig {
    V0_34,
    V0_37,
    V0_38,
    V1_0,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ConnectionConfig {
    pub host: String,
    pub port: u16,
}

impl Config {
    pub fn from_file(path: &str) -> Result<Self, SignerError> {
        let contents = fs::read_to_string(path)?;
        let config: Config = toml::from_str(&contents)?;
        Ok(config)
    }
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "snake_case")]
pub enum SigningMode {
    Vault,
    Native,
}

impl Default for SigningMode {
    fn default() -> Self {
        SigningMode::Native
    }
}

#[derive(Debug, Deserialize, Serialize, Default, Clone)]
pub struct SigningConfigs {
    #[serde(default)]
    pub vault: Option<VaultConfig>,

    #[serde(default)]
    pub native: Option<NativeConfig>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct VaultConfig {
    pub address: String,
    pub token: String,
    pub transit_path: String,
    pub key_name: String,
    pub cacert: Option<String>,
    pub skip_verify: bool,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct NativeConfig {
    pub private_key_path: PathBuf,
    pub key_type: crate::types::KeyType,
}
