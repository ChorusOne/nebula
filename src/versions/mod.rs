use crate::backend::PublicKey;
use crate::protocol::{Request, Response, ValidRequest};
use crate::types::{ConsensusData, Proposal, Vote};

pub mod v0_34;
pub mod v0_37;
pub mod v0_38;
pub mod v1_0;

use crate::error::SignerError;
pub use v0_34::VersionV0_34;
pub use v0_37::VersionV0_37;
pub use v0_38::VersionV0_38;
pub use v1_0::VersionV1_0;

pub trait ProtocolVersion {
    type Message: prost::Message + Default;
    type ProposalResponse;
    type VoteResponse;
    type PubKeyResponse;
    type PingResponse;

    fn parse_request(msg: Vec<u8>) -> Result<(Request, String), SignerError>;
    fn encode_response(
        response: Response<
            Self::ProposalResponse,
            Self::VoteResponse,
            Self::PubKeyResponse,
            Self::PingResponse,
        >,
    ) -> Result<Vec<u8>, SignerError>;
    fn proposal_to_bytes(proposal: &Proposal, chain_id: &str) -> Result<Vec<u8>, SignerError>;
    fn vote_to_bytes(vote: &Vote, chain_id: &str) -> Result<Vec<u8>, SignerError>;
    fn vote_extension_to_bytes(vote: &Vote, chain_id: &str) -> Result<Vec<u8>, SignerError>;
    fn create_proposal_response(proposal: &Proposal, signature: Vec<u8>) -> Self::ProposalResponse;
    fn create_double_sign_prop_response(cd: &ConsensusData) -> Self::ProposalResponse;
    fn create_double_sign_vote_response(cd: &ConsensusData) -> Self::VoteResponse;
    fn create_vote_response(
        vote: &Vote,
        signature: Vec<u8>,
        extension_signature: Option<Vec<u8>>,
    ) -> Self::VoteResponse;
    fn create_pub_key_response(pub_key: &PublicKey) -> Self::PubKeyResponse;
    fn create_ping_response() -> Self::PingResponse;
}

pub fn generate_error_response<V: ProtocolVersion>(
    r: &ValidRequest,
    _message: &str,
) -> Response<V::ProposalResponse, V::VoteResponse, V::PubKeyResponse, V::PingResponse> {
    let cd = ConsensusData::from(r);
    match r {
        ValidRequest::Proposal(_) => {
            Response::SignedProposal(V::create_double_sign_prop_response(&cd))
        }
        ValidRequest::Vote(_) => Response::SignedVote(V::create_double_sign_vote_response(&cd)),
    }
}
