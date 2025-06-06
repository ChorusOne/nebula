use nebula::SignerError;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Config {
    pub chain_id: String,
    pub version: ProtocolVersionConfig,
    pub connections: Vec<ConnectionConfig>,
    pub node_id: u64,
    pub peers: Vec<String>,
    pub cluster_port: u16,
    pub state_file: String,

    pub signing_mode: SigningMode,

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
    pub identity_key_path: PathBuf,
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

    pub key_type: KeyType,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "snake_case")]
pub enum KeyType {
    Ed25519,
    Secp256k1,
    Bls12_381,
}

impl TryFrom<&str> for KeyType {
    type Error = SignerError;
    fn try_from(key_type_str: &str) -> Result<KeyType, SignerError> {
        match key_type_str {
            "ed25519" => Ok(KeyType::Ed25519),
            "secp256k1" => Ok(KeyType::Secp256k1),
            "bls12_381" => Ok(KeyType::Bls12_381),
            _ => Err(SignerError::InvalidData),
        }
    }
}

impl From<KeyType> for String {
    fn from(key_type: KeyType) -> String {
        match key_type {
            KeyType::Ed25519 => "ed25519".to_string(),
            KeyType::Secp256k1 => "secp256k1".to_string(),
            KeyType::Bls12_381 => "bls12_381".to_string(),
        }
    }
}
