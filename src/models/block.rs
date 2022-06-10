use super::blockchain::Blockchain;
use chrono::prelude::*;
use sha2::{Sha256, Digest};
use serde::{Deserialize, Serialize};

// a generic block data structute
// The #[] is an attribute, and it derives some traits.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    // the index of the block
    pub index: u64,
    // block ts
    pub timestamp: u64,
    pub proof_of_work: u64,
    pub previous_hash: String,
    // The current block's hash
    pub hash: String,
    // Data for the current block: TODO: Make this actual transaction data
    pub data: String,
}

impl Block {
    // creates a new block and sets the block hash
    pub fn new (
        index: u64,
        previous_hash: String,
        data: String
    ) -> Self {
        let block = Block {
            index,
            timestamp: Utc::now().timestamp_millis() as u64,
            proof_of_work: u64::default(),
            previous_hash,
            hash: String::default(),
            data,
        };

        block
    }

    // mine the block's hash
    pub fn mine (&mut self, blockchain: Blockchain) {
        loop {
            if !self.hash.starts_with(&"0".repeat(blockchain.difficulty)) {
                self.proof_of_work += 1;
                self.hash = self.generate_block_hash();
            } else {
                break
            }
        }
    }

    pub fn generate_block_hash(&self) -> String {
        let mut block_data = self.clone();
        block_data.hash = String::default();
        // block -> JSON
        let serialized_block_data = serde_json::to_string(&block_data).unwrap();

        let mut hasher = Sha256::new();
        hasher.update(serialized_block_data);

        let result = hasher.finalize();
        format!("{:x}", result)
    }
}
