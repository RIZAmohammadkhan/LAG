// Assuming you've made necessary imports from sodiumoxide
use super::WalletKeypair;
use serde::{Serialize, Deserialize};
use sodiumoxide::crypto::{sign, sign::PublicKey, sign::Signature}; // Add necessary imports

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Transaction {
    // Your fields remain unchanged
    signature: Vec<u8>, // Add a field for signature
}

impl Transaction {
    // Your constructor remains unchanged

    pub fn sign(&mut self, keypair: &WalletKeypair) {
        let message = self.hash(); // Ensure you have a hash function
        self.signature = keypair.sign(&message).0.to_vec();
    }

    pub fn verify(&self, public_key: &PublicKey) -> bool {
        sign::verify_detached(
            &Signature::from_slice(&self.signature).unwrap(),
            &self.hash(),
            public_key,
        )
    }

    // Your hash function remains unchanged
}
