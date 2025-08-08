use crate::types::{ConsensusData, Proposal, SignedMsgType, Vote};
use log::info;
use std::cmp::Ordering;

#[derive(Debug)]
pub enum Request {
    ShowPublicKey,
    Ping,
    Proposal(Proposal),
    Vote(Vote),
}

pub enum CheckedRequest {
    DoubleSignVote(ConsensusData),
    DoubleSignProposal(ConsensusData),
    ValidRequest(ValidRequest),
}

pub enum ValidRequest {
    Proposal(Proposal),
    Vote(Vote),
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
    pub fn check(self, state: &ConsensusData) -> CheckedRequest {
        let req_state = ConsensusData {
            height: self.height,
            round: self.round,
            step: self.step as u8,
        };
        if should_sign_vote(state, &self) {
            CheckedRequest::ValidRequest(ValidRequest::Vote(self))
        } else {
            CheckedRequest::DoubleSignVote(req_state)
        }
    }
}
impl Proposal {
    pub fn check(self, state: &ConsensusData) -> CheckedRequest {
        let req_state = ConsensusData {
            height: self.height,
            round: self.round,
            step: self.step as u8,
        };
        if should_sign_proposal(state, &self) {
            CheckedRequest::ValidRequest(ValidRequest::Proposal(self))
        } else {
            CheckedRequest::DoubleSignProposal(req_state)
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
        "checking if proposal should be signed, state: {}, proposal: {}/{}/{}",
        state, proposal.height, proposal.round, proposal.step as u8
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
        "checking if vote should be signed, state: {}, vote: {}/{}/{}",
        state, vote.height, vote.round, vote.step as u8
    );
    let vote_step = vote.step;
    match (
        vote.height.cmp(&state.height),
        vote.round.cmp(&state.round),
        vote_step,
        state.step.into(),
    ) {
        // (1)
        (Ordering::Greater, _, _, _) => true,

        // (2)
        (Ordering::Equal, Ordering::Greater, _, _) => true,

        // (3)
        (Ordering::Equal, Ordering::Equal, SignedMsgType::Prevote, SignedMsgType::Proposal) => true,

        // (4)
        (Ordering::Equal, Ordering::Equal, SignedMsgType::Precommit, stp)
            if stp != SignedMsgType::Precommit =>
        {
            true
        }

        // everything else: don't sign
        _ => false,
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
        step: SignedMsgType::Proposal as u8,
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

    state.step = SignedMsgType::Prevote as u8;
    assert!(!should_sign_proposal(&state, &p3));
}

#[test]
fn should_sign_vote_logic() {
    let state = ConsensusData {
        height: 10,
        round: 1,
        step: SignedMsgType::Proposal as u8,
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
        step: SignedMsgType::Prevote as u8,
    };
    assert!(should_sign_vote(&state_after_prevote, &v4));

    let state_after_precommit = ConsensusData {
        height: 10,
        round: 1,
        step: SignedMsgType::Precommit as u8,
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
