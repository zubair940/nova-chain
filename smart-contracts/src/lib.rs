use std::collections::HashMap;

// NOC - NovaChain Original Coin
pub struct NOCToken {
    pub name: String,
    pub symbol: String,
    pub total_supply: u64,
    pub balances: HashMap<String, u64>,
    pub owner: String,
    pub decimals: u8,
}

impl NOCToken {
    // Constructor - NOC Token create karein
    pub fn new() -> Self {
        let mut balances = HashMap::new();
        let owner = "founder".to_string();
        let initial_supply = 10_000_000; // 10 Million NOC
        
        // Founder ko initial supply assign karein
        balances.insert(owner.clone(), initial_supply);
        
        Self {
            name: "NovaChain Coin".to_string(),
            symbol: "NOC".to_string(),
            total_supply: initial_supply,
            balances,
            owner,
            decimals: 18, // Standard decimals like Ethereum
        }
    }
    
    // Token details get karein
    pub fn get_details(&self) -> String {
        format!("{} ({}) - Total Supply: {} NOC", self.name, self.symbol, self.total_supply)
    }
    
    // Balance check karein
    pub fn balance_of(&self, address: &String) -> u64 {
        *self.balances.get(address).unwrap_or(&0)
    }
    
    // NOC Tokens transfer karein
    pub fn transfer(&mut self, from: String, to: String, amount: u64) -> bool {
        // Check if sender has enough balance
        let from_balance = self.balance_of(&from);
        if from_balance < amount {
            return false; // Insufficient balance
        }
        
        // Update balances
        *self.balances.entry(from).or_insert(0) -= amount;
        *self.balances.entry(to).or_insert(0) += amount;
        
        true // Success
    }
    
    // New NOC tokens create karein (Only owner)
    pub fn mint(&mut self, to: String, amount: u64) -> bool {
        if self.owner != "founder" {
            return false; // Only founder can mint
        }
        
        *self.balances.entry(to).or_insert(0) += amount;
        self.total_supply += amount;
        true
    }
    
    // NOC tokens destroy karein
    pub fn burn(&mut self, from: String, amount: u64) -> bool {
        let from_balance = self.balance_of(&from);
        if from_balance < amount {
            return false;
        }
        
        *self.balances.entry(from).or_insert(0) -= amount;
        self.total_supply -= amount;
        true
    }
    
    // Total circulation check karein
    pub fn total_circulation(&self) -> u64 {
        self.total_supply
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_noc_creation() {
        let noc = NOCToken::new();
        
        assert_eq!(noc.name, "NovaChain Coin");
        assert_eq!(noc.symbol, "NOC");
        assert_eq!(noc.total_supply, 10_000_000);
        assert_eq!(noc.decimals, 18);
    }
    
    #[test]
    fn test_get_details() {
        let noc = NOCToken::new();
        let details = noc.get_details();
        assert!(details.contains("NovaChain Coin"));
        assert!(details.contains("NOC"));
    }
    
    #[test]
    fn test_noc_transfer() {
        let mut noc = NOCToken::new();
        
        let founder = "founder".to_string();
        let user1 = "user1".to_string();
        let user2 = "user2".to_string();
        
        // Check initial balance
        assert_eq!(noc.balance_of(&founder), 10_000_000);
        
        // Transfer NOC tokens
        assert!(noc.transfer(founder.clone(), user1.clone(), 1000));
        assert_eq!(noc.balance_of(&founder), 9_999_000);
        assert_eq!(noc.balance_of(&user1), 1000);
        
        // Transfer between users
        assert!(noc.transfer(user1.clone(), user2.clone(), 500));
        assert_eq!(noc.balance_of(&user1), 500);
        assert_eq!(noc.balance_of(&user2), 500);
    }
    
    #[test]
    fn test_insufficient_noc() {
        let mut noc = NOCToken::new();
        let founder = "founder".to_string();
        let user1 = "user1".to_string();
        
        // Try to transfer more than balance
        assert!(!noc.transfer(founder.clone(), user1.clone(), 20_000_000));
    }
    
    #[test]
    fn test_mint_and_burn() {
        let mut noc = NOCToken::new();
        let user1 = "user1".to_string();
        
        // Mint new NOC tokens
        assert!(noc.mint(user1.clone(), 5000));
        assert_eq!(noc.balance_of(&user1), 5000);
        assert_eq!(noc.total_supply, 10_005_000);
        
        // Burn NOC tokens
        assert!(noc.burn(user1.clone(), 2000));
        assert_eq!(noc.balance_of(&user1), 3000);
        assert_eq!(noc.total_supply, 10_003_000);
    }
}