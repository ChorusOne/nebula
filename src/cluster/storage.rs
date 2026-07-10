use crate::types::ConsensusData;
use log::info;
use protobuf::Message as PbMessage;
use raft::{Error as RaftError, Storage, StorageError};
use raft::{GetEntriesContext, prelude::*};
use rocksdb::{DB, IteratorMode, Options, WriteBatch, WriteOptions};
use std::io;
use std::path::Path;

const KEY_HARD_STATE: &[u8] = b"hard_state";
const KEY_CONF_STATE: &[u8] = b"conf_state";
const KEY_LAST_INDEX: &[u8] = b"last_index";
const KEY_SIGNER_STATE: &[u8] = b"state_machine";
const KEY_APPLIED_INDEX: &[u8] = b"applied_index";
const KEY_TRUNCATED_INDEX: &[u8] = b"truncated_index";
const KEY_TRUNCATED_TERM: &[u8] = b"truncated_term";

const CONSENSUS_DATA_FIELDS: &[&str] = &[
    "height",
    "round",
    "step",
    "sign_data",
    "signature",
    "ext_sign_data",
    "ext_signature",
];

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
        let bytes = self
            .db
            .get(KEY_SIGNER_STATE)
            .map_err(store_error)?
            .ok_or_else(|| invalid_data("missing persisted signer state"))?;
        decode_consensus_data(&bytes)
    }

    #[cfg(test)]
    pub fn write_signer_state(&self, sm: &ConsensusData) -> raft::Result<()> {
        let value = sm.to_bytes();
        let mut opts = rocksdb::WriteOptions::default();
        opts.set_sync(true);
        self.db
            .put_opt(KEY_SIGNER_STATE, &value, &opts)
            .map_err(store_error)
    }

    /// Persist the state machine and the index it represents as one durable update.
    #[allow(dead_code)] // Wired into the Raft apply path separately from this storage-only change.
    pub fn write_signer_state_and_applied(
        &self,
        sm: &ConsensusData,
        applied_index: u64,
    ) -> raft::Result<()> {
        let mut batch = WriteBatch::default();
        batch.put(KEY_SIGNER_STATE, sm.to_bytes());
        batch.put(KEY_APPLIED_INDEX, applied_index.to_be_bytes());
        self.write_batch(batch)
    }

    pub fn append_entries(&mut self, entries: &[Entry]) -> raft::Result<()> {
        if entries.is_empty() {
            return Ok(());
        }

        let first_new_index = entries[0].get_index();
        let first_index = self.first_index()?;
        let old_last_index = self.last_index()?;
        assert!(
            first_new_index >= first_index,
            "overwrite compacted raft logs, compacted: {}, append: {}",
            first_index - 1,
            first_new_index
        );
        assert!(
            first_new_index <= old_last_index + 1,
            "raft logs should be continuous, last index: {}, new appended: {}",
            old_last_index,
            first_new_index
        );

        let mut batch = WriteBatch::default();
        let mut expected_index = first_new_index;
        for entry in entries {
            assert_eq!(
                entry.get_index(),
                expected_index,
                "raft entries must be contiguous"
            );
            expected_index += 1;

            let key = entry_key(entry.get_index());
            let value = entry.write_to_bytes().map_err(store_error)?;
            batch.put(key, value);
        }

        let new_last_index = entries.last().unwrap().get_index();
        for index in (new_last_index + 1)..=old_last_index {
            batch.delete(entry_key(index));
        }
        batch.put(KEY_LAST_INDEX, new_last_index.to_be_bytes());
        self.write_batch(batch)
    }

    pub fn set_hard_state(&mut self, hs: HardState) -> raft::Result<()> {
        let mut opts = rocksdb::WriteOptions::default();
        opts.set_sync(true);
        let value = hs.write_to_bytes().map_err(store_error)?;
        self.db
            .put_opt(KEY_HARD_STATE, &value, &opts)
            .map_err(store_error)
    }

    pub fn set_conf_state(&mut self, cs: ConfState) -> raft::Result<()> {
        let mut opts = rocksdb::WriteOptions::default();
        opts.set_sync(true);
        let value = cs.write_to_bytes().map_err(store_error)?;
        self.db
            .put_opt(KEY_CONF_STATE, &value, &opts)
            .map_err(store_error)
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
        self.read_index(KEY_APPLIED_INDEX, "applied index")
    }

    pub fn apply_snapshot(&mut self, snapshot: Snapshot) -> raft::Result<()> {
        let meta = snapshot.get_metadata();
        let term = meta.get_term();
        let index = meta.get_index();
        info!("[storage] Applying snapshot at index {}", index);

        if index == 0 {
            return Err(invalid_data("cannot apply an empty Raft snapshot"));
        }
        let snapshot_signer_state = decode_consensus_data(snapshot.get_data())?;
        let signer_state = if self.is_empty()? {
            snapshot_signer_state
        } else {
            self.read_signer_state()?
                .validated_update(&snapshot_signer_state)
                .map_err(store_error)?
        };
        let (truncated_index, _) = self.truncated_state()?;
        let current_state = self.initial_state()?;

        if index <= truncated_index || index < current_state.hard_state.get_commit() {
            return Err(RaftError::Store(StorageError::SnapshotOutOfDate));
        }

        let old_last_index = self.last_index()?;
        let mut hard_state = current_state.hard_state;
        if term > hard_state.get_term() {
            hard_state.set_term(term);
            hard_state.set_vote(0);
        }
        hard_state.set_commit(index);

        let hard_state_bytes = hard_state.write_to_bytes().map_err(store_error)?;
        let conf_state_bytes = meta
            .get_conf_state()
            .write_to_bytes()
            .map_err(store_error)?;

        let mut batch = WriteBatch::default();
        batch.put(KEY_SIGNER_STATE, signer_state.to_bytes());
        batch.put(KEY_CONF_STATE, conf_state_bytes);
        batch.put(KEY_HARD_STATE, hard_state_bytes);
        batch.put(KEY_APPLIED_INDEX, index.to_be_bytes());
        batch.put(KEY_LAST_INDEX, index.to_be_bytes());
        batch.put(KEY_TRUNCATED_INDEX, index.to_be_bytes());
        batch.put(KEY_TRUNCATED_TERM, term.to_be_bytes());

        for entry_index in (truncated_index + 1)..=old_last_index {
            batch.delete(entry_key(entry_index));
        }
        self.write_batch(batch)
    }

    fn write_batch(&self, batch: WriteBatch) -> raft::Result<()> {
        let mut opts = WriteOptions::default();
        opts.set_sync(true);
        self.db.write_opt(batch, &opts).map_err(store_error)
    }

    fn is_empty(&self) -> raft::Result<bool> {
        match self.db.iterator(IteratorMode::Start).next() {
            None => Ok(true),
            Some(Ok(_)) => Ok(false),
            Some(Err(error)) => Err(store_error(error)),
        }
    }

    fn optional_u64(&self, key: &[u8], description: &str) -> raft::Result<Option<u64>> {
        let Some(bytes) = self.db.get(key).map_err(store_error)? else {
            return Ok(None);
        };
        let encoded: [u8; 8] = bytes.as_slice().try_into().map_err(|_| {
            invalid_data(format!(
                "malformed {description}: expected 8 bytes, got {}",
                bytes.len()
            ))
        })?;
        Ok(Some(u64::from_be_bytes(encoded)))
    }

    fn read_index(&self, key: &[u8], description: &str) -> raft::Result<u64> {
        match self.optional_u64(key, description)? {
            Some(index) => Ok(index),
            None if self.is_empty()? => Ok(0),
            None => Err(invalid_data(format!("missing persisted {description}"))),
        }
    }

    fn truncated_state(&self) -> raft::Result<(u64, u64)> {
        let index = self.optional_u64(KEY_TRUNCATED_INDEX, "truncated index")?;
        let term = self.optional_u64(KEY_TRUNCATED_TERM, "truncated term")?;
        match (index, term) {
            (Some(index), Some(term)) => Ok((index, term)),
            (None, None) if self.is_empty()? => Ok((0, 0)),
            (None, None) => Err(invalid_data(
                "initialized store is missing truncated Raft log metadata",
            )),
            _ => Err(invalid_data("incomplete truncated Raft log metadata")),
        }
    }
}

