use base64::Engine;
use base64::engine::general_purpose;
use log::info;
use nebula::SignerError;
use reqwest::blocking::{Client, ClientBuilder};
use reqwest::header::{HeaderMap, HeaderValue};
use serde_json::Value;
use std::time::Duration;

use crate::backend::{PublicKey, SigningBackend};
use crate::config::VaultConfig;

pub struct VaultSigner {
    client: Client,
    base_url: String,
    transit_path: String,
    key_name: String,
    token: String,
    pub_key: PublicKey,
}

impl VaultSigner {
    pub fn new(cfg: VaultConfig) -> Result<Self, SignerError> {
        info!("starting vault signer");
        let base_url = cfg.address.trim_end_matches('/').to_string();

        let client = if cfg.skip_verify {
            ClientBuilder::new()
                .danger_accept_invalid_certs(true)
                .timeout(Duration::from_secs(10))
                .build()?
        } else {
            ClientBuilder::new()
                .timeout(Duration::from_secs(10))
                .build()?
        };

        let pub_key = Self::fetch_public_key(
            &client,
            &base_url,
            &cfg.transit_path,
            &cfg.key_name,
            &cfg.token,
        )?;

        Ok(VaultSigner {
            client,
            base_url,
            transit_path: cfg.transit_path,
            key_name: cfg.key_name,
            token: cfg.token,
            pub_key,
        })
    }

    fn fetch_public_key(
        client: &Client,
        base_url: &str,
        transit_path: &str,
        key_name: &str,
        token: &str,
    ) -> Result<PublicKey, SignerError> {
        let url = format!("{}/v1/{}/keys/{}", base_url, transit_path, key_name);
        info!("fetching vault pubkey from {}", url);

        let mut headers = HeaderMap::new();
        headers.insert("X-Vault-Token", HeaderValue::from_str(token)?);

        let resp = client
            .get(&url)
            .headers(headers)
            .send()?
            .error_for_status()?
            .json::<Value>()?;

        let data = &resp["data"];
        let key_type = data["type"]
            .as_str()
            .ok_or_else(|| SignerError::VaultError("invalid key type".to_string()))?;
        let latest_version = data["latest_version"].as_i64().ok_or_else(|| {
            SignerError::VaultError("missing or invalid latest_version".to_string())
        })?;
        let version_str = latest_version.to_string();

        let keys_map = data["keys"]
            .as_object()
            .ok_or_else(|| SignerError::VaultError("missing or invalid keys map".to_string()))?;
        let entry = keys_map.get(&version_str).ok_or_else(|| {
            SignerError::VaultError(format!(
                "no key entry for version {} under keys field",
                version_str
            ))
        })?;
        let public_key_b64 = entry["public_key"].as_str().ok_or_else(|| {
            SignerError::VaultError("public_key field missing or not a string".to_string())
        })?;

        let raw = general_purpose::STANDARD.decode(public_key_b64)?;
        if raw.len() != 32 {
            return Err(SignerError::VaultError(format!(
                "unexpected ed25519 public key length: {}",
                raw.len()
            )));
        }
        Ok(PublicKey {
            bytes: raw,
            key_type: key_type.try_into()?,
        })
    }

    fn sign_data(&self, data: &[u8]) -> Result<Vec<u8>, SignerError> {
        let url = format!(
            "{}/v1/{}/sign/{}",
            self.base_url, self.transit_path, self.key_name
        );

        let b64_input = general_purpose::STANDARD.encode(data);
        let body = serde_json::json!({
            "input": b64_input,
            "prehashed": false
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

        let raw_sig_field = resp["data"]["signature"].as_str().ok_or_else(|| {
            SignerError::VaultError("Vault returned no signature field".to_string())
        })?;

        let parts: Vec<&str> = raw_sig_field.split(':').collect();
        if parts.len() != 3 {
            return Err(SignerError::VaultError(format!(
                "invalid Vault signature format: {}",
                raw_sig_field
            )));
        }
        let sig_bytes = general_purpose::STANDARD.decode(parts[2])?;
        Ok(sig_bytes)
    }
}

impl SigningBackend for VaultSigner {
    fn sign(&self, data: &[u8]) -> Result<Vec<u8>, SignerError> {
        self.sign_data(data)
    }

    fn public_key(&self) -> Result<PublicKey, SignerError> {
        Ok(self.pub_key.clone())
    }
}
