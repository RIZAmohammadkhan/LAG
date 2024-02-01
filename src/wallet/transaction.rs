use super::keypair::WalletKeypair;
use ed25519_dalek::{Signature, Signer};
use serde::{Serialize, Deserialize};
use sha2::{Sha256, Digest};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Transaction {
    from: String, // Public key of the sender
    to: String,   // Public key of the receiver
    amount: u64,
    signature: Option<Signature>, // Digital signature to verify the transaction
}

impl Transaction {
    /// Creates a new transaction.
    pub fn new(from: String, to: String, amount: u64) -> Self {
        Transaction {
            from,
            to,
            amount,
            signature: None,
        }
    }

    /// Signs the transaction with a given keypair.
    pub fn sign(&mut self, keypair: &WalletKeypair) {
        let message = self.to_string();
        let signature = keypair.sign(message.as_bytes());
        self.signature = Some(signature);
    }

    /// Converts the transaction into a string format for signing.
    pub fn to_string(&self) -> String {
        format!("{}:{}:{}", self.from, self.to, self.amount)
    }

    /// Verifies the transaction's signature.
    pub fn verify(&self) -> bool {
        if let Some(signature) = &self.signature {
            let public_key = PublicKey::from_str(&self.from).unwrap();
            public_key.verify(self.to_string().as_bytes(), signature).is_ok()
        } else {
            false
        }
    }
}