impl Storage for RocksDBStorage {
    fn initial_state(&self) -> raft::Result<RaftState> {
        let hard_state_bytes = self.db.get(KEY_HARD_STATE).map_err(store_error)?;
        let conf_state_bytes = self.db.get(KEY_CONF_STATE).map_err(store_error)?;
        let (hard_state, conf_state) = match (hard_state_bytes, conf_state_bytes) {
            (Some(hard_state), Some(conf_state)) => (
                HardState::parse_from_bytes(&hard_state).map_err(store_error)?,
                ConfState::parse_from_bytes(&conf_state).map_err(store_error)?,
            ),
            (None, None) if self.is_empty()? => (HardState::default(), ConfState::default()),
            (None, None) => {
                return Err(invalid_data(
                    "initialized store is missing Raft hard/conf state",
                ));
            }
            _ => return Err(invalid_data("incomplete persisted Raft state")),
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
        max_size: impl Into<Option<u64>>,
        _context: GetEntriesContext,
    ) -> raft::Result<Vec<Entry>> {
        assert!(low <= high, "invalid Raft log range: {low} > {high}");
        let first_index = self.first_index()?;
        if low < first_index {
            return Err(RaftError::Store(StorageError::Compacted));
        }
        let last_index = self.last_index()?;
        assert!(
            high <= last_index + 1,
            "Raft log range ends after last index: high={high}, last={last_index}"
        );

        let mut entries = Vec::with_capacity((high - low) as usize);
        for i in low..high {
            match self.db.get(entry_key(i)).map_err(store_error)? {
                Some(bytes) => {
                    let entry = Entry::parse_from_bytes(&bytes).map_err(store_error)?;
                    if entry.get_index() != i {
                        return Err(invalid_data(format!(
                            "entry under index {i} contains index {}",
                            entry.get_index()
                        )));
                    }
                    entries.push(entry);
                }
                None => return Err(RaftError::Store(StorageError::Unavailable)),
            }
        }
        raft::util::limit_size(&mut entries, max_size.into());
        Ok(entries)
    }

    fn term(&self, idx: u64) -> raft::Result<u64> {
        let (truncated_index, truncated_term) = self.truncated_state()?;
        if idx == truncated_index {
            return Ok(truncated_term);
        }
        if idx < truncated_index {
            return Err(RaftError::Store(StorageError::Compacted));
        }
        if idx > self.last_index()? {
            return Err(RaftError::Store(StorageError::Unavailable));
        }

        match self.db.get(entry_key(idx)).map_err(store_error)? {
            Some(bytes) => {
                let entry = Entry::parse_from_bytes(&bytes).map_err(store_error)?;
                if entry.get_index() != idx {
                    return Err(invalid_data(format!(
                        "entry under index {idx} contains index {}",
                        entry.get_index()
                    )));
                }
                Ok(entry.get_term())
            }
            None => Err(RaftError::Store(StorageError::Unavailable)),
        }
    }

    fn first_index(&self) -> raft::Result<u64> {
        Ok(self.truncated_state()?.0 + 1)
    }

    fn last_index(&self) -> raft::Result<u64> {
        let (truncated_index, _) = self.truncated_state()?;
        match self.optional_u64(KEY_LAST_INDEX, "last index")? {
            Some(last_index) if last_index >= truncated_index => Ok(last_index),
            Some(last_index) => Err(invalid_data(format!(
                "last index {last_index} precedes truncated index {truncated_index}"
            ))),
            None if truncated_index == 0 && self.is_empty()? => Ok(0),
            None => Err(invalid_data("missing persisted last index")),
        }
    }

    fn snapshot(&self, request_index: u64, _to: u64) -> raft::Result<Snapshot> {
        let state = self.initial_state()?;
        let applied_index = self.applied_index()?;

        if request_index > applied_index || applied_index == 0 {
            return Err(RaftError::Store(
                StorageError::SnapshotTemporarilyUnavailable,
            ));
        }
        if applied_index > self.last_index()? {
            return Err(invalid_data(format!(
                "applied index {applied_index} is ahead of the persisted Raft log"
            )));
        }
        let applied_term = self.term(applied_index)?;

        info!("[storage] Creating snapshot at index {}", applied_index);

        let mut snapshot = Snapshot::default();
        snapshot.mut_metadata().set_conf_state(state.conf_state);
        snapshot.mut_metadata().set_index(applied_index);
        snapshot.mut_metadata().set_term(applied_term);

        let sm_data = self.read_signer_state()?;
        snapshot.set_data(sm_data.to_bytes().into());

        Ok(snapshot)
    }
}

fn store_error<E>(error: E) -> RaftError
where
    E: std::error::Error + Send + Sync + 'static,
{
    RaftError::Store(StorageError::Other(Box::new(error)))
}

fn invalid_data(message: impl Into<String>) -> RaftError {
    store_error(io::Error::new(io::ErrorKind::InvalidData, message.into()))
}

fn decode_consensus_data(bytes: &[u8]) -> raft::Result<ConsensusData> {
    let value: serde_json::Value = serde_json::from_slice(bytes).map_err(store_error)?;
    let object = value
        .as_object()
        .ok_or_else(|| invalid_data("persisted signer state must be a JSON object"))?;
    for field in CONSENSUS_DATA_FIELDS {
        if !object.contains_key(*field) {
            return Err(invalid_data(format!(
                "persisted signer state is missing required field `{field}`"
            )));
        }
    }
    serde_json::from_value(value).map_err(store_error)
}

#[cfg(test)]
mod safety_tests {
    use super::*;
    use tempfile::TempDir;

