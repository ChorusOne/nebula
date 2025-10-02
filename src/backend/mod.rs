pub mod vault_signer_plugin;
pub mod vault_transit;

use crate::backend::vault_signer_plugin::PluginVaultSigner;
use crate::backend::vault_transit::TransitVaultSigner;
use crate::config::{Config, SigningMode};
use crate::error::SignerError;
use base64::{Engine as _, engine::general_purpose};
use k256::ecdsa::signature::SignerMut;
use std::fs;
use std::path::Path;

use crate::types::KeyType;

pub trait SigningBackend: Send {
    // TODO: this is mutable because of the secp256k1 signer.
    fn sign(&mut self, data: &[u8]) -> Result<Vec<u8>, SignerError>;

    fn public_key(&self) -> Result<PublicKey, SignerError>;
}

impl SigningBackend for Box<dyn SigningBackend> {
    fn sign(&mut self, data: &[u8]) -> Result<Vec<u8>, SignerError> {
        (**self).sign(data)
    }

    fn public_key(&self) -> Result<PublicKey, SignerError> {
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
    fn sign(&mut self, data: &[u8]) -> Result<Vec<u8>, SignerError> {
        Ok(self.signing_key.sign(data).to_bytes().to_vec())
    }

    fn public_key(&self) -> Result<PublicKey, SignerError> {
        Ok(PublicKey {
            bytes: self.signing_key.verification_key().to_bytes().to_vec(),
            key_type: KeyType::Ed25519,
        })
    }
}

// todo: do not use the "config" module here
#[derive(Clone)]
pub struct PublicKey {
    pub bytes: Vec<u8>,
    pub key_type: KeyType,
}

pub struct Secp256k1Signer {
    signing_key: k256::ecdsa::SigningKey,
}

impl Secp256k1Signer {
    pub fn from_key_file<P: AsRef<Path>>(path: P) -> Result<Self, SignerError> {
        let key_string = fs::read_to_string(path)?.trim().to_string();

        let key_bytes = general_purpose::STANDARD
            .decode(key_string)
            .map_err(|e| SignerError::EncodingError(e.to_string()))?;

        if key_bytes.len() != 32 {
            return Err(SignerError::EncodingError(format!(
                "invalid secp256k1 key length: got {} bytes, expected 32",
                key_bytes.len()
            )));
        }

        let signing_key = k256::ecdsa::SigningKey::from_bytes(&key_bytes)
            .map_err(|e| SignerError::EncodingError(e.to_string()))?;

        Ok(Self { signing_key })
    }
}

impl SigningBackend for Secp256k1Signer {
    fn sign(&mut self, data: &[u8]) -> Result<Vec<u8>, SignerError> {
        let signature: k256::ecdsa::Signature = self.signing_key.try_sign(data)?;
        Ok(signature.as_ref().to_vec())
        // todo
        // let signature = Vec::new();
        // Ok(signature)
    }

    fn public_key(&self) -> Result<PublicKey, SignerError> {
        let verify_key = self.signing_key.verifying_key();

        let compressed = verify_key.to_bytes(); // returns a [u8; 33]
        Ok(PublicKey {
            bytes: compressed.to_vec(),
            key_type: KeyType::Secp256k1,
        })
    }
}

pub struct Bls12381Signer {
    secret_key: blst::min_pk::SecretKey,
    dst: Vec<u8>,
}

impl Bls12381Signer {
    pub fn from_key_file<P: AsRef<Path>>(path: P, dst: String) -> Result<Self, SignerError> {
        let key_string = fs::read_to_string(path)?.trim().to_string();

        let key_bytes = general_purpose::STANDARD
            .decode(key_string)
            .map_err(|e| SignerError::EncodingError(e.to_string()))?;

        if key_bytes.len() != 32 {
            return Err(SignerError::EncodingError(format!(
                "invalid bls12_381 key length: got {} bytes, expected 32",
                key_bytes.len()
            )));
        }

        let sk = blst::min_pk::SecretKey::from_bytes(&key_bytes)
            .map_err(|_| SignerError::EncodingError("invalid bls12_381 key bytes".into()))?;

        Ok(Self {
            secret_key: sk,
            dst: dst.into_bytes(),
        })
    }
}

impl SigningBackend for Bls12381Signer {
    fn sign(&mut self, data: &[u8]) -> Result<Vec<u8>, SignerError> {
        let signature = self.secret_key.sign(data, &self.dst, &[]);

        Ok(signature.to_bytes().to_vec())
    }

    fn public_key(&self) -> Result<PublicKey, SignerError> {
        let pk = self.secret_key.sk_to_pk();

        let compressed = pk.to_bytes();
        Ok(PublicKey {
            bytes: compressed.to_vec(),
            key_type: KeyType::Bls12381,
        })
    }
}

pub fn create_backend(config: &Config) -> Result<Box<dyn SigningBackend>, SignerError> {
    match config.signing_mode {
        SigningMode::Native => {
            let native = config.signing.native.as_ref().unwrap();
            let path = &native.private_key_path;

            match native.key_type {
                KeyType::Ed25519 => Ok(Box::new(Ed25519Signer::from_key_file(path)?)),
                KeyType::Secp256k1 => Ok(Box::new(Secp256k1Signer::from_key_file(path)?)),
                KeyType::Bls12381 => Ok(Box::new(Bls12381Signer::from_key_file(
                    path,
                    config.signing.bls_dst.as_ref().unwrap().clone(),
                )?)),
            }
        }
        SigningMode::VaultTransit => {
            let vault = config.signing.vault.as_ref().unwrap();
            Ok(Box::new(TransitVaultSigner::new(vault.clone())?))
        }
        SigningMode::VaultSignerPlugin => {
            let cfg = config.signing.vault.as_ref().unwrap();
            Ok(Box::new(PluginVaultSigner::new(
                cfg.clone(),
                config.signing.bls_dst.clone(),
            )?))
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::backend::Bls12381Signer;
    use crate::backend::SigningBackend;
    use base64::{Engine as _, engine::general_purpose};
    use hex;

    #[test]
    fn bls12381_sign_known_data() {
        let private_key_b64 = "HA63eyPQgzm4mZSfsnYTZ7r601qd80EVlvVsWuuDJMs=";
        let data_hex = "63080111c80000000000000022480a20a4b80335204fa09442dccc16a2c3524e300aabfd9b80093bf70d0b71a5965a54122408011220160f36529b1d856a0888200de49caa4d0125634cb631f1272347eb13494b4f19320c626561636f6e642d32303631";
        let expected_signature_hex = "b26ce6cedb5b7f38a4e11301e4a33e83dbd3a638fb44987fc4000fd8382f1739388beee2d8fccff4ef412464b083f4fa110608e740fd8462f2051d71d3d16bb5df7e99fd74c4650c081eed07f8d7bbba23660d90578814577bbd3a42e5bcdb3f";

        let key_bytes = general_purpose::STANDARD
            .decode(private_key_b64)
            .expect("Failed to decode base64 key");

        let secret_key =
            blst::min_pk::SecretKey::from_bytes(&key_bytes).expect("Failed to create secret key");

        let dst = "BLS_SIG_BLS12381G2_XMD:SHA-256_SSWU_RO_NUL_".to_string();

        let mut signer = Bls12381Signer {
            secret_key,
            dst: dst.into_bytes(),
        };

        let data = hex::decode(data_hex).expect("Failed to decode hex data");
        let signature = signer.sign(&data).expect("Failed to sign data");
        let signature_hex = hex::encode(&signature);

        assert_eq!(signature_hex, expected_signature_hex);
        assert_eq!(signature.len(), 96);
    }
}
