use super::message::{Message, MessageType};
use serde_json;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::Mutex; // Make sure this path is correct for your project structure

pub struct Node {
    address: String,
    peers: Arc<Mutex<HashMap<String, Arc<Mutex<TcpStream>>>>>,
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
            let shared_socket = Arc::new(Mutex::new(socket));

            peers
                .lock()
                .await
                .insert(peer_address.clone(), shared_socket.clone());

            tokio::spawn(async move {
                if let Err(e) = handle_connection(shared_socket, peers, peer_address).await {
                    eprintln!("Failed to handle connection: {}", e);
                }
            });
        }

        Ok(())
    }
}

async fn handle_connection(
    socket: Arc<Mutex<TcpStream>>,
    peers: Arc<Mutex<HashMap<String, Arc<Mutex<TcpStream>>>>>,
    peer_address: String,
) -> io::Result<()> {
    let mut buffer = [0; 1024];

    loop {
        let mut locked_socket = socket.lock().await;
        let n = locked_socket.read(&mut buffer).await?;
        drop(locked_socket); // Release the lock explicitly

        if n == 0 {
            break;
        }

        let message_str = String::from_utf8_lossy(&buffer[..n]);
        if let Ok(message) = serde_json::from_str::<Message>(&message_str) {
            println!("Received {:?} from {}", message, peer_address);
            match message.message_type {
                MessageType::NewBlock => {
                    // Implement logic for handling a new block
                }
                MessageType::NewTransaction => {
                    // Implement logic for handling a new transaction
                }
                // Handle other message types
                _ => {}
            }

            let peers_lock = peers.lock().await;
            for (addr, peer_socket) in peers_lock.iter() {
                if addr != &peer_address {
                    let mut locked_peer_socket = peer_socket.lock().await;
                    let _ = locked_peer_socket.write_all(message_str.as_bytes()).await;
                }
            }
        }
    }

    peers.lock().await.remove(&peer_address);

    Ok(())
}
