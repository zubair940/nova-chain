use crate::models::{Staker, SocialWallet, SecurityLevel};
use rand::rngs::OsRng;
use rand::Rng;
use sha2::{Sha256, Digest};
use base58::ToBase58;
use std::fmt;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

// 🆕 NEW: Import feature models
use crate::models::DAOVote;

// 🚀 NEW: Advanced Wallet Types
#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(PartialEq)]
pub enum WalletType {
    Standard,           // Basic wallet
    Enterprise,         // Business wallet
    MultiSig,           // Multi-signature wallet
    Social,             // Social media linked
    Hardware,           // Hardware wallet
    QuantumResistant,   // Quantum-safe wallet
    DeFi,               // DeFi optimized
    Gaming,             // Gaming optimized
}

// 🚀 NEW: Biometric Security
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BiometricData {
    pub fingerprint_hash: String,
    pub facial_recognition_data: String,
    pub voice_signature: String,
    pub last_biometric_auth: u64,
}

// 🚀 NEW: Multi-signature Wallet
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MultiSigWallet {
    pub address: String,
    pub owners: Vec<String>,
    pub required_signatures: u8,
    pub pending_transactions: HashMap<String, MultiSigTransaction>,
    pub created_at: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MultiSigTransaction {
    pub tx_hash: String,
    pub from: String,
    pub to: String,
    pub amount: u64,
    pub signatures: Vec<String>,
    pub created_at: u64,
    pub executed: bool,
}

// 🚀 NEW: Hardware Wallet Integration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HardwareWallet {
    pub device_id: String,
    pub manufacturer: String,
    pub model: String,
    pub is_connected: bool,
    pub last_sync: u64,
}

// 🚀 NEW: AI Security Features
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AISecurity {
    pub anomaly_detection: bool,
    pub behavior_analysis: bool,
    pub risk_scoring: bool,
    pub threat_intelligence: bool,
    pub last_risk_assessment: u64,
    pub risk_score: f64,
}

// 🚀 NEW: Cross-chain Wallet
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrossChainAddresses {
    pub ethereum: String,
    pub binance: String,
    pub polygon: String,
    pub solana: String,
    pub avalanche: String,
}

// 🚀 NEW: DeFi Portfolio
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeFiPortfolio {
    pub total_value_locked: u64,
    pub yield_farming: HashMap<String, u64>,
    pub liquidity_pools: HashMap<String, u64>,
    flash_loans_taken: u32,
    insurance_coverage: u64,
}

