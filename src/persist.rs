use crate::cluster::SignerRaftNode;
use crate::protocol::ValidRequest;
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

// TODO: make ValidRequest not pub
pub struct PersistedRequest(pub ValidRequest);

#[enum_dispatch]
pub trait Persist {
    fn persist(&mut self, request: ValidRequest) -> Result<PersistedRequest, PersistError>;
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
    fn persist(&mut self, request: ValidRequest) -> Result<PersistedRequest, PersistError> {
        self.state = ConsensusData::from(&request);
        Ok(PersistedRequest(request))
    }
    fn state(&self) -> ConsensusData {
        self.state
    }
}

impl Persist for SignerRaftNode {
    fn persist(&mut self, request: ValidRequest) -> Result<PersistedRequest, PersistError> {
        if !self.is_leader() {
            return Err(PersistError::InvalidState("Not the leader".into()));
        }
        let state = ConsensusData::from(&request);
        if let Err(e) = self.replicate_state(&state) {
            return Err(PersistError::CouldNotPersist(e.to_string()));
        }
        Ok(PersistedRequest(request))
    }
    fn state(&self) -> ConsensusData {
        *self.signer_state.read().unwrap()
    }
}
