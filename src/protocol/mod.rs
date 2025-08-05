use crate::types::{ConsensusData, Proposal, Vote};

#[derive(Debug)]
pub enum Request {
    SignProposal(Proposal),
    SignVote(Vote),
    ShowPublicKey,
    Ping,
}

#[derive(Debug)]
#[non_exhaustive]
pub enum Response<P, V, K, G> {
    SignedProposal((P, ConsensusData)),
    SignedVote((V, ConsensusData)),
    PublicKey(K),
    Ping(G),
    WouldDoubleSign,
}