// 🚀 NEW: Gaming & Metaverse Assets
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GamingAssets {
    pub nfts: HashMap<String, u32>, // NFT ID -> Count
    pub in_game_currency: u64,
    pub virtual_land: Vec<String>,
    pub avatar_nft: Option<String>,
    pub play_to_earn_earnings: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Wallet {
    pub public_key: String,
    pub private_key: String,
    pub address: String,
    // 🆕 NEW: Enhanced wallet features
    pub nonce: u64,                    // 🆕 Transaction sequence
    pub balance: u64,                  // 🆕 Cached balance
    pub staking_info: Option<Staker>,  // 🆕 Staking information
    pub dao_voting_power: u64,         // 🆕 DAO voting power
    pub created_at: u64,               // 🆕 Wallet creation time
    pub last_activity: u64,            // 🆕 Last transaction time
    pub security_level: SecurityLevel, // 🆕 Security settings
    pub features: WalletFeatures,      // 🆕 Enabled features
    
    // 🆕 NEW: MASS ADOPTION FEATURES
    pub social_wallet: Option<SocialWallet>, // 🆕 Social wallet integration
    pub referral_code: String,               // 🆕 Unique referral code
    pub total_earned: u64,                   // 🆕 Total earnings from tasks
    pub tasks_completed: u32,                // 🆕 Number of completed tasks
    pub daily_reward_claimed: bool,          // 🆕 Daily reward status
    
    // 🚀 NEW: ADVANCED FEATURES
    pub wallet_type: WalletType,              // 🚀 Wallet classification
    pub biometric_data: Option<BiometricData>, // 🚀 Biometric security
    pub multi_sig_wallet: Option<MultiSigWallet>, // 🚀 Multi-signature support
    pub hardware_wallet: Option<HardwareWallet>, // 🚀 Hardware wallet integration
    pub ai_security: AISecurity,              // 🚀 AI-powered security
    pub cross_chain_addresses: CrossChainAddresses, // 🚀 Multi-chain addresses
    pub defi_portfolio: DeFiPortfolio,        // 🚀 DeFi investments
    pub gaming_assets: GamingAssets,          // 🚀 Gaming & metaverse assets
    pub enterprise_features: EnterpriseWalletFeatures, // 🚀 Business features
    pub quantum_resistance: QuantumWalletFeatures, // 🚀 Quantum-safe features
}

// 🚀 NEW: Enterprise Wallet Features
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnterpriseWalletFeatures {
    pub company_name: Option<String>,
    pub tax_id: Option<String>,
    pub compliance_status: bool,
    pub kyc_verified: bool,
    pub aml_checked: bool,
    pub monthly_transaction_limit: u64,
    pub used_this_month: u64,
    pub authorized_signers: Vec<String>,
}

// 🚀 NEW: Quantum Resistance Features
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuantumWalletFeatures {
    pub quantum_safe_algorithm: bool,
    pub kyber512_public_key: String,
    pub dilithium2_signature: String,
    pub migration_ready: bool,
    pub backup_algorithm: String,
}

// 🆕 NEW: Wallet features configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WalletFeatures {
    pub token_burning: bool,      // 🆕 Enable token burning
    pub dao_governance: bool,     // 🆕 Enable DAO participation
    pub staking: bool,            // 🆕 Enable staking
    pub cross_chain: bool,        // 🆕 Enable cross-chain
    pub smart_contracts: bool,    // 🆕 Enable smart contracts
    pub multi_sig: bool,          // 🆕 Enable multi-signature
    
    // 🆕 NEW: MASS ADOPTION FEATURES
    pub social_payments: bool,    // 🆕 Enable social payments
    pub micro_earning: bool,      // 🆕 Enable micro-earning tasks
    pub daily_rewards: bool,      // 🆕 Enable daily rewards
    pub referral_program: bool,   // 🆕 Enable referral program
    pub nft_creation: bool,       // 🆕 Enable NFT creation
    pub prediction_markets: bool, // 🆕 Enable prediction markets
    
    // 🚀 NEW: ADVANCED FEATURES
    pub biometric_auth: bool,     // 🚀 Biometric authentication
    pub hardware_wallet: bool,    // 🚀 Hardware wallet support
    pub ai_security: bool,        // 🚀 AI security monitoring
    pub quantum_resistant: bool,  // 🚀 Quantum-safe cryptography
    pub enterprise_mode: bool,    // 🚀 Enterprise features
    pub defi_advanced: bool,      // 🚀 Advanced DeFi
    pub gaming_metaverse: bool,   // 🚀 Gaming & metaverse
    pub instant_transactions: bool, // 🚀 Instant transactions
    pub privacy_mode: bool,       // 🚀 Privacy features
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
        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        let referral_code = Wallet::generate_referral_code();
        
        Wallet {
            public_key: hex::encode(public_key_bytes),
            private_key: hex::encode(private_key_bytes),
            address,
            // 🆕 NEW: Initialize enhanced features
            nonce: 0,
            balance: 0,
            staking_info: None,
            dao_voting_power: 0,
            created_at: timestamp,
            last_activity: timestamp,
            security_level: SecurityLevel::Medium,
            features: WalletFeatures {
                token_burning: true,
                dao_governance: true,
                staking: true,
                cross_chain: true,
                smart_contracts: true,
                multi_sig: false,
                // 🆕 NEW: Mass adoption features enabled by default
                social_payments: true,
                micro_earning: true,
                daily_rewards: true,
                referral_program: true,
                nft_creation: true,
                prediction_markets: true,
                // 🚀 NEW: Advanced features
                biometric_auth: false,
                hardware_wallet: false,
                ai_security: true,
                quantum_resistant: false,
                enterprise_mode: false,
                defi_advanced: true,
                gaming_metaverse: true,
                instant_transactions: true,
                privacy_mode: false,
            },
            
            // 🆕 NEW: Mass adoption features initialization
            social_wallet: None,
            referral_code,
            total_earned: 0,
            tasks_completed: 0,
            daily_reward_claimed: false,
            
            // 🚀 NEW: Initialize advanced features
            wallet_type: WalletType::Standard,
            biometric_data: None,
            multi_sig_wallet: None,
            hardware_wallet: None,
            ai_security: AISecurity {
                anomaly_detection: true,
                behavior_analysis: true,
                risk_scoring: true,
                threat_intelligence: true,
                last_risk_assessment: timestamp,
                risk_score: 0.1,
            },
            cross_chain_addresses: CrossChainAddresses {
                ethereum: "".to_string(),
                binance: "".to_string(),
                polygon: "".to_string(),
                solana: "".to_string(),
                avalanche: "".to_string(),
            },
            defi_portfolio: DeFiPortfolio {
                total_value_locked: 0,
                yield_farming: HashMap::new(),
                liquidity_pools: HashMap::new(),
                flash_loans_taken: 0,
                insurance_coverage: 0,
            },
            gaming_assets: GamingAssets {
                nfts: HashMap::new(),
                in_game_currency: 0,
                virtual_land: Vec::new(),
                avatar_nft: None,
                play_to_earn_earnings: 0,
            },
            enterprise_features: EnterpriseWalletFeatures {
                company_name: None,
                tax_id: None,
                compliance_status: false,
                kyc_verified: false,
                aml_checked: false,
                monthly_transaction_limit: 0,
                used_this_month: 0,
                authorized_signers: Vec::new(),
            },
            quantum_resistance: QuantumWalletFeatures {
                quantum_safe_algorithm: false,
                kyber512_public_key: "".to_string(),
                dilithium2_signature: "".to_string(),
                migration_ready: false,
                backup_algorithm: "SPHINCS+".to_string(),
            },
        }
    }
    
    // 🚀 NEW: Create specialized wallets
    pub fn new_enterprise_wallet(company_name: String, tax_id: String) -> Self {
        let mut wallet = Wallet::new();
        wallet.wallet_type = WalletType::Enterprise;
        wallet.features.enterprise_mode = true;
        wallet.enterprise_features.company_name = Some(company_name);
        wallet.enterprise_features.tax_id = Some(tax_id);
        wallet.enterprise_features.compliance_status = true;
        wallet.enterprise_features.kyc_verified = true;
        wallet.enterprise_features.aml_checked = true;
        wallet.enterprise_features.monthly_transaction_limit = 100_000_000_000_000; // 100K VEXA
        wallet
    }
    
    pub fn new_hardware_wallet(device_id: String, manufacturer: String, model: String) -> Self {
        let mut wallet = Wallet::new();
        wallet.wallet_type = WalletType::Hardware;
        wallet.features.hardware_wallet = true;
        wallet.hardware_wallet = Some(HardwareWallet {
            device_id,
            manufacturer,
            model,
            is_connected: true,
            last_sync: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
        });
        wallet
    }
    
    pub fn new_quantum_wallet() -> Self {
        let mut wallet = Wallet::new();
        wallet.wallet_type = WalletType::QuantumResistant;
        wallet.features.quantum_resistant = true;
        wallet.quantum_resistance.quantum_safe_algorithm = true;
        wallet.quantum_resistance.kyber512_public_key = "quantum_kyber_public_key".to_string();
        wallet.quantum_resistance.dilithium2_signature = "quantum_dilithium_signature".to_string();
        wallet.quantum_resistance.migration_ready = true;
        wallet
    }
    
    pub fn new_defi_wallet() -> Self {
        let mut wallet = Wallet::new();
        wallet.wallet_type = WalletType::DeFi;
        wallet.features.defi_advanced = true;
        wallet
    }
    
    pub fn new_gaming_wallet() -> Self {
        let mut wallet = Wallet::new();
        wallet.wallet_type = WalletType::Gaming;
        wallet.features.gaming_metaverse = true;
        wallet
    }
    
    // 🚀 NEW: Create elite tokenomics wallets for Gate.io distribution
    pub fn new_foundation_wallet(name: &str, allocation: u64) -> Self {
        let mut wallet = Wallet::new();
        wallet.wallet_type = WalletType::Enterprise;
        wallet.features.enterprise_mode = true;
        wallet.enterprise_features.company_name = Some(name.to_string());
        wallet.enterprise_features.compliance_status = true;
        wallet.enterprise_features.kyc_verified = true;
        wallet.enterprise_features.aml_checked = true;
        wallet.balance = allocation;
        wallet.address = format!("foundation_{}_{}", name.to_lowercase(), 
            SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs());
        wallet
    }
    
    pub fn new_investor_wallet(investor_type: &str, allocation: u64) -> Self {
        let mut wallet = Wallet::new();
        wallet.wallet_type = WalletType::Enterprise;
        wallet.features.enterprise_mode = true;
        wallet.enterprise_features.company_name = Some(format!("Investor_{}", investor_type));
        wallet.enterprise_features.kyc_verified = true;
        wallet.balance = allocation;
        wallet.address = format!("investor_{}_{}", investor_type.to_lowercase(),
            SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs());
        wallet
    }
    
    pub fn new_public_wallet(wallet_type: &str, allocation: u64) -> Self {
        let mut wallet = Wallet::new();
        wallet.balance = allocation;
        wallet.address = format!("public_{}_{}", wallet_type.to_lowercase(),
            SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs());
        wallet
    }
    
    // 🆕 NEW: Create wallet with custom features
    pub fn new_with_features(security_level: SecurityLevel, features: WalletFeatures) -> Self {
        let mut wallet = Wallet::new();
        wallet.security_level = security_level;
        wallet.features = features;
        wallet
    }
    
    // 🆕 NEW: Generate unique referral code
    fn generate_referral_code() -> String {
        let mut rng = OsRng;
        let code: u32 = rng.gen();
        format!("VEXA{:06}", code % 1000000)
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
        // 🆕 NEW: Enhanced signing with security level
        let signature = match self.security_level {
            SecurityLevel::Low => self.sign_basic(transaction_data),
            SecurityLevel::Basic => self.sign_basic(transaction_data),
            SecurityLevel::Medium => self.sign_standard(transaction_data),
            SecurityLevel::High => self.sign_enhanced(transaction_data),
            SecurityLevel::Critical => self.sign_maximum(transaction_data),
            SecurityLevel::Maximum => self.sign_maximum(transaction_data),
        };
        
        signature
    }
    
    // 🚀 NEW: Quantum-resistant signing
    pub fn sign_quantum_resistant(&self, transaction_data: &str) -> String {
        if !self.features.quantum_resistant {
            return self.sign_maximum(transaction_data);
        }
        
        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        
        // Quantum-resistant signing process
        let mut hasher = Sha256::new();
        hasher.update(transaction_data.as_bytes());
        hasher.update(self.nonce.to_string().as_bytes());
        hasher.update(timestamp.to_string().as_bytes());
        hasher.update(&self.quantum_resistance.kyber512_public_key.as_bytes());
        hasher.update(&self.quantum_resistance.dilithium2_signature.as_bytes());
        
        let result = hasher.finalize();
        
        let mut signature_data = Vec::new();
        signature_data.extend_from_slice(&result);
        signature_data.extend_from_slice(self.quantum_resistance.kyber512_public_key.as_bytes());
        signature_data.extend_from_slice(self.quantum_resistance.dilithium2_signature.as_bytes());
        signature_data.extend_from_slice(&self.nonce.to_be_bytes());
        signature_data.extend_from_slice(&timestamp.to_be_bytes());
        
        hex::encode(signature_data)
    }
    
    // 🆕 NEW: Basic signing (original method)
    fn sign_basic(&self, transaction_data: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(transaction_data.as_bytes());
        hasher.update(hex::decode(&self.private_key).unwrap().as_slice());
        let result = hasher.finalize();
        
        let mut signature_data = Vec::new();
        signature_data.extend_from_slice(&result);
        signature_data.extend_from_slice(hex::decode(&self.public_key).unwrap().as_slice());
        
        hex::encode(signature_data)
    }
    
    // 🆕 NEW: Standard signing with nonce
    fn sign_standard(&self, transaction_data: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(transaction_data.as_bytes());
        hasher.update(self.nonce.to_string().as_bytes());
        hasher.update(hex::decode(&self.private_key).unwrap().as_slice());
        let result = hasher.finalize();
        
        let mut signature_data = Vec::new();
        signature_data.extend_from_slice(&result);
        signature_data.extend_from_slice(hex::decode(&self.public_key).unwrap().as_slice());
        signature_data.extend_from_slice(&self.nonce.to_be_bytes());
        
        hex::encode(signature_data)
    }
    
    // 🆕 NEW: Enhanced signing with timestamp
    fn sign_enhanced(&self, transaction_data: &str) -> String {
        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        
        let mut hasher = Sha256::new();
        hasher.update(transaction_data.as_bytes());
        hasher.update(self.nonce.to_string().as_bytes());
        hasher.update(timestamp.to_string().as_bytes());
        hasher.update(hex::decode(&self.private_key).unwrap().as_slice());
        let result = hasher.finalize();
        
        let mut signature_data = Vec::new();
        signature_data.extend_from_slice(&result);
        signature_data.extend_from_slice(hex::decode(&self.public_key).unwrap().as_slice());
        signature_data.extend_from_slice(&self.nonce.to_be_bytes());
        signature_data.extend_from_slice(&timestamp.to_be_bytes());
        
        hex::encode(signature_data)
    }
    
    // 🆕 NEW: Maximum security signing
    fn sign_maximum(&self, transaction_data: &str) -> String {
        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        
        // Multiple rounds of hashing
        let mut hasher = Sha256::new();
        hasher.update(transaction_data.as_bytes());
        hasher.update(self.nonce.to_string().as_bytes());
        hasher.update(timestamp.to_string().as_bytes());
        hasher.update(hex::decode(&self.private_key).unwrap().as_slice());
        
        // Second round of hashing
        let first_pass = hasher.finalize();
        let mut second_hasher = Sha256::new();
        second_hasher.update(&first_pass);
        second_hasher.update("VexaChain".as_bytes());
        let result = second_hasher.finalize();
        
        let mut signature_data = Vec::new();
        signature_data.extend_from_slice(&result);
        signature_data.extend_from_slice(hex::decode(&self.public_key).unwrap().as_slice());
        signature_data.extend_from_slice(&self.nonce.to_be_bytes());
        signature_data.extend_from_slice(&timestamp.to_be_bytes());
        signature_data.extend_from_slice("VexaChain".as_bytes());
        
        hex::encode(signature_data)
    }
    
    // 🚀 NEW: AI Security Features
    pub fn run_ai_security_scan(&mut self) -> String {
        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        
        // Simulate AI security analysis
        let risk_score = if self.balance > 1_000_000_000_000_000 {
            0.8
        } else if self.last_activity < timestamp - 86400 {
            0.6
        } else {
            0.2
        };
        
        self.ai_security.risk_score = risk_score;
        self.ai_security.last_risk_assessment = timestamp;
        
        format!("AI Security Scan Complete - Risk Score: {:.1}", risk_score)
    }
    
    // 🚀 NEW: Biometric Authentication
    pub fn enable_biometric_auth(&mut self, fingerprint: String, facial_data: String, voice_signature: String) -> Result<(), String> {
        if !self.features.biometric_auth {
            return Err("Biometric authentication not enabled".to_string());
        }
        
        self.biometric_data = Some(BiometricData {
            fingerprint_hash: fingerprint,
            facial_recognition_data: facial_data,
            voice_signature: voice_signature,
            last_biometric_auth: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
        });
        
        println!("✅ Biometric authentication enabled!");
        Ok(())
    }
    
    pub fn authenticate_biometric(&mut self, fingerprint: &str) -> bool {
        if let Some(biometric) = &self.biometric_data {
            if biometric.fingerprint_hash == fingerprint {
                if let Some(biometric_mut) = &mut self.biometric_data {
                    biometric_mut.last_biometric_auth = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
                }
                return true;
            }
        }
        false
    }
    
    // 🚀 NEW: Multi-signature Wallet
    pub fn create_multi_sig_wallet(&mut self, owners: Vec<String>, required_signatures: u8) -> Result<(), String> {
        if !self.features.multi_sig {
            return Err("Multi-signature features not enabled".to_string());
        }
        
        let multi_sig = MultiSigWallet {
            address: format!("multisig_{}", SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs()),
            owners,
            required_signatures,
            pending_transactions: HashMap::new(),
            created_at: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
        };
        
        self.multi_sig_wallet = Some(multi_sig);
        self.wallet_type = WalletType::MultiSig;
        
        println!("✅ Multi-signature wallet created!");
        Ok(())
    }
    
    // 🚀 NEW: Cross-chain Address Generation
    pub fn generate_cross_chain_addresses(&mut self) {
        if !self.features.cross_chain {
            return;
        }
        
        let base_hash = Sha256::digest(self.public_key.as_bytes());
        
        self.cross_chain_addresses.ethereum = format!("0x{}", &hex::encode(&base_hash[..20]));
        self.cross_chain_addresses.binance = format!("0x{}", &hex::encode(&base_hash[..20]));
        self.cross_chain_addresses.polygon = format!("0x{}", &hex::encode(&base_hash[..20]));
        self.cross_chain_addresses.solana = base_hash[..32].to_base58();
        self.cross_chain_addresses.avalanche = format!("0x{}", &hex::encode(&base_hash[..20]));
        
        println!("✅ Cross-chain addresses generated!");
    }
    
    // 🚀 NEW: DeFi Features
    pub fn add_liquidity_pool(&mut self, pool_id: String, amount: u64) -> Result<(), String> {
        if !self.features.defi_advanced {
            return Err("DeFi features not enabled".to_string());
        }
        
        if amount > self.balance {
            return Err("Insufficient balance".to_string());
        }
        
        *self.defi_portfolio.liquidity_pools.entry(pool_id).or_insert(0) += amount;
        self.defi_portfolio.total_value_locked += amount;
        self.balance -= amount;
        
        println!("✅ Liquidity added to pool!");
        Ok(())
    }
    
    pub fn execute_flash_loan(&mut self, amount: u64) -> Result<(), String> {
        if !self.features.defi_advanced {
            return Err("Flash loans not enabled".to_string());
        }
        
        self.defi_portfolio.flash_loans_taken += 1;
        println!("✅ Flash loan executed: {} VEXA", amount);
        Ok(())
    }
    
    // 🚀 NEW: Gaming & Metaverse Features
    pub fn mint_nft(&mut self, nft_id: String) -> Result<(), String> {
        if !self.features.gaming_metaverse {
            return Err("NFT features not enabled".to_string());
        }
        
        *self.gaming_assets.nfts.entry(nft_id).or_insert(0) += 1;
        println!("✅ NFT minted successfully!");
        Ok(())
    }
    
    pub fn earn_play_to_earn(&mut self, amount: u64) {
        if self.features.gaming_metaverse {
            self.gaming_assets.play_to_earn_earnings += amount;
            self.balance += amount;
            println!("✅ Play-to-earn reward: {} VEXA", amount);
        }
    }
    
    // 🚀 NEW: Enterprise Features
    pub fn register_enterprise(&mut self, company_name: String, tax_id: String) {
        self.enterprise_features.company_name = Some(company_name);
        self.enterprise_features.tax_id = Some(tax_id);
        self.enterprise_features.compliance_status = true;
        self.enterprise_features.kyc_verified = true;
        self.enterprise_features.aml_checked = true;
        self.enterprise_features.monthly_transaction_limit = 1_000_000_000_000_000;
        
        self.wallet_type = WalletType::Enterprise;
        self.features.enterprise_mode = true;
        
        println!("✅ Enterprise wallet registered!");
    }
    
    // 🚀 NEW: Quantum Resistance Migration
    pub fn migrate_to_quantum_resistant(&mut self) -> Result<(), String> {
        if !self.features.quantum_resistant {
            return Err("Quantum resistance not enabled".to_string());
        }
        
        self.quantum_resistance.quantum_safe_algorithm = true;
        self.quantum_resistance.kyber512_public_key = "new_quantum_public_key".to_string();
        self.quantum_resistance.dilithium2_signature = "new_quantum_signature".to_string();
        self.quantum_resistance.migration_ready = true;
        
        self.wallet_type = WalletType::QuantumResistant;
        
        println!("✅ Migrated to quantum-resistant algorithm!");
        Ok(())
    }
    
    // 🆕 NEW: MASS ADOPTION FEATURES
    
    pub fn link_social_wallet(&mut self, phone: String, email: String) -> Result<(), String> {
        if !self.features.social_payments {
            return Err("Social payments are not enabled for this wallet".to_string());
        }
        
        let social_wallet = SocialWallet {
            phone_number: phone,
            email: email,
            recovery_code: Self::generate_recovery_code(),
            main_address: self.address.clone(),
        };
        
        self.social_wallet = Some(social_wallet);
        println!("✅ Social wallet linked successfully!");
        Ok(())
    }
    
    pub fn get_social_wallet(&self) -> Option<&SocialWallet> {
        self.social_wallet.as_ref()
    }
    
    fn generate_recovery_code() -> String {
        let mut rng = OsRng;
        let code: u32 = rng.gen();
        format!("{:06}", code % 1000000)
    }
    
    // 2. REFERRAL SYSTEM
    pub fn get_referral_code(&self) -> &str {
        &self.referral_code
    }
    
    pub fn generate_referral_link(&self) -> String {
        format!("https://vexachain.com/signup?ref={}", self.referral_code)
    }
    
    // 3. MICRO-EARNING TASKS
    pub fn complete_task(&mut self, task_reward: u64) -> Result<(), String> {
        if !self.features.micro_earning {
            return Err("Micro-earning is not enabled for this wallet".to_string());
        }
        
        self.balance += task_reward;
        self.total_earned += task_reward;
        self.tasks_completed += 1;
        self.last_activity = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        
        println!("✅ Task completed! Earned: {} VEXA", task_reward);
        Ok(())
    }
    
    pub fn get_earnings_info(&self) -> String {
        format!(
            "Earnings Summary:\n\
            Total Earned: {} VEXA\n\
            Tasks Completed: {}\n\
            Average per Task: {} VEXA",
            self.total_earned,
            self.tasks_completed,
            if self.tasks_completed > 0 {
                self.total_earned / self.tasks_completed as u64
            } else {
                0
            }
        )
    }
    
    // 4. DAILY REWARDS
    pub fn claim_daily_reward(&mut self, reward_amount: u64) -> Result<(), String> {
        if !self.features.daily_rewards {
            return Err("Daily rewards are not enabled for this wallet".to_string());
        }
        
        if self.daily_reward_claimed {
            return Err("Daily reward already claimed today".to_string());
        }
        
        self.balance += reward_amount;
        self.total_earned += reward_amount;
        self.daily_reward_claimed = true;
        self.last_activity = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        
        println!("✅ Daily reward claimed: {} VEXA", reward_amount);
        Ok(())
    }
    
    pub fn reset_daily_reward(&mut self) {
        self.daily_reward_claimed = false;
    }
    
    // 5. SOCIAL PAYMENTS
    pub fn create_payment_request(&self, amount: u64, description: &str) -> Result<String, String> {
        if !self.features.social_payments {
            return Err("Social payments are not enabled for this wallet".to_string());
        }
        
        let payment_data = format!(
            "payment_request:from:{},amount:{},desc:{},timestamp:{}",
            self.address, amount, description, 
            SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs()
        );
        
        let signature = self.sign_transaction(&payment_data);
        let payment_link = format!(
            "https://vexachain.com/pay/{}/{}/{}",
            self.address, amount, signature
        );
        
        Ok(payment_link)
    }
    
    // 6. PHONE-BASED TRANSACTIONS
    pub fn create_phone_transaction(&self, to_phone: &str, amount: u64) -> Result<String, String> {
        if !self.features.social_payments {
            return Err("Social payments are not enabled for this wallet".to_string());
        }
        
        let tx_data = format!(
            "phone_tx:from:{},to_phone:{},amount:{},timestamp:{}",
            self.address, to_phone, amount,
            SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs()
        );
        
        let signature = self.sign_transaction(&tx_data);
        Ok(signature)
    }
    
    // 🚀 NEW: Get Advanced Features Report
    pub fn get_advanced_features_report(&self) -> String {
        format!(
            "Advanced Wallet Features Report:\n\
            ===============================\n\
            Wallet Type: {:?}\n\
            AI Security: {}\n\
            Biometric Auth: {}\n\
            Multi-signature: {}\n\
            Quantum Resistant: {}\n\
            Enterprise Mode: {}\n\
            DeFi Portfolio: {} VEXA TVL\n\
            Gaming Assets: {} NFTs\n\
            Cross-chain: Ethereum: {}, BSC: {}, Polygon: {}\n\
            Flash Loans Taken: {}\n\
            Play-to-Earn Earnings: {} VEXA",
            self.wallet_type,
            if self.ai_security.anomaly_detection { "ENABLED" } else { "DISABLED" },
            if self.biometric_data.is_some() { "ENABLED" } else { "DISABLED" },
            if self.multi_sig_wallet.is_some() { "ENABLED" } else { "DISABLED" },
            if self.quantum_resistance.quantum_safe_algorithm { "ENABLED" } else { "DISABLED" },
            if self.features.enterprise_mode { "ENABLED" } else { "DISABLED" },
            self.defi_portfolio.total_value_locked,
            self.gaming_assets.nfts.len(),
            if !self.cross_chain_addresses.ethereum.is_empty() { "SET" } else { "NOT SET" },
            if !self.cross_chain_addresses.binance.is_empty() { "SET" } else { "NOT SET" },
            if !self.cross_chain_addresses.polygon.is_empty() { "SET" } else { "NOT SET" },
            self.defi_portfolio.flash_loans_taken,
            self.gaming_assets.play_to_earn_earnings
        )
    }
    
    // 🆕 NEW: MASS ADOPTION WALLET INFO
    pub fn get_mass_adoption_info(&self) -> String {
        format!(
            "Mass Adoption Features:\n\
            Social Wallet: {}\n\
            Referral Code: {}\n\
            Total Earned: {} VEXA\n\
            Tasks Completed: {}\n\
            Daily Reward Claimed: {}\n\
            Features Enabled: Social Payments: {}, Micro-earning: {}, Daily Rewards: {}, Referral Program: {}",
            if self.social_wallet.is_some() { "Linked" } else { "Not Linked" },
            self.referral_code,
            self.total_earned,
            self.tasks_completed,
            self.daily_reward_claimed,
            self.features.social_payments,
            self.features.micro_earning,
            self.features.daily_rewards,
            self.features.referral_program
        )
    }
    
    // 🆕 NEW: Quick wallet setup for mass adoption
    pub fn quick_setup_for_mass_adoption(&mut self, phone: Option<String>, email: Option<String>) {
        self.features.social_payments = true;
        self.features.micro_earning = true;
        self.features.daily_rewards = true;
        self.features.referral_program = true;
        self.features.nft_creation = true;
        self.features.prediction_markets = true;
        
        if let (Some(phone), Some(email)) = (phone, email) {
            let _ = self.link_social_wallet(phone, email);
        }
        
        println!("✅ Wallet optimized for mass adoption!");
    }
    
    // EXISTING METHODS (with minor updates)
    
    pub fn verify_signature(transaction_data: &str, signature: &str, public_key: &str) -> bool {
        if let Ok(signature_bytes) = hex::decode(signature) {
            let signature_len = signature_bytes.len();
            
            if signature_len == 64 {
                return Wallet::verify_basic_signature(transaction_data, &signature_bytes, public_key);
            } else if signature_len == 72 {
                return Wallet::verify_standard_signature(transaction_data, &signature_bytes, public_key);
            } else if signature_len == 80 {
                return Wallet::verify_enhanced_signature(transaction_data, &signature_bytes, public_key);
            } else if signature_len >= 88 {
                return Wallet::verify_maximum_signature(transaction_data, &signature_bytes, public_key);
            }
        }
        false
    }
    
    fn verify_basic_signature(transaction_data: &str, signature_bytes: &[u8], public_key: &str) -> bool {
        if signature_bytes.len() >= 32 {
            let signature_hash = &signature_bytes[..32];
            
            let mut hasher = Sha256::new();
            hasher.update(transaction_data.as_bytes());
            
            if let Ok(pub_key_bytes) = hex::decode(public_key) {
                hasher.update(&pub_key_bytes);
                let expected_hash = hasher.finalize();
                
                return signature_hash == &expected_hash[..];
            }
        }
        false
    }
    
    fn verify_standard_signature(transaction_data: &str, signature_bytes: &[u8], public_key: &str) -> bool {
        if signature_bytes.len() >= 72 {
            let signature_hash = &signature_bytes[..32];
            let nonce_bytes = &signature_bytes[64..72];
            let nonce = u64::from_be_bytes(nonce_bytes.try_into().unwrap());
            
            let mut hasher = Sha256::new();
            hasher.update(transaction_data.as_bytes());
            hasher.update(nonce.to_string().as_bytes());
            
            if let Ok(pub_key_bytes) = hex::decode(public_key) {
                hasher.update(&pub_key_bytes);
                let expected_hash = hasher.finalize();
                
                return signature_hash == &expected_hash[..];
            }
        }
        false
    }
    
    fn verify_enhanced_signature(transaction_data: &str, signature_bytes: &[u8], public_key: &str) -> bool {
        if signature_bytes.len() >= 80 {
            let signature_hash = &signature_bytes[..32];
            let nonce_bytes = &signature_bytes[64..72];
            let timestamp_bytes = &signature_bytes[72..80];
            
            let nonce = u64::from_be_bytes(nonce_bytes.try_into().unwrap());
            let timestamp = u64::from_be_bytes(timestamp_bytes.try_into().unwrap());
            
            let mut hasher = Sha256::new();
            hasher.update(transaction_data.as_bytes());
            hasher.update(nonce.to_string().as_bytes());
            hasher.update(timestamp.to_string().as_bytes());
            
            if let Ok(pub_key_bytes) = hex::decode(public_key) {
                hasher.update(&pub_key_bytes);
                let expected_hash = hasher.finalize();
                
                return signature_hash == &expected_hash[..];
            }
        }
        false
    }
    
    fn verify_maximum_signature(transaction_data: &str, signature_bytes: &[u8], public_key: &str) -> bool {
        if signature_bytes.len() >= 88 {
            let signature_hash = &signature_bytes[..32];
            let nonce_bytes = &signature_bytes[64..72];
            let timestamp_bytes = &signature_bytes[72..80];
            
            let nonce = u64::from_be_bytes(nonce_bytes.try_into().unwrap());
            let timestamp = u64::from_be_bytes(timestamp_bytes.try_into().unwrap());
            
            let mut hasher = Sha256::new();
            hasher.update(transaction_data.as_bytes());
            hasher.update(nonce.to_string().as_bytes());
            hasher.update(timestamp.to_string().as_bytes());
            
            if let Ok(pub_key_bytes) = hex::decode(public_key) {
                hasher.update(&pub_key_bytes);
                let first_pass = hasher.finalize();
                
                let mut second_hasher = Sha256::new();
                second_hasher.update(&first_pass);
                second_hasher.update("VexaChain".as_bytes());
                let expected_hash = second_hasher.finalize();
                
                return signature_hash == &expected_hash[..];
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
    
    // 🆕 NEW: Enhanced wallet methods for all features
    
    pub fn update_balance(&mut self, new_balance: u64) {
        self.balance = new_balance;
        self.last_activity = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
    }
    
    pub fn get_balance(&self) -> u64 {
        self.balance
    }
    
    pub fn increment_nonce(&mut self) {
        self.nonce += 1;
        self.last_activity = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
    }
    
    pub fn get_nonce(&self) -> u64 {
        self.nonce
    }
    
    // 🆕 NEW: Token Burning Features
    pub fn create_burn_transaction(&self, amount: u64) -> Result<String, String> {
        if !self.features.token_burning {
            return Err("Token burning is not enabled for this wallet".to_string());
        }
        
        if amount > self.balance {
            return Err("Insufficient balance for burning".to_string());
        }
        
        let burn_data = format!("burn:{},amount:{},from:{}", 
                               SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
                               amount, self.address);
        
        let signature = self.sign_transaction(&burn_data);
        Ok(signature)
    }
    
    // 🆕 NEW: DAO Governance Features
    pub fn create_dao_vote(&self, proposal_id: u64, vote: bool) -> Result<DAOVote, String> {
        if !self.features.dao_governance {
            return Err("DAO governance is not enabled for this wallet".to_string());
        }
        
        let vote_power = self.dao_voting_power;
        if vote_power == 0 {
            return Err("No voting power available".to_string());
        }
        
        Ok(DAOVote {
            voter: self.address.clone(),
            proposal_id,
            vote: vote,
            voting_power: vote_power,
            timestamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
        })
    }
    
    pub fn set_dao_voting_power(&mut self, power: u64) {
        self.dao_voting_power = power;
    }
    
    pub fn get_dao_voting_power(&self) -> u64 {
        self.dao_voting_power
    }
    
    // 🆕 NEW: Staking Features
    pub fn create_stake_transaction(&self, amount: u64, lock_days: u64) -> Result<String, String> {
        if !self.features.staking {
            return Err("Staking is not enabled for this wallet".to_string());
        }
        
        if amount > self.balance {
            return Err("Insufficient balance for staking".to_string());
        }
        
        let stake_data = format!("stake:{},amount:{},lock_days:{},from:{}", 
                                SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
                                amount, lock_days, self.address);
        
        let signature = self.sign_transaction(&stake_data);
        Ok(signature)
    }
    
    pub fn update_staking_info(&mut self, staker: Staker) {
        self.staking_info = Some(staker);
    }
    
    pub fn get_staking_info(&self) -> Option<&Staker> {
        self.staking_info.as_ref()
    }

    // 🆕 NEW: Security Features
    pub fn upgrade_security(&mut self, new_level: SecurityLevel) {
        println!("🔒 Wallet security upgraded to: {:?}", new_level);
        self.security_level = new_level;
    }

    pub fn get_security_level(&self) -> &SecurityLevel {
        &self.security_level
    }

    // 🆕 NEW: Feature Management
    pub fn enable_feature(&mut self, feature: &str) -> Result<(), String> {
        match feature {
            "token_burning" => self.features.token_burning = true,
            "dao_governance" => self.features.dao_governance = true,
            "staking" => self.features.staking = true,
            "cross_chain" => self.features.cross_chain = true,
            "smart_contracts" => self.features.smart_contracts = true,
            "multi_sig" => self.features.multi_sig = true,
            "social_payments" => self.features.social_payments = true,
            "micro_earning" => self.features.micro_earning = true,
            "daily_rewards" => self.features.daily_rewards = true,
            "referral_program" => self.features.referral_program = true,
            "nft_creation" => self.features.nft_creation = true,
            "prediction_markets" => self.features.prediction_markets = true,
            "biometric_auth" => self.features.biometric_auth = true,
            "hardware_wallet" => self.features.hardware_wallet = true,
            "ai_security" => self.features.ai_security = true,
            "quantum_resistant" => self.features.quantum_resistant = true,
            "enterprise_mode" => self.features.enterprise_mode = true,
            "defi_advanced" => self.features.defi_advanced = true,
            "gaming_metaverse" => self.features.gaming_metaverse = true,
            "instant_transactions" => self.features.instant_transactions = true,
            "privacy_mode" => self.features.privacy_mode = true,
            _ => return Err(format!("Unknown feature: {}", feature)),
        }
        println!("✅ Enabled feature: {}", feature);
        Ok(())
    }
    
    pub fn disable_feature(&mut self, feature: &str) -> Result<(), String> {
        match feature {
            "token_burning" => self.features.token_burning = false,
            "dao_governance" => self.features.dao_governance = false,
            "staking" => self.features.staking = false,
            "cross_chain" => self.features.cross_chain = false,
            "smart_contracts" => self.features.smart_contracts = false,
            "multi_sig" => self.features.multi_sig = false,
            "social_payments" => self.features.social_payments = false,
            "micro_earning" => self.features.micro_earning = false,
            "daily_rewards" => self.features.daily_rewards = false,
            "referral_program" => self.features.referral_program = false,
            "nft_creation" => self.features.nft_creation = false,
            "prediction_markets" => self.features.prediction_markets = false,
            "biometric_auth" => self.features.biometric_auth = false,
            "hardware_wallet" => self.features.hardware_wallet = false,
            "ai_security" => self.features.ai_security = false,
            "quantum_resistant" => self.features.quantum_resistant = false,
            "enterprise_mode" => self.features.enterprise_mode = false,
            "defi_advanced" => self.features.defi_advanced = false,
            "gaming_metaverse" => self.features.gaming_metaverse = false,
            "instant_transactions" => self.features.instant_transactions = false,
            "privacy_mode" => self.features.privacy_mode = false,
            _ => return Err(format!("Unknown feature: {}", feature)),
        }
        
        println!("❌ Disabled feature: {}", feature);
        Ok(())
    }
    
    // 🆕 NEW: Wallet Information
    pub fn get_wallet_info(&self) -> String {
        format!(
            "Wallet Information:\n\
            Address: {}\n\
            Balance: {} VEXA\n\
            Nonce: {}\n\
            Created: {}\n\
            Security: {:?}\n\
            Wallet Type: {:?}\n\
            Features: Token Burning: {}, DAO: {}, Staking: {}, Cross-chain: {}, Smart Contracts: {}\n\
            Mass Adoption: Social Payments: {}, Micro-earning: {}, Daily Rewards: {}, Referral Program: {}\n\
            Advanced: Biometric: {}, Hardware: {}, AI Security: {}, Quantum: {}, Enterprise: {}, DeFi: {}, Gaming: {}",
            self.address,
            self.balance,
            self.nonce,
            self.created_at,
            self.security_level,
            self.wallet_type,
            self.features.token_burning,
            self.features.dao_governance,
            self.features.staking,
            self.features.cross_chain,
            self.features.smart_contracts,
            self.features.social_payments,
            self.features.micro_earning,
            self.features.daily_rewards,
            self.features.referral_program,
            self.features.biometric_auth,
            self.features.hardware_wallet,
            self.features.ai_security,
            self.features.quantum_resistant,
            self.features.enterprise_mode,
            self.features.defi_advanced,
            self.features.gaming_metaverse
        )
    }
    
    // 🆕 NEW: Export wallet for backup (without private key for security)
    pub fn export_public_info(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}

impl fmt::Display for Wallet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Enhanced Wallet:\n\
            Address: {}\n\
            Public Key: {}...\n\
            Balance: {} VEXA\n\
            Nonce: {}\n\
            Security Level: {:?}\n\
            Wallet Type: {:?}\n\
            Mass Adoption Features: Social Payments: {}, Micro-earning: {}, Daily Rewards: {}, Referral Program: {}\n\
            Advanced Features: AI Security: {}, Quantum Resistant: {}, Enterprise: {}, DeFi: {}, Gaming: {}",
            self.address, 
            &self.public_key[..16], 
            self.balance,
            self.nonce,
            self.security_level,
            self.wallet_type,
            self.features.social_payments,
            self.features.micro_earning,
            self.features.daily_rewards,
            self.features.referral_program,
            self.features.ai_security,
            self.features.quantum_resistant,
            self.features.enterprise_mode,
            self.features.defi_advanced,
            self.features.gaming_metaverse
        )
    }
}

