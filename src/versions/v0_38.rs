use super::ProtocolVersion;
use crate::backend::PublicKey;
use crate::protocol::{Request, Response};
use crate::types::{BlockId, PartSetHeader, Proposal, SignedMsgType, Vote};
use log::info;
use nebula::SignerError;
use nebula::proto::v0_38;
use prost::Message;

pub struct VersionV0_38;

impl ProtocolVersion for VersionV0_38 {
    type Message = v0_38::privval::Message;
    type ProposalResponse = v0_38::privval::SignedProposalResponse;
    type VoteResponse = v0_38::privval::SignedVoteResponse;
    type PubKeyResponse = v0_38::privval::PubKeyResponse;
    type PingResponse = v0_38::privval::PingResponse;

    fn parse_request(msg_bytes: Vec<u8>) -> Result<(Request, String), SignerError> {
        let msg = v0_38::privval::Message::decode_length_delimited(msg_bytes.as_ref())?;

        match msg.sum {
            Some(v0_38::privval::message::Sum::SignVoteRequest(req)) => {
                let vote = req.vote.ok_or(SignerError::InvalidData)?;
                info!("parsed vote extension: {:?}", vote.extension);
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
            Self::ProposalResponse,
            Self::VoteResponse,
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
            r#type: proposal.step as i32,
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
        canonical.encode_length_delimited(&mut bytes)?;
        Ok(bytes)
    }

    fn vote_to_bytes(vote: &Vote, chain_id: &str) -> Result<Vec<u8>, SignerError> {
        info!("changing vote to bytes, block id: {:?}", vote.block_id);

        let canonical = v0_38::types::CanonicalVote {
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
                        .map(|p| v0_38::types::CanonicalPartSetHeader {
                            total: p.total,
                            hash: p.hash.clone().into(),
                        });
                    Some(v0_38::types::CanonicalBlockId {
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
        let canonical = v0_38::types::CanonicalVoteExtension {
            extension: copied.extension.into(),
            height: copied.height,
            round: copied.round,
            chain_id: chain_id.to_string(),
        };

        let mut bytes = Vec::new();
        canonical.encode_length_delimited(&mut bytes)?;
        Ok(bytes)
    }

    fn create_proposal_response(
        proposal: Option<Proposal>,
        signature: Vec<u8>,
        error: Option<String>,
    ) -> Self::ProposalResponse {
        v0_38::privval::SignedProposalResponse {
            proposal: proposal.map(|proposal| v0_38::types::Proposal {
                r#type: proposal.step as i32,
                height: proposal.height as i64,
                round: proposal.round as i32,
                pol_round: proposal.pol_round as i32,
                block_id: proposal.block_id.map(|id| v0_38::types::BlockId {
                    hash: id.hash.into(),
                    part_set_header: id.parts.map(|p| v0_38::types::PartSetHeader {
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
            error: error.map(|desc| v0_38::privval::RemoteSignerError {
                code: 1,
                description: desc,
            }),
        }
    }

    fn create_vote_response(
        vote: Option<Vote>,
        signature: Vec<u8>,
        ext_signature: Option<Vec<u8>>,
        error: Option<String>,
    ) -> Self::VoteResponse {
        v0_38::privval::SignedVoteResponse {
            vote: vote.map(|vote| nebula::proto::v0_38::types::Vote {
                r#type: vote.step.into(),
                height: vote.height,
                round: vote.round as i32,
                block_id: vote.block_id.map(|id| id.into()),
                timestamp: vote.timestamp.map(|t| prost_types::Timestamp {
                    seconds: t / 1_000_000_000,
                    nanos: (t % 1_000_000_000) as i32,
                }),
                validator_address: vote.validator_address.into(),
                validator_index: vote.validator_index,
                signature: signature.into(),
                extension: vote.extension.into(),
                extension_signature: ext_signature.unwrap_or_default().into(),
            }),
            error: error.map(|desc| v0_38::privval::RemoteSignerError {
                code: 1,
                description: desc,
            }),
        }
    }

    fn create_pub_key_response(pub_key: PublicKey) -> Self::PubKeyResponse {
        v0_38::privval::PubKeyResponse {
            pub_key: Some(v0_38::crypto::PublicKey {
                sum: Some(match pub_key.key_type {
                    crate::types::KeyType::Ed25519 => {
                        v0_38::crypto::public_key::Sum::Ed25519(pub_key.bytes.into())
                    }
                    crate::types::KeyType::Secp256k1 => {
                        v0_38::crypto::public_key::Sum::Secp256k1(pub_key.bytes.into())
                    }
                    crate::types::KeyType::Bls12_381 => {
                        v0_38::crypto::public_key::Sum::Bls12381(pub_key.bytes.into())
                    }
                }),
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
            step: match vote.r#type {
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
            step: SignedMsgType::Proposal,
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

// tests data we got from the current go implementation
#[cfg(test)]
mod tests {
    use crate::backend::{Ed25519Signer, SigningBackend};

    use super::*;
    use hex;
    use prost::Message;

    fn create_test_proposal() -> Proposal {
        let block_hash =
            hex::decode("5b67e0b0a8ad775ec8384810bdcb2dcbf053dae67ea288c0b7bb9df367648c43")
                .unwrap();
        let parts_hash =
            hex::decode("5cfdce21a1326d26fd397a31ce964b968f743c07a68e2189c95741c9debd5355")
                .unwrap();

        Proposal {
            step: SignedMsgType::Proposal,
            height: 1,
            round: 0,
            timestamp: Some(1748510847683560856),
            pol_round: -1,
            block_id: Some(BlockId {
                hash: block_hash,
                parts: Some(PartSetHeader {
                    total: 1,
                    hash: parts_hash,
                }),
            }),
        }
    }

    #[test]
    fn test_proposal_to_bytes_matches_go_implementation() {
        let proposal = create_test_proposal();
        let chain_id = "testing";

        let result = VersionV0_38::proposal_to_bytes(&proposal, chain_id).unwrap();
        let hex_result = hex::encode(&result);

        let expected_hex = "77082011010000000000000020ffffffffffffffffff012a480a205b67e0b0a8ad775ec8384810bdcb2dcbf053dae67ea288c0b7bb9df367648c431224080112205cfdce21a1326d26fd397a31ce964b968f743c07a68e2189c95741c9debd5355320c08ffd0e0c10610989ff9c5023a0774657374696e67";

        assert_eq!(
            hex_result, expected_hex,
            "Encoded bytes don't match Go implementation"
        );
    }

    #[test]
    fn test_canonical_proposal_encoding() {
        let expected_bytes = hex::decode("77082011010000000000000020ffffffffffffffffff012a480a205b67e0b0a8ad775ec8384810bdcb2dcbf053dae67ea288c0b7bb9df367648c431224080112205cfdce21a1326d26fd397a31ce964b968f743c07a68e2189c95741c9debd5355320c08ffd0e0c10610989ff9c5023a0774657374696e67").unwrap();

        let canonical =
            v0_38::types::CanonicalProposal::decode_length_delimited(&expected_bytes[..]).unwrap();

        println!("Decoded canonical proposal:");
        println!("  type: {}", canonical.r#type);
        println!("  height: {}", canonical.height);
        println!("  round: {}", canonical.round);
        println!("  pol_round: {}", canonical.pol_round);

        if let Some(ts) = &canonical.timestamp {
            println!("  timestamp seconds: {}", ts.seconds);
            println!("  timestamp nanos: {}", ts.nanos);
            let total_nanos = ts.seconds as i128 * 1_000_000_000 + ts.nanos as i128;
            println!("  timestamp total nanos: {}", total_nanos);
        }

        println!("  chain_id: {}", canonical.chain_id);

        assert_eq!(canonical.r#type, 32);
        assert_eq!(canonical.height, 1);
        assert_eq!(canonical.round, 0);
        assert_eq!(canonical.pol_round, -1);
        assert_eq!(canonical.chain_id, "testing");
    }

    #[test]
    fn test_full_signing_flow() {
        let test_signer = Ed25519Signer::from_key_file("./keys/privkey");

        let proposal = create_test_proposal();
        let chain_id = "testing";

        let signable_bytes = VersionV0_38::proposal_to_bytes(&proposal, chain_id).unwrap();

        let expected_hex = "77082011010000000000000020ffffffffffffffffff012a480a205b67e0b0a8ad775ec8384810bdcb2dcbf053dae67ea288c0b7bb9df367648c431224080112205cfdce21a1326d26fd397a31ce964b968f743c07a68e2189c95741c9debd5355320c08ffd0e0c10610989ff9c5023a0774657374696e67";
        assert_eq!(hex::encode(&signable_bytes), expected_hex);

        let signature = test_signer.unwrap().sign(&signable_bytes);
        let expected_signature = "bfde0a738bbec89b2fb78b01caaf53912762d16cb78001aeae212b691f4e4d30f1a83369cd88aa4dc6384bfe3009c7cfcb54b0ef4ae0199f18e0cbbfa42d3701";
        let sig = &signature.unwrap();
        println!("got this signature: {}", hex::encode(sig));
        assert_eq!(hex::encode(sig), expected_signature);
    }
}
