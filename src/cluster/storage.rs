use crate::types::ConsensusData;
use log::info;
use protobuf::Message as PbMessage;
use raft::{Error as RaftError, Storage, StorageError};
use raft::{GetEntriesContext, prelude::*};
use rocksdb::{DB, Options};
use std::path::Path;

const KEY_HARD_STATE: &[u8] = b"hard_state";
const KEY_CONF_STATE: &[u8] = b"conf_state";
const KEY_LAST_INDEX: &[u8] = b"last_index";
const KEY_SIGNER_STATE: &[u8] = b"state_machine";

fn entry_key(index: u64) -> Vec<u8> {
    format!("entry:{}", index).into_bytes()
}

pub struct RaftStorage {
    db: DB,
}

impl RaftStorage {
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        let mut opts = Options::default();
        opts.create_if_missing(true);
        let db = DB::open(&opts, path).expect("Failed to open RocksDB");
        RaftStorage { db }
    }

    pub fn read_signer_state(&self) -> raft::Result<ConsensusData> {
        match self
            .db
            .get(KEY_SIGNER_STATE)
            .map_err(|e| RaftError::Store(StorageError::Other(Box::new(e))))?
        {
            Some(bytes) => Ok(ConsensusData::from_bytes(&bytes).unwrap_or_default()),
            None => Ok(ConsensusData::default()),
        }
    }

    pub fn write_signer_state(&self, sm: &ConsensusData) -> raft::Result<()> {
        let value = sm.to_bytes();
        self.db
            .put(KEY_SIGNER_STATE, &value)
            .map_err(|e| RaftError::Store(StorageError::Other(Box::new(e))))
    }

    pub fn append_entries(&mut self, entries: &[Entry]) -> raft::Result<()> {
        for entry in entries {
            let key = entry_key(entry.get_index());
            let value = entry
                .write_to_bytes()
                .map_err(|e| RaftError::Store(StorageError::Other(Box::new(e))))?;
            self.db
                .put(key, &value)
                .map_err(|e| RaftError::Store(StorageError::Other(Box::new(e))))?;
        }
        if let Some(last_entry) = entries.last() {
            self.db
                .put(KEY_LAST_INDEX, last_entry.get_index().to_be_bytes())
                .map_err(|e| RaftError::Store(StorageError::Other(Box::new(e))))?;
        }
        Ok(())
    }

    pub fn set_hard_state(&mut self, hs: HardState) -> raft::Result<()> {
        let value = hs
            .write_to_bytes()
            .map_err(|e| RaftError::Store(StorageError::Other(Box::new(e))))?;
        self.db
            .put(KEY_HARD_STATE, &value)
            .map_err(|e| RaftError::Store(StorageError::Other(Box::new(e))))
    }

    pub fn set_conf_state(&mut self, cs: ConfState) -> raft::Result<()> {
        let value = cs
            .write_to_bytes()
            .map_err(|e| RaftError::Store(StorageError::Other(Box::new(e))))?;
        self.db
            .put(KEY_CONF_STATE, &value)
            .map_err(|e| RaftError::Store(StorageError::Other(Box::new(e))))
    }

    pub fn apply_snapshot(&mut self, snapshot: Snapshot) -> raft::Result<()> {
        info!(
            "[storage] Applying snapshot at index {}",
            snapshot.get_metadata().get_index()
        );
        let meta = snapshot.get_metadata();
        let term = meta.get_term();
        let index = meta.get_index();

        if let Some(sm_data) = ConsensusData::from_bytes(snapshot.get_data()) {
            self.write_signer_state(&sm_data)?;
        }

        self.set_conf_state(meta.get_conf_state().clone())?;

        let mut hs = self.initial_state()?.hard_state;
        hs.set_term(term);
        hs.set_commit(index);
        self.set_hard_state(hs)?;

        self.db
            .put(KEY_LAST_INDEX, index.to_be_bytes())
            .map_err(|e| RaftError::Store(StorageError::Other(Box::new(e))))?;
        Ok(())
    }
}

