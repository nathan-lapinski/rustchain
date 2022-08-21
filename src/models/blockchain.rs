use super::block::Block;
pub struct Blockchain {
    blocks: Vec<Block>
}

impl Blockchain {
    pub fn new() -> Self {
        Blockchain {
            blocks: Vec::default()
        }
    }

    pub fn add_block(&mut self, data: String) {
        let block;
        let len = self.blocks.len();
        
        if len == 0 {
            block = Block::new(data, String::from(""));
        } else {
            block = Block::new(data, self.blocks[len-1].get_hash());
        }
        self.blocks.push(block);
    }

    pub fn print(&self) {
        for b in &self.blocks {
            println!("{:?}", b);
        }
    }
}