use crate::error::SignerError;
use hex;
use log::{debug, info, trace};
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
    bls_dst: Option<String>,
}

impl PluginVaultSigner {
    pub fn new(cfg: VaultSignerConfig, bls_dst: Option<String>) -> Result<Self, SignerError> {
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

        if pub_key.key_type == crate::KeyType::Bls12381 && bls_dst.is_none() {
            return Err(SignerError::ConfigError(
                "bls_dst is required for BLS12-381 keys".to_string(),
            ));
        }

        Ok(PluginVaultSigner {
            client,
            base_url,
            mount_path: cfg.transit_path,
            key_name: cfg.key_name,
            token: cfg.token,
            pub_key,
            bls_dst,
        })
    }

    fn fetch_public_key(
        client: &Client,
        base_url: &str,
        mount_path: &str,
        key_name: &str,
        token: &str,
    ) -> Result<PublicKey, SignerError> {
        // compressed=true is only honored when secp256k1 keys are involved
        // NOTE: maybe just use a config param here
        let url = format!(
            "{}/v1/{}/keys/{}?compressed=true",
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
            "{}/v1/{}/keys/{}/sign",
            self.base_url, self.mount_path, self.key_name,
        );

        let body = if let Some(ref dst) = self.bls_dst {
            serde_json::json!({
                "payload": base64::Engine::encode(&base64::prelude::BASE64_STANDARD, data),
                "dst": dst
            })
        } else {
            serde_json::json!({
                "payload": base64::Engine::encode(&base64::prelude::BASE64_STANDARD, data),
            })
        };

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

        trace!("response from vault: {:#?}", resp);
        let raw_sig_field = resp["data"]["signature"].as_str().ok_or_else(|| {
            SignerError::VaultError("Vault returned no signature field".to_string())
        })?;

        let parts: Vec<&str> = raw_sig_field.split(':').collect();

        // standard vault transit response has 3 parts
        // secp256k1 signer has 4
        if parts.len() > 4 {
            return Err(SignerError::VaultError(format!(
                "invalid Vault signature format: {}",
                raw_sig_field
            )));
        }
        let sig_bytes =
            base64::Engine::decode(&base64::engine::general_purpose::STANDARD, parts[2])?;

        debug!("signed bytes from vault: {:?}", sig_bytes);

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

#[cfg(test)]
mod tests {
    use crate::backend::vault_signer_plugin::PluginVaultSigner;
    use crate::config::VaultSignerConfig;
    use hex;

    // [2025-08-22T12:31:20Z DEBUG nebula::signer] Signature: b26ce6cedb5b7f38a4e11301e4a33e83dbd3a638fb44987fc4000fd8382f1739388beee2d8fccff4ef412464b083f4fa110608e740fd8462f2051d71d3d16bb5df7e99fd74c4650c081eed07f8d7bbba23660d90578814577bbd3a42e5bcdb3f
    // [2025-08-22T12:31:20Z DEBUG nebula::signer] Signable data: 63080111c80000000000000022480a20a4b80335204fa09442dccc16a2c3524e300aabfd9b80093bf70d0b71a5965a54122408011220160f36529b1d856a0888200de49caa4d0125634cb631f1272347eb13494b4f19320c626561636f6e642d32303631
    #[test]
    fn vault_sign_with_known_data() {
        let data_hex = "63080111c80000000000000022480a20a4b80335204fa09442dccc16a2c3524e300aabfd9b80093bf70d0b71a5965a54122408011220160f36529b1d856a0888200de49caa4d0125634cb631f1272347eb13494b4f19320c626561636f6e642d32303631";
        let expected_signature_hex = "b26ce6cedb5b7f38a4e11301e4a33e83dbd3a638fb44987fc4000fd8382f1739388beee2d8fccff4ef412464b083f4fa110608e740fd8462f2051d71d3d16bb5df7e99fd74c4650c081eed07f8d7bbba23660d90578814577bbd3a42e5bcdb3f";

        let vault_config = VaultSignerConfig {
            address: "http://127.0.0.1:8200".to_string(),
            token: "root".to_string(),
            transit_path: "signer".to_string(),
            key_name: "test4".to_string(),
            skip_verify: false,
            cacert: None,
        };

        let bls_dst = Some("BLS_SIG_BLS12381G2_XMD:SHA-256_SSWU_RO_NUL_".to_string());

        let vault_signer =
            PluginVaultSigner::new(vault_config, bls_dst).expect("Failed to create vault signer");

        let data = hex::decode(data_hex).expect("Failed to decode hex data");
        let signature = vault_signer.sign_data(&data).expect("Failed to sign data");
        let signature_hex = hex::encode(&signature);

        assert_eq!(signature_hex, expected_signature_hex);
        assert_eq!(signature.len(), 96);
    }
}
