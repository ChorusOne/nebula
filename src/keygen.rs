use base64::{Engine as _, engine::general_purpose::STANDARD};
use blst::min_pk::SecretKey as BlsSecretKey;
use ed25519_consensus::SigningKey as Ed25519SigningKey;
use k256::ecdsa::SigningKey as Secp256k1SigningKey;
use serde_json::json;
use sha2::{Digest, Sha256};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub fn generate_keys(key_type: &str) -> Result<()> {
    let mut seed = [0u8; 32];
    use rand_core::{OsRng, RngCore};
    OsRng.fill_bytes(&mut seed);

    match key_type {
        "ed25519" => generate_ed25519(&seed),
        "secp256k1" => generate_secp256k1(&seed),
        "bls12381" => generate_bls12381(&seed),
        _ => Err(format!("Unsupported key type: {}", key_type).into()),
    }
}

fn generate_ed25519(seed: &[u8; 32]) -> Result<()> {
    let sk = Ed25519SigningKey::from(*seed);
    let vk = sk.verification_key();

    let pubkey = vk.to_bytes();
    let privkey_only = sk.to_bytes();

    println!("private key: {}", STANDARD.encode(&privkey_only));

    let mut privkey_full = privkey_only.to_vec();
    privkey_full.extend_from_slice(&pubkey);

    output_tendermint_format("Ed25519", &privkey_full, &pubkey)
}

fn generate_secp256k1(seed: &[u8; 32]) -> Result<()> {
    let sk = Secp256k1SigningKey::from_bytes(seed)?;
    let vk = sk.verifying_key();

    println!("private key: {}", STANDARD.encode(&sk.to_bytes()));

    output_tendermint_format("Secp256k1", &sk.to_bytes(), &vk.to_bytes())
}

fn generate_bls12381(seed: &[u8; 32]) -> Result<()> {
    let ikm = seed;
    let sk = BlsSecretKey::key_gen(ikm, &[])
        .map_err(|e| format!("BLS key generation failed: {:?}", e))?;
    let pk = sk.sk_to_pk();

    println!("private key: {}", STANDARD.encode(&sk.to_bytes()));

    output_tendermint_format("Bls12381", &sk.to_bytes(), &pk.to_bytes())
}

fn output_tendermint_format(key_type: &str, privkey: &[u8], pubkey: &[u8]) -> Result<()> {
    let address = derive_address(pubkey);

    let output = json!({
        "address": address,
        "pub_key": {
            "type": format!("tendermint/PubKey{}", key_type),
            "value": STANDARD.encode(pubkey)
        },
        "priv_key": {
            "type": format!("tendermint/PrivKey{}", key_type),
            "value": STANDARD.encode(privkey)
        }
    });

    println!("{}", serde_json::to_string_pretty(&output)?);
    Ok(())
}

fn derive_address(pubkey: &[u8]) -> String {
    let hash = Sha256::digest(pubkey);
    hex::encode(&hash[..20]).to_uppercase()
}
