pub mod vault;

use base64::{Engine as _, engine::general_purpose};
use nebula::SignerError;
use std::fs;
use std::path::Path;

pub trait SigningBackend {
    fn sign(&self, data: &[u8]) -> Result<Vec<u8>, SignerError>;

    fn public_key(&self) -> Result<Vec<u8>, SignerError>;
}

impl SigningBackend for Box<dyn SigningBackend> {
    fn sign(&self, data: &[u8]) -> Result<Vec<u8>, SignerError> {
        (**self).sign(data)
    }

    fn public_key(&self) -> Result<Vec<u8>, SignerError> {
        (**self).public_key()
    }
}

pub struct Ed25519Signer {
    signing_key: ed25519_consensus::SigningKey,
}

impl Ed25519Signer {
    pub fn from_key_file<P: AsRef<Path>>(path: P) -> Result<Self, SignerError> {
        let key_string = fs::read_to_string(path)?.trim().to_string();

        let key_bytes = general_purpose::STANDARD.decode(key_string)?;

        let signing_key = if key_bytes.len() == 32 || key_bytes.len() == 64 {
            let mut seed = [0u8; 32];
            seed.copy_from_slice(&key_bytes[..32]);
            ed25519_consensus::SigningKey::from(seed)
        } else {
            return Err(SignerError::EncodingError(format!(
                "invalid ed25519 key length: got {} bytes",
                key_bytes.len()
            )));
        };

        Ok(Self { signing_key })
    }
}

impl SigningBackend for Ed25519Signer {
    fn sign(&self, data: &[u8]) -> Result<Vec<u8>, SignerError> {
        Ok(self.signing_key.sign(data).to_bytes().to_vec())
    }

    fn public_key(&self) -> Result<Vec<u8>, SignerError> {
        Ok(self.signing_key.verification_key().to_bytes().to_vec())
    }
}
