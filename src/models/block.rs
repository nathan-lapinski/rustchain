use super::proof_of_work::ProofOfWork;
use chrono::prelude::*;
use sha2::{Sha256, Digest};

#[derive(Debug, Clone)]
pub struct Block {
    timestamp: String,
    data: String,
    prev_block_hash: String,
    hash: String,
    nonce: u64
}

impl Block {
    pub fn new(data: String, previous_hash: String) -> Self {
        let mut block = Block {
            timestamp: Utc::now().timestamp_millis().to_string(),
            data: data,
            prev_block_hash: previous_hash,
            hash: String::default(),
            nonce: 0
        };
        block
    }
    pub fn new_block(data: String, prev_hash: String) -> Block {
        let mut block = Block {
            timestamp: Utc::now().timestamp_millis().to_string(),
            data: data,
            prev_block_hash: prev_hash,
            hash: String::default(),
            nonce: 0
        };
        let mut pow = ProofOfWork::new(block.clone());
        let (hash, nonce) = pow.mine();
        block.set_hash(hash);
        block.set_nonce(nonce);
        block
    }
    pub fn get_hash(&self) -> &String {
        &self.hash
    }
    pub fn set_hash(&mut self, hash: String) {
        self.hash = hash;
    }
    pub fn get_prev_hash(&self) -> &String {
        &self.prev_block_hash
    }
    pub fn get_timestamp(&self) -> &String {
        &self.timestamp
    }
    pub fn get_data(&self) -> &String {
        &self.data
    }
    pub fn get_nonce(&self) -> u64 {
        self.nonce
    }
    pub fn set_nonce(&mut self, nonce: u64) {
        self.nonce = nonce;
    }
}