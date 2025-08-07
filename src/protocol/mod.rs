use crate::types::{Proposal, Vote};

#[derive(Debug)]
pub enum SignRequest {
    Proposal(Proposal),
    Vote(Vote),
}

#[derive(Debug)]
pub enum Request {
    ShowPublicKey,
    Ping,
    Signable(SignRequest),
}

#[derive(Debug)]
#[non_exhaustive]
pub enum Response<P, V, K, G> {
    SignedProposal(P),
    SignedVote(V),
    PublicKey(K),
    Ping(G),
}
