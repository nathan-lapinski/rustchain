use chrono::prelude::*;
use sha2::{Sha256, Digest};

#[derive(Debug)]
pub struct Block {
    timestamp: String,
    data: String,
    prev_block_hash: String,
    hash: String
}

impl Block {
    pub fn new(data: String, previous_hash: String) -> Self {
        let mut block = Block {
            timestamp: Utc::now().timestamp_millis().to_string(),
            data: data,
            prev_block_hash: previous_hash,
            hash: String::default()
        };
        block.set_hash();
        block
    }
    pub fn set_hash(&mut self) {
        let mut hasher = Sha256::new();
        let vec: Vec<u8> = [self.timestamp.as_bytes(), self.data.as_bytes(), self.prev_block_hash.as_bytes()].concat();
        hasher.update(vec);
        let result = hasher.finalize();
        self.hash = format!("{:x}",result);
    }
    pub fn get_hash(&self) -> &String {
        &self.hash
    }
}