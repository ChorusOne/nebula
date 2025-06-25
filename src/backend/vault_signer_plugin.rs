use crate::error::SignerError;
use hex;
use log::info;
use reqwest::blocking::{Client, ClientBuilder};
use reqwest::header::{HeaderMap, HeaderValue};
use serde_json::Value;
use std::time::Duration;

use crate::backend::{PublicKey, SigningBackend};
use crate::config::VaultSignerConfig;

pub struct PluginVaultSigner {
    client: Client,
    base_url: String,
    mount_path: String,
    key_name: String,
    token: String,
    pub_key: PublicKey,
}

impl PluginVaultSigner {
    pub fn new(cfg: VaultSignerConfig) -> Result<Self, SignerError> {
        info!("starting vault signer plugin");
        let base_url = cfg.address.trim_end_matches('/').to_string();

        let client = ClientBuilder::new()
            .danger_accept_invalid_certs(cfg.skip_verify)
            .timeout(Duration::from_secs(10))
            .build()?;

        let pub_key = Self::fetch_public_key(
            &client,
            &base_url,
            &cfg.transit_path,
            &cfg.key_name,
            &cfg.token,
        )?;

        Ok(PluginVaultSigner {
            client,
            base_url,
            mount_path: cfg.transit_path,
            key_name: cfg.key_name,
            token: cfg.token,
            pub_key,
        })
    }

    fn fetch_public_key(
        client: &Client,
        base_url: &str,
        mount_path: &str,
        key_name: &str,
        token: &str,
    ) -> Result<PublicKey, SignerError> {
        let url = format!(
            "{}/v1/{}/keys/{}?compress=false",
            base_url, mount_path, key_name
        );
        info!("fetching vault pubkey from {}", url);

        let mut headers = HeaderMap::new();
        headers.insert("X-Vault-Token", HeaderValue::from_str(token)?);

        let resp = client
            .get(&url)
            .headers(headers)
            .send()?
            .error_for_status()?
            .json::<Value>()?;

        let data = resp
            .get("data")
            .ok_or_else(|| SignerError::VaultError("missing data field".to_string()))?;

        let key_type = data["key_type"]
            .as_str()
            .ok_or_else(|| SignerError::VaultError("invalid key type".to_string()))?;

        let public_key_hex = data["public_key"]
            .as_str()
            .ok_or_else(|| SignerError::VaultError("missing public_key field".to_string()))?;

        let raw = hex::decode(public_key_hex).unwrap();

        let expected_len = match key_type {
            "ed25519" => 32,
            "secp256k1" => 33,
            "bls12381" => 96,
            _ => {
                return Err(SignerError::VaultError(format!(
                    "unsupported key type: {}",
                    key_type
                )));
            }
        };

        if raw.len() != expected_len {
            return Err(SignerError::VaultError(format!(
                "unexpected {} public key length: {} (expected {})",
                key_type,
                raw.len(),
                expected_len
            )));
        }

        Ok(PublicKey {
            bytes: raw,
            key_type: key_type.try_into()?,
        })
    }

    fn sign_data(&self, data: &[u8]) -> Result<Vec<u8>, SignerError> {
        let url = format!(
            "{}/v1/{}/keys/{}/sign/cometbft",
            self.base_url, self.mount_path, self.key_name
        );

        let hex_message = hex::encode(data);
        let body = serde_json::json!({
            "message": hex_message
        });

        let mut headers = HeaderMap::new();
        headers.insert("X-Vault-Token", HeaderValue::from_str(&self.token)?);

        let resp = self
            .client
            .post(&url)
            .headers(headers)
            .json(&body)
            .send()?
            .error_for_status()?
            .json::<Value>()?;

        let data = resp
            .get("data")
            .ok_or_else(|| SignerError::VaultError("missing data field in response".to_string()))?;

        let signature_hex = data["signature"].as_str().ok_or_else(|| {
            SignerError::VaultError("Vault returned no signature field".to_string())
        })?;

        let sig_bytes = hex::decode(signature_hex).unwrap();

        let expected_sig_len = match self.pub_key.key_type {
            crate::types::KeyType::Ed25519 => 64,
            crate::types::KeyType::Secp256k1 => 64,
            crate::types::KeyType::Bls12381 => 96,
        };

        if sig_bytes.len() != expected_sig_len {
            return Err(SignerError::VaultError(format!(
                "unexpected signature length: {} (expected {})",
                sig_bytes.len(),
                expected_sig_len
            )));
        }

        Ok(sig_bytes)
    }
}

impl SigningBackend for PluginVaultSigner {
    fn sign(&mut self, data: &[u8]) -> Result<Vec<u8>, SignerError> {
        self.sign_data(data)
    }

    fn public_key(&self) -> Result<PublicKey, SignerError> {
        Ok(self.pub_key.clone())
    }
}
