use ed25519_dalek::{PublicKey, SecretKey, Signature, Signer, Verifier};
use serde::{Serialize, Deserialize};
use super::keypair::WalletKeypair;
use sha2::{Sha256, Digest};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Transaction {
    from: PublicKey,
    to: PublicKey,
    amount: u64,
    signature: Option<Signature>,
}

impl Transaction {
    /// Creates a new transaction.
    pub fn new(from: PublicKey, to: PublicKey, amount: u64) -> Self {
        Transaction {
            from,
            to,
            amount,
            signature: None,
        }
    }

    /// Signs the transaction with the sender's private key.
    pub fn sign(&mut self, keypair: &WalletKeypair) {
        let message = self.to_hashable();
        let signature = keypair.sign(message.as_bytes());
        self.signature = Some(signature);
    }

    /// Verifies the transaction's signature.
    pub fn verify(&self) -> bool {
        if let Some(signature) = &self.signature {
            self.from.verify(self.to_hashable().as_bytes(), signature).is_ok()
        } else {
            false
        }
    }

    /// Converts the transaction into a string that will be hashed and signed.
    fn to_hashable(&self) -> String {
        format!("{}{}{}", self.from.as_bytes().to_vec().iter().map(|byte| format!("{:02x}", byte)).collect::<String>(), self.to.as_bytes().to_vec().iter().map(|byte| format!("{:02x}", byte)).collect::<String>(), self.amount)
    }
}
