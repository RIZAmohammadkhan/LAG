// Declare the submodules within the wallet module.
pub mod keypair;
pub mod transaction;

// Re-export key structs and functions for external use.
pub use keypair::WalletKeypair;
pub use transaction::Transaction;
// If there are specific functions or structs within `transaction` that need to be exposed,
// they should be listed explicitly here as well.