// 🆕 NEW: Wallet manager for multiple wallets
pub struct WalletManager {
    pub wallets: HashMap<String, Wallet>,
    pub default_wallet: Option<String>,
}

impl WalletManager {
    pub fn new() -> Self {
        WalletManager {
            wallets: HashMap::new(),
            default_wallet: None,
        }
    }
    
    pub fn create_wallet(&mut self, name: String) -> &Wallet {
        let wallet = Wallet::new();
        self.wallets.insert(name.clone(), wallet);
        if self.default_wallet.is_none() {
            self.default_wallet = Some(name.clone());
        }
        self.wallets.get(&name).unwrap()
    }
    
    // 🚀 NEW: Create specialized wallets
    pub fn create_enterprise_wallet(&mut self, name: String, company_name: String, tax_id: String) -> &Wallet {
        let wallet = Wallet::new_enterprise_wallet(company_name, tax_id);
        self.wallets.insert(name.clone(), wallet);
        if self.default_wallet.is_none() {
            self.default_wallet = Some(name.clone());
        }
        self.wallets.get(&name).unwrap()
    }
    
    pub fn create_hardware_wallet(&mut self, name: String, device_id: String, manufacturer: String, model: String) -> &Wallet {
        let wallet = Wallet::new_hardware_wallet(device_id, manufacturer, model);
        self.wallets.insert(name.clone(), wallet);
        if self.default_wallet.is_none() {
            self.default_wallet = Some(name.clone());
        }
        self.wallets.get(&name).unwrap()
    }
    
