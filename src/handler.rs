use crate::backend::SigningBackend;
use crate::cluster::SignerRaftNode;
use crate::error::SignerError;
use crate::protocol::{Request, Response};
use crate::safeguards::{self, VoteCheckResult};
use crate::signer::Signer;
use crate::types::{ConsensusData, SignedMsgType};
use crate::versions::ProtocolVersion;
use log::{debug, error, info, warn};
use std::io::{Read, Write};
use std::sync::{Arc, Mutex};

pub trait SignerConnection: Read + Write + Send + 'static {}
impl<T: Read + Write + Send + 'static> SignerConnection for T {}

pub struct SigningHandler<V: ProtocolVersion> {
    _phantom: std::marker::PhantomData<V>,
}

impl<V: ProtocolVersion + Send + 'static> SigningHandler<V> {
    pub fn process_request<B: SigningBackend + 'static, C: SignerConnection>(
        signer: &mut Signer<B, V, C>,
        request: Request,
        raft_node: &Arc<SignerRaftNode>,
        signing_lock: &Arc<Mutex<()>>,
    ) -> Result<
        Response<
            V::ProposalResponse,
            V::VoteResponse,
            V::PubKeyResponse,
            V::PingResponse,
            V::BytesResponse,
        >,
        SignerError,
    > {
        match request {
            Request::SignProposal(proposal) => {
                let start = std::time::Instant::now();
                debug!("waiting for lock");
                // This is to make sure that we only serve a request from one CometBFT node at a time.
                // NOTE: another approach to this could be using try_lock and bailing early on error if the mutex is locked
                // However, this would be bad in situations where one node is behind the network and is trying to sign old blocks
                // Then, the up to date nodes would get an error because of a locked mutex
                // The behind node, which got the mutex, would error because of double signing rules
                // However, with using a blocking lock(), the node that is not signing will fall behind the network slightly.
                //
                let _guard = signing_lock.lock().unwrap();
                debug!("lock acquired, took: {:?}", start.elapsed());

                if !raft_node.is_leader() {
                    warn!(
                        "we found out we are not the leader after acquiring lock, node id: {}",
                        raft_node.node_id()
                    );
                    return Ok(Response::SignedProposal(V::create_proposal_response(
                        None,
                        Vec::new(),
                        Some("Not the leader".into()),
                    )));
                }

                let sign_data = signer.proposal_sign_bytes(&proposal)?;
                let current_state = raft_node.signer_state.read().unwrap().clone();
                let step = SignedMsgType::Proposal as u8;

                if let Some((signature, _)) = raft_node.find_cached_signature(
                    proposal.height,
                    proposal.round,
                    step,
                    &sign_data,
                    &[],
                ) {
                    info!(
                        "Replaying cached proposal signature at hrs: {}/{}/{}",
                        proposal.height, proposal.round, proposal.step as u8
                    );
                    return Ok(Response::SignedProposal(V::create_proposal_response(
                        Some(proposal),
                        signature,
                        None,
                    )));
                }

                let is_same_hrs = current_state.height == proposal.height
                    && current_state.round == proposal.round
                    && current_state.step == step;

                if is_same_hrs && !current_state.sign_data.is_empty() {
                    if current_state.sign_data == sign_data && !current_state.signature.is_empty() {
                        info!(
                            "Replaying stored proposal signature at hrs: {}/{}/{}",
                            proposal.height, proposal.round, proposal.step as u8
                        );
                        return Ok(Response::SignedProposal(V::create_proposal_response(
                            Some(proposal),
                            current_state.signature,
                            None,
                        )));
                    }

                    let only_ts = V::proposal_sign_bytes_only_differ_by_timestamp(
                        &current_state.sign_data,
                        &sign_data,
                    )?;
                    if only_ts {
                        if !current_state.signature.is_empty() {
                            raft_node.cache_signature(
                                current_state.height,
                                current_state.round,
                                current_state.step,
                                current_state.sign_data.clone(),
                                current_state.signature.clone(),
                                current_state.ext_sign_data.clone(),
                                current_state.ext_signature.clone(),
                            );
                        }

                        let signature = signer.sign_bytes(&sign_data)?;
                        let new_state = ConsensusData {
                            height: proposal.height,
                            round: proposal.round,
                            step,
                            sign_data,
                            signature: signature.clone(),
                            ext_sign_data: Vec::new(),
                            ext_signature: Vec::new(),
                        };

                        if let Err(e) = raft_node.replicate_state(new_state) {
                            error!("CRITICAL: State replication failed: {}. Not signing.", e);
                            return Ok(Response::SignedProposal(V::create_proposal_response(
                                None,
                                Vec::new(),
                                Some(format!("Raft replication failed: {}", e)),
                            )));
                        }

                        return Ok(Response::SignedProposal(V::create_proposal_response(
                            Some(proposal),
                            signature,
                            None,
                        )));
                    }

                    return Ok(Response::SignedProposal(V::create_proposal_response(
                        None,
                        Vec::new(),
                        Some(
                            "Sign bytes mismatch for same height/round/step; refusing replay"
                                .into(),
                        ),
                    )));
                }

                if safeguards::should_sign_proposal(&current_state, &proposal) {
                    if !current_state.signature.is_empty() && !current_state.sign_data.is_empty() {
                        raft_node.cache_signature(
                            current_state.height,
                            current_state.round,
                            current_state.step,
                            current_state.sign_data.clone(),
                            current_state.signature.clone(),
                            current_state.ext_sign_data.clone(),
                            current_state.ext_signature.clone(),
                        );
                    }

                    let signature = signer.sign_bytes(&sign_data)?;
                    let new_state = ConsensusData {
                        height: proposal.height,
                        round: proposal.round,
                        step,
                        sign_data,
                        signature: signature.clone(),
                        ext_sign_data: Vec::new(),
                        ext_signature: Vec::new(),
                    };

                    if let Err(e) = raft_node.replicate_state(new_state) {
                        error!("CRITICAL: State replication failed: {}. Not signing.", e);
                        Ok(Response::SignedProposal(V::create_proposal_response(
                            None,
                            Vec::new(),
                            Some(format!("Raft replication failed: {}", e)),
                        )))
                    } else {
                        Ok(Response::SignedProposal(V::create_proposal_response(
                            Some(proposal),
                            signature,
                            None,
                        )))
                    }
                } else {
                    info!(
                        "Prevented double signing proposal at hrs: {}/{}/{}",
                        proposal.height, proposal.round, proposal.step as u8
                    );
                    Ok(Response::SignedProposal(V::create_proposal_response(
                        None,
                        Vec::new(),
                        Some("Would double-sign proposal at same height/round/step".into()),
                    )))
                }
            }

            Request::SignVote(vote) => {
                let start = std::time::Instant::now();
                debug!("waiting for lock");
                let _guard = signing_lock.lock().unwrap();
                debug!("lock acquired, took: {:?}", start.elapsed());

                if !raft_node.is_leader() {
                    return Ok(Response::SignedVote(V::create_vote_response(
                        None,
                        Vec::new(),
                        None,
                        Some("Not the leader".into()),
                    )));
                }

                let sign_data = signer.vote_sign_bytes(&vote)?;
                let has_vote_ext = vote.step == SignedMsgType::Precommit
                    && vote.block_id.as_ref().is_some_and(|id| !id.hash.is_empty());
                let ext_sign_data = if has_vote_ext {
                    Some(signer.vote_ext_sign_bytes(&vote)?)
                } else {
                    None
                };
                let ext_sign_data_bytes = ext_sign_data.as_deref().unwrap_or(&[]);

                let current_state = raft_node.signer_state.read().unwrap().clone();
                let step: u8 = vote.step.into();

                if let Some((signature, ext_signature)) = raft_node.find_cached_signature(
                    vote.height,
                    vote.round,
                    step,
                    &sign_data,
                    ext_sign_data_bytes,
                ) {
                    info!(
                        "Replaying cached vote signature at hrs: {}/{}/{}",
                        vote.height, vote.round, vote.step as u8
                    );
                    let ext_sig = if ext_signature.is_empty() {
                        None
                    } else {
                        Some(ext_signature)
                    };
                    return Ok(Response::SignedVote(V::create_vote_response(
                        Some(vote),
                        signature,
                        ext_sig,
                        None,
                    )));
                }

                let is_same_hrs =
                    current_state.height == vote.height && current_state.round == vote.round
                        && current_state.step == step;

                if is_same_hrs && !current_state.sign_data.is_empty() {
                    let ext_matches = ext_sign_data
                        .as_ref()
                        .map(|bytes| current_state.ext_sign_data == *bytes)
                        .unwrap_or_else(|| {
                            current_state.ext_sign_data.is_empty()
                                && current_state.ext_signature.is_empty()
                        });

                    if current_state.sign_data == sign_data
                        && !current_state.signature.is_empty()
                        && ext_matches
                    {
                        info!(
                            "Replaying stored vote signature at hrs: {}/{}/{}",
                            vote.height, vote.round, vote.step as u8
                        );
                        let ext_signature = if current_state.ext_signature.is_empty() {
                            None
                        } else {
                            Some(current_state.ext_signature.clone())
                        };
                        return Ok(Response::SignedVote(V::create_vote_response(
                            Some(vote),
                            current_state.signature,
                            ext_signature,
                            None,
                        )));
                    }

                    let only_ts = V::vote_sign_bytes_only_differ_by_timestamp(
                        &current_state.sign_data,
                        &sign_data,
                    )?;
                    if only_ts {
                        if ext_sign_data.is_some() && !ext_matches {
                            return Ok(Response::SignedVote(V::create_vote_response(
                                None,
                                Vec::new(),
                                None,
                                Some(
                                    "Vote extension sign bytes mismatch for same height/round/step; refusing replay"
                                        .into(),
                                ),
                            )));
                        }

                        if !current_state.signature.is_empty() {
                            raft_node.cache_signature(
                                current_state.height,
                                current_state.round,
                                current_state.step,
                                current_state.sign_data.clone(),
                                current_state.signature.clone(),
                                current_state.ext_sign_data.clone(),
                                current_state.ext_signature.clone(),
                            );
                        }

                        let signature = signer.sign_bytes(&sign_data)?;
                        let ext_signature = match ext_sign_data.as_ref() {
                            Some(bytes) => {
                                if current_state.ext_sign_data == *bytes
                                    && !current_state.ext_signature.is_empty()
                                {
                                    Some(current_state.ext_signature.clone())
                                } else {
                                    Some(signer.sign_bytes(bytes)?)
                                }
                            }
                            None => None,
                        };
                        let new_state = ConsensusData {
                            height: vote.height,
                            round: vote.round,
                            step,
                            sign_data,
                            signature: signature.clone(),
                            ext_sign_data: ext_sign_data.clone().unwrap_or_default(),
                            ext_signature: ext_signature.clone().unwrap_or_default(),
                        };

                        if let Err(e) = raft_node.replicate_state(new_state) {
                            error!("CRITICAL: State replication failed: {}. Not signing.", e);
                            return Ok(Response::SignedVote(V::create_vote_response(
                                None,
                                Vec::new(),
                                None,
                                Some(format!("Raft replication failed: {}", e)),
                            )));
                        }

                        return Ok(Response::SignedVote(V::create_vote_response(
                            Some(vote),
                            signature,
                            ext_signature,
                            None,
                        )));
                    }

                    return Ok(Response::SignedVote(V::create_vote_response(
                        None,
                        Vec::new(),
                        None,
                        Some(
                            "Sign bytes mismatch for same height/round/step; refusing replay"
                                .into(),
                        ),
                    )));
                }

                match safeguards::should_sign_vote(&current_state, &vote) {
                    VoteCheckResult {
                        resend_signature: false,
                        should_sign: true,
                    } => {
                        if !current_state.signature.is_empty()
                            && !current_state.sign_data.is_empty()
                        {
                            raft_node.cache_signature(
                                current_state.height,
                                current_state.round,
                                current_state.step,
                                current_state.sign_data.clone(),
                                current_state.signature.clone(),
                                current_state.ext_sign_data.clone(),
                                current_state.ext_signature.clone(),
                            );
                        }

                        let signature = signer.sign_bytes(&sign_data)?;
                        let ext_signature = match ext_sign_data.as_ref() {
                            Some(bytes) => Some(signer.sign_bytes(bytes)?),
                            None => None,
                        };
                        let new_state = ConsensusData {
                            height: vote.height,
                            round: vote.round,
                            step,
                            sign_data,
                            signature: signature.clone(),
                            ext_sign_data: ext_sign_data.clone().unwrap_or_default(),
                            ext_signature: ext_signature.clone().unwrap_or_default(),
                        };

                        if let Err(e) = raft_node.replicate_state(new_state) {
                            error!("CRITICAL: State replication failed: {}. Not signing.", e);
                            return Ok(Response::SignedVote(V::create_vote_response(
                                None,
                                Vec::new(),
                                None,
                                Some(format!("Raft replication failed: {}", e)),
                            )));
                        } else {
                            return Ok(Response::SignedVote(V::create_vote_response(
                                Some(vote),
                                signature,
                                ext_signature,
                                None,
                            )));
                        }
                    }
                    VoteCheckResult {
                        resend_signature: true,
                        should_sign: false,
                    } => {
                        return Ok(Response::SignedVote(V::create_vote_response(
                            None,
                            Vec::new(),
                            None,
                            Some(
                                "Sign bytes mismatch for same height/round/step; refusing replay"
                                    .into(),
                            ),
                        )));
                    }
                    VoteCheckResult {
                        resend_signature: true,
                        should_sign: true,
                    } => unreachable!(),
                    VoteCheckResult {
                        resend_signature: false,
                        should_sign: false,
                    } => {
                        info!(
                            "Prevented double signing vote at hrs: {}/{}/{}",
                            vote.height, vote.round, vote.step as u8
                        );
                        return Ok(Response::SignedVote(V::create_vote_response(
                            None,
                            Vec::new(),
                            None,
                            Some("Would double-sign vote at same height/round".into()),
                        )));
                    }
                }
            }

            other => signer.sign_request_and_build_response(other),
        }
    }

    pub fn handle_single_request<B: SigningBackend + 'static, C: SignerConnection>(
        signer: &mut Signer<B, V, C>,
        raft_node: &Arc<SignerRaftNode>,
        signing_lock: &Arc<Mutex<()>>,
    ) -> Result<(), SignerError> {
        let start = std::time::Instant::now();
        let request = signer.read_request()?;
        info!(
            "Received request after {:?}: {:?}",
            start.elapsed(),
            request
        );

        let start = std::time::Instant::now();
        let response = Self::process_request(signer, request, raft_node, signing_lock)?;
        info!("Processing request took: {:?}", start.elapsed());

        let start = std::time::Instant::now();
        signer.send_response(response)?;
        info!("Sending the response took: {:?}", start.elapsed());

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::backend::{PublicKey, SigningBackend};
    use crate::cluster::SignerRaftNode;
    use crate::config::{PeerConfig, RaftConfig};
    use crate::types::{BlockId, KeyType, PartSetHeader, Proposal, Vote};
    use crate::versions::VersionV1_0;
    use std::io::Cursor;
    use std::sync::{Arc, Mutex};
    use std::thread;
    use std::time::{Duration, Instant};
    use tempfile::TempDir;

    struct DummyBackend;

    impl SigningBackend for DummyBackend {
        fn sign(&mut self, data: &[u8]) -> Result<Vec<u8>, SignerError> {
            let mut sig = data.to_vec();
            sig.extend_from_slice(b":sig");
            Ok(sig)
        }

        fn public_key(&self) -> Result<PublicKey, SignerError> {
            Ok(PublicKey {
                bytes: vec![0u8; 32],
                key_type: KeyType::Ed25519,
            })
        }
    }

    fn create_single_node_cluster(base_port: u16) -> (Arc<SignerRaftNode>, TempDir) {
        let temp_dir = TempDir::new().unwrap();
        let node_id = 1u64;
        let peers = vec![PeerConfig {
            id: node_id,
            addr: format!("127.0.0.1:{}", base_port + node_id as u16),
        }];

        let config = RaftConfig {
            node_id,
            bind_addr: format!("127.0.0.1:{}", base_port + node_id as u16),
            data_path: temp_dir
                .path()
                .join(format!("node_{}", node_id))
                .to_str()
                .unwrap()
                .to_string(),
            peers,
            initial_state_path: "./test_consensus_state.json".to_string(),
        };

        let cluster = SignerRaftNode::new(config);

        let start = Instant::now();
        while start.elapsed() < Duration::from_secs(5) {
            if cluster.is_leader() {
                return (cluster, temp_dir);
            }
            thread::sleep(Duration::from_millis(50));
        }

        panic!("leader not elected in time");
    }

    fn extract_proposal_signature(
        resp: Response<
            <VersionV1_0 as ProtocolVersion>::ProposalResponse,
            <VersionV1_0 as ProtocolVersion>::VoteResponse,
            <VersionV1_0 as ProtocolVersion>::PubKeyResponse,
            <VersionV1_0 as ProtocolVersion>::PingResponse,
            <VersionV1_0 as ProtocolVersion>::BytesResponse,
        >,
    ) -> Vec<u8> {
        match resp {
            Response::SignedProposal(resp) => resp
                .proposal
                .expect("expected proposal")
                .signature
                .to_vec(),
            other => panic!("unexpected response: {:?}", other),
        }
    }

    fn extract_vote_signatures(
        resp: Response<
            <VersionV1_0 as ProtocolVersion>::ProposalResponse,
            <VersionV1_0 as ProtocolVersion>::VoteResponse,
            <VersionV1_0 as ProtocolVersion>::PubKeyResponse,
            <VersionV1_0 as ProtocolVersion>::PingResponse,
            <VersionV1_0 as ProtocolVersion>::BytesResponse,
        >,
    ) -> (Vec<u8>, Vec<u8>) {
        match resp {
            Response::SignedVote(resp) => {
                let vote = resp.vote.expect("expected vote");
                (vote.signature.to_vec(), vote.extension_signature.to_vec())
            }
            other => panic!("unexpected response: {:?}", other),
        }
    }

    #[test]
    fn replay_proposal_signature_on_exact_match() {
        let (raft_node, _temp_dir) = create_single_node_cluster(18000);
        let signing_lock = Arc::new(Mutex::new(()));
        let mut signer = Signer::<DummyBackend, VersionV1_0, Cursor<Vec<u8>>>::new(
            DummyBackend,
            Cursor::new(Vec::new()),
            "test-chain".to_string(),
        );

        let proposal = Proposal {
            step: SignedMsgType::Proposal,
            height: 10,
            round: 0,
            ..Default::default()
        };

        let resp1 = SigningHandler::<VersionV1_0>::process_request(
            &mut signer,
            Request::SignProposal(proposal.clone()),
            &raft_node,
            &signing_lock,
        )
        .unwrap();
        let sig1 = extract_proposal_signature(resp1);

        let resp2 = SigningHandler::<VersionV1_0>::process_request(
            &mut signer,
            Request::SignProposal(proposal.clone()),
            &raft_node,
            &signing_lock,
        )
        .unwrap();
        let sig2 = extract_proposal_signature(resp2);

        assert_eq!(sig1, sig2);

        let state = raft_node.signer_state.read().unwrap().clone();
        let sign_data = signer.proposal_sign_bytes(&proposal).unwrap();
        assert_eq!(state.sign_data, sign_data);
        assert_eq!(state.signature, sig1);
    }

    #[test]
    fn replay_vote_signature_with_extension_on_exact_match() {
        let (raft_node, _temp_dir) = create_single_node_cluster(18100);
        let signing_lock = Arc::new(Mutex::new(()));
        let mut signer = Signer::<DummyBackend, VersionV1_0, Cursor<Vec<u8>>>::new(
            DummyBackend,
            Cursor::new(Vec::new()),
            "test-chain".to_string(),
        );

        let vote = Vote {
            step: SignedMsgType::Precommit,
            height: 11,
            round: 1,
            block_id: Some(BlockId {
                hash: vec![1, 2, 3],
                parts: Some(PartSetHeader {
                    total: 1,
                    hash: vec![4, 5, 6],
                }),
            }),
            extension: vec![9, 9, 9],
            ..Default::default()
        };

        let resp1 = SigningHandler::<VersionV1_0>::process_request(
            &mut signer,
            Request::SignVote(vote.clone()),
            &raft_node,
            &signing_lock,
        )
        .unwrap();
        let (sig1, ext_sig1) = extract_vote_signatures(resp1);
        assert!(!ext_sig1.is_empty());

        let resp2 = SigningHandler::<VersionV1_0>::process_request(
            &mut signer,
            Request::SignVote(vote.clone()),
            &raft_node,
            &signing_lock,
        )
        .unwrap();
        let (sig2, ext_sig2) = extract_vote_signatures(resp2);

        assert_eq!(sig1, sig2);
        assert_eq!(ext_sig1, ext_sig2);

        let state = raft_node.signer_state.read().unwrap().clone();
        let sign_data = signer.vote_sign_bytes(&vote).unwrap();
        let ext_sign_data = signer.vote_ext_sign_bytes(&vote).unwrap();
        assert_eq!(state.sign_data, sign_data);
        assert_eq!(state.signature, sig1);
        assert_eq!(state.ext_sign_data, ext_sign_data);
        assert_eq!(state.ext_signature, ext_sig1);
    }
}
