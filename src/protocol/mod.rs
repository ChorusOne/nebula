use crate::types::{Proposal, Vote};
use core::fmt;

pub enum Request {
    SignProposal(Proposal),
    SignVote(Vote),
    ShowPublicKey,
    SignBytes(Vec<u8>),
    Ping,
}

impl fmt::Debug for Request {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Request::SignProposal(p) => write!(
                f,
                "SignProposal(h:{}, r:{}, step:{})",
                p.height, p.round, p.step as u8
            ),
            Request::SignVote(v) => write!(
                f,
                "SignVote(h:{}, r:{}, step:{})",
                v.height, v.round, v.step as u8
            ),
            Request::ShowPublicKey => write!(f, "ShowPublicKey"),
            Request::Ping => write!(f, "Ping"),
            Request::SignBytes(bytes) => write!(f, "SignBytes({:?})", bytes),
        }
    }
}

#[derive(Debug)]
pub enum Response<P, V, K, G, B> {
    SignedProposal(P),
    SignedVote(V),
    PublicKey(K),
    SignBytes(B),
    Ping(G),
}
