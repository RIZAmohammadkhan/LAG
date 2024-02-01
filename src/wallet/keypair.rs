use ring::signature::{self, Ed25519KeyPair};
use ring::rand::SystemRandom;

pub struct WalletKeypair {
    keypair: Ed25519KeyPair,
}

impl WalletKeypair {
    // Load an existing PKCS#8-encoded keypair
    pub fn from_pkcs8(pkcs8_bytes: &[u8]) -> Result<Self, ring::error::KeyRejected> {
        let keypair = Ed25519KeyPair::from_pkcs8(pkcs8_bytes)?;
        Ok(WalletKeypair { keypair })
    }

    pub fn sign(&self, message: &[u8]) -> Vec<u8> {
        self.keypair.sign(message).as_ref().to_vec()
    }
}
