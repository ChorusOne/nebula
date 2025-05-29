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

    #[error("Not sure what kind of error it should be")]
    TODO,

    #[error("Malformed config")]
    InvalidConfig,

    #[error("Invalid timestamp")]
    InvalidTimestamp,

    #[error("Signer attempted double signing")]
    DoubleSignError,
}
impl From<base64::DecodeError> for SignerError {
    fn from(_b64_error: base64::DecodeError) -> SignerError {
        SignerError::TODO // possibly invalid data? but i'd like to convey in the error what kind of data did we wanna decode
    }
}

impl From<std::array::TryFromSliceError> for SignerError {
    fn from(_slice_error: std::array::TryFromSliceError) -> SignerError {
        SignerError::TODO // something like invalid key
    }
}

impl From<ed25519_consensus::Error> for SignerError {
    fn from(_ed25519_error: ed25519_consensus::Error) -> SignerError {
        SignerError::TODO // something like invalid key... too?
    }
}

impl From<toml::de::Error> for SignerError {
    fn from(_toml_error: toml::de::Error) -> SignerError {
        SignerError::InvalidConfig
    }
}
