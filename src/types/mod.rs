use crate::error::SignerError;
use crate::protocol::CheckedRequest;
use serde::{Deserialize, Serialize};
use thiserror::Error;

const CONSENSUS_DATA_FIELDS: &[&str] = &[
    "height",
    "round",
    "step",
    "sign_data",
    "signature",
    "ext_sign_data",
    "ext_signature",
];

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockId {
    pub hash: Vec<u8>,
    pub parts: Option<PartSetHeader>,
}

impl From<BlockId> for crate::proto::v1::types::BlockId {
    fn from(block_id: BlockId) -> crate::proto::v1::types::BlockId {
        crate::proto::v1::types::BlockId {
            hash: block_id.hash.into(),
            part_set_header: Some(block_id.parts.unwrap().into()),
        }
    }
}

impl From<PartSetHeader> for crate::proto::v1::types::PartSetHeader {
    fn from(part_set_header: PartSetHeader) -> crate::proto::v1::types::PartSetHeader {
        crate::proto::v1::types::PartSetHeader {
            total: part_set_header.total,
            hash: part_set_header.hash.into(),
        }
    }
}

impl From<crate::proto::v1::types::BlockId> for BlockId {
    fn from(block_id: crate::proto::v1::types::BlockId) -> BlockId {
        BlockId {
            hash: block_id.hash.into(),
            parts: Some(block_id.part_set_header.unwrap().into()),
        }
    }
}

