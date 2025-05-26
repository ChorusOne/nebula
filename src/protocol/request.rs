use crate::types::{Proposal, Vote};

#[derive(Debug)]
pub enum Request {
    SignProposal(Proposal),
    SignVote(Vote),
    ShowPublicKey,
    PingRequest,
}
