use super::backend::SigningBackend;
use base64::{Engine as _, engine::general_purpose};
use ed25519_dalek::{Signer, SigningKey};
use nebula::SignerError;
use std::fs;
use std::path::Path;

pub struct NativeSigner {
    signing_key: SigningKey,
}

impl NativeSigner {
    pub fn from_key_file<P: AsRef<Path>>(path: P) -> Result<Self, SignerError> {
        let key_string = fs::read_to_string(path)?.trim().to_string();
        let key_bytes = general_purpose::STANDARD.decode(key_string)?;

        let signing_key = if key_bytes.len() == 32 || key_bytes.len() == 64 {
            SigningKey::from(<[u8; 32]>::try_from(&key_bytes[..32])?)
        } else {
            return Err(SignerError::TODO);
        };

        Ok(Self { signing_key })
    }
}

impl SigningBackend for NativeSigner {
    fn sign(&self, data: &[u8]) -> Vec<u8> {
        self.signing_key.sign(data).to_bytes().to_vec()
    }

    fn public_key(&self) -> Vec<u8> {
        self.signing_key.verifying_key().to_bytes().to_vec()
    }
}
