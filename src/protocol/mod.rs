use crate::types::{ConsensusData, Proposal, SignedMsgType, Vote};
use core::fmt;
use log::info;
use std::cmp::Ordering;

pub enum Request {
    ShowPublicKey,
    Ping,
    Proposal(Proposal),
    Vote(Vote),
}

pub enum CheckedVoteRequest {
    DoubleSignVote(ConsensusData),
    ValidRequest(ValidRequest),
}
pub enum CheckedProposalRequest {
    DoubleSignProposal(ConsensusData),
    ValidRequest(ValidRequest),
}

pub enum ValidRequest {
    Proposal(Proposal),
    Vote(Vote),
}

impl fmt::Debug for Request {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Request::Proposal(p) => write!(
                f,
                "Proposal(h:{}, r:{}, step:{})",
                p.height, p.round, p.step as u8
            ),
            Request::Vote(v) => write!(
                f,
                "Vote(h:{}, r:{}, step:{})",
                v.height, v.round, v.step as u8
            ),
            Request::ShowPublicKey => write!(f, "ShowPublicKey"),
            Request::Ping => write!(f, "Ping"),
        }
    }
}

#[derive(Debug)]
#[non_exhaustive]
pub enum Response<P, V, K, G> {
    SignedProposal(P),
    SignedVote(V),
    PublicKey(K),
    Ping(G),
}

impl Vote {
    pub fn check(self, state: &ConsensusData) -> CheckedVoteRequest {
        let req_state = ConsensusData {
            height: self.height,
            round: self.round,
            step: self.step,
        };
        if should_sign_vote(state, &self) {
            CheckedVoteRequest::ValidRequest(ValidRequest::Vote(self))
        } else {
            CheckedVoteRequest::DoubleSignVote(req_state)
        }
    }
}
impl Proposal {
    pub fn check(self, state: &ConsensusData) -> CheckedProposalRequest {
        let req_state = ConsensusData {
            height: self.height,
            round: self.round,
            step: self.step,
        };
        if should_sign_proposal(state, &self) {
            CheckedProposalRequest::ValidRequest(ValidRequest::Proposal(self))
        } else {
            CheckedProposalRequest::DoubleSignProposal(req_state)
        }
    }
}

/*
A signer should only sign a proposal p if any of the following lines are true:

    p.Height > s.Height (1)
    p.Height == s.Height && p.Round > s.Round (2)

In other words, a proposal should only be signed if it’s at a higher height, or a higher round for the same height. Once a proposal or vote has been signed for a given height and round, a proposal should never be signed for the same height and round.
*/
fn should_sign_proposal(state: &ConsensusData, proposal: &Proposal) -> bool {
    if proposal.step != SignedMsgType::Proposal {
        return false;
    }

    info!(
        "checking if proposal should be signed, state: {}, proposal: {}/{}/{:?}",
        state, proposal.height, proposal.round, proposal.step
    );
    match (
        proposal.height.cmp(&state.height),
        proposal.round.cmp(&state.round),
    ) {
        // (1)
        (Ordering::Greater, _) => true,

        // (2)
        (Ordering::Equal, Ordering::Greater) => true,

        _ => false,
    }
}

fn valid_step_transition(state: &ConsensusData, step: SignedMsgType) -> bool {
    // have signed proposal, got asked to sign prevote
    if state.step == SignedMsgType::Proposal && step == SignedMsgType::Prevote {
        return true;
    }
    // have NOT signed precommit, got asked to sign precommit
    if state.step != SignedMsgType::Precommit && step == SignedMsgType::Precommit {
        return true;
    }
    false
}
/*
A signer should only sign a vote v if any of the following lines are true:

    v.Height > s.Height (1)
    v.Height == s.Height && v.Round > s.Round (2)
    v.Height == s.Height && v.Round == s.Round && v.Step == 0x1 && s.Step == 0x20 (3)
    v.Height == s.Height && v.Round == s.Round && v.Step == 0x2 && s.Step != 0x2 (4)

In other words, a vote should only be signed if it’s:
  - at a higher height
  - at a higher round for the same height
  - a prevote for the same height and round where we haven’t signed a prevote or precommit (but have signed a proposal)
  - a precommit for the same height and round where we haven’t signed a precommit (but have signed a proposal and/or a prevote)
*/
fn should_sign_vote(state: &ConsensusData, vote: &Vote) -> bool {
    info!(
        "checking if vote should be signed, state: {}, vote: {}/{}/{:?}",
        state, vote.height, vote.round, vote.step
    );
    match vote.height.cmp(&state.height) {
        Ordering::Greater => true,
        Ordering::Less => false,
        Ordering::Equal => match vote.round.cmp(&state.round) {
            Ordering::Greater => true,
            Ordering::Less => false,
            Ordering::Equal => valid_step_transition(state, vote.step),
        },
    }
}

/*
* func shouldSignVoteExtension(chainID string, signBz, extSignBz []byte) (bool, error) {
   var vote cmtypes.CanonicalVote
   if err := protoio.UnmarshalDelimited(signBz, &vote); err != nil {
       return false, nil
   }

   if vote.Type == cmtypes.PrecommitType && vote.BlockID != nil && len(extSignBz) > 0 {
       var ext cmtypes.CanonicalVoteExtension
       if err := protoio.UnmarshalDelimited(extSignBz, &ext); err != nil {
           return false, fmt.Errorf("failed to unmarshal vote extension: %w", err)
       }

       switch {
       case ext.ChainId != chainID:
           return false, fmt.Errorf("extension chain ID %s does not match expected %s", ext.ChainId, chainID)
       case ext.Height != vote.Height:
           return false, fmt.Errorf("extension height %d does not match vote height %d", ext.Height, vote.Height)
       case ext.Round != vote.Round:
           return false, fmt.Errorf("extension round %d does not match vote round %d", ext.Round, vote.Round)
       }

       return true, nil
   }

   return false, nil
}
*/

