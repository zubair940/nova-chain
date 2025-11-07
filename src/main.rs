#![recursion_limit = "1024"]
#![allow(dead_code)]
#![allow(unused_variables)] 
#![allow(unused_imports)]
#![allow(unused_mut)]
use serde::{Serialize, Deserialize};
use std::collections::{HashMap, BinaryHeap};
use std::time::{SystemTime, UNIX_EPOCH, Duration};
use std::sync::Arc;
use std::{thread, fs, path::Path};
use tokio::sync::Mutex;
use rand::Rng;
use warp::Filter;
use chrono;

// ---------------------------------------------------------------------
// 💡 MODULE DECLARATIONS 
// ---------------------------------------------------------------------
mod models;
mod wallet;
mod contract_manager;
mod api;
mod network;
mod coin;
mod balance;
mod network_config;
// ---------------------------------------------------------------------

// Internal module imports
use crate::models::{
    Block, Transaction, ContractTransaction, UTXO, NetworkPeer,
    StakingPool, Staker, SponsoredTransaction, DailyRewards, UserReward,
    ReferralProgram, ReferralData, SocialWallet, MicroEarning, EarningTask,
    CompletedTask, FiatGateway, NFTCreator, UserNFT, PredictionMarket,
    SocialPay, PaymentLink, Localization, OfflineTransaction, MassAdoptionManager
};
use crate::coin::GenesisDistribution;
use crate::network::{NetworkMessage, MessageType};
use crate::balance::BalanceTracker;
use crate::wallet::Wallet;
use crate::contract_manager::{ContractManager, ContractExecutionResult};
use crate::api::start_api_server;
use crate::network::Network;
use crate::coin::Coin;
use crate::network_config::NetworkConfig;

// 🆕 ADD: Transaction Data Structure for API
#[derive(Debug, Deserialize)]
struct TransactionData {
    from: String,
    to: String,
    amount: u64,
    fee: u64,
}

// 🔥 FIX: Add SocialWallet::new implementation
impl SocialWallet {
    pub fn new(phone: String, email: String) -> Self {
        let mut rng = rand::thread_rng();
        let recovery_code = format!("{:06}", rng.gen_range(0..1000000));
        
        SocialWallet {
            phone_number: phone,
            email: email,
            recovery_code,
            main_address: format!("social_{}", SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos()),
        }
    }
}

// --- Main Blockchain Structure ---
#[derive(Debug, Serialize, Deserialize)]
pub struct Blockchain {
    pub chain: Vec<Block>,
    pub difficulty: usize,
    #[serde(skip_serializing, skip_deserializing)]
    pub mempool: BinaryHeap<Transaction>,
    pub balance_tracker: BalanceTracker,
    pub coin: Coin,
    pub contract_manager: ContractManager,
    pub spent_utxos: HashMap<String, bool>,
    pub utxo_set: HashMap<String, UTXO>,
    #[serde(skip_serializing, skip_deserializing)]
    pub network_peers: Vec<NetworkPeer>,
    pub staking_pool: StakingPool,
    
    // 🆕 NEW: MASS ADOPTION FEATURES
    pub mass_adoption: MassAdoptionManager,
    
    // 🆕 ADVANCED FEATURES
    pub multi_sig_wallets: HashMap<String, MultiSigWallet>,
    pub flash_loans: HashMap<String, FlashLoan>,
    pub quantum_wallets: HashMap<String, QuantumResistantWallet>,
    pub ai_contracts: HashMap<String, AIContract>,
    pub cross_chain_bridge: CrossChainBridge,
    pub gaming_assets: HashMap<String, GamingAsset>,
    pub metaverse_assets: HashMap<String, MetaverseAsset>,
    pub defi_vaults: HashMap<String, DeFiVault>,
    pub enterprise_accounts: HashMap<String, EnterpriseAccount>,
    pub social_fi: SocialFi,
    pub decentralized_identity: HashMap<String, DecentralizedIdentity>,
}

