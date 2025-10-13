// wallet.rs
use rand::rngs::OsRng;
use rand::Rng;
use sha2::{Sha256, Digest};
use base58::ToBase58;
use std::fmt;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Wallet {
    pub public_key: String,
    pub private_key: String,
    pub address: String,
}

impl Wallet {
    pub fn new() -> Self {
        let mut rng = OsRng;
        
        // Generate random keypair (32 bytes each)
        let mut public_key_bytes = [0u8; 32];
        let mut private_key_bytes = [0u8; 32];
        rng.fill(&mut public_key_bytes);
        rng.fill(&mut private_key_bytes);
        
        let address = Wallet::generate_address(&public_key_bytes);
        
        Wallet {
            public_key: hex::encode(public_key_bytes),
            private_key: hex::encode(private_key_bytes),
            address,
        }
    }
    
    fn generate_address(public_key: &[u8]) -> String {
        let mut hasher = Sha256::new();
        hasher.update(public_key);
        let result = hasher.finalize();
        
        // Take first 20 bytes for address (like Ethereum)
        let address_bytes = &result[..20];
        address_bytes.to_base58()
    }
    
    pub fn sign_transaction(&self, transaction_data: &str) -> String {
        // Create signature using private key + transaction data
        let mut hasher = Sha256::new();
        hasher.update(transaction_data.as_bytes());
        hasher.update(hex::decode(&self.private_key).unwrap().as_slice());
        let result = hasher.finalize();
        
        // Combine with public key for verification
        let mut signature_data = Vec::new();
        signature_data.extend_from_slice(&result);
        signature_data.extend_from_slice(hex::decode(&self.public_key).unwrap().as_slice());
        
        hex::encode(signature_data)
    }
    
    pub fn verify_signature(transaction_data: &str, signature: &str, public_key: &str) -> bool {
        // Extract signature hash and public key from signature
        if let Ok(signature_bytes) = hex::decode(signature) {
            if signature_bytes.len() >= 32 {
                let signature_hash = &signature_bytes[..32];
                
                // Recreate expected signature
                let mut hasher = Sha256::new();
                hasher.update(transaction_data.as_bytes());
                
                // Use the provided public key bytes
                if let Ok(pub_key_bytes) = hex::decode(public_key) {
                    hasher.update(&pub_key_bytes);
                    let expected_hash = hasher.finalize();
                    
                    // Compare hashes
                    return signature_hash == expected_hash.as_slice();
                }
            }
        }
        false
    }
    
    pub fn get_balance_address(&self) -> String {
        self.address.clone()
    }
    
    pub fn validate_ownership(&self, signature: &str, message: &str) -> bool {
        Wallet::verify_signature(message, signature, &self.public_key)
    }
}

impl fmt::Display for Wallet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Address: {}\nPublic Key: {}\nPrivate Key: {}",
            self.address, self.public_key, self.private_key
        )
    }
}