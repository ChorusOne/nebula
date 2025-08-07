use crate::cluster::SignerRaftNode;
use crate::types::ConsensusData;
use enum_dispatch::enum_dispatch;

#[derive(Debug)]
pub enum PersistError {
    InvalidState(String),
    CouldNotPersist(String),
}

#[enum_dispatch(Persist)]
pub enum PersistVariants {
    Raft(SignerRaftNode),
    Local(LocalState),
}

#[enum_dispatch]
pub trait Persist {
    fn persist(&mut self, state: &ConsensusData) -> Result<(), PersistError>;
    fn state(&self) -> ConsensusData;
}

pub struct LocalState {
    state: ConsensusData,
}

impl LocalState {
    pub fn new(state: &ConsensusData) -> LocalState {
        LocalState { state: *state }
    }
}

impl Persist for LocalState {
    fn persist(&mut self, state: &ConsensusData) -> Result<(), PersistError> {
        self.state = *state;
        Ok(())
    }
    fn state(&self) -> ConsensusData {
        self.state
    }
}

impl Persist for SignerRaftNode {
    fn persist(&mut self, state: &ConsensusData) -> Result<(), PersistError> {
        if !self.is_leader() {
            return Err(PersistError::InvalidState("Not the leader".into()));
        }
        if let Err(e) = self.replicate_state(state) {
            return Err(PersistError::CouldNotPersist(e.to_string()));
        }
        Ok(())
    }
    fn state(&self) -> ConsensusData {
        *self.signer_state.read().unwrap()
    }
}
