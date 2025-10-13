// coin.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Coin {
    pub symbol: String,
    pub name: String,
    pub total_supply: u64,
    pub circulating_supply: u64,
    pub decimals: u8,
    pub block_reward: u64,
    pub halving_interval: u64,
}

impl Coin {
    pub fn new() -> Self {
        Coin {
            symbol: "NOVA".to_string(),
            name: "NOVA Coin".to_string(),
            total_supply: 100_000_000_000_000_000, // 100M with 9 decimals
            circulating_supply: 0,
            decimals: 9,
            block_reward: 50_000_000_000, // 50 NOVA with decimals
            halving_interval: 1_000_000,
        }
    }
    
    pub fn format_amount(&self, amount: u64) -> f64 {
        amount as f64 / 10f64.powi(self.decimals as i32)
    }
    
    pub fn parse_amount(&self, amount: f64) -> u64 {
        (amount * 10f64.powi(self.decimals as i32)) as u64
    }
    
    pub fn get_block_reward(&self, block_height: u64) -> u64 {
        let halvings = block_height / self.halving_interval;
        let mut reward = self.block_reward;
        
        for _ in 0..halvings {
            reward /= 2;
            if reward == 0 {
                break;
            }
        }
        
        reward
    }
    
    pub fn update_circulating_supply(&mut self, mined_amount: u64) {
        self.circulating_supply += mined_amount;
    }
    
    pub fn get_supply_info(&self) -> String {
        format!(
            "NOVA Coin Supply:\nTotal: {} NOVA\nCirculating: {} NOVA\nBlock Reward: {} NOVA\nDecimals: {}",
            self.format_amount(self.total_supply),
            self.format_amount(self.circulating_supply),
            self.format_amount(self.block_reward),
            self.decimals
        )
    }
}

// Genesis distribution addresses and amounts
pub struct GenesisDistribution;
impl GenesisDistribution {
    pub fn get_distribution() -> Vec<(String, u64)> {
        vec![
            // Foundation: 5,000,000 NOVA
            ("NovaFoundation".to_string(), 5_000_000_000_000_000),
            // Development: 3,000,000 NOVA  
            ("DevTeam".to_string(), 3_000_000_000_000_000),
            // Community: 2,000,000 NOVA
            ("Community".to_string(), 2_000_000_000_000_000),
        ]
    }
    
    pub fn get_total_genesis_supply() -> u64 {
        Self::get_distribution()
            .iter()
            .map(|(_, amount)| amount)
            .sum()
    }
}