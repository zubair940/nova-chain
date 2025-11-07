// balance.rs
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

// 🆕 NEW: Import feature models


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BalanceTracker {
    pub balances: HashMap<String, u64>, // address -> balance
    // 🆕 NEW: Enhanced balance tracking for all features
    pub staked_balances: HashMap<String, u64>,        // 🆕 Staked amounts
    pub locked_balances: HashMap<String, u64>,        // 🆕 Locked balances (vesting, etc.)
    pub dao_voting_power: HashMap<String, u64>,       // 🆕 DAO voting power
    pub burned_totals: HashMap<String, u64>,          // 🆕 Total burned per address
    pub transaction_history: HashMap<String, Vec<TransactionRecord>>, // 🆕 Transaction history
    pub balance_snapshots: HashMap<String, Vec<BalanceSnapshot>>, // 🆕 Historical balances
}

// 🆕 NEW: Transaction record for history
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionRecord {
    pub tx_hash: String,
    pub from: String,
    pub to: String,
    pub amount: u64,
    pub fee: u64,
    pub timestamp: u64,
    pub transaction_type: String, // "transfer", "stake", "unstake", "burn", "dao_vote"
    pub block_height: u64,
}

// 🆕 NEW: Balance snapshot for historical tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BalanceSnapshot {
    pub balance: u64,
    pub staked: u64,
    pub locked: u64,
    pub timestamp: u64,
    pub block_height: u64,
}

impl BalanceTracker {
    pub fn new() -> Self {
        BalanceTracker {
            balances: HashMap::new(),
            // 🆕 NEW: Initialize enhanced tracking
            staked_balances: HashMap::new(),
            locked_balances: HashMap::new(),
            dao_voting_power: HashMap::new(),
            burned_totals: HashMap::new(),
            transaction_history: HashMap::new(),
            balance_snapshots: HashMap::new(),
        }
    }
    
    pub fn update_balance(&mut self, address: String, amount: u64) {
        let current_balance = self.balances.entry(address.clone()).or_insert(0);
        *current_balance += amount;
        
        // 🆕 NEW: Update voting power based on new balance
        self.update_voting_power(&address);
        
        // 🆕 NEW: Create balance snapshot
        self.create_snapshot(&address);
    }
    
    pub fn get_balance(&self, address: &str) -> u64 {
        *self.balances.get(address).unwrap_or(&0)
    }
    
    // 🆕 NEW: Get total balance including staked
    pub fn get_total_balance(&self, address: &str) -> u64 {
        let available = self.get_balance(address);
        let staked = self.get_staked_balance(address);
        available + staked
    }
    
    pub fn validate_transaction(&self, sender: &str, amount: u64) -> bool {
        let balance = self.get_balance(sender);
        balance >= amount
    }
    
    // 🆕 NEW: Enhanced transaction validation
    pub fn validate_enhanced_transaction(&self, sender: &str, amount: u64, fee: u64) -> bool {
        let available_balance = self.get_balance(sender);
        let total_cost = amount + fee;
        
        if available_balance < total_cost {
            println!("❌ Insufficient balance: available {}, required {}", 
                     available_balance, total_cost);
            return false;
        }
        
        true
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
        self.update_balance(receiver.clone(), amount);
        
        // 🆕 NEW: Record transaction
        self.record_transaction(
            "transfer".to_string(),
            sender.clone(),
            receiver,
            amount,
            0, // fee will be recorded separately
        );
        
        // 🆕 NEW: Update voting power for both parties
        self.update_voting_power(&sender);
        
        true
    }
    
    // 🆕 NEW: Enhanced transaction processing with fees
    pub fn process_enhanced_transaction(
        &mut self, 
        sender: String, 
        receiver: String, 
        amount: u64, 
        fee: u64,
        transaction_type: String
    ) -> bool {
        let total_cost = amount + fee;
        
        if !self.validate_enhanced_transaction(&sender, amount, fee) {
            return false;
        }
        
        // Deduct amount and fee from sender
        if let Some(sender_balance) = self.balances.get_mut(&sender) {
            *sender_balance -= total_cost;
        }
        
        // Add amount to receiver (fee goes to miner/burn)
        if transaction_type != "burn" {
            self.update_balance(receiver.clone(), amount);
        }
        
        // 🆕 NEW: Record transaction with type
        self.record_transaction(
            transaction_type,
            sender.clone(),
            receiver,
            amount,
            fee,
        );
        
        // 🆕 NEW: Update voting power
        self.update_voting_power(&sender);
        
        true
    }
    
    pub fn get_total_supply(&self) -> u64 {
        self.balances.values().sum()
    }

