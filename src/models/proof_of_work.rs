use super::block::Block;
use num_bigint::BigUint;
use num_traits::{Zero, One};

const TARGET_BITS: u64 = 20;

pub struct ProofOfWork {
    block: Block,
    target: BigUint
}

impl ProofOfWork {
    pub fn new(block: Block ) -> Self {
        let mut target: BigUint = One::one();
        println!("{:x}", target);
        target = target << (256 - 20);
        println!("{:x}", target);
        ProofOfWork {
            block,
            target
        }
    }

    pub fn prepare_data(self, nonce: u64) -> Vec<u8> {
        let vec: Vec<u8> = [
            self.block.get_prev_hash().as_bytes(),
            self.block.get_data().as_bytes(),
            self.block.get_timestamp().as_bytes(),
            &(TARGET_BITS.to_be_bytes()),
            &(nonce.to_be_bytes())
        ].concat();
        println!("{:?}", vec);
        vec
    }
}