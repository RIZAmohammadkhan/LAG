use sodiumoxide::crypto::sign;
use hex;

pub struct WalletKeypair {
    pub public_key: sign::PublicKey,
    secret_key: sign::SecretKey,
}

impl WalletKeypair {
    /// Generates a new WalletKeypair.
    pub fn new() -> Self {
        sodiumoxide::init().expect("Failed to initialize sodiumoxide");
        let (public_key, secret_key) = sign::gen_keypair();
        WalletKeypair { public_key, secret_key }
    }

    /// Signs a message with the WalletKeypair's private key.
    pub fn sign(&self, message: &[u8]) -> sign::Signature {
        sign::sign_detached(message, &self.secret_key)
    }

    /// Returns the public key as a hex string.
    pub fn public_key_hex(&self) -> String {
        hex::encode(self.public_key.0)
    }
}
