use super::ProtocolVersion;
use crate::backend::PublicKey;
use crate::error::SignerError;
use crate::proto::v1;
use crate::protocol::{Request, Response, SignRequest};
use crate::types::{BlockId, ConsensusData, PartSetHeader, Proposal, SignedMsgType, Vote};
use log::trace;
use prost::Message;

pub struct VersionV1_0;

impl ProtocolVersion for VersionV1_0 {
    type Message = v1::privval::Message;
    type ProposalResponse = v1::privval::SignedProposalResponse;
    type VoteResponse = v1::privval::SignedVoteResponse;
    type PubKeyResponse = v1::privval::PubKeyResponse;
    type PingResponse = v1::privval::PingResponse;

    fn parse_request(msg_bytes: Vec<u8>) -> Result<(Request, String), SignerError> {
        let msg = v1::privval::Message::decode_length_delimited(msg_bytes.as_ref())?;

        // TODO: sign vote request in privval v1 has the bool field "sign vote extension"
        // we should handle it
        match msg.sum {
            Some(v1::privval::message::Sum::SignVoteRequest(req)) => {
                let vote = req.vote.ok_or(SignerError::InvalidData)?;
                Ok((
                    Request::Signable(SignRequest::Vote(tendermint_vote_to_domain(vote)?)),
                    req.chain_id,
                ))
            }
            Some(v1::privval::message::Sum::SignProposalRequest(req)) => {
                let proposal = req.proposal.ok_or(SignerError::InvalidData)?;
                Ok((
                    Request::Signable(SignRequest::Proposal(tendermint_proposal_to_domain(
                        proposal,
                    )?)),
                    req.chain_id,
                ))
            }
            Some(v1::privval::message::Sum::PubKeyRequest(req)) => {
                Ok((Request::ShowPublicKey, req.chain_id))
            }
            Some(v1::privval::message::Sum::PingRequest(_)) => Ok((Request::Ping, String::new())),
            _ => Err(SignerError::UnsupportedMessageType),
        }
    }

    fn encode_response(
        response: Response<
            Self::ProposalResponse,
            Self::VoteResponse,
            Self::PubKeyResponse,
            Self::PingResponse,
        >,
    ) -> Result<Vec<u8>, SignerError> {
        let mut buf = Vec::new();
        let msg = match response {
            Response::SignedVote(resp) => v1::privval::message::Sum::SignedVoteResponse(resp),
            Response::SignedProposal(resp) => {
                v1::privval::message::Sum::SignedProposalResponse(resp)
            }
            Response::Ping(resp) => v1::privval::message::Sum::PingResponse(resp),
            Response::PublicKey(resp) => v1::privval::message::Sum::PubKeyResponse(resp),
        };
        v1::privval::Message { sum: Some(msg) }.encode_length_delimited(&mut buf)?;
        Ok(buf)
    }

    fn proposal_to_bytes(proposal: &Proposal, chain_id: &str) -> Result<Vec<u8>, SignerError> {
        let canonical = v1::types::CanonicalProposal {
            r#type: proposal.step as i32,
            height: proposal.height,
            round: proposal.round,
            pol_round: proposal.pol_round,
            block_id: proposal
                .block_id
                .as_ref()
                .map(|id| v1::types::CanonicalBlockId {
                    hash: id.hash.clone().into(),
                    part_set_header: id
                        .parts
                        .as_ref()
                        .map(|p| v1::types::CanonicalPartSetHeader {
                            total: p.total,
                            hash: p.hash.clone().into(),
                        }),
                }),
            timestamp: proposal.timestamp.map(|t| prost_types::Timestamp {
                seconds: t / 1_000_000_000,
                nanos: (t % 1_000_000_000) as i32,
            }),
            chain_id: chain_id.to_string(),
        };

        let mut bytes = Vec::new();
        canonical.encode_length_delimited(&mut bytes)?;
        Ok(bytes)
    }

    fn vote_to_bytes(vote: &Vote, chain_id: &str) -> Result<Vec<u8>, SignerError> {
        trace!("changing vote to bytes, vote: {:?}", vote);

        let canonical = v1::types::CanonicalVote {
            r#type: vote.step as i32,
            height: vote.height,
            round: vote.round,

            block_id: vote.block_id.as_ref().and_then(|id| {
                if id.hash.is_empty() {
                    None
                } else {
                    let hdr = id
                        .parts
                        .as_ref()
                        .map(|p| v1::types::CanonicalPartSetHeader {
                            total: p.total,
                            hash: p.hash.clone().into(),
                        });
                    Some(v1::types::CanonicalBlockId {
                        hash: id.hash.clone().into(),
                        part_set_header: hdr,
                    })
                }
            }),

            timestamp: vote.timestamp.map(|t| prost_types::Timestamp {
                seconds: t / 1_000_000_000,
                nanos: (t % 1_000_000_000) as i32,
            }),

            chain_id: chain_id.to_string(),
        };

        let mut bytes = Vec::new();
        canonical.encode_length_delimited(&mut bytes)?;
        Ok(bytes)
    }