    fn initialized_storage() -> (TempDir, RocksDBStorage) {
        let temp_dir = TempDir::new().unwrap();
        let mut storage = RocksDBStorage::new(temp_dir.path());

        let mut snapshot = Snapshot::default();
        snapshot.mut_metadata().set_index(1);
        snapshot.mut_metadata().set_term(1);
        snapshot
            .mut_metadata()
            .mut_conf_state()
            .set_voters(vec![1, 2, 3]);
        snapshot.set_data(ConsensusData::default().to_bytes().into());
        storage.apply_snapshot(snapshot).unwrap();

        (temp_dir, storage)
    }

    #[test]
    fn compacted_boundary_keeps_its_log_term_when_election_term_changes() {
        let (_temp_dir, mut storage) = initialized_storage();
        let mut hard_state = storage.initial_state().unwrap().hard_state;
        hard_state.set_term(7);
        hard_state.set_vote(2);
        hard_state.set_commit(1);
        storage.set_hard_state(hard_state).unwrap();

        assert_eq!(
            storage.term(1).unwrap(),
            1,
            "the compacted boundary must retain the term of entry 1, not the current election term"
        );
    }

    #[test]
    fn initialized_store_fails_closed_when_signer_state_is_missing() {
        let (_temp_dir, storage) = initialized_storage();
        storage.db.delete(KEY_SIGNER_STATE).unwrap();

        assert!(
            storage.read_signer_state().is_err(),
            "an initialized Raft store must not silently reset its signing HRS"
        );
    }