    // NEW: Fee deduction function
    pub fn deduct_fee(&mut self, sender: String, fee: u64) {
        if sender != "MINING_REWARD" && sender != "GENESIS" {
            if let Some(sender_balance) = self.balances.get_mut(&sender) {
                *sender_balance -= fee;
            }
        }
    }

    // 🆕 NEW: STAKING FUNCTIONS
    
    pub fn stake_tokens(&mut self, staker: String, amount: u64) -> bool {
        if !self.validate_transaction(&staker, amount) {
            return false;
        }
        
        // Deduct from available balance
        if let Some(balance) = self.balances.get_mut(&staker) {
            *balance -= amount;
        }
        
        // Add to staked balance
        let staked_balance = self.staked_balances.entry(staker.clone()).or_insert(0);
        *staked_balance += amount;
        
        // 🆕 NEW: Record staking transaction
        self.record_transaction(
            "stake".to_string(),
            staker.clone(),
            "STAKING_POOL".to_string(),
            amount,
            0,
        );
        
        // 🆕 NEW: Update voting power (staking may affect voting power)
        self.update_voting_power(&staker);
        
        println!("✅ Staked {} tokens from {}", amount, staker);
        true
    }
    
    pub fn unstake_tokens(&mut self, staker: String, amount: u64) -> bool {
        let staked_balance = self.get_staked_balance(&staker);
        
        if staked_balance < amount {
            println!("❌ Not enough staked tokens: staked {}, requested {}", 
                     staked_balance, amount);
            return false;
        }
        
        // Remove from staked balance
        if let Some(balance) = self.staked_balances.get_mut(&staker) {
            *balance -= amount;
        }
        
        // Add back to available balance
        self.update_balance(staker.clone(), amount);
        
        // 🆕 NEW: Record unstaking transaction
        self.record_transaction(
            "unstake".to_string(),
            "STAKING_POOL".to_string(),
            staker.clone(),
            amount,
            0,
        );
        
        // 🆕 NEW: Update voting power
        self.update_voting_power(&staker);
        
        println!("✅ Unstaked {} tokens to {}", amount, staker);
        true
    }
    
    pub fn get_staked_balance(&self, address: &str) -> u64 {
        *self.staked_balances.get(address).unwrap_or(&0)
    }
    
    // 🆕 NEW: TOKEN BURNING FUNCTIONS
    
    pub fn burn_tokens(&mut self, burner: String, amount: u64) -> bool {
        if !self.validate_transaction(&burner, amount) {
            return false;
        }
        
        // Deduct from balance
        if let Some(balance) = self.balances.get_mut(&burner) {
            *balance -= amount;
        }
        
        // Record burned amount
        let total_burned = self.burned_totals.entry(burner.clone()).or_insert(0);
        *total_burned += amount;
        
        // 🆕 NEW: Record burn transaction
        self.record_transaction(
            "burn".to_string(),
            burner.clone(),
            "BURN_ADDRESS".to_string(),
            amount,
            0,
        );
        
        // 🆕 NEW: Update voting power (burning may reduce voting power)
        self.update_voting_power(&burner);
        
        println!("🔥 Burned {} tokens from {}", amount, burner);
        true
    }
    
    pub fn get_burned_total(&self, address: &str) -> u64 {
        *self.burned_totals.get(address).unwrap_or(&0)
    }
    
    // 🆕 NEW: DAO GOVERNANCE FUNCTIONS
    
    pub fn update_voting_power(&mut self, address: &str) {
        let available_balance = self.get_balance(address);
        let staked_balance = self.get_staked_balance(address);
        
        // 🆕 NEW: Calculate voting power (available + staked balances)
        let voting_power = available_balance + staked_balance;
        
        self.dao_voting_power.insert(address.to_string(), voting_power);
    }
    
    pub fn get_voting_power(&self, address: &str) -> u64 {
        *self.dao_voting_power.get(address).unwrap_or(&0)
    }
    
    // 🆕 NEW: LOCKED BALANCES (for vesting, etc.)
    
    pub fn lock_tokens(&mut self, address: String, amount: u64, unlock_time: u64) -> bool {
        if !self.validate_transaction(&address, amount) {
            return false;
        }
        
        // Deduct from available balance
        if let Some(balance) = self.balances.get_mut(&address) {
            *balance -= amount;
        }
        
        // Add to locked balance
        let locked_balance = self.locked_balances.entry(address.clone()).or_insert(0);
        *locked_balance += amount;
        
        // 🆕 NEW: Record locking transaction
        self.record_transaction(
            "lock".to_string(),
            address.clone(),
            "LOCKED_FUNDS".to_string(),
            amount,
            0,
        );
        
        println!("🔒 Locked {} tokens until {} from {}", amount, unlock_time, address);
        true
    }
    
