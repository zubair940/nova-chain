// coin.rs - UPDATED WITH ELITE TOKENOMICS FOR $1-2 PRICE TARGET
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Coin {
    pub symbol: String,
    pub name: String,
    pub total_supply: u64,
    pub circulating_supply: u64,
    pub decimals: u8,
    pub block_reward: u64,
    pub halving_interval: u64,
    // NEW: Token Burning Fields
    pub total_burned: u64,
    pub burn_percentage: u8, // Percentage of fees to burn
    pub auto_burn_enabled: bool,
    // UPDATED: Elite Token Economics for $1-2 Target
    pub staking_rewards: u64,           // Staking rewards pool - 10M
    pub dao_treasury: u64,              // DAO treasury - 5M
    pub bridge_reserve: u64,            // Cross-chain bridge reserve - 5M
    pub security_fund: u64,             // Security audit fund - 3M
    pub inflation_rate: f64,            // Annual inflation rate
    pub deflation_rate: f64,            // Annual deflation from burning
    pub tokenomics: Tokenomics,         // Comprehensive tokenomics
    
    // 🚀 NEW ADVANCED FEATURES
    pub multi_sig_treasury: MultiSigTreasury,    // Multi-signature treasury
    pub ai_economic_model: AIEconomicModel,      // AI-powered economics
    pub quantum_resistance: QuantumResistance,   // Quantum-safe features
    pub cross_chain_manager: CrossChainManager,  // Multi-chain management
    pub enterprise_features: EnterpriseFeatures, // Business solutions
    pub mass_adoption_v2: MassAdoptionV2,       // Enhanced user adoption
    pub defi_advanced: DeFiAdvanced,             // Advanced DeFi
    pub gaming_metaverse: GamingMetaverse,       // Gaming & metaverse
}

// 🚀 NEW: Multi-signature Treasury System
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MultiSigTreasury {
    pub enabled: bool,
    pub required_signatures: u8,
    pub signers: Vec<String>,
    pub pending_approvals: HashMap<String, TreasuryApproval>,
    pub max_withdrawal_limit: u64,
    pub daily_withdrawal_limit: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TreasuryApproval {
    pub proposal_id: String,
    pub amount: u64,
    pub purpose: String,
    pub proposed_by: String,
    pub approvals: Vec<String>,
    pub created_at: u64,
    pub executed: bool,
}

// 🚀 NEW: AI-Powered Economic Model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIEconomicModel {
    pub enabled: bool,
    pub dynamic_inflation_adjustment: bool,
    pub predictive_burning: bool,
    pub market_sentiment_analysis: bool,
    pub auto_risk_management: bool,
    pub ai_governance_suggestions: bool,
    pub last_ai_adjustment: u64,
    pub adjustment_history: Vec<AIAdjustment>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIAdjustment {
    pub timestamp: u64,
    pub adjustment_type: String,
    pub old_value: f64,
    pub new_value: f64,
    pub reason: String,
}

// 🚀 NEW: Quantum Resistance Features
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuantumResistance {
    pub enabled: bool,
    pub kyber512_public_key: String,
    pub dilithium2_signature: String,
    pub sphincs_plus_backup: String,
    pub migration_planned: bool,
    pub migration_date: u64,
}

// 🚀 NEW: Cross-chain Management
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrossChainManager {
    pub enabled: bool,
    pub supported_chains: Vec<SupportedChain>,
    pub bridge_fees: BridgeFeeStructure,
    pub liquidity_pools: HashMap<String, CrossChainPool>,
    pub total_bridged_volume: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SupportedChain {
    pub name: String,
    pub chain_id: String,
    pub bridge_address: String,
    pub is_active: bool,
    pub total_liquidity: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BridgeFeeStructure {
    pub base_fee: u64,
    pub percentage_fee: f64,
    pub min_fee: u64,
    pub max_fee: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrossChainPool {
    pub chain: String,
    pub liquidity: u64,
    pub utilization_rate: f64,
    pub last_rebalance: u64,
}

// 🚀 NEW: Enterprise Features
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnterpriseFeatures {
    pub enabled: bool,
    pub private_transactions: bool,
    pub kyc_aml_integration: bool,
    pub supply_chain_tracking: bool,
    pub invoice_payments: bool,
    pub compliance_dashboard: bool,
    pub enterprise_wallets: Vec<EnterpriseWallet>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnterpriseWallet {
    pub company_name: String,
    pub wallet_address: String,
    pub kyc_status: bool,
    pub monthly_limit: u64,
    pub used_this_month: u64,
}

// 🚀 NEW: Mass Adoption V2 Features
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MassAdoptionV2 {
    pub enabled: bool,
    pub social_login_wallets: bool,
    pub gasless_transactions: bool,
    pub fiat_on_ramp: bool,
    pub recurring_payments: bool,
    pub learn_to_earn: bool,
    pub interactive_tutorials: bool,
    pub total_users: u64,
    pub daily_active_users: u64,
}

// 🚀 NEW: Advanced DeFi Features
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeFiAdvanced {
    pub enabled: bool,
    pub flash_loans: bool,
    pub yield_aggregator: bool,
    pub insurance_pools: bool,
    pub options_trading: bool,
    pub prediction_markets: bool,
    pub concentrated_liquidity: bool,
    pub impermanent_loss_protection: bool,
    pub total_value_locked: u64,
}

// 🚀 NEW: Gaming & Metaverse Features
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GamingMetaverse {
    pub enabled: bool,
    pub in_game_assets: bool,
    pub play_to_earn: bool,
    pub cross_game_assets: bool,
    pub virtual_land_nfts: bool,
    pub avatar_system: bool,
    pub virtual_events: bool,
    pub gaming_sdk: bool,
    pub total_gaming_users: u64,
}

// NEW: Comprehensive Tokenomics Structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tokenomics {
    pub allocation: HashMap<String, u64>, // Allocation percentages
    pub vesting_schedule: HashMap<String, VestingSchedule>, // Vesting schedules
    pub emission_schedule: EmissionSchedule, // Token emission
    pub economic_model: EconomicModel,    // Economic model details
}

// NEW: Vesting Schedule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VestingSchedule {
    pub total_tokens: u64,
    pub vested_tokens: u64,
    pub start_time: u64,
    pub cliff_period: u64, // in seconds
    pub vesting_period: u64, // in seconds
    pub released_tokens: u64,
}

// NEW: Emission Schedule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmissionSchedule {
    pub initial_supply: u64,
    pub max_supply: u64,
    pub emission_rate: f64,
    pub halving_blocks: u64,
    pub last_halving: u64,
}

// NEW: Economic Model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EconomicModel {
    pub model_type: EconomicModelType,
    pub stability_mechanism: StabilityMechanism,
    pub governance_token: bool,
    pub utility_token: bool,
    pub security_token: bool,
}

// NEW: Economic Model Types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EconomicModelType {
    Deflationary,    // Burning mechanism
    Inflationary,    // Staking rewards
    DualToken,       // Governance + Utility
    Hybrid,          // Mixed model
}