impl Storage for RaftStorage {
    fn initial_state(&self) -> raft::Result<RaftState> {
        let hard_state = match self
            .db
            .get(KEY_HARD_STATE)
            .map_err(|e| RaftError::Store(StorageError::Other(Box::new(e))))?
        {
            Some(bytes) => HardState::parse_from_bytes(&bytes)
                .map_err(|e| RaftError::Store(StorageError::Other(Box::new(e))))?,
            None => HardState::default(),
        };
        let conf_state = match self
            .db
            .get(KEY_CONF_STATE)
            .map_err(|e| RaftError::Store(StorageError::Other(Box::new(e))))?
        {
            Some(bytes) => ConfState::parse_from_bytes(&bytes)
                .map_err(|e| RaftError::Store(StorageError::Other(Box::new(e))))?,
            None => ConfState::default(),
        };
        Ok(RaftState {
            hard_state,
            conf_state,
        })
    }

    fn entries(
        &self,
        low: u64,
        high: u64,
        _max_size: impl Into<Option<u64>>,
        _context: GetEntriesContext,
    ) -> raft::Result<Vec<Entry>> {
        let mut entries = Vec::with_capacity((high - low) as usize);
        for i in low..high {
            match self
                .db
                .get(entry_key(i))
                .map_err(|e| RaftError::Store(StorageError::Other(Box::new(e))))?
            {
                Some(bytes) => {
                    let entry = Entry::parse_from_bytes(&bytes)
                        .map_err(|e| RaftError::Store(StorageError::Other(Box::new(e))))?;
                    entries.push(entry);
                }
                None => return Err(RaftError::Store(StorageError::Unavailable)),
            }
        }
        Ok(entries)
    }

    fn term(&self, idx: u64) -> raft::Result<u64> {
        if idx == 0 {
            return Ok(0);
        }
        let state = self.initial_state()?;
        let snapshot_index = state.hard_state.get_commit();
        if idx == snapshot_index {
            return Ok(state.hard_state.get_term());
        }
        if idx < snapshot_index {
            return Err(RaftError::Store(StorageError::Compacted));
        }

        match self
            .db
            .get(entry_key(idx))
            .map_err(|e| RaftError::Store(StorageError::Other(Box::new(e))))?
        {
            Some(bytes) => Ok(Entry::parse_from_bytes(&bytes)
                .map_err(|e| RaftError::Store(StorageError::Other(Box::new(e))))?
                .get_term()),
            None => Err(RaftError::Store(StorageError::Unavailable)),
        }
    }

    fn first_index(&self) -> raft::Result<u64> {
        Ok(self.initial_state()?.hard_state.get_commit() + 1)
    }

    fn last_index(&self) -> raft::Result<u64> {
        match self
            .db
            .get(KEY_LAST_INDEX)
            .map_err(|e| RaftError::Store(StorageError::Other(Box::new(e))))?
        {
            Some(bytes) => Ok(u64::from_be_bytes(bytes.try_into().unwrap_or_default())),
            None => Ok(self.initial_state()?.hard_state.get_commit()),
        }
    }

    fn snapshot(&self, request_index: u64, _to: u64) -> raft::Result<Snapshot> {
        let state = self.initial_state()?;
        let last_index = self.last_index()?;

        if request_index > last_index {
            return Err(RaftError::Store(StorageError::SnapshotOutOfDate));
        }

        info!(
            "[storage] Creating snapshot at index {}",
            state.hard_state.get_commit()
        );

        let mut snapshot = Snapshot::default();
        snapshot.mut_metadata().set_conf_state(state.conf_state);
        snapshot
            .mut_metadata()
            .set_index(state.hard_state.get_commit());
        snapshot
            .mut_metadata()
            .set_term(state.hard_state.get_term());

        let sm_data = self.read_signer_state()?;
        snapshot.set_data(sm_data.to_bytes().into());

        Ok(snapshot)
    }
}
