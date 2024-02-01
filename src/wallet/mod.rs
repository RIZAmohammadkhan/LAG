mod keypair;
mod transaction;

pub use keypair::WalletKeypair;
pub use transaction::{Transaction, TransactionData};

pub struct Wallet {
    pub keypair: WalletKeypair,
    // other wallet properties
}

impl Wallet {
    pub fn new() -> Self {
        let keypair = WalletKeypair::new();
        Wallet { keypair }
        // Initialize other properties
    }

    // Add more wallet-related functionalities here, such as:
    // - Generating transactions
    // - Signing transactions
    // - Verifying transactions
    // - Managing balances
    // - etc.
}
