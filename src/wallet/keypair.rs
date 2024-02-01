use ed25519_dalek::{PublicKey, SecretKey, Keypair};
use rand::rngs::OsRng;

pub struct WalletKeypair {
    pub keypair: Keypair,
}

impl WalletKeypair {
    pub fn new() -> Self {
        let mut csprng = OsRng{};
        let keypair: Keypair = Keypair::generate(&mut csprng);

        WalletKeypair { keypair }
    }

    pub fn public_key(&self) -> PublicKey {
        self.keypair.public
    }

    pub fn secret_key(&self) -> SecretKey {
        self.keypair.secret
    }
}
