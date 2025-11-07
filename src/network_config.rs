// src/network_config.rs
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NetworkType {
    Mainnet,
    Testnet,
    Devnet,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkConfig {
    pub network_type: NetworkType,
    pub port: u16,
    pub rpc_port: u16,
    pub bootstrap_nodes: Vec<String>,
    pub genesis_timestamp: u64,
    pub block_reward: u64,
}

impl NetworkConfig {
    pub fn mainnet(port: u16, rpc_port: u16) -> Self {
        NetworkConfig {
            network_type: NetworkType::Mainnet,
            port: port,        // DYNAMIC PORT
            rpc_port: rpc_port, // DYNAMIC PORT
            bootstrap_nodes: vec![
                "127.0.0.1:8080".to_string(),
                "127.0.0.1:8081".to_string(),
            ],
            genesis_timestamp: 1735765200,
            block_reward: 25,
        }
    }
    
    pub fn testnet() -> Self {
        NetworkConfig {
            network_type: NetworkType::Testnet,
            port: 18080,
            rpc_port: 13001,
            bootstrap_nodes: vec![
                "127.0.0.1:18080".to_string(),
                "testnet.vexachain.com:18080".to_string(),
            ],
            genesis_timestamp: 1730457600,
            block_reward: 100,
        }
    }
    
    pub fn devnet() -> Self {
        NetworkConfig {
            network_type: NetworkType::Devnet,
            port: 28080,
            rpc_port: 23001,
            bootstrap_nodes: vec![],
            genesis_timestamp: 1730457600,
            block_reward: 1000,
        }
    }
    
    pub fn get_network_id(&self) -> String {
        match self.network_type {
            NetworkType::Mainnet => "vexa-mainnet-v1".to_string(),
            NetworkType::Testnet => "vexa-testnet-1".to_string(),
            NetworkType::Devnet => "vexa-devnet-1".to_string(),
        }
    }
}