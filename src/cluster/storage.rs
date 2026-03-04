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
const KEY_APPLIED_INDEX: &[u8] = b"applied_index";

fn entry_key(index: u64) -> Vec<u8> {
    format!("entry:{}", index).into_bytes()
}

pub struct RocksDBStorage {
    db: DB,
}

impl RocksDBStorage {
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        let mut opts = Options::default();
        opts.create_if_missing(true);
        let db = DB::open(&opts, path).expect("Failed to open RocksDB");
        RocksDBStorage { db }
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
        let mut opts = rocksdb::WriteOptions::default();
        opts.set_sync(true);
        self.db
            .put_opt(KEY_SIGNER_STATE, &value, &opts)
            .map_err(|e| RaftError::Store(StorageError::Other(Box::new(e))))
    }

    pub fn append_entries(&mut self, entries: &[Entry]) -> raft::Result<()> {
        let mut opts = rocksdb::WriteOptions::default();
        opts.set_sync(true);
        for entry in entries {
            let key = entry_key(entry.get_index());
            let value = entry
                .write_to_bytes()
                .map_err(|e| RaftError::Store(StorageError::Other(Box::new(e))))?;
            self.db
                .put_opt(key, &value, &opts)
                .map_err(|e| RaftError::Store(StorageError::Other(Box::new(e))))?;
        }
        if let Some(last_entry) = entries.last() {
            self.db
                .put_opt(KEY_LAST_INDEX, last_entry.get_index().to_be_bytes(), &opts)
                .map_err(|e| RaftError::Store(StorageError::Other(Box::new(e))))?;
        }
        Ok(())
    }

    pub fn set_hard_state(&mut self, hs: HardState) -> raft::Result<()> {
        let mut opts = rocksdb::WriteOptions::default();
        opts.set_sync(true);
        let value = hs
            .write_to_bytes()
            .map_err(|e| RaftError::Store(StorageError::Other(Box::new(e))))?;
        self.db
            .put_opt(KEY_HARD_STATE, &value, &opts)
            .map_err(|e| RaftError::Store(StorageError::Other(Box::new(e))))
    }

    pub fn set_conf_state(&mut self, cs: ConfState) -> raft::Result<()> {
        let mut opts = rocksdb::WriteOptions::default();
        opts.set_sync(true);
        let value = cs
            .write_to_bytes()
            .map_err(|e| RaftError::Store(StorageError::Other(Box::new(e))))?;
        self.db
            .put_opt(KEY_CONF_STATE, &value, &opts)
            .map_err(|e| RaftError::Store(StorageError::Other(Box::new(e))))
    }

    pub fn set_commit_index(&mut self, commit: u64) -> raft::Result<()> {
        let current = self.initial_state()?.hard_state;
        if commit <= current.get_commit() {
            return Ok(());
        }

        let mut hs = current;
        hs.set_commit(commit);
        self.set_hard_state(hs)
    }

    pub fn applied_index(&self) -> raft::Result<u64> {
        match self
            .db
            .get(KEY_APPLIED_INDEX)
            .map_err(|e| RaftError::Store(StorageError::Other(Box::new(e))))?
        {
            Some(bytes) => Ok(u64::from_be_bytes(bytes.try_into().unwrap_or_default())),
            None => Ok(0),
        }
    }

    pub fn set_applied_index(&mut self, index: u64) -> raft::Result<()> {
        let mut opts = rocksdb::WriteOptions::default();
        opts.set_sync(true);
        self.db
            .put_opt(KEY_APPLIED_INDEX, index.to_be_bytes(), &opts)
            .map_err(|e| RaftError::Store(StorageError::Other(Box::new(e))))
    }

    pub fn apply_snapshot(&mut self, snapshot: Snapshot) -> raft::Result<()> {
        let mut opts = rocksdb::WriteOptions::default();
        opts.set_sync(true);
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
        self.set_applied_index(index)?;

        self.db
            .put_opt(KEY_LAST_INDEX, index.to_be_bytes(), &opts)
            .map_err(|e| RaftError::Store(StorageError::Other(Box::new(e))))?;
        Ok(())
    }

    pub fn recent_consensus_records(&self, limit: usize) -> raft::Result<Vec<ConsensusData>> {
        if limit == 0 {
            return Ok(Vec::new());
        }
        let first = self.first_index()?;
        let last = self.last_index()?;
        if last < first {
            return Ok(Vec::new());
        }

        let mut records = Vec::new();
        let mut start = last.saturating_add(1).saturating_sub(limit as u64);
        if start < first {
            start = first;
        }

        for index in start..=last {
            let Some(bytes) = self
                .db
                .get(entry_key(index))
                .map_err(|e| RaftError::Store(StorageError::Other(Box::new(e))))?
            else {
                continue;
            };

            let entry = Entry::parse_from_bytes(&bytes)
                .map_err(|e| RaftError::Store(StorageError::Other(Box::new(e))))?;
            if entry.get_entry_type() != EntryType::EntryNormal || entry.get_data().is_empty() {
                continue;
            }

            if let Some(record) = ConsensusData::from_bytes(entry.get_data()) {
                records.push(record);
            }
        }

        Ok(records)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::SignedMsgType;
    use tempfile::TempDir;

    #[test]
    fn applied_index_roundtrip() {
        let dir = TempDir::new().unwrap();
        let mut storage = RocksDBStorage::new(dir.path());
        assert_eq!(storage.applied_index().unwrap(), 0);

        storage.set_applied_index(42).unwrap();
        drop(storage);

        let reopened = RocksDBStorage::new(dir.path());
        assert_eq!(reopened.applied_index().unwrap(), 42);
    }

    #[test]
    fn commit_index_is_persisted() {
        let dir = TempDir::new().unwrap();
        let mut storage = RocksDBStorage::new(dir.path());

        storage.set_commit_index(7).unwrap();
        let hs = storage.initial_state().unwrap().hard_state;
        assert_eq!(hs.get_commit(), 7);
    }

    #[test]
    fn applying_snapshot_updates_applied_index() {
        let dir = TempDir::new().unwrap();
        let mut storage = RocksDBStorage::new(dir.path());

        let mut snapshot = Snapshot::default();
        snapshot.mut_metadata().set_index(5);
        snapshot.mut_metadata().set_term(2);

        storage.apply_snapshot(snapshot).unwrap();
        assert_eq!(storage.applied_index().unwrap(), 5);
        assert_eq!(storage.initial_state().unwrap().hard_state.get_commit(), 5);
    }

    #[test]
    fn recent_consensus_records_returns_last_normal_entries() {
        let dir = TempDir::new().unwrap();
        let mut storage = RocksDBStorage::new(dir.path());

        let mut hs = HardState::default();
        hs.set_commit(1);
        hs.set_term(1);
        storage.set_hard_state(hs).unwrap();

        let mut records = Vec::new();
        for i in 2..=8 {
            let record = ConsensusData {
                height: i as i64,
                round: 0,
                step: SignedMsgType::Proposal,
                sign_bytes_hash: vec![i as u8],
                signature: vec![i as u8, i as u8],
                extension_signature: Vec::new(),
            };
            records.push(record.clone());

            let mut entry = Entry::default();
            entry.set_index(i);
            entry.set_term(1);
            entry.set_entry_type(EntryType::EntryNormal);
            entry.set_data(record.to_bytes().into());
            storage.append_entries(&[entry]).unwrap();
        }

        let recent = storage.recent_consensus_records(3).unwrap();
        assert_eq!(recent.len(), 3);
        assert_eq!(recent[0].height, 6);
        assert_eq!(recent[1].height, 7);
        assert_eq!(recent[2].height, 8);
    }
}

impl Storage for RocksDBStorage {
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
