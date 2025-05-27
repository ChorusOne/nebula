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

#[derive(Debug, Clone)]
pub struct Proposal {
    pub msg_type: SignedMsgType,
    pub height: i64,
    pub round: i64,
    pub timestamp: Option<i64>,
    pub pol_round: i64,
    pub block_id: Option<BlockId>,
}
