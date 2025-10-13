// network.rs
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::net::{TcpListener, TcpStream};
use std::io::{Write, BufReader, BufRead};
use std::sync::{Arc, Mutex};
use std::thread;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MessageType {
    NewBlock,
    NewTransaction,
    RequestChain,
    SendChain,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkMessage {
    pub message_type: MessageType,
    pub data: String,
    pub timestamp: u64,
}

pub struct Network {
    port: u16,
    peers: Arc<Mutex<Vec<String>>>,
    messages: Arc<Mutex<VecDeque<NetworkMessage>>>,
}

impl Network {
    pub fn new(port: u16) -> Self {
        Network {
            port,
            peers: Arc::new(Mutex::new(Vec::new())),
            messages: Arc::new(Mutex::new(VecDeque::new())),
        }
    }

    pub fn start(&self) {
        let listener = TcpListener::bind(format!("127.0.0.1:{}", self.port))
            .expect("Failed to bind to address");

        println!("🌐 Nova Chain P2P Node started on port {}", self.port);

        // Start peer discovery in background
        let peers_clone = self.peers.clone();
        let port = self.port;
        thread::spawn(move || {
            Network::discover_peers(peers_clone, port);
        });

        // Handle incoming connections
        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    let messages_clone = self.messages.clone();
                    thread::spawn(move || {
                        Network::handle_client(stream, messages_clone);
                    });
                }
                Err(e) => {
                    eprintln!("Failed to accept connection: {}", e);
                }
            }
        }
    }

    fn handle_client(stream: TcpStream, messages: Arc<Mutex<VecDeque<NetworkMessage>>>) {
        let reader = BufReader::new(stream.try_clone().unwrap());
        
        for line in reader.lines() {
            if let Ok(message_str) = line {
                if let Ok(message) = serde_json::from_str::<NetworkMessage>(&message_str) {
                    println!("📨 Received message: {:?}", message.message_type);
                    messages.lock().unwrap().push_back(message);
                }
            }
        }
    }

    fn discover_peers(peers: Arc<Mutex<Vec<String>>>, my_port: u16) {
        // Discover peers on local network
        for port in 8080..8090 {
            if port == my_port {
                continue; // Skip self
            }

            let peer_addr = format!("127.0.0.1:{}", port);
            if TcpStream::connect(&peer_addr).is_ok() {
                println!("✅ Discovered peer: {}", peer_addr);
                peers.lock().unwrap().push(peer_addr);
            }
        }
    }

    pub fn broadcast_message(&self, message: NetworkMessage) {
        let peers = self.peers.lock().unwrap();
        for peer in peers.iter() {
            if let Ok(mut stream) = TcpStream::connect(peer) {
                let message_str = serde_json::to_string(&message).unwrap();
                writeln!(stream, "{}", message_str).ok();
                println!("📤 Broadcast message to: {}", peer);
            }
        }
    }

    pub fn get_messages(&self) -> Vec<NetworkMessage> {
        let mut messages = self.messages.lock().unwrap();
        let collected: Vec<NetworkMessage> = messages.drain(..).collect();
        collected
    }

    pub fn add_peer(&self, address: String) {
        self.peers.lock().unwrap().push(address);
    }
}

// ADD THIS CLONE IMPLEMENTATION
impl Clone for Network {
    fn clone(&self) -> Self {
        Network {
            port: self.port,
            peers: Arc::clone(&self.peers),
            messages: Arc::clone(&self.messages),
        }
    }
}