#[test]
fn should_sign_proposal_logic() {
    let mut state = ConsensusData {
        height: 10,
        round: 1,
        step: SignedMsgType::Proposal,
    };

    let p1 = Proposal {
        step: SignedMsgType::Proposal,
        height: 11,
        round: 0,
        ..Default::default()
    };
    assert!(should_sign_proposal(&state, &p1));

    let p2 = Proposal {
        step: SignedMsgType::Proposal,
        height: 10,
        round: 2,
        ..Default::default()
    };
    assert!(should_sign_proposal(&state, &p2));
    let p3 = Proposal {
        step: SignedMsgType::Proposal,
        height: 10,
        round: 1,
        ..Default::default()
    };
    assert!(!should_sign_proposal(&state, &p3));

    let p4 = Proposal {
        step: SignedMsgType::Proposal,
        height: 10,
        round: 0,
        ..Default::default()
    };
    assert!(!should_sign_proposal(&state, &p4));

    let p5 = Proposal {
        step: SignedMsgType::Proposal,
        height: 9,
        round: 5,
        ..Default::default()
    };
    assert!(!should_sign_proposal(&state, &p5));

    state.step = SignedMsgType::Prevote;
    assert!(!should_sign_proposal(&state, &p3));
}

#[test]
fn should_sign_vote_logic() {
    let state = ConsensusData {
        height: 10,
        round: 1,
        step: SignedMsgType::Proposal,
    };
    let block_id = Some(crate::types::BlockId {
        hash: vec![1],
        parts: Some(crate::types::PartSetHeader {
            total: 1,
            hash: vec![2],
        }),
    });

    let v1 = Vote {
        height: 11,
        round: 0,
        step: SignedMsgType::Prevote,
        ..Default::default()
    };
    assert!(should_sign_vote(&state, &v1));

    let v2 = Vote {
        height: 10,
        round: 2,
        step: SignedMsgType::Prevote,
        ..Default::default()
    };
    assert!(should_sign_vote(&state, &v2));

    let v3 = Vote {
        height: 10,
        round: 1,
        step: SignedMsgType::Prevote,
        ..Default::default()
    };
    assert!(should_sign_vote(&state, &v3));

    let v4 = Vote {
        height: 10,
        round: 1,
        step: SignedMsgType::Precommit,
        block_id: block_id.clone(),
        ..Default::default()
    };
    assert!(should_sign_vote(&state, &v4));

    let state_after_prevote = ConsensusData {
        height: 10,
        round: 1,
        step: SignedMsgType::Prevote,
    };
    assert!(should_sign_vote(&state_after_prevote, &v4));

    let state_after_precommit = ConsensusData {
        height: 10,
        round: 1,
        step: SignedMsgType::Precommit,
    };
    assert!(!should_sign_vote(&state_after_precommit, &v4));

    assert!(!should_sign_vote(&state_after_prevote, &v3));

    let v5 = Vote {
        height: 9,
        round: 5,
        step: SignedMsgType::Prevote,
        ..Default::default()
    };
    assert!(!should_sign_vote(&state, &v5));
}

#[test]
fn test_step_transition_proposal() {
    let state = ConsensusData {
        height: 10,
        round: 1,
        step: SignedMsgType::Proposal,
    };
    assert_eq!(valid_step_transition(&state, SignedMsgType::Prevote), true);
    // moving to pre-commit is always valid from non-precommit states
    assert_eq!(
        valid_step_transition(&state, SignedMsgType::Precommit),
        true
    );
    assert_eq!(valid_step_transition(&state, SignedMsgType::Unknown), false);
    assert_eq!(
        valid_step_transition(&state, SignedMsgType::Proposal),
        false
    );
}

#[test]
fn test_step_transition_precommit() {
    let state = ConsensusData {
        height: 10,
        round: 1,
        step: SignedMsgType::Precommit,
    };
    // moving from pre-commit is never allowed
    assert_eq!(
        valid_step_transition(&state, SignedMsgType::Precommit),
        false
    );
    assert_eq!(valid_step_transition(&state, SignedMsgType::Prevote), false);
    assert_eq!(valid_step_transition(&state, SignedMsgType::Unknown), false);
    assert_eq!(
        valid_step_transition(&state, SignedMsgType::Proposal),
        false
    );
}

#[test]
fn test_step_transition_unknown() {
    let state = ConsensusData {
        height: 10,
        round: 1,
        step: SignedMsgType::Unknown,
    };
    // can only transition to precommit
    assert_eq!(
        valid_step_transition(&state, SignedMsgType::Precommit),
        true
    );
    assert_eq!(valid_step_transition(&state, SignedMsgType::Prevote), false);
    assert_eq!(valid_step_transition(&state, SignedMsgType::Unknown), false);
    assert_eq!(
        valid_step_transition(&state, SignedMsgType::Proposal),
        false
    );
}

#[test]
fn test_step_transition_prevote() {
    let state = ConsensusData {
        height: 10,
        round: 1,
        step: SignedMsgType::Prevote,
    };
    assert_eq!(
        valid_step_transition(&state, SignedMsgType::Precommit),
        true
    );
    assert_eq!(valid_step_transition(&state, SignedMsgType::Prevote), false);
    assert_eq!(valid_step_transition(&state, SignedMsgType::Unknown), false);
    assert_eq!(
        valid_step_transition(&state, SignedMsgType::Proposal),
        false
    );
}
