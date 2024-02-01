use LAG::wallet::{WalletKeypair, Transaction};

#[test]
fn transaction_flow() {
    let sender_keypair = WalletKeypair::new();
    let sender_public_key_hex = sender_keypair.public_key_hex();
    let recipient_public_key_hex = "some_valid_hex_public_key";

    let mut transaction = Transaction::new(sender_public_key_hex, recipient_public_key_hex.to_string(), 100);
    transaction.sign(&sender_keypair);

    assert!(transaction.verify_signature(), "The transaction signature should be valid.");
}
