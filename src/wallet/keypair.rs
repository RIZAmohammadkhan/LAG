use sodiumoxide::crypto::sign;
use sodiumoxide::crypto::sign::ed25519::{PublicKey, SecretKey, Signature};

pub struct WalletKeypair {
    pub public_key: PublicKey,
    secret_key: SecretKey,
}

impl WalletKeypair {
    pub fn new() -> Self {
        let (public_key, secret_key) = sign::gen_keypair();
        WalletKeypair { public_key, secret_key }
    }

    pub fn sign(&self, message: &[u8]) -> Signature {
        sign::sign_detached(message, &self.secret_key)
    }
}
