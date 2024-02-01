use sha2::{Sha256, Digest};
use serde::{Serialize, Deserialize};

/// Calculates the SHA-256 hash of the input data.
pub fn hash(data: &[u8]) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.update(data);
    hasher.finalize().to_vec()
}

/// A utility function to hash any serializable data using SHA-256.
/// Returns the hash as a hexadecimal string.
pub fn hash_serialize<T: Serialize>(data: &T) -> String {
    let serialized = serde_json::to_string(data).expect("Failed to serialize data");
    hex::encode(hash(serialized.as_bytes()))
}