    pub fn unlock_tokens(&mut self, address: String, amount: u64) -> bool {
        let locked_balance = self.get_locked_balance(&address);
        
        if locked_balance < amount {
            println!("❌ Not enough locked tokens: locked {}, requested {}", 
                     locked_balance, amount);
            return false;
        }
        
        // Remove from locked balance
        if let Some(balance) = self.locked_balances.get_mut(&address) {
            *balance -= amount;
        }
        
        // Add back to available balance
        self.update_balance(address.clone(), amount);
        
        // 🆕 NEW: Record unlocking transaction
        self.record_transaction(
            "unlock".to_string(),
            "LOCKED_FUNDS".to_string(),
            address.clone(),
            amount,
            0,
        );
        
        println!("🔓 Unlocked {} tokens to {}", amount, address);
        true
    }
    
    pub fn get_locked_balance(&self, address: &str) -> u64 {
        *self.locked_balances.get(address).unwrap_or(&0)
    }
    
    // 🆕 NEW: TRANSACTION HISTORY FUNCTIONS
    
    fn record_transaction(&mut self, transaction_type: String, from: String, to: String, amount: u64, fee: u64) {
        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        let tx_hash = format!("tx_{}_{}_{}", from, to, timestamp);
        
        let record = TransactionRecord {
            tx_hash,
            from: from.clone(),
            to: to.clone(),
            amount,
            fee,
            timestamp,
            transaction_type,
            block_height: 0, // Will be set when included in block
        };
        
        // Add to sender's history
        self.transaction_history
            .entry(from)
            .or_insert_with(Vec::new)
            .push(record.clone());
            
        // Add to receiver's history (if not burn)
        if to != "BURN_ADDRESS" {
            self.transaction_history
                .entry(to)
                .or_insert_with(Vec::new)
                .push(record);
        }
    }
    
    pub fn get_transaction_history(&self, address: &str) -> Vec<&TransactionRecord> {
        self.transaction_history
            .get(address)
            .map(|history| history.iter().collect())
            .unwrap_or_else(Vec::new)
    }
    
    // 🆕 NEW: BALANCE SNAPSHOTS
    
    fn create_snapshot(&mut self, address: &str) {
        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        
        let snapshot = BalanceSnapshot {
            balance: self.get_balance(address),
            staked: self.get_staked_balance(address),
            locked: self.get_locked_balance(address),
            timestamp,
            block_height: 0, // Will be set when block is mined
        };
        
        self.balance_snapshots
            .entry(address.to_string())
            .or_insert_with(Vec::new)
            .push(snapshot);
    }
    
    pub fn get_balance_history(&self, address: &str) -> Vec<&BalanceSnapshot> {
        self.balance_snapshots
            .get(address)
            .map(|snapshots| snapshots.iter().collect())
            .unwrap_or_else(Vec::new)
    }
    
    // 🆕 NEW: COMPREHENSIVE BALANCE INFO
    
    pub fn get_comprehensive_balance(&self, address: &str) -> String {
        let available = self.get_balance(address);
        let staked = self.get_staked_balance(address);
        let locked = self.get_locked_balance(address);
        let burned = self.get_burned_total(address);
        let voting_power = self.get_voting_power(address);
        let total = available + staked + locked;
        
        format!(
            "Comprehensive Balance for {}:\n\
            Available: {} VEXA\n\
            Staked: {} VEXA\n\
            Locked: {} VEXA\n\
            Total: {} VEXA\n\
            Burned: {} VEXA\n\
            Voting Power: {} VEXA\n\
            Transaction History: {} transactions",
            address, available, staked, locked, total, burned, voting_power,
            self.get_transaction_history(address).len()
        )
    }
    
    // 🆕 NEW: ENHANCED TOTAL SUPPLY CALCULATION
    
    pub fn get_enhanced_total_supply(&self) -> String {
        let circulating = self.get_total_supply();
        let total_staked: u64 = self.staked_balances.values().sum();
        let total_locked: u64 = self.locked_balances.values().sum();
        let total_burned: u64 = self.burned_totals.values().sum();
        
        format!(
            "Enhanced Supply Overview:\n\
            Circulating: {} VEXA\n\
            Total Staked: {} VEXA\n\
            Total Locked: {} VEXA\n\
            Total Burned: {} VEXA\n\
            Active Addresses: {}",
            circulating, total_staked, total_locked, total_burned,
            self.balances.len()
        )
    }
    
    // 🆕 NEW: RICH LIST (top holders)
    
    pub fn get_top_holders(&self, limit: usize) -> Vec<(String, u64)> {
        let mut holders: Vec<(String, u64)> = self.balances
            .iter()
            .map(|(address, balance)| (address.clone(), *balance))
            .collect();
            
        holders.sort_by(|a, b| b.1.cmp(&a.1));
        holders.into_iter().take(limit).collect()
    }
}