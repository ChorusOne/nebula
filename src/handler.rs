use crate::backend::SigningBackend;
use crate::cluster::SignerRaftNode;
use crate::error::SignerError;
use crate::protocol::{Request, Response};
use crate::safeguards;
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
        Response<V::ProposalResponse, V::VoteResponse, V::PubKeyResponse, V::PingResponse>,
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

                let current_state = *raft_node.signer_state.read().unwrap();
                if !safeguards::should_sign_proposal(&current_state, &proposal) {
                    info!(
                        "Prevented double signing proposal at hrs: {}/{}/{}",
                        proposal.height, proposal.round, proposal.step as u8
                    );
                    Ok(Response::SignedProposal(V::create_proposal_response(
                        None,
                        Vec::new(),
                        Some("Would double-sign proposal at same height/round/step".into()),
                    )))
                } else {
                    let new_state = ConsensusData {
                        height: proposal.height,
                        round: proposal.round,
                        step: SignedMsgType::Proposal as u8,
                    };

                    if let Err(e) = raft_node.replicate_state(new_state) {
                        error!("CRITICAL: State replication failed: {}. Not signing.", e);
                        Ok(Response::SignedProposal(V::create_proposal_response(
                            None,
                            Vec::new(),
                            Some(format!("Raft replication failed: {}", e)),
                        )))
                    } else {
                        signer.process_request(Request::SignProposal(proposal))
                    }
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

                let current_state = *raft_node.signer_state.read().unwrap();
                if !safeguards::should_sign_vote(&current_state, &vote) {
                    info!(
                        "Prevented double signing vote at hrs: {}/{}/{}",
                        vote.height, vote.round, vote.step as u8
                    );
                    Ok(Response::SignedVote(V::create_vote_response(
                        None,
                        Vec::new(),
                        None,
                        Some("Would double-sign vote at same height/round".into()),
                    )))
                } else {
                    let new_state = ConsensusData {
                        height: vote.height,
                        round: vote.round,
                        step: vote.step.into(),
                    };

                    if let Err(e) = raft_node.replicate_state(new_state) {
                        error!("CRITICAL: State replication failed: {}. Not signing.", e);
                        Ok(Response::SignedVote(V::create_vote_response(
                            None,
                            Vec::new(),
                            None,
                            Some(format!("Raft replication failed: {}", e)),
                        )))
                    } else {
                        signer.process_request(Request::SignVote(vote))
                    }
                }
            }

            other => signer.process_request(other),
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
