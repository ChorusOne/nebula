use nebula::SignerError;

use super::ProtocolVersion;
use crate::protocol::{Request, Response};
use crate::types::{Proposal, Vote};

pub struct VersionV0_34;

impl ProtocolVersion for VersionV0_34 {
    type Message = ();
    type SignedProposalResponse = ();
    type SignedVoteResponse = ();
    type PubKeyResponse = ();
    type PingResponse = ();

    fn parse_request(_msg_bytes: Vec<u8>) -> Result<(Request, String), SignerError> {
        todo!("v0.34")
    }

    fn encode_response(
        _response: Response<
            Self::SignedProposalResponse,
            Self::SignedVoteResponse,
            Self::PubKeyResponse,
            Self::PingResponse,
        >,
    ) -> Result<Vec<u8>, SignerError> {
        todo!("v0.34")
    }

    fn proposal_to_bytes(_proposal: &Proposal, _chain_id: &str) -> Result<Vec<u8>, SignerError> {
        todo!("v0.34")
    }

    fn vote_to_bytes(_vote: &Vote, _chain_id: &str) -> Result<Vec<u8>, SignerError> {
        todo!("v0.34")
    }

    fn create_signed_proposal_response(
        _proposal: Proposal,
        _signature: Vec<u8>,
        _error: Option<String>,
    ) -> Self::SignedProposalResponse {
        todo!()
    }

    fn create_signed_vote_response(
        _vote: Vote,
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
