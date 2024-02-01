use super::block::Block;

pub struct Blockchain {
    pub blocks: Vec<Block>,
}

impl Blockchain {
    /// Creates a new blockchain with a genesis block.
    pub fn new() -> Self {
        let mut blockchain = Blockchain { blocks: Vec::new() };
        blockchain.create_genesis_block();
        blockchain
    }

    /// Creates the genesis block, the first block in the blockchain.
    fn create_genesis_block(&mut self) {
        let genesis_block = Block::new(0, "Genesis Block".to_string(), String::new());
        self.blocks.push(genesis_block);
    }

    /// Adds a new block to the blockchain.
    pub fn add_block(&mut self, data: String) {
        if let Some(last_block) = self.blocks.last() {
            let new_block = Block::new(last_block.index + 1, data, last_block.hash.clone());
            self.blocks.push(new_block);
        }
    }

    /// Validates the blockchain's integrity.
    pub fn is_valid(&self) -> bool {
        for (i, current_block) in self.blocks.iter().enumerate() {
            if i == 0 { continue; } // Skip genesis block

            let previous_block = &self.blocks[i - 1];
            if current_block.hash != Block::calculate_hash(current_block) {
                return false;
            }

            if current_block.previous_hash != previous_block.hash {
                return false;
            }
        }
        true
    }
}
