pub mod request;
pub mod response;

use crate::types::{Proposal, Vote};

#[derive(Debug)]
pub enum Request {
    SignProposal(Proposal),
    SignVote(Vote),
    ShowPublicKey,
    Ping,
}

#[derive(Debug)]
pub enum Response<P, V, K, G> {
    SignedProposal(P),
    SignedVote(V),
    PublicKey(K),
    Ping(G),
}