    #[test]
    fn initialized_store_fails_closed_when_signer_state_is_malformed() {
        let (_temp_dir, storage) = initialized_storage();
        storage
            .db
            .put(KEY_SIGNER_STATE, b"not valid signer state")
            .unwrap();

        assert!(
            storage.read_signer_state().is_err(),
            "malformed signing state must stop the signer instead of becoming HRS zero"
        );
    }

    #[test]
    fn initialized_store_fails_closed_when_signer_state_is_incomplete_json() {
        let (_temp_dir, storage) = initialized_storage();
        storage.db.put(KEY_SIGNER_STATE, b"{}").unwrap();

        assert!(
            storage.read_signer_state().is_err(),
            "serde defaults must not turn structurally incomplete state into HRS zero"
        );
    }

    #[test]
    fn committing_an_entry_does_not_compact_it_or_replace_its_term() {
        let (_temp_dir, mut storage) = initialized_storage();
        let mut entry = Entry::default();
        entry.set_index(2);
        entry.set_term(3);
        storage.append_entries(&[entry.clone()]).unwrap();

        let mut hard_state = storage.initial_state().unwrap().hard_state;
        hard_state.set_term(7);
        hard_state.set_commit(2);
        storage.set_hard_state(hard_state).unwrap();

        assert_eq!(storage.first_index().unwrap(), 2);
        assert_eq!(storage.term(1).unwrap(), 1);
        assert_eq!(storage.term(2).unwrap(), 3);
        assert_eq!(
            storage
                .entries(2, 3, None, GetEntriesContext::empty(false))
                .unwrap(),
            vec![entry]
        );
    }

