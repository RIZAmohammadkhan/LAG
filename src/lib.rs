// Declare the main modules of the cryptocurrency application.
pub mod blockchain;
pub mod network;
pub mod wallet;
pub mod utils;

// Re-export key components for easier access from outside the library.
// This allows other parts of your application or external users to directly use these components.
pub use blockchain::{Blockchain, Block, ProofOfWork}; // Assuming ProofOfWork is part of your blockchain logic.
pub use network::{Node, Message, MessageType}; // Assuming these are key parts of your network logic.
pub use wallet::{WalletKeypair, Transaction}; // Directly use WalletKeypair and Transaction from the wallet module.
pub use utils::{hash, hash_serialize}; // Assuming these are utility functions you've implemented for hashing.
