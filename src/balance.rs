// balance.rs
use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BalanceTracker {
    pub balances: HashMap<String, u64>, // address -> balance
}

impl BalanceTracker {
    pub fn new() -> Self {
        BalanceTracker {
            balances: HashMap::new(),
        }
    }
    
    pub fn update_balance(&mut self, address: String, amount: u64) {
        let current_balance = self.balances.entry(address).or_insert(0);
        *current_balance += amount;
    }
    
    pub fn get_balance(&self, address: &str) -> u64 {
        *self.balances.get(address).unwrap_or(&0)
    }
    
    pub fn validate_transaction(&self, sender: &str, amount: u64) -> bool {
        let balance = self.get_balance(sender);
        balance >= amount
    }
    
    pub fn process_transaction(&mut self, sender: String, receiver: String, amount: u64) -> bool {
        if !self.validate_transaction(&sender, amount) {
            return false;
        }
        
        // Deduct from sender
        if let Some(sender_balance) = self.balances.get_mut(&sender) {
            *sender_balance -= amount;
        }
        
        // Add to receiver
        self.update_balance(receiver, amount);
        
        true
    }
    
    pub fn get_total_supply(&self) -> u64 {
        self.balances.values().sum()
    }
}