    #[test]
    fn snapshot_describes_the_applied_state_and_its_log_term() {
        let (_temp_dir, mut storage) = initialized_storage();
        let applied_state = ConsensusData {
            height: 22,
            round: 3,
            ..Default::default()
        };
        let mut entry = Entry::default();
        entry.set_index(2);
        entry.set_term(4);
        entry.set_data(applied_state.to_bytes().into());
        storage.append_entries(&[entry]).unwrap();
        storage
            .write_signer_state_and_applied(&applied_state, 2)
            .unwrap();

        let mut hard_state = storage.initial_state().unwrap().hard_state;
        hard_state.set_term(9);
        hard_state.set_commit(2);
        storage.set_hard_state(hard_state).unwrap();

        let snapshot = storage.snapshot(2, 2).unwrap();
        assert_eq!(snapshot.get_metadata().get_index(), 2);
        assert_eq!(snapshot.get_metadata().get_term(), 4);
        assert_eq!(
            decode_consensus_data(snapshot.get_data()).unwrap(),
            applied_state
        );
    }

    #[test]
    fn applying_snapshot_preserves_newer_election_term() {
        let (_temp_dir, mut storage) = initialized_storage();
        let mut hard_state = storage.initial_state().unwrap().hard_state;
        hard_state.set_term(9);
        hard_state.set_vote(2);
        storage.set_hard_state(hard_state).unwrap();

        let mut snapshot = Snapshot::default();
        snapshot.mut_metadata().set_index(5);
        snapshot.mut_metadata().set_term(4);
        snapshot
            .mut_metadata()
            .mut_conf_state()
            .set_voters(vec![1, 2, 3]);
        snapshot.set_data(ConsensusData::default().to_bytes().into());
        storage.apply_snapshot(snapshot).unwrap();

        let persisted = storage.initial_state().unwrap().hard_state;
        assert_eq!(persisted.get_term(), 9);
        assert_eq!(persisted.get_vote(), 2);
        assert_eq!(persisted.get_commit(), 5);
        assert_eq!(storage.first_index().unwrap(), 6);
        assert_eq!(storage.term(5).unwrap(), 4);
    }

    #[test]
    fn initialized_store_without_truncation_metadata_fails_closed() {
        let (_temp_dir, storage) = initialized_storage();
        storage.db.delete(KEY_TRUNCATED_INDEX).unwrap();
        storage.db.delete(KEY_TRUNCATED_TERM).unwrap();

        assert!(
            storage.first_index().is_err(),
            "the compacted boundary term cannot be reconstructed from HardState"
        );
    }

    #[test]
    fn snapshot_never_precedes_the_requested_index() {
        let (_temp_dir, mut storage) = initialized_storage();
        let mut entry = Entry::default();
        entry.set_index(2);
        entry.set_term(2);
        storage.append_entries(&[entry]).unwrap();

        let result = storage.snapshot(2, 2);
        assert!(
            result.is_err(),
            "a snapshot at index 1 cannot satisfy a request for index 2"
        );
    }

    #[test]
    fn malformed_snapshot_state_is_rejected() {
        let (_temp_dir, mut storage) = initialized_storage();
        let old_hard_state = storage.initial_state().unwrap().hard_state;
        let old_signer_state = storage.read_signer_state().unwrap();
        let mut snapshot = Snapshot::default();
        snapshot.mut_metadata().set_index(2);
        snapshot.mut_metadata().set_term(2);
        snapshot
            .mut_metadata()
            .mut_conf_state()
            .set_voters(vec![1, 2, 3]);
        snapshot.set_data(b"not valid signer state".to_vec().into());

        let result = storage.apply_snapshot(snapshot);
        assert!(
            result.is_err(),
            "snapshot metadata must not advance when its signer state is invalid"
        );
        assert_eq!(storage.initial_state().unwrap().hard_state, old_hard_state);
        assert_eq!(storage.read_signer_state().unwrap(), old_signer_state);
        assert_eq!(storage.first_index().unwrap(), 2);
        assert_eq!(storage.last_index().unwrap(), 1);
        assert_eq!(storage.applied_index().unwrap(), 1);
    }

