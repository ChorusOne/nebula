use super::BlockId;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SignedMsgType {
    Unknown = 0,
    Prevote = 1,
    Precommit = 2,
    Proposal = 32,
}

#[derive(Debug, Clone)]
pub struct Vote {
    pub msg_type: SignedMsgType,
    pub height: i64,
    pub round: i64,
    pub timestamp: Option<i64>,
    pub block_id: Option<BlockId>,
    pub validator_address: Vec<u8>,
    pub validator_index: i32,
    pub extension: Vec<u8>,
    pub extension_signature: Vec<u8>,
}
