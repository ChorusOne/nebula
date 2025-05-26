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
