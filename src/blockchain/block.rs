use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Block {
    pub index: u64,
    pub timestamp: u64,
    pub data: String,
    pub previous_hash: String,
    pub hash: String,
    pub nonce: u64, // Nonce for Proof of Work
}

impl Block {
    /// Creates a new block.
    pub fn new(index: u64, data: String, previous_hash: String) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();

        let mut block = Block {
            index,
            timestamp,
            data,
            previous_hash,
            hash: String::new(),
            nonce: 0,
        };

        block.hash = Block::calculate_hash(&block);
        block
    }

    /// Calculates the block's hash.
    pub fn calculate_hash(&self) -> String {
        let contents = format!(
            "{}:{}:{}:{}:{}",
            self.index, self.timestamp, self.previous_hash, self.data, self.nonce
        );
        let mut hasher = Sha256::new();
        hasher.update(contents);
        format!("{:x}", hasher.finalize())
    }
}