    fn vote_extension_to_bytes(vote: &Vote, chain_id: &str) -> Result<Vec<u8>, SignerError> {
        // todo: vote extension has to match what's in the vote
        // possibly can be different if theres a bug
        let copied = vote.clone();
        let canonical = v1::types::CanonicalVoteExtension {
            extension: copied.extension.into(),
            height: copied.height,
            round: copied.round,
            chain_id: chain_id.to_string(),
        };

        let mut bytes = Vec::new();
        canonical.encode_length_delimited(&mut bytes)?;
        Ok(bytes)
    }

    fn create_double_sign_vote_response(cd: &ConsensusData) -> Self::VoteResponse {
        v1::privval::SignedVoteResponse {
            vote: None,
            error: Some(v1::privval::RemoteSignerError {
                code: 1,
                description: format!("Would double-sign vote at height/round/step {}/{}/{}", cd.height, cd.round, cd.step),
            }),
        }
    }

    fn create_double_sign_prop_response(cd: &ConsensusData) -> Self::ProposalResponse {
        v1::privval::SignedProposalResponse {
            proposal: None,
            error: Some(v1::privval::RemoteSignerError {
                code: 1,
                description: format!("Would double-sign proposal at height/round/step {}/{}/{}", cd.height, cd.round, cd.step),
            }),
        }
    }

    fn create_proposal_response(proposal: &Proposal, signature: Vec<u8>) -> Self::ProposalResponse {
        v1::privval::SignedProposalResponse {
            proposal: Some(v1::types::Proposal {
                r#type: proposal.step as i32,
                height: proposal.height,
                round: proposal.round as i32,
                pol_round: proposal.pol_round as i32,
                block_id: proposal.block_id.clone().map(|id| v1::types::BlockId {
                    hash: id.hash.into(),
                    part_set_header: id.parts.map(|p| v1::types::PartSetHeader {
                        total: p.total,
                        hash: p.hash.into(),
                    }),
                }),
                timestamp: proposal.timestamp.map(|t| prost_types::Timestamp {
                    seconds: t / 1_000_000_000,
                    nanos: (t % 1_000_000_000) as i32,
                }),
                signature: signature.into(),
            }),
            error: None,
        }
    }

    fn create_vote_response(
        vote: &Vote,
        signature: Vec<u8>,
        ext_signature: Option<Vec<u8>>,
    ) -> Self::VoteResponse {
        v1::privval::SignedVoteResponse {
            vote: Some(crate::proto::v1::types::Vote {
                r#type: vote.step.into(),
                height: vote.height,
                round: vote.round as i32,
                block_id: vote.block_id.clone().map(|id| id.into()),
                timestamp: vote.timestamp.map(|t| prost_types::Timestamp {
                    seconds: t / 1_000_000_000,
                    nanos: (t % 1_000_000_000) as i32,
                }),
                validator_address: vote.validator_address.clone().into(),
                validator_index: vote.validator_index,
                signature: signature.into(),
                extension: vote.extension.clone().into(),
                extension_signature: ext_signature.unwrap_or_default().into(),
            }),
            error: None,
        }
    }

    fn create_pub_key_response(pub_key: &PublicKey) -> Self::PubKeyResponse {
        v1::privval::PubKeyResponse {
            error: None,
            pub_key_bytes: pub_key.bytes.clone().into(),
            pub_key_type: pub_key.key_type.clone().into(),
        }
    }

    fn create_ping_response() -> Self::PingResponse {
        v1::privval::PingResponse {}
    }

    fn create_error_response(message: &str) -> Response<Self::ProposalResponse, Self::VoteResponse, Self::PubKeyResponse, Self::PingResponse> {
        Response::SignedProposal(v1::privval::SignedProposalResponse {
            proposal: None,
            error: Some(v1::privval::RemoteSignerError {
                code: 1,
                description: message.to_string(),
            }),
        })
    }
}
fn tendermint_vote_to_domain(vote: v1::types::Vote) -> Result<Vote, SignerError> {
    Ok(Vote {
        step: match vote.r#type {
            1 => SignedMsgType::Prevote,
            2 => SignedMsgType::Precommit,
            32 => SignedMsgType::Proposal,
            _ => SignedMsgType::Unknown,
        },
        height: vote.height,
        round: vote.round as i64,
        timestamp: vote.timestamp.and_then(|t| {
            if t.seconds < 0 {
                None
            } else {
                Some(t.seconds * 1_000_000_000 + t.nanos as i64)
            }
        }),
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
fn tendermint_proposal_to_domain(proposal: v1::types::Proposal) -> Result<Proposal, SignerError> {
    Ok(Proposal {
        step: SignedMsgType::Proposal,
        height: proposal.height,
        round: proposal.round as i64,
        timestamp: proposal.timestamp.and_then(|t| {
            if t.seconds < 0 {
                None
            } else {
                Some(t.seconds * 1_000_000_000 + t.nanos as i64)
            }
        }),
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