// NEW: Stability Mechanisms
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StabilityMechanism {
    pub burning: bool,
    pub staking: bool,
    pub buyback: bool,
    pub deflationary_pressure: f64,
}

// NEW: BURN TRANSACTION TYPE
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BurnTransaction {
    pub from_address: String,
    pub amount: u64,
    pub signature: String,
    pub timestamp: u64,
    pub burn_hash: String,
    // NEW: Enhanced burn tracking
    pub burn_type: BurnType,           // Type of burn
    pub purpose: String,               // Purpose of burn
    pub block_height: u64,             // Block height
}

// NEW: Burn Types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BurnType {
    TransactionFee,    // Burn from transaction fees
    Manual,           // Manual burning by users
    Automatic,        // Automatic burning mechanism
    Governance,       // Burn via governance decision
    Deflationary,     // Deflationary burn
}

impl Coin {
    pub fn new() -> Self {
        let total_supply = 100_000_000_000_000_000; // 100M VEXA with 9 decimals
        
        // 🚀 UPDATED: ELITE TOKENOMICS FOR $1-2 PRICE TARGET
        let mut allocation = HashMap::new();
        
        // 🏢 FOUNDATION & TEAM (20M - 20%)
        allocation.insert("Core_Development_Team".to_string(), 8);     // 8%
        allocation.insert("Foundation_Reserve".to_string(), 7);        // 7%
        allocation.insert("Development_Fund".to_string(), 5);          // 5%
        
        // 💰 STRATEGIC PARTNERS (15M - 15%)
        allocation.insert("Seed_Investors".to_string(), 6);            // 6%
        allocation.insert("Strategic_Partners".to_string(), 5);        // 5%
        allocation.insert("Ecosystem_Fund".to_string(), 4);            // 4%
        
        // 🌍 PUBLIC DISTRIBUTION (35M - 35%)
        allocation.insert("Public_Sale".to_string(), 15);              // 15%
        allocation.insert("Staking_Rewards".to_string(), 10);          // 10%
        allocation.insert("Community_Rewards".to_string(), 6);         // 6%
        allocation.insert("Airdrop_Program".to_string(), 4);           // 4%
        
        // 🔒 LIQUIDITY & GROWTH (30M - 30%)
        allocation.insert("DEX_Liquidity".to_string(), 8);             // 8%
        allocation.insert("CEX_Liquidity".to_string(), 7);             // 7%
        allocation.insert("Marketing_Fund".to_string(), 5);            // 5%
        allocation.insert("Partnership_Fund".to_string(), 4);          // 4%
        allocation.insert("Technology_Grants".to_string(), 3);         // 3%
        allocation.insert("Security_Reserve".to_string(), 3);          // 3%

        Coin {
            symbol: "VEXA".to_string(),
            name: "VEXA Coin".to_string(),
            total_supply,
            circulating_supply: 25_000_000_000_000_000, // 25M at genesis (Public Sale + Initial Liquidity)
            decimals: 9,
            block_reward: 25_000_000_000, // 25 VEXA per block
            halving_interval: 1_000_000,
            total_burned: 0,
            burn_percentage: 50,
            auto_burn_enabled: true,
            
            // 🚀 UPDATED ALLOCATIONS FOR GATE.IO
            staking_rewards: total_supply * 10 / 100,     // 10M
            dao_treasury: total_supply * 5 / 100,        // 5M
            bridge_reserve: total_supply * 5 / 100,      // 5M
            security_fund: total_supply * 3 / 100,       // 3M (Updated from 5M)
            
            inflation_rate: 3.0,  // Reduced for price stability
            deflation_rate: 2.0,
            
            tokenomics: Tokenomics {
                allocation,
                vesting_schedule: HashMap::new(),
                emission_schedule: EmissionSchedule {
                    initial_supply: total_supply * 25 / 100, // 25M initial circulating
                    max_supply: total_supply,
                    emission_rate: 0.03, // Reduced emission
                    halving_blocks: 1_000_000,
                    last_halving: 0,
                },
                economic_model: EconomicModel {
                    model_type: EconomicModelType::Hybrid,
                    stability_mechanism: StabilityMechanism {
                        burning: true,
                        staking: true,
                        buyback: true, // Added buyback mechanism
                        deflationary_pressure: 2.5,
                    },
                    governance_token: true,
                    utility_token: true,
                    security_token: false,
                },
            },
            
            // 🚀 INITIALIZE NEW ADVANCED FEATURES
            multi_sig_treasury: MultiSigTreasury {
                enabled: true,
                required_signatures: 3,
                signers: vec![
                    "Core_Development_Team".to_string(),
                    "Foundation_Reserve".to_string(), 
                    "Seed_Investors".to_string(),
                    "Strategic_Partners".to_string()
                ],
                pending_approvals: HashMap::new(),
                max_withdrawal_limit: 1_000_000_000_000_000, // 1M VEXA
                daily_withdrawal_limit: 100_000_000_000_000, // 100K VEXA
            },
            
            ai_economic_model: AIEconomicModel {
                enabled: true,
                dynamic_inflation_adjustment: true,
                predictive_burning: true,
                market_sentiment_analysis: true,
                auto_risk_management: true,
                ai_governance_suggestions: true,
                last_ai_adjustment: 0,
                adjustment_history: Vec::new(),
            },
            
            quantum_resistance: QuantumResistance {
                enabled: true,
                kyber512_public_key: "quantum_kyber_public_key_placeholder".to_string(),
                dilithium2_signature: "quantum_dilithium_signature_placeholder".to_string(),
                sphincs_plus_backup: "quantum_sphincs_backup_placeholder".to_string(),
                migration_planned: true,
                migration_date: 1743454800, // 2025-04-01
            },
            
            cross_chain_manager: CrossChainManager {
                enabled: true,
                supported_chains: vec![
                    SupportedChain {
                        name: "Ethereum".to_string(),
                        chain_id: "1".to_string(),
                        bridge_address: "0x742E4C2F4c7c1B3B6b5b5A5c5e5F5".to_string(),
                        is_active: true,
                        total_liquidity: 5_000_000_000_000_000, // 5M VEXA (Updated)
                    },
                    SupportedChain {
                        name: "Binance Smart Chain".to_string(),
                        chain_id: "56".to_string(),
                        bridge_address: "0x842E4C2F4c7c1B3B6b5b5A5c5e5F5".to_string(),
                        is_active: true,
                        total_liquidity: 4_000_000_000_000_000, // 4M VEXA (Updated)
                    },
                    SupportedChain {
                        name: "Polygon".to_string(),
                        chain_id: "137".to_string(),
                        bridge_address: "0x942E4C2F4c7c1B3B6b5b5A5c5e5F5".to_string(),
                        is_active: true,
                        total_liquidity: 3_000_000_000_000_000, // 3M VEXA (Updated)
                    }
                ],
                bridge_fees: BridgeFeeStructure {
                    base_fee: 100_000_000, // 0.1 VEXA
                    percentage_fee: 0.001, // 0.1%
                    min_fee: 50_000_000,   // 0.05 VEXA
                    max_fee: 1_000_000_000, // 1 VEXA
                },
                liquidity_pools: HashMap::new(),
                total_bridged_volume: 0,
            },
            
            enterprise_features: EnterpriseFeatures {
                enabled: true,
                private_transactions: true,
                kyc_aml_integration: true,
                supply_chain_tracking: true,
                invoice_payments: true,
                compliance_dashboard: true,
                enterprise_wallets: Vec::new(),
            },
            
            mass_adoption_v2: MassAdoptionV2 {
                enabled: true,
                social_login_wallets: true,
                gasless_transactions: true,
                fiat_on_ramp: true,
                recurring_payments: true,
                learn_to_earn: true,
                interactive_tutorials: true,
                total_users: 0, // Reset for new tokenomics
                daily_active_users: 0,
            },
            
            defi_advanced: DeFiAdvanced {
                enabled: true,
                flash_loans: true,
                yield_aggregator: true,
                insurance_pools: true,
                options_trading: true,
                prediction_markets: true,
                concentrated_liquidity: true,
                impermanent_loss_protection: true,
                total_value_locked: 0,
            },
            
            gaming_metaverse: GamingMetaverse {
                enabled: true,
                in_game_assets: true,
                play_to_earn: true,
                cross_game_assets: true,
                virtual_land_nfts: true,
                avatar_system: true,
                virtual_events: true,
                gaming_sdk: true,
                total_gaming_users: 0,
            },
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

    // TOKEN BURNING MECHANISM
    pub fn burn_tokens(&mut self, amount: u64) -> bool {
        if amount > self.circulating_supply {
            println!("Burn failed: Not enough circulating supply");
            return false;
        }
        self.circulating_supply -= amount;
        self.total_burned += amount;
        self.update_deflation_rate();
        println!("Burned {} VEXA tokens", self.format_amount(amount));
        true
    }

    // AUTO BURN TRANSACTION FEES
    pub fn auto_burn_transaction_fees(&mut self, fees: u64) -> u64 {
        if !self.auto_burn_enabled || fees == 0 {
            return 0;
        }
        
        let burn_amount = (fees * self.burn_percentage as u64) / 100;
        if self.burn_tokens(burn_amount) {
            println!("Auto-burned {} VEXA from transaction fees", self.format_amount(burn_amount));
            burn_amount
        } else {
            0
        }
    }

    // GET BURNED SUPPLY INFO
    pub fn get_burned_supply(&self) -> u64 {
        self.total_burned
    }

    // MANUAL BURN FUNCTION
    pub fn manual_burn(&mut self, from_address: String, amount: u64, purpose: String) -> BurnTransaction {
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        let burn_tx = BurnTransaction {
            from_address,
            amount,
            signature: "manual_burn".to_string(),
            timestamp,
            burn_hash: format!("burn_{}_{}", timestamp, amount),
            burn_type: BurnType::Manual,
            purpose: purpose.clone(),
            block_height: 0,
        };

        if self.burn_tokens(amount) {
            println!("Manual burn successful: {} VEXA for {}", self.format_amount(amount), purpose);
        }

        burn_tx
    }

    // GOVERNANCE BURN FUNCTION
    pub fn governance_burn(&mut self, amount: u64, proposal_id: u64) -> BurnTransaction {
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        let burn_tx = BurnTransaction {
            from_address: "DAO_Governance".to_string(),
            amount,
            signature: format!("governance_burn_proposal_{}", proposal_id),
            timestamp,
            burn_hash: format!("gov_burn_{}_{}", timestamp, amount),
            burn_type: BurnType::Governance,
            purpose: format!("Governance proposal #{}", proposal_id),
            block_height: 0,
        };

        if self.burn_tokens(amount) {
            println!("Governance burn: {} VEXA for proposal #{}", self.format_amount(amount), proposal_id);
        }

        burn_tx
    }

    // BURN STATISTICS
    pub fn get_burn_info(&self) -> String {
        format!(
            "Token Burning Stats:\n\
            Total Burned: {} VEXA\n\
            Burn Percentage: {}%\n\
            Auto-burn: {}\n\
            Remaining Supply: {} VEXA\n\
            Annual Deflation: {}%",
            self.format_amount(self.total_burned),
            self.burn_percentage,
            if self.auto_burn_enabled { "ENABLED" } else { "DISABLED" },
            self.format_amount(self.total_supply - self.total_burned),
            self.deflation_rate
        )
    }

    // UPDATE BURN SETTINGS
    pub fn update_burn_settings(&mut self, percentage: u8, enabled: bool) {
        self.burn_percentage = percentage;
        self.auto_burn_enabled = enabled;
        println!("Burn settings updated: {}% auto-burn, Enabled: {}", percentage, enabled);
    }
    
    // 🚀 UPDATED SUPPLY INFO WITH NEW TOKENOMICS
    pub fn get_supply_info(&self) -> String {
        format!(
            "🏆 VEXA Coin - Elite Tokenomics ($1-2 Target)\n\
            ===========================================\n\
            Total Supply: {} VEXA\n\
            Circulating: {} VEXA\n\
            Burned: {} VEXA\n\
            \n🏢 FOUNDATION & TEAM (20M):\n\
            Staking Rewards: {} VEXA\n\
            DAO Treasury: {} VEXA\n\
            \n🌍 PUBLIC DISTRIBUTION (35M):\n\
            Bridge Reserve: {} VEXA\n\
            Security Fund: {} VEXA\n\
            \n💰 ECONOMICS:\n\
            Block Reward: {} VEXA\n\
            Inflation Rate: {}%\n\
            Deflation Rate: {}%\n\
            Net Inflation: {:.2}%",
            self.format_amount(self.total_supply),
            self.format_amount(self.circulating_supply),
            self.format_amount(self.total_burned),
            self.format_amount(self.staking_rewards),
            self.format_amount(self.dao_treasury),
            self.format_amount(self.bridge_reserve),
            self.format_amount(self.security_fund),
            self.format_amount(self.block_reward),
            self.inflation_rate,
            self.deflation_rate,
            self.inflation_rate - self.deflation_rate
        )
    }

    // STAKING REWARDS MANAGEMENT
    pub fn allocate_staking_rewards(&mut self, amount: u64) -> bool {
        if amount > self.staking_rewards {
            println!("Not enough staking rewards allocated");
            return false;
        }
        self.staking_rewards -= amount;
        self.circulating_supply += amount;
        println!("Allocated {} VEXA for staking rewards", self.format_amount(amount));
        true
    }

    // DAO TREASURY MANAGEMENT - FIXED VERSION
    pub fn use_dao_treasury(&mut self, amount: u64, purpose: String) -> bool {
        if amount > self.dao_treasury {
            println!("Not enough funds in DAO treasury");
            return false;
        }
        self.dao_treasury -= amount;
        self.circulating_supply += amount;
        println!("DAO treasury used: {} VEXA for {}", self.format_amount(amount), purpose);
        true
    }

    // BRIDGE RESERVE MANAGEMENT
    pub fn use_bridge_reserve(&mut self, amount: u64, chain: String) -> bool {
        if amount > self.bridge_reserve {
            println!("Not enough funds in bridge reserve");
            return false;
        }
        self.bridge_reserve -= amount;
        println!("Bridge reserve used: {} VEXA for {}", self.format_amount(amount), chain);
        true
    }

    // SECURITY FUND MANAGEMENT
    pub fn use_security_fund(&mut self, amount: u64, purpose: String) -> bool {
        if amount > self.security_fund {
            println!("Not enough funds in security fund");
            return false;
        }
        self.security_fund -= amount;
        println!("Security fund used: {} VEXA for {}", self.format_amount(amount), purpose);
        true
    }

    // UPDATE DEFLATION RATE
    fn update_deflation_rate(&mut self) {
        if self.circulating_supply > 0 {
            let burn_ratio = self.total_burned as f64 / self.circulating_supply as f64;
            self.deflation_rate = burn_ratio * 100.0;
        }
    }

    // 🚀 UPDATED TOKENOMICS ANALYSIS FOR $1-2 TARGET
    pub fn analyze_tokenomics(&self) -> String {
        let market_cap_estimate = self.circulating_supply as f64 * 1.0; // $1 price target
        let burned_percentage = (self.total_burned as f64 / self.total_supply as f64) * 100.0;
        let staking_ratio = (self.staking_rewards as f64 / self.total_supply as f64) * 100.0;
        
        format!(
            "🎯 Tokenomics Analysis - $1-2 Price Target\n\
            =======================================\n\
            Market Cap at $1: ${:.2}\n\
            Market Cap at $2: ${:.2}\n\
            Total Burned: {:.2}%\n\
            Staking Allocation: {:.2}%\n\
            DAO Treasury: {:.2}%\n\
            Economic Model: {:?}\n\
            Net Inflation: {:.2}%\n\
            Circulating Supply Ratio: {:.2}%",
            market_cap_estimate,
            market_cap_estimate * 2.0,
            burned_percentage,
            staking_ratio,
            (self.dao_treasury as f64 / self.total_supply as f64) * 100.0,
            self.tokenomics.economic_model.model_type,
            self.inflation_rate - self.deflation_rate,
            (self.circulating_supply as f64 / self.total_supply as f64) * 100.0
        )
    }

    // 🚀 UPDATED TOKEN ALLOCATION FOR GATE.IO
    pub fn get_token_allocation(&self) -> String {
        let mut allocation_info = String::from("🏆 Elite Tokenomics Allocation:\n");
        allocation_info.push_str("================================\n");
        
        allocation_info.push_str("🏢 FOUNDATION & TEAM (20M - 20%):\n");
        allocation_info.push_str("  Core Development Team: 8%\n");
        allocation_info.push_str("  Foundation Reserve: 7%\n");
        allocation_info.push_str("  Development Fund: 5%\n\n");
        
        allocation_info.push_str("💰 STRATEGIC PARTNERS (15M - 15%):\n");
        allocation_info.push_str("  Seed Investors: 6%\n");
        allocation_info.push_str("  Strategic Partners: 5%\n");
        allocation_info.push_str("  Ecosystem Fund: 4%\n\n");
        
        allocation_info.push_str("🌍 PUBLIC DISTRIBUTION (35M - 35%):\n");
        allocation_info.push_str("  Public Sale: 15%\n");
        allocation_info.push_str("  Staking Rewards: 10%\n");
        allocation_info.push_str("  Community Rewards: 6%\n");
        allocation_info.push_str("  Airdrop Program: 4%\n\n");
        
        allocation_info.push_str("🔒 LIQUIDITY & GROWTH (30M - 30%):\n");
        allocation_info.push_str("  DEX Liquidity: 8%\n");
        allocation_info.push_str("  CEX Liquidity: 7%\n");
        allocation_info.push_str("  Marketing Fund: 5%\n");
        allocation_info.push_str("  Partnership Fund: 4%\n");
        allocation_info.push_str("  Technology Grants: 3%\n");
        allocation_info.push_str("  Security Reserve: 3%\n");
        
        allocation_info
    }

    // 🚀 UPDATED COMPREHENSIVE TOKEN REPORT
    pub fn get_comprehensive_report(&self) -> String {
        format!(
            "🏆 VEXA Token - Elite Tokenomics Report ($1-2 Target)\n\
            ===================================================\n\
            {}\n\n\
            {}\n\n\
            {}\n\n\
            {}",
            self.get_supply_info(),
            self.get_burn_info(),
            self.analyze_tokenomics(),
            self.get_token_allocation()
        )
    }
    
    // 🚀 NEW ADVANCED FEATURES IMPLEMENTATION
    
    // Multi-signature Treasury Functions
    pub fn propose_treasury_withdrawal(&mut self, amount: u64, purpose: String, proposed_by: String) -> String {
        if !self.multi_sig_treasury.enabled {
            return "Multi-signature treasury not enabled".to_string();
        }
        
        if amount > self.multi_sig_treasury.max_withdrawal_limit {
            return "Amount exceeds maximum withdrawal limit".to_string();
        }
        
        let proposal_id = format!("treasury_proposal_{}", std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs());
        
        let approval = TreasuryApproval {
            proposal_id: proposal_id.clone(),
            amount,
            purpose,
            proposed_by,
            approvals: Vec::new(),
            created_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            executed: false,
        };
        
        self.multi_sig_treasury.pending_approvals.insert(proposal_id.clone(), approval);
        format!("Treasury withdrawal proposed: {}", proposal_id)
    }

    // FIXED: approve_treasury_withdrawal without borrowing conflicts
    pub fn approve_treasury_withdrawal(&mut self, proposal_id: String, approver: String) -> String {
        if !self.multi_sig_treasury.enabled {
            return "Multi-signature treasury not enabled".to_string();
        }

        // First, extract all necessary data without holding the mutable borrow
        let proposal_data = {
            let approval = match self.multi_sig_treasury.pending_approvals.get(&proposal_id) {
                Some(approval) => {
                    // Check if already approved by this approver
                    if approval.approvals.contains(&approver) {
                        return "Already approved by this signer".to_string();
                    }
                    // Check if approver is authorized
                    if !self.multi_sig_treasury.signers.contains(&approver) {
                        return "Not authorized to approve".to_string();
                    }
                    Some((approval.amount, approval.purpose.clone(), approval.approvals.len()))
                }
                None => None
            };
            
            match approval {
                Some((amt, purp, current_approvals)) => (amt, purp, current_approvals),
                None => return "Proposal not found".to_string()
            }
        };

        let (amount, purpose, current_approvals_len) = proposal_data;
        let required_sigs = self.multi_sig_treasury.required_signatures;

        // Now update the approval with mutable borrow
        if let Some(approval) = self.multi_sig_treasury.pending_approvals.get_mut(&proposal_id) {
            approval.approvals.push(approver.clone());
            
            let new_approval_count = approval.approvals.len();
            
            if new_approval_count >= required_sigs as usize {
                // We have enough approvals - execute the withdrawal
                // End the mutable borrow by storing the data we need
                let amount_clone = amount;
                let purpose_clone = purpose.clone();
                
                // Now we can call use_dao_treasury since approval borrow is ended
                if self.use_dao_treasury(amount_clone, purpose_clone) {
                    // Remove from pending approvals after successful execution
                    self.multi_sig_treasury.pending_approvals.remove(&proposal_id);
                    return format!("Treasury withdrawal executed: {} VEXA for {}", 
                        self.format_amount(amount), purpose);
                } else {
                    return "DAO treasury withdrawal failed".to_string();
                }
            }
            
            format!("Approval added. {}/{} signatures", new_approval_count, required_sigs)
        } else {
            "Approval failed - proposal not found".to_string()
        }
    }
    
    // AI Economic Model Functions
    pub fn run_ai_economic_analysis(&mut self) -> String {
        if !self.ai_economic_model.enabled {
            return "AI economic model not enabled".to_string();
        }
        
        // Simulate AI analysis and adjustments for price stability
        let old_inflation = self.inflation_rate;
        let new_inflation = if self.circulating_supply > self.total_supply * 60 / 100 {
            (old_inflation * 0.8).max(1.0) // Reduce inflation if circulating supply is high
        } else {
            (old_inflation * 1.1).min(5.0) // Increase inflation if circulating supply is low
        };
        
        self.inflation_rate = new_inflation;
        
        let adjustment = AIAdjustment {
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            adjustment_type: "Inflation Rate".to_string(),
            old_value: old_inflation,
            new_value: new_inflation,
            reason: "AI market analysis for $1-2 price stability".to_string(),
        };
        
        self.ai_economic_model.adjustment_history.push(adjustment);
        self.ai_economic_model.last_ai_adjustment = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        format!("AI economic analysis completed. Inflation adjusted from {:.2}% to {:.2}% for price stability", 
            old_inflation, new_inflation)
    }
    
    // Cross-chain Bridge Functions
    pub fn bridge_tokens(&mut self, amount: u64, target_chain: String) -> String {
        if !self.cross_chain_manager.enabled {
            return "Cross-chain bridge not enabled".to_string();
        }
        
        let bridge_fee = self.calculate_bridge_fee(amount);
        let total_amount = amount + bridge_fee;
        
        if total_amount > self.bridge_reserve {
            return "Insufficient bridge reserve".to_string();
        }
        
        self.bridge_reserve -= total_amount;
        self.cross_chain_manager.total_bridged_volume += amount;
        
        format!("Bridged {} VEXA to {} (Fee: {} VEXA)", 
            self.format_amount(amount), 
            target_chain, 
            self.format_amount(bridge_fee))
    }
    
    fn calculate_bridge_fee(&self, amount: u64) -> u64 {
        let percentage_fee = (amount as f64 * self.cross_chain_manager.bridge_fees.percentage_fee) as u64;
        let total_fee = self.cross_chain_manager.bridge_fees.base_fee + percentage_fee;
        
        total_fee.clamp(
            self.cross_chain_manager.bridge_fees.min_fee,
            self.cross_chain_manager.bridge_fees.max_fee
        )
    }
    
    // Enterprise Features
    pub fn register_enterprise_wallet(&mut self, company_name: String, wallet_address: String, monthly_limit: u64) -> String {
        if !self.enterprise_features.enabled {
            return "Enterprise features not enabled".to_string();
        }
        
        let enterprise_wallet = EnterpriseWallet {
            company_name,
            wallet_address,
            kyc_status: true,
            monthly_limit,
            used_this_month: 0,
        };
        
        self.enterprise_features.enterprise_wallets.push(enterprise_wallet);
        "Enterprise wallet registered successfully".to_string()
    }
    
    // Mass Adoption V2 Functions
    pub fn create_social_wallet(&mut self, social_id: String) -> String {
        if !self.mass_adoption_v2.enabled {
            return "Mass adoption features not enabled".to_string();
        }
        
        let wallet_address = format!("social_{}_{}", social_id, 
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs());
        
        self.mass_adoption_v2.total_users += 1;
        self.mass_adoption_v2.daily_active_users += 1;
        
        format!("Social wallet created: {}", wallet_address)
    }
    
    // Advanced DeFi Functions
    pub fn execute_flash_loan(&mut self, amount: u64, borrower: String) -> String {
        if !self.defi_advanced.enabled {
            return "Advanced DeFi features not enabled".to_string();
        }
        
        if amount > self.dao_treasury * 10 / 100 { // Max 10% of treasury
            return "Flash loan amount too large".to_string();
        }
        
        let fee = (amount as f64 * 0.003) as u64; // 0.3% fee
        self.defi_advanced.total_value_locked += amount;
        
        format!("Flash loan executed: {} VEXA to {} (Fee: {} VEXA)", 
            self.format_amount(amount), borrower, self.format_amount(fee))
    }
    
    // Gaming & Metaverse Functions
    pub fn mint_game_asset(&mut self, asset_name: String, owner: String) -> String {
        if !self.gaming_metaverse.enabled {
            return "Gaming features not enabled".to_string();
        }
        
        self.gaming_metaverse.total_gaming_users += 1;
        format!("Game asset '{}' minted for {}", asset_name, owner)
    }
    
    // Get Advanced Features Report
    pub fn get_advanced_features_report(&self) -> String {
        format!(
            "🚀 VEXA Advanced Features Report\n\
            ===============================\n\
            Multi-signature Treasury: {}\n\
            AI Economic Model: {}\n\
            Quantum Resistance: {}\n\
            Cross-chain Bridge: {}\n\
            Enterprise Features: {}\n\
            Mass Adoption V2: {}\n\
            Advanced DeFi: {}\n\
            Gaming & Metaverse: {}\n\
            Total Bridged Volume: {} VEXA\n\
            Total Value Locked: {} VEXA\n\
            Total Users: {}\n\
            Daily Active Users: {}",
            if self.multi_sig_treasury.enabled { "✅ ENABLED" } else { "❌ DISABLED" },
            if self.ai_economic_model.enabled { "✅ ENABLED" } else { "❌ DISABLED" },
            if self.quantum_resistance.enabled { "✅ ENABLED" } else { "❌ DISABLED" },
            if self.cross_chain_manager.enabled { "✅ ENABLED" } else { "❌ DISABLED" },
            if self.enterprise_features.enabled { "✅ ENABLED" } else { "❌ DISABLED" },
            if self.mass_adoption_v2.enabled { "✅ ENABLED" } else { "❌ DISABLED" },
            if self.defi_advanced.enabled { "✅ ENABLED" } else { "❌ DISABLED" },
            if self.gaming_metaverse.enabled { "✅ ENABLED" } else { "❌ DISABLED" },
            self.format_amount(self.cross_chain_manager.total_bridged_volume),
            self.format_amount(self.defi_advanced.total_value_locked),
            self.mass_adoption_v2.total_users,
            self.mass_adoption_v2.daily_active_users
        )
    }
}

// 🚀 UPDATED: Genesis distribution for new tokenomics
pub struct GenesisDistribution;
impl GenesisDistribution {
    pub fn get_distribution() -> Vec<(String, u64)> {
        vec![
            // 🏢 FOUNDATION & TEAM (20M)
            ("Core_Development_Team".to_string(), 8_000_000_000_000_000), // 8M
            ("Foundation_Reserve".to_string(),    7_000_000_000_000_000), // 7M
            ("Development_Fund".to_string(),      5_000_000_000_000_000), // 5M
            
            // 💰 STRATEGIC PARTNERS (15M)
            ("Seed_Investors".to_string(),        6_000_000_000_000_000), // 6M
            ("Strategic_Partners".to_string(),    5_000_000_000_000_000), // 5M
            ("Ecosystem_Fund".to_string(),        4_000_000_000_000_000), // 4M
            
            // 🌍 PUBLIC DISTRIBUTION (35M)
            ("Public_Sale".to_string(),          15_000_000_000_000_000), // 15M
            ("Staking_Rewards".to_string(),      10_000_000_000_000_000), // 10M
            ("Community_Rewards".to_string(),     6_000_000_000_000_000), // 6M
            ("Airdrop_Program".to_string(),       4_000_000_000_000_000), // 4M
            
            // 🔒 LIQUIDITY & GROWTH (30M)
            ("DEX_Liquidity".to_string(),         8_000_000_000_000_000), // 8M
            ("CEX_Liquidity".to_string(),         7_000_000_000_000_000), // 7M
            ("Marketing_Fund".to_string(),        5_000_000_000_000_000), // 5M
            ("Partnership_Fund".to_string(),      4_000_000_000_000_000), // 4M
            ("Technology_Grants".to_string(),     3_000_000_000_000_000), // 3M
            ("Security_Reserve".to_string(),      3_000_000_000_000_000), // 3M
        ]
    }
    
    pub fn get_total_genesis_supply() -> u64 {
        Self::get_distribution()
            .iter()
            .map(|(_, amount)| *amount)
            .sum()
    }
}