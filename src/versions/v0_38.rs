use super::ProtocolVersion;
use crate::protocol::{Request, Response};
use crate::types::{BlockId, PartSetHeader, Proposal, SignedMsgType, Vote};
use nebula::SignerError;
use nebula::proto::v0_38;
use prost::Message;

pub struct VersionV0_38;

impl ProtocolVersion for VersionV0_38 {
    type Message = v0_38::privval::Message;
    type SignedProposalResponse = v0_38::privval::SignedProposalResponse;
    type SignedVoteResponse = v0_38::privval::SignedVoteResponse;
    type PubKeyResponse = v0_38::privval::PubKeyResponse;
    type PingResponse = v0_38::privval::PingResponse;

    fn parse_request(msg_bytes: Vec<u8>) -> Result<(Request, String), SignerError> {
        let msg = v0_38::privval::Message::decode_length_delimited(msg_bytes.as_ref())?;

        match msg.sum {
            Some(v0_38::privval::message::Sum::SignVoteRequest(req)) => {
                let vote = req.vote.ok_or(SignerError::InvalidData)?;
                Ok((Request::SignVote(vote.try_into()?), req.chain_id))
            }
            Some(v0_38::privval::message::Sum::SignProposalRequest(req)) => {
                let proposal = req.proposal.ok_or(SignerError::InvalidData)?;
                Ok((Request::SignProposal(proposal.try_into()?), req.chain_id))
            }
            Some(v0_38::privval::message::Sum::PubKeyRequest(req)) => {
                Ok((Request::ShowPublicKey, req.chain_id))
            }
            Some(v0_38::privval::message::Sum::PingRequest(_)) => {
                Ok((Request::Ping, String::new()))
            }
            _ => Err(SignerError::UnsupportedMessageType),
        }
    }

    fn encode_response(
        response: Response<
            Self::SignedProposalResponse,
            Self::SignedVoteResponse,
            Self::PubKeyResponse,
            Self::PingResponse,
        >,
    ) -> Result<Vec<u8>, SignerError> {
        let mut buf = Vec::new();
        let msg = match response {
            Response::SignedVote(resp) => v0_38::privval::message::Sum::SignedVoteResponse(resp),
            Response::SignedProposal(resp) => {
                v0_38::privval::message::Sum::SignedProposalResponse(resp)
            }
            Response::Ping(resp) => v0_38::privval::message::Sum::PingResponse(resp),
            Response::PublicKey(resp) => v0_38::privval::message::Sum::PubKeyResponse(resp),
        };
        v0_38::privval::Message { sum: Some(msg) }.encode_length_delimited(&mut buf)?;
        Ok(buf)
    }

    fn proposal_to_bytes(proposal: &Proposal, chain_id: &str) -> Result<Vec<u8>, SignerError> {
        let canonical = v0_38::types::CanonicalProposal {
            r#type: proposal.msg_type as i32,
            height: proposal.height,
            round: proposal.round,
            pol_round: proposal.pol_round,
            block_id: proposal
                .block_id
                .as_ref()
                .map(|id| v0_38::types::CanonicalBlockId {
                    hash: id.hash.clone().into(),
                    part_set_header: id.parts.as_ref().map(|p| {
                        v0_38::types::CanonicalPartSetHeader {
                            total: p.total,
                            hash: p.hash.clone().into(),
                        }
                    }),
                }),
            timestamp: proposal.timestamp.map(|t| prost_types::Timestamp {
                seconds: t / 1_000_000_000,
                nanos: (t % 1_000_000_000) as i32,
            }),
            chain_id: chain_id.to_string(),
        };

        let mut bytes = Vec::new();
        canonical.encode(&mut bytes)?;
        Ok(bytes)
    }

