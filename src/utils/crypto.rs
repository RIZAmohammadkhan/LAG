use sha2::{Sha256, Digest};

/// Calculates the SHA-256 hash of the input data.
pub fn hash(data: &[u8]) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.update(data);
    hasher.finalize().to_vec()
}

/// Verifies if the given data matches a given SHA-256 hash.
pub fn verify_hash(data: &[u8], hash: &[u8]) -> bool {
    hash == hash(data)
}