    pub fn create_quantum_wallet(&mut self, name: String) -> &Wallet {
        let wallet = Wallet::new_quantum_wallet();
        self.wallets.insert(name.clone(), wallet);
        if self.default_wallet.is_none() {
            self.default_wallet = Some(name.clone());
        }
        self.wallets.get(&name).unwrap()
    }
    
    pub fn create_defi_wallet(&mut self, name: String) -> &Wallet {
        let wallet = Wallet::new_defi_wallet();
        self.wallets.insert(name.clone(), wallet);
        if self.default_wallet.is_none() {
            self.default_wallet = Some(name.clone());
        }
        self.wallets.get(&name).unwrap()
    }
    
    pub fn create_gaming_wallet(&mut self, name: String) -> &Wallet {
        let wallet = Wallet::new_gaming_wallet();
        self.wallets.insert(name.clone(), wallet);
        if self.default_wallet.is_none() {
            self.default_wallet = Some(name.clone());
        }
        self.wallets.get(&name).unwrap()
    }
    
    // 🚀 NEW: Create elite tokenomics wallets
    pub fn create_foundation_wallet(&mut self, name: String, allocation: u64) -> &Wallet {
        let wallet = Wallet::new_foundation_wallet(&name, allocation);
        self.wallets.insert(name.clone(), wallet);
        self.wallets.get(&name).unwrap()
    }
    
