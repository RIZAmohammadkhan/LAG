mod wallet;
mod utils;

use crate::wallet::keypair::WalletKeypair;
use crate::wallet::transaction::Transaction;

fn main() {
    // Initialize sodiumoxide library (necessary for cryptographic operations)
    sodiumoxide::init().expect("Failed to initialize sodiumoxide");

    // Generate a new keypair for the sender
    let sender_keypair = WalletKeypair::new();
    let sender_public_key_hex = sender_keypair.public_key_hex();

    // Assume we have a recipient's public key in hex format
    let recipient_public_key_hex = "some_hex_public_key_of_recipient".to_string();

    // Create a new transaction
    let mut transaction = Transaction::new(
        sender_public_key_hex.clone(),
        recipient_public_key_hex,
        100, // Let's say we're transferring 100 units of our cryptocurrency
    );

    // Sign the transaction with the sender's keypair
    transaction.sign(&sender_keypair);

    // Output the transaction details
    println!("Transaction: {:?}", transaction);

    // Verify the transaction signature
    let is_signature_valid = transaction.verify_signature();
    println!("Is the signature valid? {}", is_signature_valid);

}