impl From<crate::proto::v1::types::PartSetHeader> for PartSetHeader {
    fn from(part_set_header: crate::proto::v1::types::PartSetHeader) -> PartSetHeader {
        PartSetHeader {
            total: part_set_header.total,
            hash: part_set_header.hash.into(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PartSetHeader {
    pub total: u32,
    pub hash: Vec<u8>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum SignedMsgType {
    #[default]
    Unknown = 0,
    Prevote = 1,
    Precommit = 2,
    Proposal = 32,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Vote {
    pub step: SignedMsgType,
    pub height: i64,
    pub round: i64,
    pub timestamp: Option<i64>,
    pub block_id: Option<BlockId>,
    pub validator_address: Vec<u8>,
    pub validator_index: i32,
    pub extension: Vec<u8>,
    pub extension_signature: Vec<u8>,
}

impl std::fmt::Display for Vote {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Vote {{ step: {:?}, height: {}, round: {}, timestamp: {:?}, block_id: {:?}, validator_address: {:02X?}, validator_index: {}, extension: {:02X?}, extension_signature: {:02X?} }}",
            self.step as u8,
            self.height,
            self.round,
            self.timestamp,
            self.block_id,
            self.validator_address,
            self.validator_index,
            self.extension,
            self.extension_signature
        )
    }
}

impl From<SignedMsgType> for u8 {
    fn from(r#type: SignedMsgType) -> u8 {
        match r#type {
            SignedMsgType::Unknown => 0,
            SignedMsgType::Prevote => 1,
            SignedMsgType::Precommit => 2,
            SignedMsgType::Proposal => 32,
        }
    }
}

impl From<SignedMsgType> for i32 {
    fn from(r#type: SignedMsgType) -> i32 {
        match r#type {
            SignedMsgType::Unknown => 0,
            SignedMsgType::Prevote => 1,
            SignedMsgType::Precommit => 2,
            SignedMsgType::Proposal => 32,
        }
    }
}

impl From<u8> for SignedMsgType {
    fn from(n: u8) -> Self {
        match n {
            1 => SignedMsgType::Prevote,
            2 => SignedMsgType::Precommit,
            32 => SignedMsgType::Proposal,
            _ => SignedMsgType::Unknown,
        }
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Proposal {
    pub step: SignedMsgType,
    pub height: i64,
    pub round: i64,
    pub timestamp: Option<i64>,
    pub pol_round: i64,
    pub block_id: Option<BlockId>,
}

#[derive(Debug, Error)]
pub enum BufferError {
    #[error("Insufficient amount of bytes in the buffer")]
    NeedMoreBytes,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Clone, clap::ValueEnum)]
#[serde(rename_all = "snake_case")]
pub enum KeyType {
    Ed25519,
    Secp256k1,
    Bls12381,
}

impl TryFrom<&str> for KeyType {
    type Error = SignerError;

    fn try_from(key_type_str: &str) -> Result<KeyType, SignerError> {
        match key_type_str {
            "ed25519" => Ok(KeyType::Ed25519),
            "secp256k1" => Ok(KeyType::Secp256k1),
            "bls12_381" => Ok(KeyType::Bls12381),
            "bls12381" => Ok(KeyType::Bls12381),
            _ => Err(SignerError::InvalidData),
        }
    }
}

impl From<KeyType> for String {
    fn from(key_type: KeyType) -> String {
        match key_type {
            KeyType::Ed25519 => "ed25519".to_string(),
            KeyType::Secp256k1 => "secp256k1".to_string(),
            KeyType::Bls12381 => "bls12_381".to_string(),
        }
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
#[serde(default)]
pub struct ConsensusData {
    pub height: i64,
    pub round: i64,
    pub step: SignedMsgType,
    pub sign_data: Vec<u8>,
    pub signature: Vec<u8>,
    pub ext_sign_data: Vec<u8>,
    pub ext_signature: Vec<u8>,
}

impl From<&CheckedRequest> for ConsensusData {
    fn from(value: &CheckedRequest) -> Self {
        match value {
            CheckedRequest::Vote(v) => Self {
                height: v.height,
                round: v.round,
                step: v.step,
                ..Default::default()
            },
            CheckedRequest::Proposal(p) => Self {
                height: p.height,
                round: p.round,
                step: p.step,
                ..Default::default()
            },
        }
    }
}

impl std::fmt::Display for ConsensusData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "ConsensusData {}/{}/{:?}",
            self.height, self.round, self.step,
        )
    }
}

impl ConsensusData {
    /// Validates and normalizes a replicated signer-state transition.
    ///
    /// H/R/S may only move forward. At the same H/R/S, the already persisted
    /// core sign bytes and signature are immutable; vote-extension fields may
    /// be refreshed because CometBFT does not include them in duplicate-vote
    /// evidence.
    pub fn validated_update(&self, next: &ConsensusData) -> Result<ConsensusData, SignerError> {
        match next.hrs_cmp(self) {
            std::cmp::Ordering::Less => {
                return Err(SignerError::StateReplication(format!(
                    "refusing signer-state regression from {self} to {next}"
                )));
            }
            std::cmp::Ordering::Equal if self.has_core_signature() => {
                let same_sign_bytes = next.sign_data == self.sign_data;
                let core_is_identified = !self.sign_data.is_empty()
                    || (!self.signature.is_empty() && next.signature == self.signature);
                if !same_sign_bytes || !core_is_identified {
                    return Err(SignerError::DoubleSignError);
                }
            }
            _ => {}
        }

        let mut validated = next.clone();
        if next.hrs_cmp(self) == std::cmp::Ordering::Equal && self.has_core_signature() {
            validated.sign_data = self.sign_data.clone();
            if !self.signature.is_empty() {
                validated.signature = self.signature.clone();
            }
        }
        Ok(validated)
    }

    fn has_core_signature(&self) -> bool {
        !self.sign_data.is_empty() || !self.signature.is_empty()
    }

    fn hrs_cmp(&self, other: &ConsensusData) -> std::cmp::Ordering {
        self.height
            .cmp(&other.height)
            .then_with(|| self.round.cmp(&other.round))
            .then_with(|| consensus_step_rank(self.step).cmp(&consensus_step_rank(other.step)))
    }

    pub fn _persist_to_file(&self, path: &std::path::Path) -> std::io::Result<()> {
        let json = serde_json::to_string_pretty(self)?;
        let temp_path = path.with_extension("json.tmp");
        std::fs::write(&temp_path, json)?;
        std::fs::rename(&temp_path, path)
    }

    pub fn load_from_file(path: &std::path::Path) -> Option<ConsensusData> {
        let bytes = std::fs::read(path).ok()?;
        Self::from_bytes(&bytes)
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        serde_json::to_vec(self).unwrap()
    }

    pub fn from_bytes(buf: &[u8]) -> Option<ConsensusData> {
        let value: serde_json::Value = serde_json::from_slice(buf).ok()?;
        let object = value.as_object()?;
        if CONSENSUS_DATA_FIELDS
            .iter()
            .any(|field| !object.contains_key(*field))
        {
            return None;
        }
        serde_json::from_value(value).ok()
    }
}

fn consensus_step_rank(step: SignedMsgType) -> u8 {
    match step {
        SignedMsgType::Unknown => 0,
        SignedMsgType::Proposal => 1,
        SignedMsgType::Prevote => 2,
        SignedMsgType::Precommit => 3,
    }
}