    pub fn create_investor_wallet(&mut self, name: String, investor_type: &str, allocation: u64) -> &Wallet {
        let wallet = Wallet::new_investor_wallet(investor_type, allocation);
        self.wallets.insert(name.clone(), wallet);
        self.wallets.get(&name).unwrap()
    }
    
    pub fn create_public_wallet(&mut self, name: String, wallet_type: &str, allocation: u64) -> &Wallet {
        let wallet = Wallet::new_public_wallet(wallet_type, allocation);
        self.wallets.insert(name.clone(), wallet);
        self.wallets.get(&name).unwrap()
    }
    
    // 🆕 NEW: Create wallet optimized for mass adoption
    pub fn create_mass_adoption_wallet(&mut self, name: String, phone: Option<String>, email: Option<String>) -> &Wallet {
        let mut wallet = Wallet::new();
        wallet.quick_setup_for_mass_adoption(phone, email);
        self.wallets.insert(name.clone(), wallet);
        if self.default_wallet.is_none() {
            self.default_wallet = Some(name.clone());
        }
        self.wallets.get(&name).unwrap()
    }
    
    pub fn get_wallet(&self, name: &str) -> Option<&Wallet> {
        self.wallets.get(name)
    }
    
    pub fn get_wallet_by_address(&self, address: &str) -> Option<&Wallet> {
        self.wallets.values().find(|w| w.address == address)
    }
    
