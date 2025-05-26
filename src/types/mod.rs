pub mod proposal;
pub mod vote;

pub use proposal::Proposal;
pub use vote::{SignedMsgType, Vote};

#[derive(Debug, Clone)]
pub struct BlockId {
    pub hash: Vec<u8>,
    pub parts: Option<PartSetHeader>,
}

#[derive(Debug, Clone)]
pub struct PartSetHeader {
    pub total: u32,
    pub hash: Vec<u8>,
}
