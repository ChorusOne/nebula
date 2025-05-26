use super::{BlockId, vote::SignedMsgType};

#[derive(Debug, Clone)]
pub struct Proposal {
    pub msg_type: SignedMsgType,
    pub height: i64,
    pub round: i64,
    pub timestamp: Option<i64>,
    pub pol_round: i64,
    pub block_id: Option<BlockId>,
}
