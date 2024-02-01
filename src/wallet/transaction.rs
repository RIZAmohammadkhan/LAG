use ring::signature::{Ed25519KeyPair, Signature, verify, ED25519};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Transaction {
    pub from: Vec<u8>, // PublicKey as bytes
    pub to: Vec<u8>, // PublicKey as bytes for recipient
    pub amount: u64,
    pub signature: Vec<u8>,
}

impl Transaction {
    pub fn new(from: Vec<u8>, to: Vec<u8>, amount: u64) -> Self {
        Transaction {
            from,
            to,
            amount,
            signature: vec![],
        }
    }

    pub fn sign(&mut self, keypair: &Ed25519KeyPair, message: &[u8]) {
        let sig = keypair.sign(message);
        self.signature = sig.as_ref().to_vec();
    }

    pub fn verify(&self, message: &[u8]) -> bool {
        verify(&ED25519, self.from.as_slice().into(), message, self.signature.as_slice()).is_ok()
    }
}
