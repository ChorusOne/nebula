use std::fmt;

#[derive(Debug)]
pub enum SignerError {
    IoError(std::io::Error),
    ConnectionError(String),
    ProtocolError(String),
    InvalidData,
}

impl fmt::Display for SignerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SignerError::IoError(e) => write!(f, "IO error: {}", e),
            SignerError::ConnectionError(msg) => write!(f, "Connection error: {}", msg),
            SignerError::ProtocolError(msg) => write!(f, "Protocol error: {}", msg),
            SignerError::InvalidData => write!(f, "Invalid data received"),
        }
    }
}

impl std::error::Error for SignerError {}

impl From<std::io::Error> for SignerError {
    fn from(error: std::io::Error) -> Self {
        SignerError::IoError(error)
    }
}
