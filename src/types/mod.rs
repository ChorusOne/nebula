use nebula::SignerError;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Clone)]
pub struct BlockId {
    pub hash: Vec<u8>,
    pub parts: Option<PartSetHeader>,
}

impl From<BlockId> for nebula::proto::v1::types::BlockId {
    fn from(block_id: BlockId) -> nebula::proto::v1::types::BlockId {
        nebula::proto::v1::types::BlockId {
            hash: block_id.hash.into(),
            part_set_header: Some(block_id.parts.unwrap().into()),
        }
    }
}

impl From<PartSetHeader> for nebula::proto::v1::types::PartSetHeader {
    fn from(part_set_header: PartSetHeader) -> nebula::proto::v1::types::PartSetHeader {
        nebula::proto::v1::types::PartSetHeader {
            total: part_set_header.total,
            hash: part_set_header.hash.into(),
        }
    }
}
impl From<nebula::proto::v1::types::BlockId> for BlockId {
    fn from(block_id: nebula::proto::v1::types::BlockId) -> BlockId {
        BlockId {
            hash: block_id.hash.into(),
            parts: Some(block_id.part_set_header.unwrap().into()),
        }
    }
}

impl From<nebula::proto::v1::types::PartSetHeader> for PartSetHeader {
    fn from(part_set_header: nebula::proto::v1::types::PartSetHeader) -> PartSetHeader {
        PartSetHeader {
            total: part_set_header.total,
            hash: part_set_header.hash.into(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct PartSetHeader {
    pub total: u32,
    pub hash: Vec<u8>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SignedMsgType {
    Unknown = 0,
    Prevote = 1,
    Precommit = 2,
    Proposal = 32,
}

#[derive(Debug, Clone)]
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

// this is getting messy, probably something wrong with the types somewhere?
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

#[derive(Debug, Clone)]
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

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "snake_case")]
pub enum KeyType {
    Ed25519,
    Secp256k1,
    Bls12_381,
}

impl TryFrom<&str> for KeyType {
    type Error = SignerError;
    fn try_from(key_type_str: &str) -> Result<KeyType, SignerError> {
        match key_type_str {
            "ed25519" => Ok(KeyType::Ed25519),
            "secp256k1" => Ok(KeyType::Secp256k1),
            "bls12_381" => Ok(KeyType::Bls12_381),
            _ => Err(SignerError::InvalidData),
        }
    }
}

impl From<KeyType> for String {
    fn from(key_type: KeyType) -> String {
        match key_type {
            KeyType::Ed25519 => "ed25519".to_string(),
            KeyType::Secp256k1 => "secp256k1".to_string(),
            KeyType::Bls12_381 => "bls12_381".to_string(),
        }
    }
}

#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct ConsensusData {
    pub height: i64,
    pub round: i64,
    pub step: u8,
}

impl std::fmt::Display for ConsensusData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "ConsensusData {{ height: {}, round: {}, step: {} }}",
            self.height, self.round, self.step
        )
    }
}

impl ConsensusData {
    pub fn persist_to_file(&self, path: &std::path::Path) -> std::io::Result<()> {
        let json = serde_json::to_string_pretty(self)?;
        let temp_path = path.with_extension("json.tmp");
        std::fs::write(&temp_path, json)?;
        std::fs::rename(&temp_path, path)
    }

    pub fn load_from_file(path: &std::path::Path) -> Option<ConsensusData> {
        let json = std::fs::read_to_string(path).ok()?;
        serde_json::from_str(&json).ok()
    }
    pub fn to_bytes(&self) -> Vec<u8> {
        serde_json::to_vec(self).unwrap()
    }

    pub fn from_bytes(buf: &[u8]) -> Option<ConsensusData> {
        serde_json::from_slice(buf).ok()
    }
}