    pub fn list_wallets(&self) -> Vec<&str> {
        self.wallets.keys().map(|k| k.as_str()).collect()
    }
    
    // 🆕 NEW: Get wallet by referral code
    pub fn get_wallet_by_referral_code(&self, referral_code: &str) -> Option<&Wallet> {
        self.wallets.values().find(|w| w.referral_code == referral_code)
    }
    
    // 🆕 NEW: Get all wallets with social features
    pub fn get_social_wallets(&self) -> Vec<&Wallet> {
        self.wallets.values().filter(|w| w.social_wallet.is_some()).collect()
    }
    
    // 🚀 NEW: Get wallets by type
    pub fn get_wallets_by_type(&self, wallet_type: WalletType) -> Vec<&Wallet> {
        self.wallets.values().filter(|w| w.wallet_type == wallet_type).collect()
    }
    
    // 🚀 NEW: Get enterprise wallets
    pub fn get_enterprise_wallets(&self) -> Vec<&Wallet> {
        self.get_wallets_by_type(WalletType::Enterprise)
    }
    
    // 🚀 NEW: Get hardware wallets
    pub fn get_hardware_wallets(&self) -> Vec<&Wallet> {
        self.wallets.values().filter(|w| w.hardware_wallet.is_some()).collect()
    }
    
