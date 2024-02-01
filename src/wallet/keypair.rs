use ed25519_dalek::{Keypair, PublicKey, SecretKey, Signature, Signer};
use rand::rngs::OsRng;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct WalletKeypair {
    keypair: Keypair,
}

impl WalletKeypair {
    /// Creates a new WalletKeypair.
    pub fn new() -> Self {
        let mut csprng = OsRng{};
        let keypair = Keypair::generate(&mut csprng);
        WalletKeypair { keypair }
    }

    /// Returns the public key of the keypair.
    pub fn public_key(&self) -> PublicKey {
        self.keypair.public
    }

    /// Returns the secret key of the keypair.
    pub fn secret_key(&self) -> SecretKey {
        self.keypair.secret
    }

    /// Signs a message with the secret key.
    pub fn sign(&self, message: &[u8]) -> Signature {
        self.keypair.sign(message)
    }

    // Additional functionalities like verifying signatures can be added here.
}
