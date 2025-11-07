// src/flash_loan.rs
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

// Flash Loan Contract Structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlashLoan {
    pub id: String,
    pub borrower: String,
    pub amount: u64,
    pub asset: String,                    // VEXA, USDC, etc.
    pub fee_percentage: f64,              // 0.3% typical
    pub fee_amount: u64,
    pub collateral_required: u64,         // For risky loans
    pub timestamp: u64,
    pub expiry_time: u64,
    pub status: FlashLoanStatus,
    pub repayment_time: Option<u64>,
    pub liquidated: bool,
    pub loan_type: LoanType,
    pub risk_level: RiskLevel,
}

// Flash Loan Pool for Liquidity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlashLoanPool {
    pub pool_id: String,
    pub asset: String,
    pub total_liquidity: u64,
    pub available_liquidity: u64,
    pub borrowed_liquidity: u64,
    pub interest_earned: u64,
    pub pool_fee: f64,
    pub pool_owners: Vec<String>,
    pub min_loan: u64,
    pub max_loan: u64,
    pub created_at: u64,
}

// Advanced Flash Loan with AI Risk Assessment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdvancedFlashLoan {
    pub base_loan: FlashLoan,
    pub risk_score: f64,                  // AI-generated risk score (0-1)
    pub collateralization_ratio: f64,
    pub borrower_credit_score: f64,
    pub loan_history: Vec<LoanRecord>,
    pub insurance_coverage: bool,
    pub automated_liquidation: bool,
    pub yield_farming_integration: bool,
}

// Supporting Enums
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum FlashLoanStatus {
    Active,
    Repaid,
    Defaulted,
    Liquidated,
    Pending,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LoanType {
    Arbitrage,           // Price differences across exchanges
    Liquidation,         // Liquidate undercollateralized positions
    YieldFarming,        // Farm yield without capital
    CollateralSwap,      // Swap collateral types
    DebtRefinancing,     // Refinance existing debt
    NFTPurchase,         // Buy NFTs and sell later
    CrossChainArbitrage, // Arbitrage between different chains
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RiskLevel {
    Low,     // Safe arbitrage
    Medium,  // Yield farming
    High,    // NFT trading
    VeryHigh,// Experimental strategies
}

// Loan History Record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoanRecord {
    pub loan_id: String,
    pub amount: u64,
    pub success: bool,
    pub profit: i64,     // Can be negative
    pub timestamp: u64,
}

// Flash Loan Manager
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlashLoanManager {
    pub active_loans: HashMap<String, FlashLoan>,
    pub loan_pools: HashMap<String, FlashLoanPool>,
    pub loan_history: HashMap<String, Vec<LoanRecord>>,
    pub total_volume: u64,
    pub total_fees_earned: u64,
    pub default_rate: f64,
    pub insurance_fund: u64,
    pub max_loan_to_value: f64,
    pub risk_parameters: RiskParameters,
    pub ai_risk_analyzer: AIRiskAnalyzer,
}

// AI Risk Analysis System
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIRiskAnalyzer {
    pub model_version: String,
    pub risk_threshold: f64,
    pub market_volatility_factor: f64,
    pub borrower_reputation_score: bool,
    pub real_time_monitoring: bool,
}

// Risk Management Parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskParameters {
    pub max_loan_amount: u64,
    pub min_collateral_ratio: f64,
    pub liquidation_threshold: f64,
    pub insurance_coverage_ratio: f64,
    pub volatility_adjustment: f64,
}

impl FlashLoan {
    pub fn new(
        borrower: String,
        amount: u64,
        asset: String,
        fee_percentage: f64,
        loan_type: LoanType,
        duration_seconds: u64,
    ) -> Self {
        let current_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        let fee_amount = (amount as f64 * fee_percentage / 100.0) as u64;
        let collateral_required = match loan_type {
            LoanType::Arbitrage => (amount as f64 * 0.05) as u64, // 5% collateral for safe arbitrage
            LoanType::Liquidation => (amount as f64 * 0.1) as u64, // 10% for liquidation
            LoanType::YieldFarming => (amount as f64 * 0.15) as u64, // 15% for yield farming
            LoanType::NFTPurchase => (amount as f64 * 0.25) as u64, // 25% for NFT trading
            _ => (amount as f64 * 0.2) as u64, // 20% for others
        };

        Self {
            id: format!("flash_loan_{}_{}", borrower, current_time),
            borrower,
            amount,
            asset,
            fee_percentage,
            fee_amount,
            collateral_required,
            timestamp: current_time,
            expiry_time: current_time + duration_seconds,
            status: FlashLoanStatus::Pending,
            repayment_time: None,
            liquidated: false,
            loan_type,
            risk_level: RiskLevel::Medium,
        }
    }