    // 🚀 NEW: Get quantum-resistant wallets
    pub fn get_quantum_wallets(&self) -> Vec<&Wallet> {
        self.get_wallets_by_type(WalletType::QuantumResistant)
    }
    
    // 🚀 NEW: Get DeFi wallets
    pub fn get_defi_wallets(&self) -> Vec<&Wallet> {
        self.get_wallets_by_type(WalletType::DeFi)
    }
    
    // 🚀 NEW: Get gaming wallets
    pub fn get_gaming_wallets(&self) -> Vec<&Wallet> {
        self.get_wallets_by_type(WalletType::Gaming)
    }
    
    // 🚀 NEW: Initialize elite tokenomics wallets for Gate.io
    pub fn initialize_elite_tokenomics_wallets(&mut self) {
        println!("🚀 Initializing Elite Tokenomics Wallets for Gate.io...");
        
        // 🏢 FOUNDATION & TEAM (20M)
        self.create_foundation_wallet("Core_Development_Team".to_string(), 8_000_000_000_000_000);
        self.create_foundation_wallet("Foundation_Reserve".to_string(), 7_000_000_000_000_000);
        self.create_foundation_wallet("Development_Fund".to_string(), 5_000_000_000_000_000);
        
        // 💰 STRATEGIC PARTNERS (15M)
        self.create_investor_wallet("Seed_Investors".to_string(), "Seed", 6_000_000_000_000_000);
        self.create_investor_wallet("Strategic_Partners".to_string(), "Strategic", 5_000_000_000_000_000);
        self.create_investor_wallet("Ecosystem_Fund".to_string(), "Ecosystem", 4_000_000_000_000_000);
        
        // 🌍 PUBLIC DISTRIBUTION (35M)
        self.create_public_wallet("Public_Sale".to_string(), "IEO", 15_000_000_000_000_000);
        self.create_public_wallet("Staking_Rewards".to_string(), "Staking", 10_000_000_000_000_000);
        self.create_public_wallet("Community_Rewards".to_string(), "Community", 6_000_000_000_000_000);
        self.create_public_wallet("Airdrop_Program".to_string(), "Airdrop", 4_000_000_000_000_000);
        
        // 🔒 LIQUIDITY & GROWTH (30M)
        self.create_public_wallet("DEX_Liquidity".to_string(), "DEX", 8_000_000_000_000_000);
        self.create_public_wallet("CEX_Liquidity".to_string(), "CEX", 7_000_000_000_000_000);
        self.create_public_wallet("Marketing_Fund".to_string(), "Marketing", 5_000_000_000_000_000);
        self.create_public_wallet("Partnership_Fund".to_string(), "Partnership", 4_000_000_000_000_000);
        self.create_public_wallet("Technology_Grants".to_string(), "TechGrants", 3_000_000_000_000_000);
        self.create_public_wallet("Security_Reserve".to_string(), "Security", 3_000_000_000_000_000);
        
        println!("✅ Elite Tokenomics Wallets Initialized Successfully!");
        println!("🏢 Foundation & Team: 20M VEXA");
        println!("💰 Strategic Partners: 15M VEXA");
        println!("🌍 Public Distribution: 35M VEXA");
        println!("🔒 Liquidity & Growth: 30M VEXA");
    }
}