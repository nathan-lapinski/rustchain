use super::block::Block;
use chrono::prelude::*;

type Blocks = Vec<Block>;

#[derive(Debug, Clone)]
pub struct Blockchain {
    pub genesis_block: Block,
    pub chain: Blocks,
    pub difficulty: usize,
}

impl Blockchain {
    pub fn new(difficulty: usize) -> Self {
        let genesis_block = Block {
            index: 0,
            timestamp: Utc::now().timestamp_millis() as u64,
            proof_of_work: u64::default(),
            previous_hash: String::default(),
            hash: String::default(),
            data: String::from("Genesis block"),
        };

        let mut chain = Vec::new();
        chain.push(genesis_block.clone());

        let blockchain = Blockchain {
            genesis_block,
            chain,
            difficulty
        };

        blockchain
    }

    pub fn add_block(&mut self, data: String) {
        let mut new_block = Block::new(
            self.chain.len() as u64,
            self.chain[&self.chain.len() -1].hash.clone(),
            data
        );

        new_block.mine(self.clone());
        self.chain.push(new_block.clone());
        println!("New block added to chain -> {:?}", new_block);
    }

    pub fn print(&mut self) {
        for b in self.chain.iter() {
            println!("------------------------------------");    
            println!("Previous Hash: {}", b.previous_hash);    
            println!("Data: {}", b.data);
            println!("Hash: {}", b.hash);    
            println!("------------------------------------");    
        }
    }
}
