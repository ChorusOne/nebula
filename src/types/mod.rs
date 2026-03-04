use crate::{SignerError, protocol::ValidRequest};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, VecDeque};
use thiserror::Error;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockId {
    pub hash: Vec<u8>,
    pub parts: Option<PartSetHeader>,
}

impl From<BlockId> for crate::proto::v1::types::BlockId {
    fn from(block_id: BlockId) -> crate::proto::v1::types::BlockId {
        crate::proto::v1::types::BlockId {
            hash: block_id.hash.into(),
            part_set_header: Some(block_id.parts.unwrap().into()),
        }
    }
}

impl From<PartSetHeader> for crate::proto::v1::types::PartSetHeader {
    fn from(part_set_header: PartSetHeader) -> crate::proto::v1::types::PartSetHeader {
        crate::proto::v1::types::PartSetHeader {
            total: part_set_header.total,
            hash: part_set_header.hash.into(),
        }
    }
}
impl From<crate::proto::v1::types::BlockId> for BlockId {
    fn from(block_id: crate::proto::v1::types::BlockId) -> BlockId {
        BlockId {
            hash: block_id.hash.into(),
            parts: Some(block_id.part_set_header.unwrap().into()),
        }
    }
}

impl From<crate::proto::v1::types::PartSetHeader> for PartSetHeader {
    fn from(part_set_header: crate::proto::v1::types::PartSetHeader) -> PartSetHeader {
        PartSetHeader {
            total: part_set_header.total,
            hash: part_set_header.hash.into(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PartSetHeader {
    pub total: u32,
    pub hash: Vec<u8>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum SignedMsgType {
    #[default]
    Unknown = 0,
    Prevote = 1,
    Precommit = 2,
    Proposal = 32,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Vote {
    pub step: SignedMsgType,
    pub height: i64,
    pub round: i64,
    pub timestamp: Option<i64>,
    pub block_id: Option<BlockId>,
    pub validator_address: Vec<u8>,
    pub validator_index: i32,
    pub extension: Vec<u8>,
    pub extension_signature: Vec<u8>,
}
impl std::fmt::Display for Vote {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Vote {{ step: {:?}, height: {}, round: {}, timestamp: {:?}, block_id: {:?}, validator_address: {:02X?}, validator_index: {}, extension: {:02X?}, extension_signature: {:02X?} }}",
            self.step as u8,
            self.height,
            self.round,
            self.timestamp,
            self.block_id,
            self.validator_address,
            self.validator_index,
            self.extension,
            self.extension_signature
        )
    }
}

impl From<SignedMsgType> for u8 {
    fn from(r#type: SignedMsgType) -> u8 {
        match r#type {
            SignedMsgType::Unknown => 0,
            SignedMsgType::Prevote => 1,
            SignedMsgType::Precommit => 2,
            SignedMsgType::Proposal => 32,
        }
    }
}

impl From<SignedMsgType> for i32 {
    fn from(r#type: SignedMsgType) -> i32 {
        match r#type {
            SignedMsgType::Unknown => 0,
            SignedMsgType::Prevote => 1,
            SignedMsgType::Precommit => 2,
            SignedMsgType::Proposal => 32,
        }
    }
}

// this is getting messy, probably something wrong with the types somewhere?
impl From<u8> for SignedMsgType {
    fn from(n: u8) -> Self {
        match n {
            1 => SignedMsgType::Prevote,
            2 => SignedMsgType::Precommit,
            32 => SignedMsgType::Proposal,
            _ => SignedMsgType::Unknown,
        }
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Proposal {
    pub step: SignedMsgType,
    pub height: i64,
    pub round: i64,
    pub timestamp: Option<i64>,
    pub pol_round: i64,
    pub block_id: Option<BlockId>,
}

#[derive(Debug, Error)]
pub enum BufferError {
    #[error("Insufficient amount of bytes in the buffer")]
    NeedMoreBytes,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Clone, clap::ValueEnum)]
#[serde(rename_all = "snake_case")]
pub enum KeyType {
    Ed25519,
    Secp256k1,
    Bls12381,
}

impl TryFrom<&str> for KeyType {
    type Error = SignerError;
    fn try_from(key_type_str: &str) -> Result<KeyType, SignerError> {
        match key_type_str {
            "ed25519" => Ok(KeyType::Ed25519),
            "secp256k1" => Ok(KeyType::Secp256k1),
            "bls12_381" => Ok(KeyType::Bls12381),
            "bls12381" => Ok(KeyType::Bls12381), // TODO
            _ => Err(SignerError::InvalidData),
        }
    }
}

impl From<KeyType> for String {
    fn from(key_type: KeyType) -> String {
        match key_type {
            KeyType::Ed25519 => "ed25519".to_string(),
            KeyType::Secp256k1 => "secp256k1".to_string(),
            KeyType::Bls12381 => "bls12_381".to_string(),
        }
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct ConsensusData {
    pub height: i64,
    pub round: i64,
    pub step: SignedMsgType,
    #[serde(default)]
    pub request_key: Vec<u8>,
    #[serde(default)]
    pub sign_bytes_hash: Vec<u8>,
    #[serde(default)]
    pub signature: Vec<u8>,
    #[serde(default)]
    pub extension_signature: Vec<u8>,
}

impl From<&ValidRequest> for ConsensusData {
    fn from(value: &ValidRequest) -> Self {
        match value {
            ValidRequest::Vote(v) => Self {
                height: v.height,
                round: v.round,
                step: v.step,
                ..Default::default()
            },
            ValidRequest::Proposal(p) => Self {
                height: p.height,
                round: p.round,
                step: p.step,
                ..Default::default()
            },
        }
    }
}

impl std::fmt::Display for ConsensusData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "ConsensusData {}/{}/{:?}",
            self.height, self.round, self.step
        )
    }
}

impl ConsensusData {
    pub fn _persist_to_file(&self, path: &std::path::Path) -> std::io::Result<()> {
        let json = serde_json::to_string_pretty(self)?;
        let temp_path = path.with_extension("json.tmp");
        std::fs::write(&temp_path, json)?;
        std::fs::rename(&temp_path, path)
    }

    pub fn load_from_file(path: &std::path::Path) -> Option<ConsensusData> {
        let json = std::fs::read_to_string(path).ok()?;
        serde_json::from_str(&json).ok()
    }
    pub fn to_bytes(&self) -> Vec<u8> {
        serde_json::to_vec(&self).unwrap()
    }

    pub fn from_bytes(buf: &[u8]) -> Option<ConsensusData> {
        serde_json::from_slice(buf).ok()
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct SignatureSlot {
    pub height: i64,
    pub round: i64,
    pub step: SignedMsgType,
}

impl From<&ConsensusData> for SignatureSlot {
    fn from(value: &ConsensusData) -> Self {
        Self {
            height: value.height,
            round: value.round,
            step: value.step,
        }
    }
}

#[derive(Clone, Debug)]
pub enum SignatureCacheLookup {
    Hit(ConsensusData),
    Conflict(ConsensusData),
    Miss,
}

#[derive(Clone, Debug)]
pub struct SignatureCache {
    capacity: usize,
    by_key: HashMap<Vec<u8>, ConsensusData>,
    by_slot: HashMap<SignatureSlot, Vec<u8>>,
    order: VecDeque<Vec<u8>>,
}

impl Default for SignatureCache {
    fn default() -> Self {
        Self::new(100)
    }
}

impl SignatureCache {
    pub fn new(capacity: usize) -> Self {
        Self {
            capacity: capacity.max(1),
            by_key: HashMap::new(),
            by_slot: HashMap::new(),
            order: VecDeque::new(),
        }
    }

    fn key_for(record: &ConsensusData) -> Option<Vec<u8>> {
        if !record.request_key.is_empty() {
            Some(record.request_key.clone())
        } else if !record.sign_bytes_hash.is_empty() {
            Some(record.sign_bytes_hash.clone())
        } else {
            None
        }
    }

    pub fn insert(&mut self, record: ConsensusData) {
        if record.signature.is_empty() {
            return;
        }

        let Some(key) = Self::key_for(&record) else {
            return;
        };
        let slot = SignatureSlot::from(&record);
        self.by_slot.insert(slot, key.clone());
        self.by_key.insert(key.clone(), record);
        self.order.push_back(key);

        while self.order.len() > self.capacity {
            if let Some(old_key) = self.order.pop_front() {
                if let Some(existing) = self.by_key.remove(&old_key) {
                    let existing_slot = SignatureSlot::from(&existing);
                    if self
                        .by_slot
                        .get(&existing_slot)
                        .is_some_and(|h| h == &old_key)
                    {
                        self.by_slot.remove(&existing_slot);
                    }
                }
            }
        }
    }

    pub fn lookup(&self, record: &ConsensusData) -> SignatureCacheLookup {
        let Some(key) = Self::key_for(record) else {
            return SignatureCacheLookup::Miss;
        };
        let slot = SignatureSlot::from(record);
        match self.by_slot.get(&slot) {
            Some(existing_key) if existing_key == &key => self
                .by_key
                .get(existing_key)
                .cloned()
                .map(SignatureCacheLookup::Hit)
                .unwrap_or(SignatureCacheLookup::Miss),
            Some(existing_key) => self
                .by_key
                .get(existing_key)
                .cloned()
                .map(SignatureCacheLookup::Conflict)
                .unwrap_or(SignatureCacheLookup::Miss),
            None => SignatureCacheLookup::Miss,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn record(height: i64, round: i64, step: SignedMsgType, hash: u8) -> ConsensusData {
        ConsensusData {
            height,
            round,
            step,
            request_key: vec![hash],
            sign_bytes_hash: vec![hash],
            signature: vec![hash, hash],
            extension_signature: Vec::new(),
        }
    }

    #[test]
    fn signature_cache_hit_and_conflict() {
        let mut cache = SignatureCache::new(100);
        let first = record(10, 0, SignedMsgType::Proposal, 1);
        cache.insert(first.clone());

        match cache.lookup(&first) {
            SignatureCacheLookup::Hit(found) => assert_eq!(found.signature, first.signature),
            _ => panic!("expected cache hit"),
        }

        let conflict = record(10, 0, SignedMsgType::Proposal, 2);
        match cache.lookup(&conflict) {
            SignatureCacheLookup::Conflict(found) => {
                assert_eq!(found.sign_bytes_hash, first.sign_bytes_hash)
            }
            _ => panic!("expected conflict"),
        }

        let miss = record(11, 0, SignedMsgType::Proposal, 3);
        match cache.lookup(&miss) {
            SignatureCacheLookup::Miss => {}
            _ => panic!("expected miss"),
        }
    }

    #[test]
    fn signature_cache_honors_capacity() {
        let mut cache = SignatureCache::new(2);
        let first = record(1, 0, SignedMsgType::Proposal, 1);
        let second = record(2, 0, SignedMsgType::Proposal, 2);
        let third = record(3, 0, SignedMsgType::Proposal, 3);

        cache.insert(first.clone());
        cache.insert(second.clone());
        cache.insert(third.clone());

        match cache.lookup(&first) {
            SignatureCacheLookup::Miss => {}
            _ => panic!("oldest entry should be evicted"),
        }
        match cache.lookup(&second) {
            SignatureCacheLookup::Hit(_) => {}
            _ => panic!("second entry should still exist"),
        }
        match cache.lookup(&third) {
            SignatureCacheLookup::Hit(_) => {}
            _ => panic!("third entry should still exist"),
        }
    }

    #[test]
    fn signature_cache_replays_when_sign_bytes_change_but_request_key_is_same() {
        let mut cache = SignatureCache::new(100);
        let first = ConsensusData {
            height: 10,
            round: 0,
            step: SignedMsgType::Prevote,
            request_key: vec![42],
            sign_bytes_hash: vec![1],
            signature: vec![7, 7],
            extension_signature: Vec::new(),
        };
        cache.insert(first.clone());

        let timestamp_variant = ConsensusData {
            sign_bytes_hash: vec![2],
            ..first
        };

        match cache.lookup(&timestamp_variant) {
            SignatureCacheLookup::Hit(found) => assert_eq!(found.signature, vec![7, 7]),
            _ => panic!("expected cache hit based on request key"),
        }
    }
}