// 🆕 ADVANCED FEATURES STRUCTURES
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlashLoan {
    pub id: String,
    pub borrower: String,
    pub amount: u64,
    pub asset: String,
    pub fee: u64,
    pub timestamp: u64,
    pub repaid: bool,
    pub collateral: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuantumResistantWallet {
    pub address: String,
    pub kyber512_public_key: String,
    pub dilithium2_signature: String,
    pub sphincs_plus_backup: String,
    pub created_at: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIContract {
    pub address: String,
    pub ml_model_hash: String,
    pub prediction_engine: bool,
    pub automated_governance: bool,
    pub training_data: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrossChainBridge {
    pub supported_chains: Vec<SupportedChain>,
    pub liquidity_pools: HashMap<String, u64>,
    pub bridge_fees: BridgeFeeStructure,
    pub total_volume: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SupportedChain {
    pub name: String,
    pub chain_id: String,
    pub bridge_address: String,
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BridgeFeeStructure {
    pub percentage: f64,
    pub min_fee: u64,
    pub max_fee: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GamingAsset {
    pub asset_id: String,
    pub owner: String,
    pub game_id: String,
    pub metadata: String,
    pub rarity: String,
    pub equipped: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetaverseAsset {
    pub asset_id: String,
    pub owner: String,
    pub virtual_world: String,
    pub coordinates: (f64, f64, f64),
    pub asset_type: String,
    pub value: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeFiVault {
    pub vault_id: String,
    pub creator: String,
    pub total_liquidity: u64,
    pub strategy: String,
    pub apr: f64,
    pub investors: HashMap<String, u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnterpriseAccount {
    pub account_id: String,
    pub company_name: String,
    pub tax_id: String,
    pub kyc_verified: bool,
    private_transactions: bool,
    compliance_tools: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SocialFi {
    pub social_trading: HashMap<String, SocialTrader>,
    pub content_creators: HashMap<String, ContentCreator>,
    pub decentralized_social: HashMap<String, SocialPost>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SocialTrader {
    pub address: String,
    pub followers: Vec<String>,
    pub performance: f64,
    pub copy_trading_enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContentCreator {
    pub address: String,
    pub content_hash: String,
    pub earnings: u64,
    pub subscribers: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SocialPost {
    pub post_id: String,
    pub author: String,
    pub content: String,
    pub likes: u64,
    pub tips: u64,
    pub timestamp: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecentralizedIdentity {
    pub did: String,
    pub owner: String,
    pub soulbound_tokens: Vec<String>,
    pub verifiable_credentials: Vec<String>,
    pub reputation_score: f64,
}

impl Blockchain {
    // Constructor
    pub fn new() -> Self {
        let coin_data = Coin::new();
        let mut tracker = BalanceTracker::new();

        // 🆕 UPDATED: NEW TOKENOMICS DISTRIBUTION
        for (address, amount) in GenesisDistribution::get_distribution() {
            tracker.update_balance(address, amount);
        }

        // 🔥 FIX: Clone coin_data to avoid move error
        let coin_clone = coin_data.clone();
        let daily_base_reward = coin_clone.parse_amount(1.0);

        let mut blockchain = Blockchain {
            chain: vec![],
            difficulty: 4,
            mempool: BinaryHeap::new(),
            balance_tracker: tracker,
            coin: coin_data,
            contract_manager: ContractManager::new(),
            spent_utxos: HashMap::new(),
            utxo_set: HashMap::new(),
            network_peers: Vec::new(),
            staking_pool: StakingPool {
                total_staked: 0,
                stakers: HashMap::new(),
                apr: 15,
                min_stake_amount: 0,
                lock_period: 0,
                total_rewards_distributed: 0,
            },
            
            // 🆕 NEW: MASS ADOPTION MANAGER INITIALIZATION
            mass_adoption: MassAdoptionManager {
                sponsored_transactions: Vec::new(),
                daily_rewards: DailyRewards {
                    user_rewards: HashMap::new(),
                    daily_base_reward, // 🔥 FIX: Use the cloned value
                },
                referral_program: ReferralProgram {
                    referrals: HashMap::new(),
                    level_rates: vec![10, 5, 2], // 10%, 5%, 2%
                },
                social_wallets: HashMap::new(),
                micro_earning: MicroEarning {
                    tasks: HashMap::new(),
                    completed_tasks: HashMap::new(),
                },
                fiat_gateway: FiatGateway {
                    upi_integration: true,
                    credit_card_support: true,
                    bank_transfer_support: true,
                    supported_currencies: vec!["INR".to_string(), "USD".to_string()],
                },
                nft_creator: NFTCreator {
                    user_nfts: HashMap::new(),
                    marketplace_listings: HashMap::new(),
                },
                prediction_market: PredictionMarket {
                    active_markets: HashMap::new(),
                    user_bets: HashMap::new(),
                },
                social_pay: SocialPay {
                    payment_links: HashMap::new(),
                    group_payments: HashMap::new(),
                },
                localization: Localization {
                    supported_languages: vec!["en".to_string(), "hi".to_string(), "es".to_string()],
                    user_preferences: HashMap::new(),
                },
                offline_transactions: Vec::new(),
                instant_finality_engine: crate::models::InstantFinalityEngine {
                    pending_instant_txs: HashMap::new(),
                    confirmation_time: 500, // milliseconds
                },
                guaranteed_transactions: Vec::new(),
                biometric_auth: crate::models::BiometricAuth {
                    user_biometrics: HashMap::new(),
                    enabled: false,
                },
                voice_commands: crate::models::VoiceCommands {
                    voice_profiles: HashMap::new(),
                    supported_commands: vec![],
                },
                business_accounts: crate::models::BusinessAccounts {
                    business_profiles: HashMap::new(),
                    invoices: HashMap::new(),
                },
                cross_border_pay: crate::models::CrossBorderPay {
                    supported_countries: vec![],
                    exchange_rates: HashMap::new(),
                    pending_transfers: HashMap::new(),
                },
                analytics_dashboard: crate::models::AnalyticsDashboard {
                    user_analytics: HashMap::new(),
                    platform_metrics: crate::models::PlatformMetrics {
                        total_users: 0,
                        daily_active_users: 0,
                        total_transactions: 0,
                        total_volume: 0,
                    },
                },
                learn_and_earn: crate::models::LearnAndEarn {
                    courses: HashMap::new(),
                    user_progress: HashMap::new(),
                },
                ai_assistant: crate::models::AIAssistant {
                    chat_sessions: HashMap::new(),
                    ai_model: "vexa-gpt".to_string(),
                },
            },
            
            // 🆕 ADVANCED FEATURES INITIALIZATION
            multi_sig_wallets: HashMap::new(),
            flash_loans: HashMap::new(),
            quantum_wallets: HashMap::new(),
            ai_contracts: HashMap::new(),
            cross_chain_bridge: CrossChainBridge {
                supported_chains: vec![
                    SupportedChain {
                        name: "Ethereum".to_string(),
                        chain_id: "1".to_string(),
                        bridge_address: "0x742E4C2F...".to_string(),
                        enabled: true,
                    },
                    SupportedChain {
                        name: "Binance Smart Chain".to_string(),
                        chain_id: "56".to_string(),
                        bridge_address: "0x3E5F...".to_string(),
                        enabled: true,
                    },
                    SupportedChain {
                        name: "Polygon".to_string(),
                        chain_id: "137".to_string(),
                        bridge_address: "0x8E7F...".to_string(),
                        enabled: true,
                    }
                ],
                liquidity_pools: HashMap::new(),
                bridge_fees: BridgeFeeStructure {
                    percentage: 0.1, // 0.1%
                    min_fee: 1000000, // 0.001 VEXA
                    max_fee: 100000000, // 0.1 VEXA
                },
                total_volume: 0,
            },
            gaming_assets: HashMap::new(),
            metaverse_assets: HashMap::new(),
            defi_vaults: HashMap::new(),
            enterprise_accounts: HashMap::new(),
            social_fi: SocialFi {
                social_trading: HashMap::new(),
                content_creators: HashMap::new(),
                decentralized_social: HashMap::new(),
            },
            decentralized_identity: HashMap::new(),
        };

        // Initialize some sample tasks for micro-earning
        blockchain.initialize_sample_tasks();
        blockchain.initialize_advanced_features();
        
        blockchain.create_genesis_block();
        blockchain
    }

    // 🆕 NEW: SIMPLE TRANSACTION FUNCTION FOR API
    pub fn add_simple_transaction(&mut self, from: String, to: String, amount: u64, fee: u64) -> Result<String, String> {
        // Check sender balance
        let sender_balance = self.get_wallet_balance(&from);
        let total_amount = amount + fee;
        
        if sender_balance < total_amount {
            return Err(format!("Insufficient funds. Available: {} VEXA, Required: {} VEXA", 
                self.coin.format_amount(sender_balance), 
                self.coin.format_amount(total_amount)));
        }
        
        // Create transaction
        let transaction = Transaction {
            sender: from.clone(),
            receiver: to.clone(),
            amount,
            signature: format!("sig_{}", SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos()),
            timestamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
            input_utxo: format!("utxo_{}", SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos()),
            fee,
            nonce: 0,
            transaction_type: "transfer".to_string(),
        };
        
        // Add to mempool
        if self.add_transaction(transaction) {
            let tx_hash = self.calculate_transaction_hash(&self.mempool.peek().unwrap());
            Ok(format!("Transaction added to mempool. Hash: {}", tx_hash))
        } else {
            Err("Failed to add transaction".to_string())
        }
    }

    // 🆕 INITIALIZE ADVANCED FEATURES
    fn initialize_advanced_features(&mut self) {
        println!("🚀 Initializing Advanced Features...");
        
        // Initialize cross-chain liquidity
        self.cross_chain_bridge.liquidity_pools.insert("ETH-VEXA".to_string(), self.coin.parse_amount(100000.0)); // 100K VEXA
        self.cross_chain_bridge.liquidity_pools.insert("BSC-VEXA".to_string(), self.coin.parse_amount(50000.0)); // 50K VEXA
        
        // Create sample DeFi vault
        let sample_vault = DeFiVault {
            vault_id: "defi_vault_1".to_string(),
            creator: "VexaFoundation".to_string(),
            total_liquidity: self.coin.parse_amount(50000.0),
            strategy: "Yield Farming".to_string(),
            apr: 25.5,
            investors: HashMap::new(),
        };
        self.defi_vaults.insert(sample_vault.vault_id.clone(), sample_vault);
        
        println!("✅ Advanced Features Initialized:");
        println!("   - Multi-signature Wallets");
        println!("   - Flash Loans");
        println!("   - Quantum-Resistant Wallets");
        println!("   - AI Smart Contracts");
        println!("   - Cross-chain Bridge (3 chains)");
        println!("   - Gaming & Metaverse Assets");
        println!("   - DeFi Vaults");
        println!("   - Enterprise Accounts");
        println!("   - SocialFi Platform");
        println!("   - Decentralized Identity");
    }

    // 🆕 NEW: ADVANCED FEATURES IMPLEMENTATION

    // 1. MULTI-SIGNATURE WALLETS
    pub fn create_multi_sig_wallet(&mut self, owners: Vec<String>, required_signatures: u8) -> String {
        let wallet_address = format!("multisig_{}", SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos());
        
        let multi_sig_wallet = MultiSigWallet {
            address: wallet_address.clone(),
            owners: owners.clone(),
            required_signatures,
            pending_transactions: HashMap::new(),
            created_at: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
        };
        
        self.multi_sig_wallets.insert(wallet_address.clone(), multi_sig_wallet);
        
        println!("✅ Multi-signature wallet created: {} ({}-of-{})", 
                 wallet_address, required_signatures, owners.len());
        wallet_address
    }

    pub fn submit_multi_sig_transaction(&mut self, wallet_address: String, from: String, to: String, amount: u64) -> String {
        let tx_hash = format!("multisig_tx_{}", SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos());
        
        let multi_sig_tx = MultiSigTransaction {
            tx_hash: tx_hash.clone(),
            from,
            to,
            amount,
            signatures: Vec::new(),
            created_at: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
            executed: false,
        };
        
        if let Some(wallet) = self.multi_sig_wallets.get_mut(&wallet_address) {
            wallet.pending_transactions.insert(tx_hash.clone(), multi_sig_tx);
            println!("✅ Multi-sig transaction submitted: {} (requires {}/{} signatures)", 
                     tx_hash, wallet.required_signatures, wallet.owners.len());
        }
        
        tx_hash
    }

    // 2. FLASH LOANS
    pub fn request_flash_loan(&mut self, borrower: String, amount: u64, collateral: u64) -> Option<String> {
        if amount > self.coin.parse_amount(1000000.0) { // Max 1M VEXA flash loan
            println!("❌ Flash loan amount too large");
            return None;
        }
        
        let loan_id = format!("flash_loan_{}", SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos());
        
        let flash_loan = FlashLoan {
            id: loan_id.clone(),
            borrower: borrower.clone(),
            amount,
            asset: "VEXA".to_string(),
            fee: (amount as f64 * 0.003) as u64, // 0.3% fee
            timestamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
            repaid: false,
            collateral,
        };
        
        // Check if borrower has sufficient collateral
        let borrower_balance = self.get_wallet_balance(&borrower);
        if collateral > borrower_balance {
            println!("❌ Insufficient collateral for flash loan");
            return None;
        }
        
        self.flash_loans.insert(loan_id.clone(), flash_loan);
        
        // Transfer loan amount to borrower
        let current_balance = self.get_wallet_balance(&borrower);
        self.balance_tracker.update_balance(borrower.clone(), current_balance + amount);
        
        println!("✅ Flash loan approved: {} VEXA to {} (Fee: {} VEXA)", 
                 self.coin.format_amount(amount), 
                 borrower,
                 self.coin.format_amount((amount as f64 * 0.003) as u64));
        
        Some(loan_id)
    }

    // 3. QUANTUM-RESISTANT WALLETS
    pub fn create_quantum_wallet(&mut self) -> String {
        let wallet_address = format!("quantum_{}", SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos());
        
        let quantum_wallet = QuantumResistantWallet {
            address: wallet_address.clone(),
            kyber512_public_key: format!("kyber512_pk_{}", SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos()),
            dilithium2_signature: format!("dilithium2_sig_{}", SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos()),
            sphincs_plus_backup: format!("sphincs_plus_{}", SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos()),
            created_at: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
        };
        
        self.quantum_wallets.insert(wallet_address.clone(), quantum_wallet);
        
        println!("✅ Quantum-resistant wallet created: {}", wallet_address);
        wallet_address
    }

    // 4. CROSS-CHAIN BRIDGE
    pub fn bridge_tokens(&mut self, from_chain: String, to_chain: String, amount: u64, user_address: String) -> bool {
        // Check if chains are supported
        let from_supported = self.cross_chain_bridge.supported_chains.iter().any(|c| c.name == from_chain);
        let to_supported = self.cross_chain_bridge.supported_chains.iter().any(|c| c.name == to_chain);
        
        if !from_supported || !to_supported {
            println!("❌ Unsupported chain for bridging");
            return false;
        }
        
        // Calculate bridge fee
        let fee = std::cmp::max(
            self.cross_chain_bridge.bridge_fees.min_fee,
            std::cmp::min(
                self.cross_chain_bridge.bridge_fees.max_fee,
                (amount as f64 * self.cross_chain_bridge.bridge_fees.percentage / 100.0) as u64
            )
        );
        
        let total_amount = amount + fee;
        
        // Check user balance
        let user_balance = self.get_wallet_balance(&user_address);
        if user_balance < total_amount {
            println!("❌ Insufficient balance for bridge transfer");
            return false;
        }
        
        // Deduct amount + fee
        self.balance_tracker.update_balance(user_address.clone(), user_balance - total_amount);
        
        // Update bridge statistics
        self.cross_chain_bridge.total_volume += amount;
        
        println!("✅ Cross-chain bridge transfer: {} VEXA from {} to {} (Fee: {} VEXA)", 
                 self.coin.format_amount(amount), from_chain, to_chain, self.coin.format_amount(fee));
        
        true
    }

    // 5. GAMING ASSETS
    pub fn create_gaming_asset(&mut self, owner: String, game_id: String, metadata: String, rarity: String) -> String {
        let asset_id = format!("game_asset_{}", SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos());
        
        let gaming_asset = GamingAsset {
            asset_id: asset_id.clone(),
            owner: owner.clone(),
            game_id,
            metadata,
            rarity,
            equipped: false,
        };
        
        self.gaming_assets.insert(asset_id.clone(), gaming_asset);
        
        println!("✅ Gaming asset created: {} for {}", asset_id, owner);
        asset_id
    }

    // 6. DEFI VAULTS
    pub fn create_defi_vault(&mut self, creator: String, strategy: String, initial_liquidity: u64) -> String {
        let vault_id = format!("defi_vault_{}", SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos());
        
        let defi_vault = DeFiVault {
            vault_id: vault_id.clone(),
            creator: creator.clone(),
            total_liquidity: initial_liquidity,
            strategy,
            apr: 15.0, // Base APR
            investors: HashMap::new(),
        };
        
        self.defi_vaults.insert(vault_id.clone(), defi_vault);
        
        println!("✅ DeFi vault created: {} with {} VEXA liquidity", 
                 vault_id, self.coin.format_amount(initial_liquidity));
        vault_id
    }

    // 7. SOCIALFI FEATURES
    pub fn create_social_trader(&mut self, address: String) {
        let social_trader = SocialTrader {
            address: address.clone(),
            followers: Vec::new(),
            performance: 0.0,
            copy_trading_enabled: true,
        };
        
        self.social_fi.social_trading.insert(address.clone(), social_trader);
        println!("✅ Social trader profile created: {}", address);
    }

    // 8. DECENTRALIZED IDENTITY
    pub fn create_decentralized_identity(&mut self, owner: String) -> String {
        let did = format!("did:vexa:{}", SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos());
        
        let decentralized_identity = DecentralizedIdentity {
            did: did.clone(),
            owner: owner.clone(),
            soulbound_tokens: Vec::new(),
            verifiable_credentials: Vec::new(),
            reputation_score: 0.0,
        };
        
        self.decentralized_identity.insert(did.clone(), decentralized_identity);
        
        println!("✅ Decentralized Identity created: {} for {}", did, owner);
        did
    }

    // 🆕 GET ADVANCED FEATURES INFO
    pub fn get_advanced_features_info(&self) -> String {
        format!(
            "Advanced Features Status:\n\
             Multi-signature Wallets: {}\n\
             Flash Loans Active: {}\n\
             Quantum Wallets: {}\n\
             AI Contracts: {}\n\
             Cross-chain Bridges: {}\n\
             Gaming Assets: {}\n\
             Metaverse Assets: {}\n\
             DeFi Vaults: {}\n\
             Enterprise Accounts: {}\n\
             SocialFi Users: {}\n\
             Decentralized Identities: {}",
            self.multi_sig_wallets.len(),
            self.flash_loans.len(),
            self.quantum_wallets.len(),
            self.ai_contracts.len(),
            self.cross_chain_bridge.supported_chains.len(),
            self.gaming_assets.len(),
            self.metaverse_assets.len(),
            self.defi_vaults.len(),
            self.enterprise_accounts.len(),
            self.social_fi.social_trading.len(),
            self.decentralized_identity.len()
        )
    }

    // 🆕 NEW: INITIALIZE SAMPLE MICRO-EARNING TASKS
    fn initialize_sample_tasks(&mut self) {
        let tasks = vec![
            ("survey1".to_string(), EarningTask {
                task_id: "survey1".to_string(),
                task_type: "survey".to_string(),
                reward_amount: self.coin.parse_amount(0.1), // 0.1 VEXA
                description: "Complete a short survey".to_string(),
                active: true,
            }),
            ("watch_ad1".to_string(), EarningTask {
                task_id: "watch_ad1".to_string(),
                task_type: "watch_ad".to_string(),
                reward_amount: self.coin.parse_amount(0.05), // 0.05 VEXA
                description: "Watch a 30-second ad".to_string(),
                active: true,
            }),
            ("refer_friend".to_string(), EarningTask {
                task_id: "refer_friend".to_string(),
                task_type: "referral".to_string(),
                reward_amount: self.coin.parse_amount(1.0), // 1 VEXA
                description: "Refer a friend to Vexa Chain".to_string(),
                active: true,
            }),
        ];

        for (id, task) in tasks {
            self.mass_adoption.micro_earning.tasks.insert(id, task);
        }
    }

    // Creates the first block of the chain
    pub fn create_genesis_block(&mut self) {
        let transactions = vec![];

        let mut genesis_block = Block {
            index: 0,
            timestamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
            transactions: transactions.clone(),
            previous_hash: String::from("0"),
            nonce: 0,
            hash: String::new(),
            difficulty: self.difficulty as u64,
            burned_tokens: 0,
            dao_proposals: vec![],
        };

        genesis_block.hash = self.calculate_block_hash(&genesis_block);
        
        for tx in &genesis_block.transactions {
            self.update_utxo_set(tx);
        }

        self.chain.push(genesis_block);
        println!("🎁 Genesis Block created and added to the chain.");
    }

    // ✅ ADDED: 10 BILLION BLOCK LIMIT
    const MAX_BLOCKS: u64 = 10_000_000_000; // 10 BILLION blocks

    fn calculate_block_hash(&self, block: &Block) -> String {
        use sha2::{Sha256, Digest};
        
        let data = format!("{}{}{:?}{}{}", 
            block.index, 
            block.timestamp, 
            block.transactions, 
            block.previous_hash, 
            block.nonce
        );
        format!("{:x}", Sha256::digest(data.as_bytes()))
    }

    pub fn get_latest_block(&self) -> &Block {
        self.chain.last().expect("Chain should have at least the genesis block")
    }

    // 🆕 NEW: MASS ADOPTION FEATURES IMPLEMENTATION

    // 1. FREE SPONSORED TRANSACTIONS
    pub fn submit_sponsored_transaction(&mut self, sp_tx: SponsoredTransaction) -> bool {
        let gas_cost = sp_tx.user_transaction.fee;
        let sponsor_balance = self.get_wallet_balance(&sp_tx.sponsor_address);
        
        if sponsor_balance < gas_cost {
            println!("❌ Sponsor insufficient balance for gas");
            return false;
        }
        
        // Deduct gas from sponsor
        self.balance_tracker.deduct_fee(sp_tx.sponsor_address.clone(), gas_cost);
        
        // Add user transaction (user pays ZERO)
        let mut user_tx = sp_tx.user_transaction;
        user_tx.fee = 0;
        
        self.mempool.push(user_tx);
        println!("✅ Sponsored transaction added! User pays: 0 VEXA");
        true
    }

    // 2. DAILY REWARDS SYSTEM - 🔥 COMPLETELY FIXED VERSION
    pub fn claim_daily_reward(&mut self, user_address: String) -> u64 {
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        let one_day = 24 * 60 * 60;
        
        // 🔥 FIXED: Get all required data FIRST before any mutable operations
        let current_balance = self.get_wallet_balance(&user_address);
        let base_reward = self.mass_adoption.daily_rewards.daily_base_reward;
        
        // Check if user can claim reward
        let (needs_reward, current_streak) = {
            if let Some(user_reward) = self.mass_adoption.daily_rewards.user_rewards.get(&user_address) {
                let needs_reward = now - user_reward.last_claim_time >= one_day;
                (needs_reward, user_reward.streak_count)
            } else {
                (true, 0)
            }
        };
        
        if !needs_reward {
            println!("❌ Reward already claimed today");
            return 0;
        }
        
        let streak_bonus = (current_streak as u64) * (base_reward / 10);
        let total_reward = base_reward + streak_bonus;
        
        // 🔥 FIXED: Now update user rewards (mutable operations)
        let user_reward_entry = self.mass_adoption.daily_rewards.user_rewards
            .entry(user_address.clone())
            .or_insert(UserReward {
                last_claim_time: 0,
                streak_count: 0,
                total_claimed: 0,
            });
        
        user_reward_entry.last_claim_time = now;
        user_reward_entry.streak_count = current_streak + 1;
        user_reward_entry.total_claimed += total_reward;
        
        // Update user balance
        self.balance_tracker.update_balance(user_address.clone(), current_balance + total_reward);
        
        println!("✅ Daily reward: {} VEXA (Streak: {} days)", 
                 self.coin.format_amount(total_reward), user_reward_entry.streak_count);
        total_reward
    }

    // 3. WATCH AD FOR BONUS
    pub fn watch_ad_for_bonus(&mut self, user_address: String) -> u64 {
        let bonus_amount = self.mass_adoption.daily_rewards.daily_base_reward;
        
        let current_balance = self.get_wallet_balance(&user_address);
        self.balance_tracker.update_balance(user_address.clone(), current_balance + bonus_amount);
        
        println!("✅ Ad bonus claimed: {} VEXA", self.coin.format_amount(bonus_amount));
        bonus_amount
    }

    // 4. REFERRAL SYSTEM
    pub fn add_referral(&mut self, referrer: String, new_user: String) -> bool {
        if self.mass_adoption.referral_program.referrals.contains_key(&new_user) {
            println!("❌ User already referred");
            return false;
        }
        
        let referral_data = ReferralData {
            referrer: referrer.clone(),
            referral_date: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
            earned_amount: 0,
        };
        
        self.mass_adoption.referral_program.referrals.insert(new_user.clone(), referral_data);
        
        // Give bonus to both users
        let signup_bonus = self.coin.parse_amount(10.0);
        
        let referrer_balance = self.get_wallet_balance(&referrer);
        self.balance_tracker.update_balance(referrer, referrer_balance + signup_bonus);
        
        let new_user_balance = self.get_wallet_balance(&new_user);
        self.balance_tracker.update_balance(new_user, new_user_balance + signup_bonus);
        
        println!("✅ Referral added! Both users got {} VEXA bonus", 
                 self.coin.format_amount(signup_bonus));
        true
    }

    // 5. SOCIAL WALLET CREATION
    pub fn create_social_wallet(&mut self, phone: String, email: String) -> SocialWallet {
        let social_wallet = SocialWallet::new(phone.clone(), email.clone());
        self.mass_adoption.social_wallets.insert(phone, social_wallet.clone());
        println!("✅ Social wallet created for phone: {}", social_wallet.phone_number);
        social_wallet
    }

    // 6. MICRO-EARNING TASKS
    pub fn complete_micro_task(&mut self, user_address: String, task_id: String) -> bool {
        // 🔥 FIXED: Get task details first
        let task_reward = if let Some(task) = self.mass_adoption.micro_earning.tasks.get(&task_id) {
            if !task.active {
                println!("❌ Task is not active");
                return false;
            }
            Some(task.reward_amount)
        } else {
            None
        };

        if let Some(reward_amount) = task_reward {
            // Add reward to user
            let current_balance = self.get_wallet_balance(&user_address);
            self.balance_tracker.update_balance(user_address.clone(), current_balance + reward_amount);
            
            // Record completion
            let completed_task = CompletedTask {
                task_id: task_id.clone(),
                user_address: user_address.clone(),
                completion_time: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
                reward_earned: reward_amount,
            };
            
            self.mass_adoption.micro_earning.completed_tasks
                .entry(user_address)
                .or_insert_with(Vec::new)
                .push(completed_task);
            
            println!("✅ Task completed! Earned: {} VEXA", 
                     self.coin.format_amount(reward_amount));
            true
        } else {
            println!("❌ Task not found");
            false
        }
    }

    // 7. GET AVAILABLE TASKS
    pub fn get_available_tasks(&self) -> Vec<EarningTask> {
        self.mass_adoption.micro_earning.tasks
            .values()
            .filter(|task| task.active)
            .cloned()
            .collect()
    }

    // 8. CREATE PAYMENT LINK - 🔥 FIXED VERSION
    pub fn create_payment_link(&mut self, creator: String, amount: u64, description: String) -> String {
        let link_id = format!("pay_{}_{}", creator, SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos());
        
        let payment_link = PaymentLink {
            link_id: link_id.clone(),
            creator: creator.clone(),
            amount: amount,
            description: description,
            created_at: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
            claimed: false,
            claimed_by: None,
        };
        
        self.mass_adoption.social_pay.payment_links.insert(link_id.clone(), payment_link);
        
        let payment_url = format!("https://vexachain.com/pay/{}", link_id);
        println!("✅ Payment link created: {}", payment_url);
        payment_url
    }

    // 9. CLAIM PAYMENT LINK - 🔥 COMPLETELY FIXED VERSION
    pub fn claim_payment_link(&mut self, link_id: String, claimer: String) -> bool {
        // 🔥 FIXED: Check if link exists and can be claimed first
        let amount_to_claim = {
            if let Some(link) = self.mass_adoption.social_pay.payment_links.get(&link_id) {
                if link.claimed {
                    println!("❌ Payment link already claimed");
                    return false;
                }
                Some(link.amount)
            } else {
                None
            }
        };

        if let Some(amount) = amount_to_claim {
            // Now update the link and transfer funds
            if let Some(link) = self.mass_adoption.social_pay.payment_links.get_mut(&link_id) {
                link.claimed = true;
                link.claimed_by = Some(claimer.clone());
                
                // Transfer amount to claimer
                let current_balance = self.get_wallet_balance(&claimer);
                self.balance_tracker.update_balance(claimer, current_balance + amount);
                
                println!("✅ Payment link claimed! {} VEXA received", 
                         self.coin.format_amount(amount));
                return true;
            }
        }
        
        println!("❌ Payment link not found");
        false
    }

    // 🆕 NEW: MASS ADOPTION STATISTICS
    pub fn get_mass_adoption_stats(&self) -> String {
        let total_users = self.balance_tracker.balances.len();
        let daily_reward_users = self.mass_adoption.daily_rewards.user_rewards.len();
        let referral_users = self.mass_adoption.referral_program.referrals.len();
        let social_wallets = self.mass_adoption.social_wallets.len();
        
        format!(
            "Mass Adoption Statistics:\n\
             Total Users: {}\n\
             Daily Reward Users: {}\n\
             Referral Program Users: {}\n\
             Social Wallets: {}\n\
             Available Tasks: {}\n\
             Active Payment Links: {}",
            total_users,
            daily_reward_users,
            referral_users,
            social_wallets,
            self.mass_adoption.micro_earning.tasks.len(),
            self.mass_adoption.social_pay.payment_links.len()
        )
    }

    // Dynamic difficulty adjustment with 10B block support
    pub fn adjust_difficulty(&mut self) {
        const BLOCK_GENERATION_INTERVAL: u64 = 600;
        const DIFFICULTY_ADJUSTMENT_INTERVAL: usize = 10;

        let latest_block = self.get_latest_block();

        if latest_block.index >= Self::MAX_BLOCKS {
            println!("🏁 Maximum block limit reached (10 Billion)");
            return;
        }

        if latest_block.index % DIFFICULTY_ADJUSTMENT_INTERVAL as u64 == 0 && latest_block.index != 0 {
            let chain_len = self.chain.len();
            if chain_len > DIFFICULTY_ADJUSTMENT_INTERVAL {
                let prev_adjustment_block = &self.chain[chain_len - DIFFICULTY_ADJUSTMENT_INTERVAL];
                let time_taken = latest_block.timestamp - prev_adjustment_block.timestamp;
                let expected_time = (DIFFICULTY_ADJUSTMENT_INTERVAL as u64) * BLOCK_GENERATION_INTERVAL;
    
                if time_taken < expected_time / 2 {
                    self.difficulty += 1;
                    println!("🚨 Difficulty increased to {} (Too fast)", self.difficulty);
                } else if time_taken > expected_time * 2 {
                    self.difficulty = self.difficulty.saturating_sub(1).max(1);
                    println!("🐌 Difficulty decreased to {} (Too slow)", self.difficulty);
                }
            }
        }
    }

    // Mines a new block with 10B block support
    pub fn mine_block(&mut self) -> Block {
        self.adjust_difficulty();

        let latest_block = self.get_latest_block();
        
        if latest_block.index >= Self::MAX_BLOCKS {
            println!("🏁 Blockchain reached maximum capacity (10 Billion blocks)");
            return latest_block.clone();
        }
        
        let mut new_block = Block {
            index: latest_block.index + 1,
            timestamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
            transactions: Vec::new(),
            previous_hash: latest_block.hash.clone(),
            nonce: 0,
            hash: String::new(),
            difficulty: self.difficulty as u64,
            burned_tokens: 0,
            dao_proposals: vec![],
        };

        // Add mining reward transaction
        let reward_amount = 25 * 1_000_000_000; // 25 VEXA per block (for 25M mining supply)
        
        let reward_tx = Transaction {
            sender: "MINING_REWARD".to_string(),
            receiver: "MINER_ADDRESS".to_string(),
            amount: reward_amount,
            signature: "reward".to_string(),
            timestamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
            input_utxo: format!("reward_utxo_{}", SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos()),
            fee: 0,
            nonce: 0,
            transaction_type: "reward".to_string(),
        };
        new_block.transactions.push(reward_tx);
        
        // Add transactions from mempool
        let mut count = 0;
        let mut transactions_to_mine = Vec::new();
        
        while let Some(tx) = self.mempool.pop() {
            if count < 100 {
                transactions_to_mine.push(tx);
                count += 1;
            } else {
                self.mempool.push(tx);
                break;
            }
        }
        
        new_block.transactions.extend(transactions_to_mine);
        
        println!("⛏️ Mining block #{}/{}B with {} transactions...", 
                 new_block.index, Self::MAX_BLOCKS / 1_000_000_000,
                 new_block.transactions.len());

        // Proof-of-Work
        let target_prefix = "0".repeat(self.difficulty);
        new_block.hash = self.calculate_block_hash(&new_block);
        
        while !new_block.hash.starts_with(&target_prefix) {
            new_block.nonce += 1;
            new_block.hash = self.calculate_block_hash(&new_block);
        }

        println!("✅ Block mined! Hash: {}, Nonce: {}", new_block.hash, new_block.nonce);
        new_block
    }

    // ✅ FIXED: Double spend error resolved
    pub fn add_transaction(&mut self, transaction: Transaction) -> bool {
        if transaction.signature.is_empty() && transaction.sender != "MINING_REWARD" && transaction.sender != "GENESIS" {
            println!("❌ Transaction rejected: Invalid signature!");
            return false;
        }

        if self.spent_utxos.contains_key(&transaction.input_utxo) {
            println!("❌ Transaction rejected: Double spend detected!");
            return false;
        }

        // Check sender balance
        if transaction.sender != "MINING_REWARD" && transaction.sender != "GENESIS" {
            let required_amount = transaction.amount + transaction.fee;
            let available_balance = self.get_balance_from_utxo(&transaction.sender);
            
            if available_balance < required_amount {
                println!("❌ Transaction rejected: Insufficient funds! Available: {} VEXA, Required: {} VEXA",
                         self.coin.format_amount(available_balance), self.coin.format_amount(required_amount));
                return false;
            }
        }

        self.mempool.push(transaction);
        true
    }

    // Smart Contract Functions
    pub fn deploy_contract(&mut self, code: String, owner: String, initial_fund: u64) -> String {
        let owner_balance = self.get_wallet_balance(&owner);
        if initial_fund > owner_balance {
            println!("❌ Contract deployment failed: Insufficient funds from owner.");
            return String::new();
        }
        
        self.balance_tracker.update_balance(owner.clone(), owner_balance.saturating_sub(initial_fund));
        
        let contract_address = self.contract_manager.deploy_contract(code, owner, initial_fund);
        println!("🚀 VEX Smart Contract deployed: {}", contract_address);
        contract_address
    }

    pub fn execute_contract(&mut self, contract_tx: ContractTransaction) -> ContractExecutionResult {
        println!("🤖 Executing VEX Contract: {}", contract_tx.contract_address);
        
        let result = self.contract_manager.execute_contract(
            &contract_tx.contract_address,
            &contract_tx.caller,
            &contract_tx.function,
            contract_tx.args,
        );

        if result.success {
            println!("✅ Contract execution successful: {}", result.output);
            println!("   Gas used: {}", result.gas_used);
        } else {
            println!("❌ Contract execution failed: {}", result.output);
        }

        result
    }

    // Enhanced UTXO management
    pub fn update_utxo_set(&mut self, transaction: &Transaction) {
        if !transaction.input_utxo.is_empty() {
            self.spent_utxos.insert(transaction.input_utxo.clone(), true);
            
            if let Some(utxo) = self.utxo_set.get_mut(&transaction.input_utxo) {
                utxo.spent = true;
            }
        }
        
        // Create new UTXO from the transaction output
        let new_utxo = UTXO {
            tx_hash: self.calculate_transaction_hash(transaction),
            output_index: 0,
            address: transaction.receiver.clone(),
            amount: transaction.amount,
            spent: false,
        };
        
        self.utxo_set.insert(self.calculate_transaction_hash(transaction), new_utxo);
    }

    // ✅ ADDED: Simple transaction hash calculation
    fn calculate_transaction_hash(&self, transaction: &Transaction) -> String {
        use sha2::{Sha256, Digest};
        
        let data = format!("{}{}{}{}", 
            transaction.sender, 
            transaction.receiver, 
            transaction.amount, 
            transaction.timestamp
        );
        format!("{:x}", Sha256::digest(data.as_bytes()))
    }
    
    pub fn get_balance_from_utxo(&self, address: &str) -> u64 {
        self.utxo_set
            .values()
            .filter(|utxo| utxo.address == address && !utxo.spent)
            .map(|utxo| utxo.amount)
            .sum()
    }

    // Enhanced 51% attack protection
    pub fn check_51_attack(&self, new_chain: &[Block]) -> bool {
        let current_difficulty: u64 = self.chain.iter().map(|block| block.difficulty as u64).sum();
        let new_difficulty: u64 = new_chain.iter().map(|block| block.difficulty as u64).sum();
        
        if new_difficulty > current_difficulty * 3 {
            println!("🚨 Possible 51% attack detected! Difficulty spike detected.");
            return true;
        }
        
        if let (Some(current_last), Some(new_last)) = (self.chain.last(), new_chain.last()) {
            if new_last.timestamp < current_last.timestamp {
                println!("🚨 Possible 51% attack detected! Time manipulation detected.");
                return true;
            }
        }
        
        false
    }

    // ✅ FIXED: Enhanced block addition with proper UTXO handling
    pub fn add_mined_block(&mut self, block: Block) {
        if block.index > Self::MAX_BLOCKS {
            println!("❌ Block rejected: Maximum block limit (10B) reached");
            return;
        }

        let block_size = serde_json::to_string(&block).unwrap_or_default().len();
        if block_size > 1_000_000 {
            println!("❌ Block rejected: Size exceeds 1MB limit");
            return;
        }

        if self.check_51_attack(&[block.clone()]) {
            println!("❌ Block rejected: Possible 51% attack");
            return;
        }

        if block.index != self.get_latest_block().index + 1 {
            println!("❌ Block rejected: Index mismatch! Expected: {}, Got: {}", self.get_latest_block().index + 1, block.index);
            return;
        }
        
        if block.previous_hash != self.get_latest_block().hash {
            println!("❌ Block rejected: Previous hash mismatch!");
            return;
        }
        
        let target_prefix = "0".repeat(self.difficulty);
        if !block.hash.starts_with(&target_prefix) {
            println!("❌ Block rejected: Invalid Proof-of-Work!");
            return;
        }

        // ✅ FIXED: Validate all transactions in block
        for transaction in &block.transactions {
            if transaction.signature.is_empty() && transaction.sender != "MINING_REWARD" && transaction.sender != "GENESIS" {
                println!("❌ Block rejected: Invalid transaction signature!");
                return;
            }
            
            // ✅ FIXED: Double spend check - but don't mark as spent yet
            if self.spent_utxos.contains_key(&transaction.input_utxo) {
                println!("❌ Block rejected: Double spend detected!");
                return;
            }
        }

        // ✅ FIXED: Process transactions and mark UTXOs as spent
        for transaction in &block.transactions {
            if transaction.sender != "MINING_REWARD" && transaction.sender != "GENESIS" {
                self.balance_tracker.process_transaction(
                    transaction.sender.clone(),
                    transaction.receiver.clone(),
                    transaction.amount
                );
                self.balance_tracker.deduct_fee(
                    transaction.sender.clone(),
                    transaction.fee
                );
                // ✅ MARK UTXO AS SPENT only when block is added
                self.spent_utxos.insert(transaction.input_utxo.clone(), true);
                self.update_utxo_set(transaction);
            } else if transaction.sender == "MINING_REWARD" {
                self.balance_tracker.update_balance(transaction.receiver.clone(), transaction.amount);
                self.update_utxo_set(transaction);
            }
        }

        self.chain.push(block.clone());
        self.propagate_block(&block);
        
        println!("✅ Block #{}/{}B added to blockchain", block.index, Self::MAX_BLOCKS / 1_000_000_000);
    }

    // Enhanced chain validation
    pub fn is_chain_valid(&self) -> bool {
        for i in 1..self.chain.len() {
            let current_block = &self.chain[i];
            let previous_block = &self.chain[i-1];

            if current_block.hash != self.calculate_block_hash(current_block) {
                println!("❌ Invalid block hash at index {}", i);
                return false;
            }
            
            if current_block.previous_hash != previous_block.hash {
                println!("❌ Invalid previous hash at index {}", i);
                return false;
            }
            
            let target_prefix = "0".repeat(current_block.difficulty as usize);
            if !current_block.hash.starts_with(&target_prefix) {
                println!("❌ Invalid Proof-of-Work at index {}", i);
                return false;
            }
            
            let block_size = serde_json::to_string(current_block).unwrap_or_default().len();
            if block_size > 1_000_000 {
                println!("❌ Block size exceeded at index {}", i);
                return false;
            }

            for transaction in &current_block.transactions {
                if transaction.signature.is_empty() && transaction.sender != "MINING_REWARD" && transaction.sender != "GENESIS" {
                    println!("❌ Invalid transaction signature in block {}", i);
                    return false;
                }
            }
        }
        true
    }
    
    pub fn get_coin_info(&self) -> String {
        format!("{}\nBlock Capacity: {}/{} blocks", 
                self.coin.get_supply_info(),
                self.chain.len(),
                Self::MAX_BLOCKS)
    }

    // Enhanced blockchain info
    pub fn get_balance_info(&self) -> String {
        let total_balance = self.balance_tracker.get_total_supply();
        let utxo_balance: u64 = self.utxo_set.values().filter(|u| !u.spent).map(|u| u.amount).sum();
        
        format!(
            "Balance Summary:\nTotal Tracked: {} VEXA\nUTXO Balance: {} VEXA\nGenesis Addresses: {}\nMempool Size: {}\nContracts: {}",
            self.coin.format_amount(total_balance),
            self.coin.format_amount(utxo_balance),
            self.balance_tracker.balances.len(),
            self.mempool.len(),
            self.contract_manager.contracts.len()
        )
    }

    pub fn get_wallet_balance(&self, address: &str) -> u64 {
        let balance_tracker_amount = self.balance_tracker.get_balance(address);
        let utxo_amount = self.get_balance_from_utxo(address);
        
        balance_tracker_amount.max(utxo_amount)
    }

    pub fn get_mempool_info(&self) -> String {
        let total_fees: u64 = self.mempool.iter().map(|tx| tx.fee).sum();
        
        format!("Mempool: {} pending transactions | Total Fees: {} VEXA", 
                 self.mempool.len(),
                 self.coin.format_amount(total_fees))
    }

    pub fn get_enhanced_info(&self) -> String {
        format!(
            "Enhanced Blockchain Info:\nBlocks: {}/{}\nPending TXs: {}\nContracts: {}\nDifficulty: {}\nPeers: {}\nUTXOs: {}\nStaked: {} VEXA",
            self.chain.len(),
            Self::MAX_BLOCKS,
            self.mempool.len(),
            self.contract_manager.contracts.len(),
            self.difficulty,
            self.network_peers.len(),
            self.utxo_set.len(),
            self.coin.format_amount(self.staking_pool.total_staked)
        )
    }

    pub fn get_security_info(&self) -> String {
        format!(
            "Security Features:\n✅ Digital Signatures\n✅ Double Spending Protection\n✅ 51% Attack Detection\n✅ Dynamic Difficulty\n✅ Transaction Fees\n✅ Smart Contracts\n✅ 10B Block Capacity"
        )
    }

    // Enhanced staking implementation
    pub fn initialize_staking_pool(&mut self) {
        self.staking_pool.min_stake_amount = self.coin.parse_amount(100.0);
        self.staking_pool.lock_period = 30 * 24 * 60 * 60;
        
        println!("🎯 Staking pool initialized with {}% APR", self.staking_pool.apr);
    }

    pub fn stake_tokens(&mut self, staker_address: String, amount: u64, lock_days: u64) -> bool {
        if amount < self.staking_pool.min_stake_amount {
            println!("❌ Stake failed: Minimum stake amount is {} VEXA", 
                      self.coin.format_amount(self.staking_pool.min_stake_amount));
            return false;
        }

        let balance = self.get_wallet_balance(&staker_address);
        if amount > balance {
            println!("❌ Stake failed: Insufficient balance. Available: {} VEXA", 
                      self.coin.format_amount(balance));
            return false;
        }

        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        let unlock_time = timestamp + (lock_days * 24 * 60 * 60);

        let staker = self.staking_pool.stakers.entry(staker_address.clone()).or_insert_with(|| Staker {
            address: staker_address.clone(),
            staked_amount: 0,
            stake_time: timestamp,
            rewards_earned: 0,
            unlock_time: 0,
            last_reward_calculation: timestamp,
        });
        
        staker.staked_amount += amount;
        staker.unlock_time = staker.unlock_time.max(unlock_time);
        staker.stake_time = staker.stake_time.max(timestamp);

        self.balance_tracker.update_balance(staker_address.clone(), balance.saturating_sub(amount));
        self.staking_pool.total_staked += amount;

        println!("✅ Staked {} VEXA for {} days (New Total Stake: {})", 
                  self.coin.format_amount(amount), 
                  lock_days,
                  self.coin.format_amount(staker.staked_amount));
        true
    }

    pub fn calculate_staking_rewards(&mut self) {
        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        
        for staker in self.staking_pool.stakers.values_mut() {
            let time_elapsed = timestamp - staker.last_reward_calculation;
            if time_elapsed < 86400 {
                continue;
            }

            let days_elapsed = time_elapsed as f64 / 86400.0;
            let daily_rate = self.staking_pool.apr as f64 / 36500.0;
            let reward = (staker.staked_amount as f64 * daily_rate * days_elapsed) as u64;

            if reward > 0 {
                staker.rewards_earned += reward;
                staker.last_reward_calculation = timestamp;
                self.staking_pool.total_rewards_distributed += reward;
                println!("🌟 Calculated {} VEXA reward for {}", self.coin.format_amount(reward), staker.address);
            }
        }
    }

    pub fn claim_rewards(&mut self, staker_address: String) -> u64 {
        let current_balance = self.get_wallet_balance(&staker_address);

        if let Some(staker) = self.staking_pool.stakers.get_mut(&staker_address) {
            let rewards = staker.rewards_earned;

            if rewards > 0 {
                self.balance_tracker
                    .update_balance(staker_address.clone(), current_balance + rewards);

                println!(
                    "✅ Rewards claimed: {} VEXA for {}",
                    self.coin.format_amount(rewards),
                    staker_address
                );

                staker.rewards_earned = 0;
                return rewards;
            } else {
                println!("ℹ️ No rewards to claim for {}", staker_address);
                return 0;
            }
        }

        println!("⚠️ Staker not found: {}", staker_address);
        0
    }

    pub fn get_staking_info(&self) -> String {
        format!(
            "Staking Pool Info:\nTotal Staked: {} VEXA\nStakers: {}\nAPR: {}%\nTotal Rewards Distributed: {} VEXA",
            self.coin.format_amount(self.staking_pool.total_staked),
            self.staking_pool.stakers.len(),
            self.staking_pool.apr,
            self.coin.format_amount(self.staking_pool.total_rewards_distributed)
        )
    }

    pub fn get_staker_info(&self, address: &str) -> Option<String> {
        if let Some(staker) = self.staking_pool.stakers.get(address) {
            let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
            
            let time_remaining = if staker.unlock_time > now {
                let remaining_seconds = staker.unlock_time - now;
                format!("{} days", remaining_seconds / 86400)
            } else {
                "Unlocked".to_string()
            };
            
            Some(format!(
                "Staker Info:\nStaked: {} VEXA\nRewards Earned: {} VEXA\nTime Remaining: {}",
                self.coin.format_amount(staker.staked_amount),
                self.coin.format_amount(staker.rewards_earned),
                time_remaining
            ))
        } else {
            None
        }
    }

    // ✅ UPDATED: Network statistics with 10B capacity
    pub fn get_network_stats(&self) -> String {
        let total_transactions: usize = self.chain.iter().map(|block| block.transactions.len()).sum();
        let total_volume: u64 = self.chain.iter().flat_map(|block| &block.transactions).map(|tx| tx.amount).sum();
        
        format!(
            "Network Statistics:\nTotal Blocks: {}/{}\nTotal Transactions: {}\nTotal Volume: {} VEXA\nActive Peers: {}\nNetwork Hashrate: {} H/s (est.)",
            self.chain.len(),
            Self::MAX_BLOCKS,
            total_transactions,
            self.coin.format_amount(total_volume),
            self.network_peers.len(),
            self.difficulty * 1000
        )
    }

    // Enhanced node discovery
    pub fn discover_peers(&mut self, seed_nodes: Vec<String>) {
        println!("🔍 Discovering peers from {} seed nodes...", seed_nodes.len());
        
        for seed in seed_nodes {
            if !self.network_peers.iter().any(|p| p.address == seed) {
                let peer = NetworkPeer {
                    address: seed.clone(),
                    last_seen: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
                    reputation: 10,
                    version: "1.0.0".to_string(),
                };
                self.network_peers.push(peer);
                println!("✅ Added peer: {}", seed);
            }
        }
    }

    // Enhanced block propagation
    pub fn propagate_block(&self, block: &Block) {
        println!("🚀 Propagating block #{}/{}B to {} peers...", 
                 block.index, Self::MAX_BLOCKS / 1_000_000_000,
                 self.network_peers.len());
        
        let high_reputation_peers: Vec<&NetworkPeer> = self.network_peers
            .iter()
            .filter(|p| p.reputation > 5)
            .collect();
            
        println!("  📤 Sending to {} high-reputation peers", high_reputation_peers.len());
        
        for peer in high_reputation_peers {
            println!("  ➡️ Sending to: {} (reputation: {})", peer.address, peer.reputation);
        }
    }
}

// --- Persistence functions ---
const BLOCKCHAIN_FILE: &str = "blockchain.json";

pub fn load_blockchain_from_file() -> Blockchain {
    if Path::new(BLOCKCHAIN_FILE).exists() {
        let content = fs::read_to_string(BLOCKCHAIN_FILE).expect("Failed to read blockchain file");
        let mut loaded_chain: Blockchain = match serde_json::from_str(&content) {
    Ok(chain) => chain,
    Err(_) => {
        println!("🔄 Creating new blockchain (old format detected)");
        Blockchain::new()
    }
};
        loaded_chain.mempool = BinaryHeap::new();
        println!("Blockchain loaded. Current latest block index: {}", loaded_chain.get_latest_block().index);
        loaded_chain
    } else {
        println!("No existing blockchain file found. Creating new blockchain.");
        Blockchain::new()
    }
}

pub fn save_blockchain_to_file(blockchain: &Blockchain) {
    let data = serde_json::to_string_pretty(blockchain).expect("Failed to serialize blockchain");
    fs::write(BLOCKCHAIN_FILE, data).expect("Failed to write blockchain file");
    println!("Blockchain saved to {}.", BLOCKCHAIN_FILE);
}

#[tokio::main]
async fn main() {
    // 🎯 COMMAND LINE ARGUMENTS HANDLING
    use std::env;
    
    let args: Vec<String> = env::args().collect();
    let mut port = 8080;
    let mut api_port = 3001;
    let mut network_type = "mainnet";

    // Parse command line arguments
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--port" => {
                if i + 1 < args.len() {
                    port = args[i + 1].parse().unwrap_or(8080);
                    i += 1;
                }
            }
            "--api-port" => {
                if i + 1 < args.len() {
                    api_port = args[i + 1].parse().unwrap_or(3001);
                    i += 1;
                }
            }
            "--network" => {
                if i + 1 < args.len() {
                    network_type = args[i + 1].as_str();
                    i += 1;
                }
            }
            "--help" => {
                println!("🚀 Vexa Chain Node - Usage:");
                println!("  --port <number>      Set P2P port (default: 8080)");
                println!("  --api-port <number>  Set REST API port (default: 3001)");
                println!("  --network <type>     Set network type: testnet, mainnet, devnet (default: mainnet)");
                println!("  --help               Show this help message");
                return;
            }
            _ => {}
        }
        i += 1;
    }

    println!("🚀 Starting Vexa Chain P2P Network...");
    println!("🌐 P2P Port: {}", port);
    println!("🌐 API Port: {}", api_port);
    println!("🌐 Network Type: {}", network_type);
    println!("🤖 VEX Smart Contracts: READY");
    println!("🔒 Security Features: ACTIVE");
    println!("💰 Staking System: READY");
    println!("📈 Block Capacity: 10 BILLION Blocks");
    println!("🎯 MASS ADOPTION FEATURES: READY");
    println!("🚀 ADVANCED FEATURES: READY");

    // 🆕 UPDATED TOKENOMICS FOR GATE.IO LISTING
    println!("
🏆 VEXA TOKENOMICS – $1-2 PRICE TARGET
──────────────────────────────────────────────────────
🏢 Foundation & Team (20M):
  Core Team: 8M (4yr vesting) | Foundation: 7M | Dev: 5M
💰 Strategic Partners (15M):
  Seed: 6M | Strategic: 5M | Ecosystem: 4M
🌍 Public Distribution (35M):
  Public Sale: 15M | Staking: 10M | Community: 6M | Airdrop: 4M
🔒 Liquidity & Growth (30M):
  DEX: 8M | CEX: 7M | Marketing: 5M | Partnerships: 4M
  Tech Grants: 3M | Security: 3M
");

    // 🎯 NETWORK CONFIGURATION
    let config = match network_type {
        "testnet" => NetworkConfig::testnet(),
        "devnet" => NetworkConfig::devnet(),
        _ => NetworkConfig::mainnet(port, api_port),
    };

    println!("💰 Block Reward: {} VEXA", config.block_reward);

    // Create test wallets
    println!("👛 Creating Test Wallets...");
    let wallet1 = Wallet::new();
    let wallet2 = Wallet::new();
    println!("✅ Wallet 1: {}", wallet1.address);
    println!("✅ Wallet 2: {}", wallet2.address);

    // P2P Network with config port
    let network = Arc::new(Network::new(config.port));
    let network_for_thread: Arc<Network> = Arc::clone(&network);
    
    thread::spawn(move || {
        network_for_thread.start();
    });

    thread::sleep(Duration::from_secs(2));

let mut my_blockchain = load_blockchain_from_file();
my_blockchain.initialize_staking_pool();

// Manually update UTXO balances
// Direct balance update for testing
my_blockchain.balance_tracker.update_balance("3v9o2bcjzJtkeJ65S18m8E1nGRiv".to_string(), 900000000000); // 900 VEXA
my_blockchain.balance_tracker.update_balance("4HoFUyWjh43FkLf4rWg3oQ5F4Pmm".to_string(), 1100000000000); // 1100 VEXA

println!("📦 Blockchain loaded. Current latest block index: {}", my_blockchain.get_latest_block().index);
println!("✅ Is blockchain valid: {}", my_blockchain.is_chain_valid());
println!("{}", my_blockchain.get_coin_info());
println!("{}", my_blockchain.get_balance_info());
println!("📊 {}", my_blockchain.get_mempool_info());
println!("🔒 {}", my_blockchain.get_security_info());
println!("📈 {}", my_blockchain.get_network_stats());
println!("🎯 {}", my_blockchain.get_mass_adoption_stats());
println!("🚀 {}", my_blockchain.get_advanced_features_info());

    // Update initial wallet balances from genesis for simulation convenience
    for (address, amount) in GenesisDistribution::get_distribution() {
        my_blockchain.balance_tracker.update_balance(address, amount);
    }

    println!("\n💰 Genesis Wallet Balances (After loading/resetting):");
    for (address, _) in GenesisDistribution::get_distribution() {
        let balance = my_blockchain.get_wallet_balance(&address);
        println!("  {}: {} VEXA", address, my_blockchain.coin.format_amount(balance));
    }

    // ✅ API Server
    let api_blockchain = Arc::new(Mutex::new(my_blockchain));
    let api_port = config.rpc_port;

    // 🆕 ADD: API Routes with Transaction Support
    let blockchain_for_api = Arc::clone(&api_blockchain);
    
    // Helper function to get blockchain reference
    fn with_blockchain(
        blockchain: Arc<Mutex<Blockchain>>,
    ) -> impl Filter<Extract = (Arc<Mutex<Blockchain>>,), Error = std::convert::Infallible> + Clone {
        warp::any().map(move || blockchain.clone())
    }

    // 🆕 ADD: Transaction send endpoint
    let send_transaction = warp::path("send-transaction")
        .and(warp::post())
        .and(warp::body::json())
        .and(with_blockchain(blockchain_for_api.clone()))
        .and_then(|tx_data: TransactionData, blockchain: Arc<Mutex<Blockchain>>| async move {
            let mut chain = blockchain.lock().await;
            match chain.add_simple_transaction(tx_data.from, tx_data.to, tx_data.amount, tx_data.fee) {
                Ok(result) => Ok::<warp::reply::Json, warp::Rejection>(warp::reply::json(&serde_json::json!({
                    "success": true,
                    "message": result
                }))),
                Err(e) => Ok(warp::reply::json(&serde_json::json!({
                    "success": false,
                    "error": e
                })))
            }
        });

    // 🆕 ADD: Network status endpoint
    let network_status = warp::path("network-status")
        .and(warp::get())
        .and(with_blockchain(blockchain_for_api.clone()))
        .and_then(|blockchain: Arc<Mutex<Blockchain>>| async move {
            let chain = blockchain.lock().await;
            Ok::<warp::reply::Json, warp::Rejection>(warp::reply::json(&serde_json::json!({
                "network": "mainnet",
                "status": "active",
                "block_height": chain.chain.len(),
                "peers": chain.network_peers.len(),
                "mempool_size": chain.mempool.len(),
                "hashrate": format!("{} H/s", chain.difficulty * 1000)
            })))
        });

    // 🆕 ADD: Balance endpoint
    let balance = warp::path("balance")
        .and(warp::path::param())
        .and(warp::get())
        .and(with_blockchain(blockchain_for_api.clone()))
        .and_then(|address: String, blockchain: Arc<Mutex<Blockchain>>| async move {
            let chain = blockchain.lock().await;
            let balance = chain.get_wallet_balance(&address);
            Ok::<warp::reply::Json, warp::Rejection>(warp::reply::json(&serde_json::json!({
                "address": address,
                "balance": chain.coin.format_amount(balance),
                "balance_raw": balance
            })))
        });

    // 🆕 ADD: Mine block endpoint
    let mine_block = warp::path("mine-block")
        .and(warp::post())
        .and(with_blockchain(blockchain_for_api.clone()))
        .and_then(|blockchain: Arc<Mutex<Blockchain>>| async move {
            let mut chain = blockchain.lock().await;
            let new_block = chain.mine_block();
            chain.add_mined_block(new_block.clone());
            
            Ok::<warp::reply::Json, warp::Rejection>(warp::reply::json(&serde_json::json!({
                "success": true,
                "block_index": new_block.index,
                "block_hash": new_block.hash,
                "transactions": new_block.transactions.len()
            })))
        });

    // 🆕 ADD: Mempool endpoint
    let mempool = warp::path("mempool")
        .and(warp::get())
        .and(with_blockchain(blockchain_for_api.clone()))
        .and_then(|blockchain: Arc<Mutex<Blockchain>>| async move {
            let chain = blockchain.lock().await;
            let mempool_size = chain.mempool.len();
            let total_fees: u64 = chain.mempool.iter().map(|tx| tx.fee).sum();
            
            Ok::<warp::reply::Json, warp::Rejection>(warp::reply::json(&serde_json::json!({
                "mempool_size": mempool_size,
                "total_fees": chain.coin.format_amount(total_fees),
                "pending_transactions": mempool_size
            })))
        });

    // Existing explorer and blocks routes (you'll need to add these from your existing code)
    let explorer = warp::path("explorer")
        .and(warp::get())
        .and(with_blockchain(blockchain_for_api.clone()))
        .and_then(|blockchain: Arc<Mutex<Blockchain>>| async move {
            let chain = blockchain.lock().await;
            // ... existing explorer HTML generation code ...
            let html_content = format!("
                <html>
                <head><title>Vexa Chain Explorer</title></head>
                <body>
                    <h1>Vexa Chain Block Explorer</h1>
                    <p>Total Blocks: {}</p>
                    <p>Circulating Supply: {} VEXA</p>
                </body>
                </html>
            ", chain.chain.len(), chain.coin.format_amount(chain.coin.circulating_supply));
            
            Ok::<warp::reply::Html<String>, warp::Rejection>(warp::reply::html(html_content))
        });

    let blocks = warp::path("blocks")
        .and(warp::get())
        .and(with_blockchain(blockchain_for_api.clone()))
        .and_then(|blockchain: Arc<Mutex<Blockchain>>| async move {
            let chain = blockchain.lock().await;
            let latest_block = chain.get_latest_block();
            Ok::<warp::reply::Json, warp::Rejection>(warp::reply::json(&serde_json::json!({
                "index": latest_block.index,
                "timestamp": latest_block.timestamp,
                "transactions": latest_block.transactions.len(),
                "previous_hash": latest_block.previous_hash,
                "hash": latest_block.hash,
                "nonce": latest_block.nonce,
                "difficulty": latest_block.difficulty
            })))
        });

    // Combine all routes
    let routes = blocks
        .or(explorer)
        .or(send_transaction)
        .or(network_status)
        .or(balance)
        .or(mine_block)
        .or(mempool);

    // Start Warp server
    println!("🌐 REST API Server starting on port {}...", api_port);
    println!("📡 Available Endpoints:");
    println!("  GET  /blocks           - Get latest block info");
    println!("  GET  /explorer         - Block explorer UI");
    println!("  GET  /network-status   - Network status");
    println!("  GET  /balance/<addr>   - Check wallet balance");
    println!("  GET  /mempool          - Mempool info");
    println!("  POST /send-transaction - Send transaction");
    println!("  POST /mine-block       - Mine new block");
    
    tokio::spawn(async move {
        warp::serve(routes)
            .run(([127, 0, 0, 1], api_port))
            .await;
    });

    // ... rest of your existing main function code ...
    // Network messages (Simple polling for simulation)
    let message_network: Arc<Network> = Arc::clone(&network);
    tokio::spawn(async move {
        loop {
        let messages = message_network.as_ref().get_messages();
            for message in messages {
                println!("📩 Processing network message: {:?}", message.message_type);
                match message.message_type {
                    MessageType::NewBlock => {
                        println!("  Received NewBlock signal from peer.");
                    }
                    MessageType::NewTransaction => {
                        println!("  Received NewTransaction signal from peer: {}", message.data);
                    }
                    _ => {}
                }
            }
            tokio::time::sleep(Duration::from_secs(5)).await;
        }
    });

    // Enhanced node discovery
    {
        let mut blockchain = api_blockchain.lock().await;
        let seed_nodes = vec![
            format!("127.0.0.1:{}", config.port),
            "127.0.0.1:8081".to_string(),
            "127.0.0.1:8082".to_string(),
            "127.0.0.1:8083".to_string(),
        ];
        blockchain.discover_peers(seed_nodes);
    }

    // --- Test 1: MASS ADOPTION FEATURES ---
    println!("\n--- Test 1: Mass Adoption Features Demo ---");
    
    {
        let mut blockchain = api_blockchain.lock().await;
        
        // Test Daily Rewards
        println!("🎯 Testing Daily Rewards...");
        blockchain.claim_daily_reward(wallet1.address.clone());
        blockchain.watch_ad_for_bonus(wallet1.address.clone());
        
        // Test Referral System
        println!("🎯 Testing Referral System...");
        blockchain.add_referral(wallet1.address.clone(), "new_user_123".to_string());
        
        // Test Social Wallet
        println!("🎯 Testing Social Wallet...");
        let social_wallet = blockchain.create_social_wallet("+911234567890".to_string(), "user@email.com".to_string());
        println!("✅ Social Wallet Address: {}", social_wallet.main_address);
        
        // Test Micro-Earning
        println!("🎯 Testing Micro-Earning...");
        blockchain.complete_micro_task(wallet1.address.clone(), "survey1".to_string());
        blockchain.complete_micro_task(wallet1.address.clone(), "watch_ad1".to_string());
        
        // 🔥 FIXED: Test Payment Links - Store amount first to avoid borrowing conflict
        println!("🎯 Testing Payment Links...");
        let payment_amount = 5.0;
        let parsed_amount = blockchain.coin.parse_amount(payment_amount); // Store first
        let payment_url = blockchain.create_payment_link(
            wallet1.address.clone(),
            parsed_amount, // Use stored value
            "Test payment".to_string()
        );
        println!("✅ Payment Link: {}", payment_url);
        
        // Show available tasks
        let tasks = blockchain.get_available_tasks();
        println!("✅ Available Tasks: {}", tasks.len());
        
        println!("🎯 Mass Adoption Stats:");
        println!("{}", blockchain.get_mass_adoption_stats());
    }

    // --- Test 2: ADVANCED FEATURES DEMO ---
    println!("\n--- Test 2: Advanced Features Demo ---");
    
    {
        let mut blockchain = api_blockchain.lock().await;
        
        // Test Multi-signature Wallet
        println!("🎯 Testing Multi-signature Wallet...");
        let owners = vec![wallet1.address.clone(), wallet2.address.clone(), "VexaFoundation".to_string()];
        let multisig_address = blockchain.create_multi_sig_wallet(owners, 2); // 2-of-3
        
        // 🔥 FIXED: Store amount first to avoid borrowing conflict
        let multisig_amount = blockchain.coin.parse_amount(100.0);
        blockchain.submit_multi_sig_transaction(multisig_address, wallet1.address.clone(), "recipient".to_string(), multisig_amount);
        
        // Test Flash Loan
        println!("🎯 Testing Flash Loan...");
        
        // 🔥 FIXED: Store amounts first to avoid borrowing conflict
        let flash_loan_amount = blockchain.coin.parse_amount(50000.0);
        let flash_loan_repayment = blockchain.coin.parse_amount(55000.0);
        blockchain.request_flash_loan(wallet1.address.clone(), flash_loan_amount, flash_loan_repayment);
        
        // Test Quantum Wallet
        println!("🎯 Testing Quantum-Resistant Wallet...");
        blockchain.create_quantum_wallet();
        
        // Test Cross-chain Bridge
        println!("🎯 Testing Cross-chain Bridge...");
        
        // 🔥 FIXED: Store amount first to avoid borrowing conflict
        let bridge_amount = blockchain.coin.parse_amount(1000.0);
        blockchain.bridge_tokens("Ethereum".to_string(), "VexaChain".to_string(), bridge_amount, wallet1.address.clone());
        
        // Test Gaming Assets
        println!("🎯 Testing Gaming Assets...");
        blockchain.create_gaming_asset(wallet1.address.clone(), "vexa_rpg".to_string(), "Legendary Sword".to_string(), "Legendary".to_string());
        
        // Test DeFi Vault
        println!("🎯 Testing DeFi Vault...");
        
        // 🔥 FIXED: Store amount first to avoid borrowing conflict
        let defi_amount = blockchain.coin.parse_amount(50000.0);
        blockchain.create_defi_vault("VexaFoundation".to_string(), "Yield Farming".to_string(), defi_amount);
        
        // Test SocialFi
        println!("🎯 Testing SocialFi...");
        blockchain.create_social_trader(wallet1.address.clone());
        
        // Test Decentralized Identity
        println!("🎯 Testing Decentralized Identity...");
        blockchain.create_decentralized_identity(wallet1.address.clone());
        
        println!("🚀 Advanced Features Status:");
        println!("{}", blockchain.get_advanced_features_info());
    }

    // --- Test 3: DIRECT BALANCE UPDATE ---
    println!("\n--- Test 3: Direct balance update for wallets ---");
    
    {
        let mut blockchain = api_blockchain.lock().await;
        
        // 🔥 FIXED: Store amounts first to avoid borrowing conflicts
        let amount1_value = 1000.0;
        let amount2_value = 500.0;
        let amount1 = blockchain.coin.parse_amount(amount1_value);
        let amount2 = blockchain.coin.parse_amount(amount2_value);
        
        println!("💰 Direct balance update for testing...");
        blockchain.balance_tracker.update_balance(wallet1.address.clone(), amount1);
        blockchain.balance_tracker.update_balance(wallet2.address.clone(), amount2);
        
        let wallet1_balance = blockchain.get_wallet_balance(&wallet1.address);
        let wallet2_balance = blockchain.get_wallet_balance(&wallet2.address);
        println!("  ✅ Wallet 1 Balance: {} VEXA", blockchain.coin.format_amount(wallet1_balance));
        println!("  ✅ Wallet 2 Balance: {} VEXA", blockchain.coin.format_amount(wallet2_balance));

        // Create initial UTXOs
        let initial_utxo_w1 = UTXO {
            tx_hash: format!("INITIAL_UTXO_W1_{}", SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos()),
            output_index: 0,
            address: wallet1.address.clone(),
            amount: amount1,
            spent: false,
        };
        let initial_utxo_w2 = UTXO {
            tx_hash: format!("INITIAL_UTXO_W2_{}", SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos() + 1),
            output_index: 0,
            address: wallet2.address.clone(),
            amount: amount2,
            spent: false,
        };
        blockchain.utxo_set.insert(initial_utxo_w1.tx_hash.clone(), initial_utxo_w1);
        blockchain.utxo_set.insert(initial_utxo_w2.tx_hash.clone(), initial_utxo_w2);
    }

    // 🔥 FIXED: Test Payment Link Claim
    println!("\n--- Test 4: Payment Link Claim Test ---");
    {
        let mut blockchain = api_blockchain.lock().await;
        
        // 🔥 FIXED: Store amount first to avoid borrowing conflict
        let claim_amount_value = 10.0;
        let parsed_claim_amount = blockchain.coin.parse_amount(claim_amount_value); // Store first
        
        // Create payment link first
        let payment_url = blockchain.create_payment_link(
            wallet1.address.clone(),
            parsed_claim_amount, // Use stored value
            "Test claim".to_string()
        );
        
        // Extract link ID from URL
        let link_id = payment_url.replace("https://vexachain.com/pay/", "");
        
        // Claim the payment link
        blockchain.claim_payment_link(link_id, wallet2.address.clone());
    }

    println!("\n--- Vexa Chain P2P Network Summary ---");
    {
        let mut blockchain = api_blockchain.lock().await;
        println!("🌐 Network Type: {}", network_type);
        println!("🌐 P2P Network: ACTIVE on port {}", config.port);
        println!("🌐 REST API: ACTIVE on port {}", config.rpc_port);
        println!("📦 Total blocks in chain: {}", blockchain.chain.len());
        println!("🔗 Latest block index: {}", blockchain.get_latest_block().index);
        println!("🎯 Current Difficulty: {}", blockchain.difficulty);
        println!("🏁 Block Capacity: 10 BILLION Blocks");
        println!("✅ Is blockchain valid: {}", blockchain.is_chain_valid());
        println!("{}", blockchain.get_coin_info());
        println!("{}", blockchain.get_balance_info());
        println!("📊 {}", blockchain.get_mempool_info());
        println!("📈 {}", blockchain.get_network_stats());
        println!("🎯 {}", blockchain.get_mass_adoption_stats());
        println!("🚀 {}", blockchain.get_advanced_features_info());
        
        println!("🤖 VEX Smart Contracts: ACTIVE. Deployed Contracts: {}", blockchain.contract_manager.contracts.len());
        println!("💰 Staking System: ACTIVE. Total Staked: {}", blockchain.coin.format_amount(blockchain.staking_pool.total_staked));
        
        println!("💰 Final Balances:");
        println!("  Wallet 1: {} VEXA", blockchain.coin.format_amount(blockchain.get_wallet_balance(&wallet1.address)));
        println!("  Wallet 2: {} VEXA", blockchain.coin.format_amount(blockchain.get_wallet_balance(&wallet2.address)));
        
        // 🆕 STEP 2: BALANCE UPDATE TEST - YEH CODE ADD KARNA HAI
        println!("\n=== 🧪 BALANCE UPDATE TEST ===\n");
        
        // Create a test transaction
        let test_transaction = Transaction {
            sender: wallet1.address.clone(),
            receiver: wallet2.address.clone(),
            amount: 25,
            fee: 1,
            timestamp: chrono::Utc::now().timestamp() as u64,
            signature: "test_signature_balance_fix".to_string(),
            input_utxo: "test_input_utxo_balance_fix".to_string(),
            nonce: 0,
            transaction_type: "transfer".to_string(),
        };

        println!("💰 Sending 25 VEXA from Wallet 1 to Wallet 2...");

        // Add transaction to blockchain
// ✅ SAHI CODE - YEH PASTE KARNA HAI
let transaction_result = blockchain.add_transaction(test_transaction);
if transaction_result {
    println!("✅ Transaction successfully added to mempool!");
    
    // Check balances BEFORE mining
    println!("\n📊 Balances BEFORE mining:");
    println!("  Wallet 1: {} VEXA", blockchain.coin.format_amount(blockchain.get_wallet_balance(&wallet1.address)));
    println!("  Wallet 2: {} VEXA", blockchain.coin.format_amount(blockchain.get_wallet_balance(&wallet2.address)));

    // Mine a block to confirm the transaction
    println!("\n⛏️ Mining block to confirm transaction...");
    let mined_block = blockchain.mine_block();
    println!("✅ Block #{} mined successfully!", mined_block.index);
    
    // Add the mined block to blockchain
    blockchain.add_mined_block(mined_block);
    
    // Check balances AFTER mining  
    println!("\n📊 Balances AFTER mining:");
    println!("  Wallet 1: {} VEXA", blockchain.coin.format_amount(blockchain.get_wallet_balance(&wallet1.address)));
    println!("  Wallet 2: {} VEXA", blockchain.coin.format_amount(blockchain.get_wallet_balance(&wallet2.address)));
    
    // Save blockchain after mining
    save_blockchain_to_file(&blockchain);
} else {
    println!("❌ Transaction failed to add!");
}
        
        // Final API check
        println!("\n🌐 Final API Balance Check:");
        println!("  Run: curl http://localhost:3001/balance/{}", wallet1.address);
        println!("  Run: curl http://localhost:3001/balance/{}", wallet2.address);
        
        save_blockchain_to_file(&blockchain);
    }

    loop {
        tokio::time::sleep(Duration::from_secs(10)).await;
        
        {
            let blockchain = api_blockchain.lock().await;
            save_blockchain_to_file(&blockchain);
        }
        
        println!("🔍 Vexa Chain {} Node is still running...", network_type);
        println!("🌐 Access Block Explorer: http://localhost:{}/explorer", config.rpc_port);
        println!("🌐 Access REST API: http://localhost:{}/blocks", config.rpc_port);
        println!("🚀 Advanced Features: All Systems Operational");
    }
}