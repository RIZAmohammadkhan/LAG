use tokio::net::{TcpListener, TcpStream};
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use super::message::{Message, MessageType};
use serde_json;
use std::sync::Arc;
use std::collections::HashMap;
use tokio::sync::Mutex;

pub struct Node {
    address: String,
    peers: Arc<Mutex<HashMap<String, TcpStream>>>,
}

impl Node {
    pub fn new(address: String) -> Self {
        Node {
            address,
            peers: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub async fn start(&self) -> io::Result<()> {
        let listener = TcpListener::bind(&self.address).await?;
        println!("Node is running on {}", self.address);

        while let Ok((socket, addr)) = listener.accept().await {
            println!("New connection from {}", addr);
            let peer_address = addr.to_string();
            let peers = self.peers.clone();

            // Add the peer to the peers list
            peers.lock().await.insert(peer_address.clone(), socket.try_clone().unwrap());

            tokio::spawn(async move {
                if let Err(e) = handle_connection(socket, peers, peer_address).await {
                    eprintln!("Failed to handle connection: {}", e);
                }
            });
        }

        Ok(())
    }

    // Additional functionalities like connecting to other nodes, broadcasting messages
    // can be implemented here
}

async fn handle_connection(mut socket: TcpStream, peers: Arc<Mutex<HashMap<String, TcpStream>>>, peer_address: String) -> io::Result<()> {
    let mut buffer = [0; 1024];

    loop {
        let n = socket.read(&mut buffer).await?;
        if n == 0 { break; }

        let message_str = String::from_utf8_lossy(&buffer[..n]);
        if let Ok(message) = serde_json::from_str::<Message>(&message_str) {
            println!("Received {:?} from {}", message, peer_address);
            match message.message_type {
                MessageType::NewBlock => {
                    // Implement logic for handling a new block
                },
                MessageType::NewTransaction => {
                    // Implement logic for handling a new transaction
                },
                // Handle other message types
                _ => {}
            }

            // Example: Echo the received message back to all peers
            for (addr, peer_socket) in peers.lock().await.iter_mut() {
                if addr != &peer_address {
                    let _ = peer_socket.write_all(message_str.as_bytes()).await;
                }
            }
        }
    }

    // Remove the peer from the list when disconnected
    peers.lock().await.remove(&peer_address);

    Ok(())
}
