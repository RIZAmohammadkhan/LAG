mod blockchain;
mod network;
mod wallet;
mod utils;

use blockchain::{Blockchain, Block, ProofOfWork};
use network::Node;
use wallet::{Wallet, WalletKeypair, Transaction};
use std::env;

#[tokio::main]
async fn main() {
    // Initialize Blockchain with a genesis block
    let mut blockchain = Blockchain::new();
    let proof_of_work = ProofOfWork::new(2); // Example difficulty

    // Initialize Network Node
    let node_address = env::args().nth(1).unwrap_or_else(|| "127.0.0.1:8080".to_string());
    let node = Node::new(node_address.clone());
    tokio::spawn(async move {
        if let Err(e) = node.start().await {
            eprintln!("Node encountered an error: {}", e);
        }
    });

    // Initialize Wallet
    let wallet_keypair = WalletKeypair::new();
    let wallet = Wallet::new(wallet_keypair.clone());

    // Create a new transaction
    let transaction = Transaction::new(wallet.public_key(), "recipient_public_key".to_string(), 100);
    let signed_transaction = wallet.sign_transaction(transaction);

    // Example: Mining a new block with the transaction
    let mut new_block = Block::new(blockchain.blocks.last().unwrap().index + 1, serde_json::to_string(&signed_transaction).unwrap(), blockchain.blocks.last().unwrap().hash.clone());
    proof_of_work.mine(&mut new_block);
    blockchain.add_block(new_block);

    println!("LAG Cryptocurrency running on {}", node_address);

    // Display the blockchain for demonstration
    for block in blockchain.blocks.iter() {
        println!("Block Index: {}", block.index);
        println!("Block Timestamp: {}", block.timestamp);
        println!("Block Data: {}", block.data);
        println!("Prev Hash: {}", block.previous_hash);
        println!("Hash: {}", block.hash);
        println!("Nonce: {}", block.nonce);
        println!("-----------------------");
    }

    // Prevent the program from exiting immediately
    loop {
        tokio::time::sleep(tokio::time::Duration::from_secs(60)).await;
    }
}