    fn vote_to_bytes(vote: &Vote, chain_id: &str) -> Result<Vec<u8>, SignerError> {
        let canonical = v0_38::types::CanonicalVote {
            r#type: vote.msg_type as i32,
            height: vote.height,
            round: vote.round,
            block_id: vote
                .block_id
                .as_ref()
                .map(|id| v0_38::types::CanonicalBlockId {
                    hash: id.hash.clone().into(),
                    part_set_header: id.parts.as_ref().map(|p| {
                        v0_38::types::CanonicalPartSetHeader {
                            total: p.total,
                            hash: p.hash.clone().into(),
                        }
                    }),
                }),
            timestamp: vote.timestamp.map(|t| prost_types::Timestamp {
                seconds: t / 1_000_000_000,
                nanos: (t % 1_000_000_000) as i32,
            }),
            chain_id: chain_id.to_string(),
        };

        let mut bytes = Vec::new();
        canonical.encode(&mut bytes)?;
        Ok(bytes)
    }

    fn create_signed_proposal_response(
        proposal: &Proposal,
        signature: Vec<u8>,
        error: Option<String>,
    ) -> Self::SignedProposalResponse {
        v0_38::privval::SignedProposalResponse {
            proposal: None,
            error: error.map(|e| v0_38::privval::RemoteSignerError {
                code: 1,
                description: e,
            }),
        }
    }

    fn create_signed_vote_response(
        vote: &Vote,
        signature: Vec<u8>,
        error: Option<String>,
    ) -> Self::SignedVoteResponse {
        v0_38::privval::SignedVoteResponse {
            vote: Some(nebula::proto::v1::types::Vote {
                r#type: todo!(),
                height: todo!(),
                round: todo!(),
                block_id: todo!(),
                timestamp: todo!(),
                validator_address: todo!(),
                validator_index: todo!(),
                signature: signature.into(),
                extension: todo!(),
                extension_signature: todo!(),
            }),
            error: error.map(|e| v0_38::privval::RemoteSignerError {
                code: 1,
                description: e,
            }),
        }
    }

    fn create_pub_key_response(pub_key: Vec<u8>) -> Self::PubKeyResponse {
        v0_38::privval::PubKeyResponse {
            pub_key: Some(v0_38::crypto::PublicKey {
                sum: Some(v0_38::crypto::public_key::Sum::Ed25519(pub_key.into())),
            }),
            error: None,
        }
    }

    fn create_ping_response() -> Self::PingResponse {
        v0_38::privval::PingResponse {}
    }
}

impl TryFrom<v0_38::types::Vote> for Vote {
    type Error = SignerError;

    fn try_from(vote: v0_38::types::Vote) -> Result<Self, Self::Error> {
        Ok(Vote {
            msg_type: match vote.r#type {
                1 => SignedMsgType::Prevote,
                2 => SignedMsgType::Precommit,
                32 => SignedMsgType::Proposal,
                _ => SignedMsgType::Unknown,
            },
            height: vote.height,
            round: vote.round as i64,
            timestamp: vote
                .timestamp
                .map(|t| t.seconds * 1_000_000_000 + t.nanos as i64),
            block_id: vote.block_id.map(|id| BlockId {
                hash: id.hash.to_vec(),
                parts: id.part_set_header.map(|p| PartSetHeader {
                    total: p.total,
                    hash: p.hash.to_vec(),
                }),
            }),
            validator_address: vote.validator_address.to_vec(),
            validator_index: vote.validator_index,
            extension: vote.extension.to_vec(),
            extension_signature: vote.extension_signature.to_vec(),
        })
    }
}

impl TryFrom<v0_38::types::Proposal> for Proposal {
    type Error = SignerError;

    fn try_from(proposal: v0_38::types::Proposal) -> Result<Self, Self::Error> {
        Ok(Proposal {
            msg_type: SignedMsgType::Proposal,
            height: proposal.height,
            round: proposal.round as i64,
            timestamp: proposal
                .timestamp
                .map(|t| t.seconds * 1_000_000_000 + t.nanos as i64),
            pol_round: proposal.pol_round as i64,
            block_id: proposal.block_id.map(|id| BlockId {
                hash: id.hash.to_vec(),
                parts: id.part_set_header.map(|p| PartSetHeader {
                    total: p.total,
                    hash: p.hash.to_vec(),
                }),
            }),
        })
    }
}
