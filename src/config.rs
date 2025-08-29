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
    pub log_level: String,
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
    // todo: change this to "new_from_file" and add validation
    pub fn from_file(path: &str) -> Result<Self, SignerError> {
        let contents = fs::read_to_string(path)?;
        let config: Config = toml::from_str(&contents)?;
        Ok(config)
    }

    pub fn default_config(backend: SigningMode) -> Self {
        Config {
            log_level: "info".to_string(),
            chain_id: "test-chain-v1".to_string(),
            version: ProtocolVersionConfig::V1_0,
            connections: vec![
                ConnectionConfig {
                    host: "127.0.0.1".to_string(),
                    port: 36558,
                },
                ConnectionConfig {
                    host: "127.0.0.1".to_string(),
                    port: 26558,
                },
            ],
            signing_mode: backend.clone(),
            raft: RaftConfig {
                node_id: 1,
                bind_addr: "127.0.0.1:8080".to_string(),
                data_path: "./raft_data".to_string(),
                peers: vec![PeerConfig {
                    id: 1,
                    addr: "127.0.0.1:8080".into(),
                }],
                initial_state_path: "./initial_state.json".to_string(),
            },
            signing: match backend {
                SigningMode::Native => SigningConfigs {
                    bls_dst: None,
                    native: Some(NativeConfig {
                        private_key_path: PathBuf::from("./privkey"),
                        key_type: crate::types::KeyType::Ed25519,
                    }),
                    vault: None,
                },
                SigningMode::VaultSignerPlugin | SigningMode::VaultTransit => SigningConfigs {
                    vault: Some(VaultSignerConfig {
                        address: "https://vault.example.com:8200".to_string(),
                        token_file_path: PathBuf::new(),
                        transit_path: "transit".to_string(),
                        key_name: "signing-key".to_string(),
                        cacert: None,
                        skip_verify: false,
                    }),
                    native: None,
                    bls_dst: None,
                },
            },
        }
    }

    // fn validate() -> Result<(), SignerError> {
    //     Ok(())
    // }

    pub fn write_to_file(&self, path: &str) -> Result<(), SignerError> {
        let toml_string = toml::to_string_pretty(self).expect("Unable to write config to file");
        fs::write(path, toml_string)?;
        Ok(())
    }
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "snake_case")]
#[derive(Default, clap::ValueEnum)]
pub enum SigningMode {
    VaultTransit,
    VaultSignerPlugin,
    #[default]
    Native,
}

#[derive(Debug, Deserialize, Serialize, Default, Clone)]
pub struct SigningConfigs {
    /// Domain Separation Tag for bls12_381 signing.
    /// Vanilla CometBFT: BLS_SIG_BLS12381G2_XMD:SHA-256_SSWU_RO_NUL_
    /// Berachain:        BLS_SIG_BLS12381G2_XMD:SHA-256_SSWU_RO_NUL_
    /// Currently they are the same but they _might_ change in the future.
    pub bls_dst: Option<String>,
    #[serde(default)]
    pub vault: Option<VaultSignerConfig>,

    #[serde(default)]
    pub native: Option<NativeConfig>,
}

// Shared config for both vault signer plugin and the transit module
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct VaultSignerConfig {
    pub address: String,
    pub token_file_path: PathBuf,
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