    // Execute flash loan - must be repaid in same transaction
    pub fn execute_loan(&mut self) -> bool {
        if self.status == FlashLoanStatus::Pending {
            self.status = FlashLoanStatus::Active;
            println!("🚀 Flash loan executed: {} {} to {}", self.amount, self.asset, self.borrower);
            true
        } else {
            false
        }
    }

    // Repay flash loan
    pub fn repay_loan(&mut self, repaid_amount: u64) -> bool {
        let total_owed = self.amount + self.fee_amount;
        
        if repaid_amount >= total_owed && self.status == FlashLoanStatus::Active {
            self.status = FlashLoanStatus::Repaid;
            self.repayment_time = Some(
                SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs()
            );
            println!("✅ Flash loan repaid: {} {}", self.amount, self.asset);
            true
        } else {
            false
        }
    }

    // Liquidate if not repaid in time
    pub fn liquidate(&mut self) -> bool {
        let current_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        if current_time > self.expiry_time && self.status == FlashLoanStatus::Active {
            self.status = FlashLoanStatus::Liquidated;
            self.liquidated = true;
            println!("💥 Flash loan liquidated: {}", self.id);
            true
        } else {
            false
        }
    }

    // Calculate profit for arbitrage
    pub fn calculate_arbitrage_profit(&self, buy_price: f64, sell_price: f64) -> i64 {
        let amount_f64 = self.amount as f64;
        let buy_cost = amount_f64 * buy_price;
        let sell_revenue = amount_f64 * sell_price;
        let profit = sell_revenue - buy_cost - (self.fee_amount as f64);
        profit as i64
    }
}

impl FlashLoanPool {
    pub fn new(asset: String, initial_liquidity: u64, pool_fee: f64, owners: Vec<String>) -> Self {
        let current_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        Self {
            pool_id: format!("pool_{}_{}", asset, current_time),
            asset,
            total_liquidity: initial_liquidity,
            available_liquidity: initial_liquidity,
            borrowed_liquidity: 0,
            interest_earned: 0,
            pool_fee,
            pool_owners: owners,
            min_loan: 1000, // Minimum 1000 tokens
            max_loan: initial_liquidity / 10, // Max 10% of pool
            created_at: current_time,
        }
    }

    // Provide liquidity to pool
    pub fn provide_liquidity(&mut self, amount: u64, provider: String) -> bool {
        self.total_liquidity += amount;
        self.available_liquidity += amount;
        
        if !self.pool_owners.contains(&provider) {
            self.pool_owners.push(provider);
        }
        
        println!("💰 Liquidity provided: {} {} to pool", amount, self.asset);
        true
    }

    // Withdraw liquidity from pool
    pub fn withdraw_liquidity(&mut self, amount: u64, provider: String) -> bool {
        if self.pool_owners.contains(&provider) && self.available_liquidity >= amount {
            self.total_liquidity -= amount;
            self.available_liquidity -= amount;
            println!("💸 Liquidity withdrawn: {} {} from pool", amount, self.asset);
            true
        } else {
            false
        }
    }

    // Check if loan can be granted
    pub fn can_grant_loan(&self, amount: u64) -> bool {
        amount >= self.min_loan && 
        amount <= self.max_loan && 
        amount <= self.available_liquidity
    }

    // Grant flash loan
    pub fn grant_loan(&mut self, amount: u64) -> bool {
        if self.can_grant_loan(amount) {
            self.available_liquidity -= amount;
            self.borrowed_liquidity += amount;
            true
        } else {
            false
        }
    }

    // Process loan repayment
    pub fn process_repayment(&mut self, amount: u64, fee: u64) -> bool {
        self.available_liquidity += amount + fee;
        self.borrowed_liquidity -= amount;
        self.interest_earned += fee;
        true
    }
}

