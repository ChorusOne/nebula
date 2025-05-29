use crate::protocol::{Request, Response};
use crate::types::{Proposal, Vote};

pub mod v0_34;
pub mod v0_37;
pub mod v0_38;
pub mod v1_0;

use nebula::SignerError;
pub use v0_34::VersionV0_34;
pub use v0_37::VersionV0_37;
pub use v0_38::VersionV0_38;
pub use v1_0::VersionV1_0;

pub trait ProtocolVersion {
    type Message: prost::Message + Default;
    type SignedProposalResponse;
    type SignedVoteResponse;
    type PubKeyResponse;
    type PingResponse;

    fn parse_request(msg: Vec<u8>) -> Result<(Request, String), SignerError>;
    fn encode_response(
        response: Response<
            Self::SignedProposalResponse,
            Self::SignedVoteResponse,
            Self::PubKeyResponse,
            Self::PingResponse,
        >,
    ) -> Result<Vec<u8>, SignerError>;
    fn proposal_to_bytes(proposal: &Proposal, chain_id: &str) -> Result<Vec<u8>, SignerError>;
    fn vote_to_bytes(vote: &Vote, chain_id: &str) -> Result<Vec<u8>, SignerError>;
    fn create_signed_proposal_response(
        proposal: Option<Proposal>,
        signature: Vec<u8>,
        error: Option<String>,
    ) -> Self::SignedProposalResponse;
    fn create_signed_vote_response(
        vote: Option<Vote>,
        signature: Vec<u8>,
        error: Option<String>,
    ) -> Self::SignedVoteResponse;
    fn create_pub_key_response(pub_key: Vec<u8>) -> Self::PubKeyResponse;
    fn create_ping_response() -> Self::PingResponse;
}
