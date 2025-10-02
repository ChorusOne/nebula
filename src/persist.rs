use crate::cluster::SignerRaftNode;
use crate::protocol::ValidRequest;
use crate::types::ConsensusData;
use enum_dispatch::enum_dispatch;
use std::path::PathBuf;

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
    path: PathBuf,
}

impl LocalState {
    pub fn from_file(path: &PathBuf) -> Result<LocalState, std::io::Error> {
        let data = std::fs::File::open(path)?;
        let cd: ConsensusData =
            serde_json::de::from_reader(data).map_err(|_| std::io::ErrorKind::InvalidData)?;
        Ok(LocalState {
            state: cd,
            path: path.clone(),
        })
    }
}

impl Persist for LocalState {
    fn persist(&mut self, request: ValidRequest) -> Result<PersistedRequest, PersistError> {
        let new_cd = ConsensusData::from(&request);
        let serialized = serde_json::to_string(&new_cd)
            .map_err(|_| PersistError::InvalidState("Could not convert CD to JSON".to_string()))?;
        let tmp_path = self.path.with_extension("tmp");

        {
            let mut file = std::fs::OpenOptions::new()
                .write(true)
                .create(true)
                .truncate(true)
                .open(&tmp_path)
                .map_err(|e| PersistError::CouldNotPersist(e.to_string()))?;

            std::io::Write::write_all(&mut file, serialized.as_bytes())
                .map_err(|e| PersistError::CouldNotPersist(e.to_string()))?;
            file.sync_all()
                .map_err(|e| PersistError::CouldNotPersist(e.to_string()))?;
        }

        std::fs::rename(&tmp_path, &self.path)
            .map_err(|e| PersistError::CouldNotPersist(e.to_string()))?;

        if let Some(parent) = self.path.parent() {
            let dir = std::fs::File::open(parent)
                .map_err(|e| PersistError::CouldNotPersist(e.to_string()))?;
            dir.sync_all()
                .map_err(|e| PersistError::CouldNotPersist(e.to_string()))?;
        } else {
            return Err(PersistError::InvalidState(
                "No parent directory for persistence path".to_string(),
            ));
        }

        self.state = new_cd;
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
