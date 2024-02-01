use super::block::Block;

pub struct ProofOfWork {
    pub difficulty: usize,
}

impl ProofOfWork {
    /// Creates a new Proof of Work structure with a given difficulty.
    pub fn new(difficulty: usize) -> Self {
        ProofOfWork { difficulty }
    }

    /// Performs the proof of work algorithm on the given block.
    pub fn mine(&self, block: &mut Block) {
        println!("Mining block with difficulty: {}", self.difficulty);
        let target = String::from_utf8(vec![b'0'; self.difficulty]).unwrap();

        while !block.hash.starts_with(&target) {
            block.nonce += 1;
            block.hash = Block::calculate_hash(block);
        }

        println!("Block mined! Hash: {}", block.hash);
    }

    /// Validates that a block's hash meets the difficulty criteria.
    pub fn validate(&self, block: &Block) -> bool {
        block
            .hash
            .starts_with(&String::from_utf8(vec![b'0'; self.difficulty]).unwrap())
    }
}
