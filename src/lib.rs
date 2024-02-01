// Declare the main modules of the cryptocurrency application.
pub mod blockchain;
pub mod network;
pub mod utils;
pub mod wallet;

// Re-export key components for easier access from outside the library.
// This allows other parts of your application or external users to directly use these components.
pub use blockchain::{Block, Blockchain, ProofOfWork}; // Assuming ProofOfWork is part of your blockchain logic.
pub use network::{Message, MessageType, Node}; // Assuming these are key parts of your network logic.
pub use utils::{hash, hash_serialize};
pub use wallet::{Transaction, WalletKeypair}; // Directly use WalletKeypair and Transaction from the wallet module. // Assuming these are utility functions you've implemented for hashing.
