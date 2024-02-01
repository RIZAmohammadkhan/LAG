use crate::utils::crypto::hash_serialize;
use crate::wallet::keypair::WalletKeypair;
use hex;
use serde::{Deserialize, Serialize};
use sodiumoxide::crypto::sign;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Transaction {
    pub sender: String,
    pub recipient: String,
    pub amount: u64,
    pub signature: String,
}

impl Transaction {
    pub fn new(sender: String, recipient: String, amount: u64) -> Self {
        Transaction {
            sender,
            recipient,
            amount,
            signature: String::new(),
        }
    }

    pub fn sign(&mut self, keypair: &WalletKeypair) {
        let message = hash_serialize(&self).expect("Failed to serialize transaction");
        let signature = keypair.sign(message.as_bytes());
        self.signature = hex::encode(signature.as_ref());
    }

    pub fn verify_signature(&self) -> bool {
        let message =
            hash_serialize(&self).expect("Failed to serialize transaction for verification");
        let public_key_bytes = hex::decode(&self.sender).expect("Failed to decode public key");
        let signature_bytes = hex::decode(&self.signature).expect("Failed to decode signature");

        let pk = sign::PublicKey::from_slice(&public_key_bytes)
            .expect("Failed to create public key from slice");
        let sig = sign::Signature::from_bytes(&signature_bytes)
            .expect("Failed to create signature from bytes");

        sign::verify_detached(&sig, message.as_bytes(), &pk)
    }
}