    #[test]
    fn snapshot_cannot_roll_back_the_signer_hrs() {
        let (_temp_dir, mut storage) = initialized_storage();
        let signed = ConsensusData {
            height: 50,
            round: 2,
            step: crate::types::SignedMsgType::Prevote,
            sign_data: b"block-a".to_vec(),
            signature: b"signature-a".to_vec(),
            ..Default::default()
        };
        storage.write_signer_state(&signed).unwrap();

        let mut snapshot = Snapshot::default();
        snapshot.mut_metadata().set_index(5);
        snapshot.mut_metadata().set_term(4);
        snapshot
            .mut_metadata()
            .mut_conf_state()
            .set_voters(vec![1, 2, 3]);
        snapshot.set_data(ConsensusData::default().to_bytes().into());

        assert!(
            storage.apply_snapshot(snapshot).is_err(),
            "a higher Raft index must not roll signer H/R/S back to zero"
        );
        assert_eq!(storage.read_signer_state().unwrap(), signed);
    }

    #[test]
    fn snapshot_cannot_replace_core_sign_bytes_at_the_same_hrs() {
        let (_temp_dir, mut storage) = initialized_storage();
        let signed = ConsensusData {
            height: 50,
            round: 2,
            step: crate::types::SignedMsgType::Precommit,
            sign_data: b"block-a".to_vec(),
            signature: b"signature-a".to_vec(),
            ..Default::default()
        };
        storage.write_signer_state(&signed).unwrap();
        let conflicting = ConsensusData {
            sign_data: b"block-b".to_vec(),
            signature: b"signature-b".to_vec(),
            ..signed.clone()
        };

        let mut snapshot = Snapshot::default();
        snapshot.mut_metadata().set_index(5);
        snapshot.mut_metadata().set_term(4);
        snapshot
            .mut_metadata()
            .mut_conf_state()
            .set_voters(vec![1, 2, 3]);
        snapshot.set_data(conflicting.to_bytes().into());

        assert!(
            storage.apply_snapshot(snapshot).is_err(),
            "snapshot installation must enforce the same-H/R/S core-signature invariant"
        );
        assert_eq!(storage.read_signer_state().unwrap(), signed);
    }

    #[test]
    fn snapshot_normalizes_a_changed_signature_for_the_same_core_sign_bytes() {
        let (_temp_dir, mut storage) = initialized_storage();
        let signed = ConsensusData {
            height: 50,
            round: 2,
            step: crate::types::SignedMsgType::Precommit,
            sign_data: b"block-a".to_vec(),
            signature: b"original-signature".to_vec(),
            ext_sign_data: b"extension-a".to_vec(),
            ext_signature: b"extension-signature-a".to_vec(),
        };
        storage.write_signer_state(&signed).unwrap();
        let replacement = ConsensusData {
            signature: b"replacement-signature".to_vec(),
            ext_sign_data: b"extension-b".to_vec(),
            ext_signature: b"extension-signature-b".to_vec(),
            ..signed.clone()
        };

        let mut snapshot = Snapshot::default();
        snapshot.mut_metadata().set_index(5);
        snapshot.mut_metadata().set_term(4);
        snapshot
            .mut_metadata()
            .mut_conf_state()
            .set_voters(vec![1, 2, 3]);
        snapshot.set_data(replacement.to_bytes().into());
        storage.apply_snapshot(snapshot).unwrap();

        let persisted = storage.read_signer_state().unwrap();
        assert_eq!(persisted.signature, signed.signature);
        assert_eq!(persisted.ext_sign_data, b"extension-b");
        assert_eq!(persisted.ext_signature, b"extension-signature-b");
    }
}
