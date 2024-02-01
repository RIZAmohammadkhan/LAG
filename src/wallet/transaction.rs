use ed25519_dalek::{PublicKey, SecretKey, Signature, Signer, Verifier};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Transaction {
    sender: PublicKey,
    receiver: PublicKey,
    amount: u64,
    signature: Option<Signature>,
}

impl Transaction {
    pub fn new(sender: PublicKey, receiver: PublicKey, amount: u64) -> Self {
        Transaction {
            sender,
            receiver,
            amount,
            signature: None,
        }
    }

    pub fn sign(&mut self, secret_key: &SecretKey) {
        let message = self.to_bytes();
        self.signature = Some(secret_key.sign(&message));
    }

    pub fn verify(&self) -> bool {
        self.signature
            .map(|sig| self.sender.verify(&self.to_bytes(), &sig).is_ok())
            .unwrap_or(false)
    }

    fn to_bytes(&self) -> Vec<u8> {
        // Convert the transaction to bytes; for simplicity, we're using serde to serialize
        // Adjust this method based on how you choose to serialize your transaction data.
        serde_json::to_vec(self).expect("Failed to serialize transaction")
    }
}