impl FlashLoanManager {
    pub fn new() -> Self {
        Self {
            active_loans: HashMap::new(),
            loan_pools: HashMap::new(),
            loan_history: HashMap::new(),
            total_volume: 0,
            total_fees_earned: 0,
            default_rate: 0.0,
            insurance_fund: 0,
            max_loan_to_value: 0.9, // 90% LTV
            risk_parameters: RiskParameters {
                max_loan_amount: 1_000_000_000, // 1M VEXA max
                min_collateral_ratio: 0.05,     // 5% minimum collateral
                liquidation_threshold: 0.03,    // 3% liquidation threshold
                insurance_coverage_ratio: 0.01, // 1% insurance
                volatility_adjustment: 1.2,     // 20% volatility buffer
            },
            ai_risk_analyzer: AIRiskAnalyzer {
                model_version: "v2.1".to_string(),
                risk_threshold: 0.7,
                market_volatility_factor: 1.0,
                borrower_reputation_score: true,
                real_time_monitoring: true,
            },
        }
    }

    // Create new flash loan pool
    pub fn create_pool(&mut self, asset: String, initial_liquidity: u64, pool_fee: f64, owners: Vec<String>) -> String {
        let pool = FlashLoanPool::new(asset, initial_liquidity, pool_fee, owners);
        let pool_id = pool.pool_id.clone();
        self.loan_pools.insert(pool_id.clone(), pool);
        pool_id
    }

    // Request flash loan
    pub fn request_flash_loan(
        &mut self,
        borrower: String,
        amount: u64,
        asset: String,
        loan_type: LoanType,
        duration_seconds: u64,
    ) -> Option<String> {
        // Find suitable pool
        let suitable_pool = self.loan_pools.values_mut().find(|pool| 
            pool.asset == asset && pool.can_grant_loan(amount)
        );

        if let Some(pool) = suitable_pool {
            // AI Risk Assessment
            let risk_score = self.assess_loan_risk(&borrower, amount, &loan_type);
            
            if risk_score <= self.ai_risk_analyzer.risk_threshold {
                let fee_percentage = pool.pool_fee;
                let mut loan = FlashLoan::new(
                    borrower.clone(),
                    amount,
                    asset,
                    fee_percentage,
                    loan_type,
                    duration_seconds,
                );

                // Adjust risk level based on AI assessment
                loan.risk_level = if risk_score < 0.3 {
                    RiskLevel::Low
                } else if risk_score < 0.6 {
                    RiskLevel::Medium
                } else if risk_score < 0.8 {
                    RiskLevel::High
                } else {
                    RiskLevel::VeryHigh
                };

                if pool.grant_loan(amount) {
                    loan.execute_loan();
                    let loan_id = loan.id.clone();
                    self.active_loans.insert(loan_id.clone(), loan);
                    self.total_volume += amount;
                    
                    // Add to borrower history
                    self.loan_history
                        .entry(borrower)
                        .or_insert_with(Vec::new)
                        .push(LoanRecord {
                            loan_id: loan_id.clone(),
                            amount,
                            success: true,
                            profit: 0,
                            timestamp: SystemTime::now()
                                .duration_since(UNIX_EPOCH)
                                .unwrap()
                                .as_secs(),
                        });

                    Some(loan_id)
                } else {
                    None
                }
            } else {
                println!("❌ Loan rejected: High risk score {}", risk_score);
                None
            }
        } else {
            None
        }
    }

    // Repay flash loan
    pub fn repay_flash_loan(&mut self, loan_id: String, repaid_amount: u64) -> bool {
        if let Some(loan) = self.active_loans.get_mut(&loan_id) {
            if loan.repay_loan(repaid_amount) {
                // Find pool and process repayment
                if let Some(pool) = self.loan_pools.get_mut(&loan.asset) {
                    pool.process_repayment(loan.amount, loan.fee_amount);
                    self.total_fees_earned += loan.fee_amount;
                    
                    // Update loan history with profit calculation
                    if let Some(history) = self.loan_history.get_mut(&loan.borrower) {
                        if let Some(record) = history.last_mut() {
                            record.success = true;
                            // Simple profit calculation (in real implementation, this would be more complex)
                            record.profit = (repaid_amount as i64) - (loan.amount as i64) - (loan.fee_amount as i64);
                        }
                    }
                    
                    self.active_loans.remove(&loan_id);
                    return true;
                }
            }
        }
        false
    }

