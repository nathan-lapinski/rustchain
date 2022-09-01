use super::block::Block;
use num_bigint::{BigUint};
use num_traits::{One};
use sha2::{Sha256, Digest};

const TARGET_BITS: u64 = 20;
const DIFFICULTY_PREFIX: &str = "0000";

pub struct ProofOfWork {
    block: Block,
    target: BigUint
}

impl ProofOfWork {
    pub fn new(block: Block) -> Self {
        let mut target: BigUint = One::one();
        target = target << (256 - 20);
        ProofOfWork {
            block,
            target
        }
    }

    pub fn prepare_data(&mut self, nonce: u64) -> Vec<u8> {
        let vec: Vec<u8> = [
            self.block.get_prev_hash().as_bytes(),
            self.block.get_data().as_bytes(),
            self.block.get_timestamp().as_bytes(),
            &(TARGET_BITS.to_be_bytes()),
            &(nonce.to_be_bytes())
        ].concat();
        vec
    }

    // TODO: Can this be multithreaded?
    pub fn mine(&mut self) -> (String, u64) {
        let mut hash: String = String::default();
        let mut nonce = 0;
        while nonce < u64::MAX {
            let mut hasher = Sha256::new();
            // TODO: It seems ineffecient to convert all header data each time 
            // when only the nonce changes. Consider refactoring
            let data = self.prepare_data(nonce);
            hasher.update(data);
            let result = hasher.finalize();
            hash = format!("{:x}", result);
            if hash.starts_with(DIFFICULTY_PREFIX) {
                println!("Success: {:?}{:?}", hash, nonce);
                break;
            }
            nonce += 1;
        }
        (hash, nonce)
    }
}