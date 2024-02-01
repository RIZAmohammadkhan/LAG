pub mod blockchain;
pub mod network;
pub mod wallet;
pub mod utils;

// Re-exporting important structs or functions for easier access
pub use blockchain::chain::Blockchain;
pub use network::node::Node;
pub use wallet::Wallet;
