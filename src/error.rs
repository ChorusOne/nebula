use thiserror::Error;

#[derive(Debug, Error)]
pub enum SignerError {
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Connection error: {0}")]
    ConnectionError(String),

    #[error("Protocol error: {0}")]
    ProtocolError(String),

    #[error("Unreadable data received from the CometBFT node")]
    InvalidData,

    #[error("Unable to parse request from the CometBFT node")]
    RequestParseError,

    #[error("Encoding error: {0}")]
    EncodingError(String),

    #[error("Prost encode error: {0}")]
    ProstError(#[from] prost::EncodeError),

    #[error("Prost decode error: {0}")]
    ProstDecodeError(#[from] prost::DecodeError),

    #[error("Unsupported message type")]
    UnsupportedMessageType,

    #[error("Malformed config: {0}")]
    InvalidConfig(toml::de::Error),

    #[error("Invalid timestamp")]
    InvalidTimestamp,

    #[error("Signer attempted double signing")]
    DoubleSignError,

    #[error("Todo: signing: {0}")]
    Other(String),

    #[error("Raft state replication failed: {0}")]
    StateReplication(String),

    #[error("This node: {0} is not the leader")]
    NotLeader(String),

    #[error("http error: {0}")]
    Http(#[from] reqwest::Error),

    #[error("Error producing a Vault signature: {0}")]
    VaultSigningError(String),

    #[error("Todo: signing: {0}")]
    Crypto(String),

    #[error("Invalid public key: {0}")]
    InvalidPublicKey(String),

    #[error("Config validation error: {0}")]
    ConfigError(String),
}

impl From<base64::DecodeError> for SignerError {
    fn from(_b64_error: base64::DecodeError) -> SignerError {
        SignerError::InvalidData // possibly invalid data? but i'd like to convey in the error what kind of data did we wanna decode
    }
}

impl From<std::array::TryFromSliceError> for SignerError {
    fn from(error: std::array::TryFromSliceError) -> SignerError {
        SignerError::InvalidPublicKey(error.to_string())
    }
}

impl From<ed25519_consensus::Error> for SignerError {
    fn from(error: ed25519_consensus::Error) -> SignerError {
        SignerError::InvalidPublicKey(error.to_string())
    }
}

impl From<toml::de::Error> for SignerError {
    fn from(toml_error: toml::de::Error) -> SignerError {
        SignerError::InvalidConfig(toml_error)
    }
}

impl From<k256::ecdsa::Error> for SignerError {
    fn from(toml_error: k256::ecdsa::Error) -> SignerError {
        SignerError::Other(toml_error.to_string())
    }
}

impl From<serde_json::Error> for SignerError {
    fn from(e: serde_json::Error) -> Self {
        SignerError::Other(e.to_string())
    }
}

// impl From<reqwest::Error> for SignerError {
//     fn from(e: reqwest::Error) -> Self {
//         SignerError::HttpError(e.to_string())
//     }
// }

impl From<reqwest::header::InvalidHeaderValue> for SignerError {
    fn from(e: reqwest::header::InvalidHeaderValue) -> Self {
        SignerError::Other(e.to_string())
    }
}
