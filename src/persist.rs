use crate::cluster::SignerRaftNode;
use crate::types::ConsensusData;

#[derive(Debug)]
pub enum PersistError {
    InvalidState,
    CouldNotPersist,
}
pub trait Persist {
    fn persist(&mut self, state: &ConsensusData) -> Result<(), PersistError>;
    fn state(&self) -> Result<ConsensusData, PersistError>;
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
    fn state(&self) -> Result<ConsensusData, PersistError> {
        Ok(self.state)
    }
}

impl Persist for SignerRaftNode {
    fn persist(&mut self, state: &ConsensusData) -> Result<(), PersistError> {
        if !self.is_leader() {
            return Err(PersistError::InvalidState);
        }
        if let Err(e) = self.replicate_state(state) {
            return Err(PersistError::CouldNotPersist);
        }
        Ok(())
    }
    fn state(&self) -> Result<ConsensusData, PersistError> {
        if !self.is_leader() {
            return Err(PersistError::InvalidState);
        }
        Ok(*self.signer_state.read().unwrap())
    }
}