    // AI-powered risk assessment
    fn assess_loan_risk(&self, borrower: &str, amount: u64, loan_type: &LoanType) -> f64 {
        let mut risk_score = 0.5; // Base risk

        // Adjust based on loan type
        match loan_type {
            LoanType::Arbitrage => risk_score -= 0.2,      // Lower risk
            LoanType::Liquidation => risk_score -= 0.1,    // Medium-low risk
            LoanType::YieldFarming => risk_score += 0.1,   // Medium-high risk
            LoanType::NFTPurchase => risk_score += 0.3,    // High risk
            LoanType::CrossChainArbitrage => risk_score += 0.2, // Medium risk
            _ => risk_score += 0.1,                        // Default medium risk
        }

        // Adjust based on amount (larger loans = higher risk)
        if amount > 1_000_000 {
            risk_score += 0.2;
        } else if amount > 100_000 {
            risk_score += 0.1;
        }

        // Adjust based on borrower history
        if let Some(history) = self.loan_history.get(borrower) {
            let success_rate = history.iter().filter(|r| r.success).count() as f64 / history.len() as f64;
            risk_score += (1.0 - success_rate) * 0.3; // Higher risk for poor history
        }

        // Apply market volatility
        risk_score *= self.ai_risk_analyzer.market_volatility_factor;

        risk_score.max(0.0).min(1.0) // Clamp between 0 and 1
    }

    // Liquidate expired loans
    pub fn liquidate_expired_loans(&mut self) -> Vec<String> {
        let mut liquidated = Vec::new();
        let current_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let expired_loans: Vec<String> = self.active_loans
            .iter()
            .filter(|(_, loan)| current_time > loan.expiry_time && loan.status == FlashLoanStatus::Active)
            .map(|(id, _)| id.clone())
            .collect();

        for loan_id in expired_loans {
            if let Some(loan) = self.active_loans.get_mut(&loan_id) {
                if loan.liquidate() {
                    liquidated.push(loan_id.clone());
                    
                    // Use insurance fund to cover losses
                    let loss = loan.amount + loan.fee_amount;
                    if self.insurance_fund >= loss {
                        self.insurance_fund -= loss;
                    } else {
                        // Partial coverage or pool takes loss
                        println!("⚠️  Insurance fund insufficient for loan: {}", loan_id);
                    }
                }
            }
        }

        liquidated
    }

    // Get pool statistics
    pub fn get_pool_stats(&self, pool_id: &str) -> Option<String> {
        self.loan_pools.get(pool_id).map(|pool| {
            format!(
                "Pool {}: Total Liquidity: {}, Available: {}, Borrowed: {}, Fees Earned: {}",
                pool.asset, pool.total_liquidity, pool.available_liquidity, 
                pool.borrowed_liquidity, pool.interest_earned
            )
        })
    }

    // Add to insurance fund
    pub fn add_insurance_fund(&mut self, amount: u64) {
        self.insurance_fund += amount;
        println!("🛡️  Insurance fund increased to: {}", self.insurance_fund);
    }

    // Get system statistics
    pub fn get_system_stats(&self) -> String {
        format!(
            "Flash Loan System Stats:\n\
            Total Volume: {} VEXA\n\
            Total Fees Earned: {} VEXA\n\
            Active Loans: {}\n\
            Loan Pools: {}\n\
            Insurance Fund: {} VEXA\n\
            Default Rate: {:.2}%",
            self.total_volume,
            self.total_fees_earned,
            self.active_loans.len(),
            self.loan_pools.len(),
            self.insurance_fund,
            self.default_rate * 100.0
        )
    }
}

// Implementation of Default trait for easy initialization
impl Default for FlashLoanManager {
    fn default() -> Self {
        Self::new()
    }
}

// Unit tests for flash loan system
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_flash_loan_creation() {
        let loan = FlashLoan::new(
            "test_borrower".to_string(),
            1000,
            "VEXA".to_string(),
            0.3,
            LoanType::Arbitrage,
            300, // 5 minutes
        );

        assert_eq!(loan.amount, 1000);
        assert_eq!(loan.asset, "VEXA");
        assert_eq!(loan.fee_percentage, 0.3);
        assert_eq!(loan.status, FlashLoanStatus::Pending);
    }

    #[test]
    fn test_loan_repayment() {
        let mut loan = FlashLoan::new(
            "test_borrower".to_string(),
            1000,
            "VEXA".to_string(),
            0.3,
            LoanType::Arbitrage,
            300,
        );

        loan.execute_loan();
        assert!(loan.repay_loan(1003)); // 1000 + 3 fee
        assert_eq!(loan.status, FlashLoanStatus::Repaid);
    }

    #[test]
    fn test_pool_operations() {
        let mut pool = FlashLoanPool::new(
            "VEXA".to_string(),
            10000,
            0.3,
            vec!["owner1".to_string()],
        );

        assert!(pool.provide_liquidity(5000, "owner2".to_string()));
        assert_eq!(pool.total_liquidity, 15000);
        assert!(pool.can_grant_loan(1000));
    }
}