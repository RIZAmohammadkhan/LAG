use sha2::{Sha256, Digest};
use serde::{Serialize};
use hex;

/// Calculates the SHA-256 hash of the input data.
/// Returns the hash as a hexadecimal string.
pub fn hash(data: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data);
    hex::encode(hasher.finalize())
}

/// Hashes any serializable data using SHA-256 and returns the hash as a hexadecimal string.
/// This function is useful for hashing complex data structures.
pub fn hash_serialize<T: Serialize>(data: &T) -> Result<String, serde_json::Error> {
    let serialized = serde_json::to_string(data)?;
    Ok(hash(serialized.as_bytes()))
}
