use nebula::SignerError;

use super::ProtocolVersion;
use crate::protocol::{Request, Response};
use crate::types::{Proposal, Vote};

pub struct VersionV1_0;

impl ProtocolVersion for VersionV1_0 {
    type Message = ();
    type SignedProposalResponse = ();
    type SignedVoteResponse = ();
    type PubKeyResponse = ();
    type PingResponse = ();

    fn parse_request(_msg_bytes: Vec<u8>) -> Result<(Request, String), SignerError> {
        todo!("v1.0")
    }

    fn encode_response(
        _response: Response<
            Self::SignedProposalResponse,
            Self::SignedVoteResponse,
            Self::PubKeyResponse,
            Self::PingResponse,
        >,
    ) -> Result<Vec<u8>, SignerError> {
        todo!("v1.0")
    }

    fn proposal_to_bytes(_proposal: &Proposal, _chain_id: &str) -> Result<Vec<u8>, SignerError> {
        todo!("v1.0")
    }

    fn vote_to_bytes(_vote: &Vote, _chain_id: &str) -> Result<Vec<u8>, SignerError> {
        todo!("v1.0")
    }

    fn create_signed_proposal_response(
        proposal: &Proposal,
        _signature: Vec<u8>,
        _error: Option<String>,
    ) -> Self::SignedProposalResponse {
        todo!()
    }

    fn create_signed_vote_response(
        vote: &Vote,
        _signature: Vec<u8>,
        _error: Option<String>,
    ) -> Self::SignedVoteResponse {
        todo!()
    }

    fn create_pub_key_response(_pub_key: Vec<u8>) -> Self::PubKeyResponse {
        todo!()
    }

    fn create_ping_response() -> Self::PingResponse {
        todo!()
    }
}
