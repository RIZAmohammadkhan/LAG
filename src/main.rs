mod blockchain;
mod network;
mod wallet;
mod utils;

use blockchain::{Blockchain, Block};
use network::Node;
use wallet::{Wallet, WalletKeypair};
use std::env;

#[tokio::main]
async fn main() {
    // Example: Initialize Blockchain with a genesis block
    let mut blockchain = Blockchain::new();
    blockchain.add_block(Block::new(0, "Genesis Block".to_string(), String::new()));

    // Example: Initialize Network Node
    let node_address = env::args().nth(1).unwrap_or_else(|| "127.0.0.1:8080".to_string());
    let node = Node::new(node_address.clone());
    tokio::spawn(async move {
        if let Err(e) = node.start().await {
            eprintln!("Node encountered an error: {}", e);
        }
    });

    // Example: Initialize Wallet
    let wallet = Wallet::new(WalletKeypair::new());

    // Your cryptocurrency main logic here
    println!("LAG Cryptocurrency running on {}", node_address);

    // Prevent the program from exiting immediately
    loop {
        tokio::time::sleep(tokio::time::Duration::from_secs(60)).await;
    }
}
