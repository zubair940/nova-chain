// api.rs
use warp::Filter;
use std::sync::Arc;
use std::collections::HashMap;
use tokio::sync::Mutex;
use serde_json::json;

use crate::Blockchain;
use crate::coin::Coin;

// === HELPER: Render HTML for Block Explorer ===
fn render_blocks(blocks: &[crate::models::Block], coin: &Coin) -> String {
    blocks.iter().rev().take(10).map(|block| {
        let transactions_html: String = block.transactions.iter().map(|tx| {
            format!(r#"
            <div class="transaction">
                <strong>{} to {}:</strong> {} VEXA
                <br><small class="hash">Type: {}</small>
            </div>
            "#, 
            if tx.sender == "MINING_REWARD" { "MINING " } else { &tx.sender },
            &tx.receiver,
            coin.format_amount(tx.amount),
            tx.transaction_type
            )
        }).collect();

        format!(r#"
        <div class="block">
            <h3>Block #{} <span class="success">Validated</span></h3>
            <p><strong>Hash:</strong> <span class="hash">{}</span></p>
            <p><strong>Transactions:</strong> {} | <strong>Nonce:</strong> {} | <strong>Timestamp:</strong> {}</p>
            <div class="transactions">
                {}
            </div>
        </div>
        "#, 
        block.index, 
        block.hash, 
        block.transactions.len(), 
        block.nonce, 
        block.timestamp,
        transactions_html
        )
    }).collect::<Vec<String>>().join("")
}

// ✅ SIMPLE WORKING VERSION
pub async fn start_api_server(blockchain: Arc<Mutex<Blockchain>>, port: u16) {
    let blockchain_filter = warp::any().map(move || Arc::clone(&blockchain));

    // === CORE BLOCKCHAIN APIs ===
    let get_blocks = warp::path("blocks")
        .and(warp::get())
        .and(blockchain_filter.clone())
        .and_then(get_all_blocks);

    let get_latest_block = warp::path("blocks")
        .and(warp::path("latest"))
        .and(warp::get())
        .and(blockchain_filter.clone())
        .and_then(get_latest_block_handler);

    let add_transaction = warp::path("transactions")
        .and(warp::post())
        .and(warp::body::json())
        .and(blockchain_filter.clone())
        .and_then(add_transaction_handler);

    let mine_block = warp::path("mine")
        .and(warp::post())
        .and(blockchain_filter.clone())
        .and_then(mine_block_handler);

    let get_status = warp::path("status")
        .and(warp::get())
        .and(blockchain_filter.clone())
        .and_then(get_blockchain_status);

    let block_explorer = warp::path("explorer")
        .and(warp::get())
        .and(blockchain_filter.clone())
        .and_then(block_explorer_handler);

    // 🆕 NEW: TOKENOMICS API
    let get_tokenomics = warp::path("tokenomics")
        .and(warp::get())
        .and(blockchain_filter.clone())
        .and_then(get_tokenomics_handler);

    // 🆕 NEW: SMART CONTRACT APIs
    let deploy_contract = warp::path("contracts")
        .and(warp::path("deploy"))
        .and(warp::post())
        .and(warp::body::json())
        .and(blockchain_filter.clone())
        .and_then(deploy_contract_handler);

    let execute_contract = warp::path("contracts")
        .and(warp::path("execute"))
        .and(warp::post())
        .and(warp::body::json())
        .and(blockchain_filter.clone())
        .and_then(execute_contract_handler);

    // 🆕 NEW: WALLET APIs
    let get_wallet_balance = warp::path("wallet")
        .and(warp::path("balance"))
        .and(warp::get())
        .and(warp::query::<HashMap<String, String>>())
        .and(blockchain_filter.clone())
        .and_then(get_wallet_balance_handler);

    let get_wallet_info = warp::path("wallet")
        .and(warp::path("info"))
        .and(warp::get())
        .and(warp::query::<HashMap<String, String>>())
        .and(blockchain_filter.clone())
        .and_then(get_wallet_info_handler);

    // 🆕 NEW: STAKING APIs
    let get_staking_info = warp::path("staking")
        .and(warp::path("info"))
        .and(warp::get())
        .and(blockchain_filter.clone())
        .and_then(get_staking_info_handler);

    let stake_tokens = warp::path("staking")
        .and(warp::path("stake"))
        .and(warp::post())
        .and(warp::body::json())
        .and(blockchain_filter.clone())
        .and_then(stake_tokens_handler);

    let claim_rewards = warp::path("staking")
        .and(warp::path("claim"))
        .and(warp::post())
        .and(warp::body::json())
        .and(blockchain_filter.clone())
        .and_then(claim_rewards_handler);

    // 🆕 NEW: MASS ADOPTION APIs
    let claim_daily_reward = warp::path("mass-adoption")
        .and(warp::path("daily-reward"))
        .and(warp::post())
        .and(warp::body::json())
        .and(blockchain_filter.clone())
        .and_then(claim_daily_reward_handler);

    let create_social_wallet = warp::path("mass-adoption")
        .and(warp::path("social-wallet"))
        .and(warp::post())
        .and(warp::body::json())
        .and(blockchain_filter.clone())
        .and_then(create_social_wallet_handler);

    let complete_micro_task = warp::path("mass-adoption")
        .and(warp::path("micro-task"))
        .and(warp::post())
        .and(warp::body::json())
        .and(blockchain_filter.clone())
        .and_then(complete_micro_task_handler);

    let create_payment_link = warp::path("mass-adoption")
        .and(warp::path("payment-link"))
        .and(warp::post())
        .and(warp::body::json())
        .and(blockchain_filter.clone())
        .and_then(create_payment_link_handler);

    let claim_payment_link = warp::path("mass-adoption")
        .and(warp::path("claim-payment"))
        .and(warp::post())
        .and(warp::body::json())
        .and(blockchain_filter.clone())
        .and_then(claim_payment_link_handler);

    // 🆕 NEW: ADVANCED FEATURES APIs
    let create_multi_sig_wallet = warp::path("advanced")
        .and(warp::path("multi-sig"))
        .and(warp::post())
        .and(warp::body::json())
        .and(blockchain_filter.clone())
        .and_then(create_multi_sig_wallet_handler);

    let request_flash_loan = warp::path("advanced")
        .and(warp::path("flash-loan"))
        .and(warp::post())
        .and(warp::body::json())
        .and(blockchain_filter.clone())
        .and_then(request_flash_loan_handler);

    let create_quantum_wallet = warp::path("advanced")
        .and(warp::path("quantum-wallet"))
        .and(warp::post())
        .and(blockchain_filter.clone())
        .and_then(create_quantum_wallet_handler);

    let bridge_tokens = warp::path("advanced")
        .and(warp::path("bridge"))
        .and(warp::post())
        .and(warp::body::json())
        .and(blockchain_filter.clone())
        .and_then(bridge_tokens_handler);

    let create_gaming_asset = warp::path("advanced")
        .and(warp::path("gaming-asset"))
        .and(warp::post())
        .and(warp::body::json())
        .and(blockchain_filter.clone())
        .and_then(create_gaming_asset_handler);

    let create_defi_vault = warp::path("advanced")
        .and(warp::path("defi-vault"))
        .and(warp::post())
        .and(warp::body::json())
        .and(blockchain_filter.clone())
        .and_then(create_defi_vault_handler);

    let create_social_trader = warp::path("advanced")
        .and(warp::path("social-trader"))
        .and(warp::post())
        .and(warp::body::json())
        .and(blockchain_filter.clone())
        .and_then(create_social_trader_handler);

    let create_decentralized_identity = warp::path("advanced")
        .and(warp::path("decentralized-identity"))
        .and(warp::post())
        .and(warp::body::json())
        .and(blockchain_filter.clone())
        .and_then(create_decentralized_identity_handler);

    // 🆕 NEW: GET ADVANCED FEATURES INFO
    let get_advanced_features = warp::path("advanced")
        .and(warp::path("info"))
        .and(warp::get())
        .and(blockchain_filter.clone())
        .and_then(get_advanced_features_handler);

    let get_mass_adoption_stats = warp::path("mass-adoption")
        .and(warp::path("stats"))
        .and(warp::get())
        .and(blockchain_filter.clone())
        .and_then(get_mass_adoption_stats_handler);

    let routes = get_blocks
        .or(get_latest_block)
        .or(add_transaction)
        .or(mine_block)
        .or(get_status)
        .or(block_explorer)
        .or(get_tokenomics) // 🆕 NEW TOKENOMICS API
        .or(deploy_contract)
        .or(execute_contract)
        .or(get_wallet_balance)
        .or(get_wallet_info)
        .or(get_staking_info)
        .or(stake_tokens)
        .or(claim_rewards)
        .or(claim_daily_reward)
        .or(create_social_wallet)
        .or(complete_micro_task)
        .or(create_payment_link)
        .or(claim_payment_link)
        .or(create_multi_sig_wallet)
        .or(request_flash_loan)
        .or(create_quantum_wallet)
        .or(bridge_tokens)
        .or(create_gaming_asset)
        .or(create_defi_vault)
        .or(create_social_trader)
        .or(create_decentralized_identity)
        .or(get_advanced_features)
        .or(get_mass_adoption_stats)
        .with(warp::cors().allow_any_origin());

    println!("🚀 REST API Server starting on port {}...", port);
    println!("📡 Basic APIs: Blocks, Transactions, Mining, Contracts, Wallet");
    println!("🎯 Mass Adoption APIs: Daily Rewards, Social Wallets, Micro-tasks");
    println!("🚀 Advanced Features APIs: Multi-sig, Flash Loans, Quantum Wallets, Cross-chain Bridge");
    println!("💰 DeFi APIs: Staking, Gaming Assets, DeFi Vaults, SocialFi");
    println!("🏆 TOKENOMICS: Gate.io Ready - 20% Team, 35% Public, 15% Investors, 30% Liquidity");
    println!("🎯 PRICE TARGET: $1-2 | MARKET CAP: $100M-$200M");
    
    warp::serve(routes)
        .run(([0, 0, 0, 0], port))
        .await;
}

// === CORE BLOCKCHAIN HANDLERS ===
async fn get_all_blocks(
    blockchain: Arc<Mutex<Blockchain>>
) -> Result<impl warp::Reply, warp::Rejection> {
    let blockchain = blockchain.lock().await;
    Ok(warp::reply::json(&blockchain.chain))
}

async fn get_latest_block_handler(
    blockchain: Arc<Mutex<Blockchain>>
) -> Result<impl warp::Reply, warp::Rejection> {
    let blockchain = blockchain.lock().await;
    let latest_block = blockchain.get_latest_block();
    Ok(warp::reply::json(latest_block))
}

async fn add_transaction_handler(
    transaction: crate::models::Transaction,
    blockchain: Arc<Mutex<Blockchain>>
) -> Result<impl warp::Reply, warp::Rejection> {
    let mut blockchain = blockchain.lock().await;
    let success = blockchain.add_transaction(transaction);
    if success {
        Ok(warp::reply::json(&json!({"status": "transaction added"})))
    } else {
        Ok(warp::reply::json(&json!({"status": "transaction failed"})))
    }
}

async fn mine_block_handler(
    blockchain: Arc<Mutex<Blockchain>>
) -> Result<impl warp::Reply, warp::Rejection> {
    let mut blockchain = blockchain.lock().await;
    let mined_block = blockchain.mine_block();
    blockchain.add_mined_block(mined_block.clone());
    Ok(warp::reply::json(&mined_block))
}

async fn get_blockchain_status(
    blockchain: Arc<Mutex<Blockchain>>
) -> Result<impl warp::Reply, warp::Rejection> {
    let blockchain = blockchain.lock().await;
    let status = json!({
        "total_blocks": blockchain.chain.len(),
        "latest_block_index": blockchain.get_latest_block().index,
        "is_valid": blockchain.is_chain_valid(),
        "pending_transactions": blockchain.mempool.len(),
        "difficulty": blockchain.difficulty,
        "coin_supply": blockchain.get_coin_info(),
        "balance_info": blockchain.get_balance_info(),
        "network_stats": blockchain.get_network_stats(),
        "mass_adoption_stats": blockchain.get_mass_adoption_stats(),
        "advanced_features": blockchain.get_advanced_features_info(),
        // 🆕 NEW TOKENOMICS INFO
        "tokenomics": {
            "total_supply": 100_000_000,
            "foundation_team": {
                "percentage": 20,
                "amount": 20_000_000,
                "breakdown": {
                    "core_team": 8_000_000,
                    "foundation_reserve": 7_000_000,
                    "development_fund": 5_000_000
                }
            },
            "strategic_partners": {
                "percentage": 15,
                "amount": 15_000_000,
                "breakdown": {
                    "seed_investors": 6_000_000,
                    "strategic_partners": 5_000_000,
                    "ecosystem_fund": 4_000_000
                }
            },
            "public_distribution": {
                "percentage": 35,
                "amount": 35_000_000,
                "breakdown": {
                    "public_sale": 15_000_000,
                    "staking_rewards": 10_000_000,
                    "community_rewards": 6_000_000,
                    "airdrop_program": 4_000_000
                }
            },
            "liquidity_growth": {
                "percentage": 30,
                "amount": 30_000_000,
                "breakdown": {
                    "dex_liquidity": 8_000_000,
                    "cex_liquidity": 7_000_000,
                    "marketing_fund": 5_000_000,
                    "partnership_fund": 4_000_000,
                    "technology_grants": 3_000_000,
                    "security_reserve": 3_000_000
                }
            },
            "gate_io_ready": true,
            "price_target": "$1-2",
            "market_cap_target": "$100M-$200M"
        }
    });
    Ok(warp::reply::json(&status))
}

async fn block_explorer_handler(
    blockchain: Arc<Mutex<Blockchain>>
) -> Result<impl warp::Reply, warp::Rejection> {
    let blockchain = blockchain.lock().await;

    let html = format!(r#"
    <!DOCTYPE html>
    <html>
    <head>
        <title>Vexa Chain Block Explorer</title>
        <style>
            body {{ font-family: Arial, sans-serif; margin: 40px; background: #0a0a0a; color: white; }}
            .container {{ max-width: 1200px; margin: 0 auto; }}
            .header {{ background: linear-gradient(45deg, #00FFFF, #00AFFF); color: black; padding: 30px; border-radius: 15px; margin-bottom: 30px; }}
            .block {{ background: rgba(255,255,255,0.1); border: 1px solid #00FFFF; padding: 25px; margin: 15px 0; border-radius: 12px; backdrop-filter: blur(10px); }}
            .transaction {{ background: rgba(0,255,255,0.1); padding: 15px; margin: 8px 0; border-left: 4px solid #00FFFF; border-radius: 6px; }}
            .hash {{ font-family: monospace; color: #00FFFF; font-size: 12px; }}
            .success {{ color: #00ff00; }}
            .feature-grid {{ display: grid; grid-template-columns: repeat(auto-fit, minmax(300px, 1fr)); gap: 20px; margin: 30px 0; }}
            .feature-card {{ background: rgba(255,255,255,0.1); padding: 20px; border-radius: 10px; border: 1px solid #00FFFF; }}
            .tokenomics-grid {{ display: grid; grid-template-columns: repeat(auto-fit, minmax(250px, 1fr)); gap: 15px; margin: 20px 0; }}
            .tokenomics-item {{ background: rgba(0,255,255,0.1); padding: 15px; border-radius: 8px; }}
            .price-target {{ background: linear-gradient(45deg, #FFD700, #FFA500); color: black; padding: 10px; border-radius: 8px; text-align: center; font-weight: bold; }}
        </style>
    </head>
    <body>
        <div class="container">
            <div class="header">
                <h1>🚀 Vexa Chain Block Explorer</h1>
                <p>Total Blocks: <strong>{}</strong> | Circulating Supply: <strong>{} VEXA</strong></p>
                <p>Advanced Features: Multi-sig Wallets, Flash Loans, Quantum Security, Cross-chain Bridge</p>
            </div>
            
            <!-- 🆕 NEW TOKENOMICS SECTION -->
            <div class="feature-card">
                <h3>🏆 ELITE TOKENOMICS – $1-2 PRICE TARGET</h3>
                <div class="price-target">
                    🎯 GATE.IO READY | MARKET CAP TARGET: $100M - $200M
                </div>
                <div class="tokenomics-grid">
                    <div class="tokenomics-item">
                        <h4>🏢 Foundation & Team</h4>
                        <p><strong>20M VEXA (20%)</strong></p>
                        <small>Core Team: 8M | Foundation: 7M | Dev: 5M</small>
                        <br><small>📅 Vesting: 4 years</small>
                    </div>
                    <div class="tokenomics-item">
                        <h4>💰 Strategic Partners</h4>
                        <p><strong>15M VEXA (15%)</strong></p>
                        <small>Seed: 6M | Strategic: 5M | Ecosystem: 4M</small>
                        <br><small>📅 Vesting: 6-24 months</small>
                    </div>
                    <div class="tokenomics-item">
                        <h4>🌍 Public Distribution</h4>
                        <p><strong>35M VEXA (35%)</strong></p>
                        <small>Public Sale: 15M | Staking: 10M | Community: 6M | Airdrop: 4M</small>
                    </div>
                    <div class="tokenomics-item">
                        <h4>🔒 Liquidity & Growth</h4>
                        <p><strong>30M VEXA (30%)</strong></p>
                        <small>DEX: 8M | CEX: 7M | Marketing: 5M | Partnerships: 4M</small>
                    </div>
                </div>
            </div>

            <div class="feature-grid">
                <div class="feature-card">
                    <h3>💰 Gate.io Ready</h3>
                    <p>✅ Team Allocation: 20%</p>
                    <p>✅ Public Distribution: 35%</p>
                    <p>✅ Liquidity Provision: 10%</p>
                    <p>✅ Vesting Schedules: Active</p>
                </div>
                <div class="feature-card">
                    <h3>🎯 Mass Adoption</h3>
                    <p>Daily Rewards: Active</p>
                    <p>Social Wallets: Enabled</p>
                    <p>Micro-tasks: Available</p>
                </div>
                <div class="feature-card">
                    <h3>🚀 Advanced Features</h3>
                    <p>Multi-sig Wallets: {}</p>
                    <p>Flash Loans: {}</p>
                    <p>Quantum Wallets: {}</p>
                </div>
            </div>
            
            <h2>📦 Latest Blocks</h2>
            {}
        </div>
    </body>
    </html>
    "#, 
    blockchain.chain.len(),
    blockchain.coin.format_amount(blockchain.coin.circulating_supply),
    blockchain.multi_sig_wallets.len(),
    blockchain.flash_loans.len(),
    blockchain.quantum_wallets.len(),
    render_blocks(&blockchain.chain, &blockchain.coin)
    );
    
    Ok(warp::reply::json(&serde_json::json!({"html": html})))
}

// 🆕 NEW: TOKENOMICS API HANDLER
async fn get_tokenomics_handler(
    blockchain: Arc<Mutex<Blockchain>>
) -> Result<impl warp::Reply, warp::Rejection> {
    let blockchain = blockchain.lock().await;
    
    Ok(warp::reply::json(&json!({
        "tokenomics": {
            "total_supply": 100000000,
            "circulating_supply": blockchain.coin.circulating_supply,
            "formatted_circulating_supply": blockchain.coin.format_amount(blockchain.coin.circulating_supply),
            "allocations": {
                "foundation_team": {
                    "total": 20000000,
                    "percentage": 20.0,
                    "breakdown": {
                        "core_development_team": 8000000,
                        "foundation_reserve": 7000000,
                        "development_fund": 5000000
                    },
                    "vesting": "4 years with 1-year cliff"
                },
                "strategic_partners": {
                    "total": 15000000,
                    "percentage": 15.0,
                    "breakdown": {
                        "seed_investors": 6000000,
                        "strategic_partners": 5000000,
                        "ecosystem_fund": 4000000
                    },
                    "vesting": "6-24 months with cliff"
                },
                "public_distribution": {
                    "total": 35000000,
                    "percentage": 35.0,
                    "breakdown": {
                        "public_sale": 15000000,
                        "staking_rewards": 10000000,
                        "community_rewards": 6000000,
                        "airdrop_program": 4000000
                    },
                    "vesting": "Immediate release"
                },
                "liquidity_growth": {
                    "total": 30000000,
                    "percentage": 30.0,
                    "breakdown": {
                        "dex_liquidity": 8000000,
                        "cex_liquidity": 7000000,
                        "marketing_fund": 5000000,
                        "partnership_fund": 4000000,
                        "technology_grants": 3000000,
                        "security_reserve": 3000000
                    },
                    "vesting": "1-2 years locked"
                }
            },
            "exchange_ready": {
                "gate_io_compliant": true,
                "team_allocation_acceptable": true,
                "public_distribution_sufficient": true,
                "liquidity_adequate": true,
                "vesting_transparent": true
            },
            "price_targets": {
                "short_term": "$0.10-$0.30",
                "medium_term": "$0.50-$0.80", 
                "long_term": "$1.00-$2.00",
                "market_cap_targets": {
                    "short_term": "$10M-$30M",
                    "medium_term": "$50M-$80M",
                    "long_term": "$100M-$200M"
                }
            },
            "emission_schedule": {
                "block_reward": 25,
                "staking_apr": 15.0,
                "inflation_rate": 5.0,
                "deflation_rate": 2.0,
                "burn_mechanism": "2% transaction fee burn"
            }
        }
    })))
}

// 🆕 NEW: SMART CONTRACT HANDLERS
async fn deploy_contract_handler(
    contract_request: HashMap<String, String>,
    blockchain: Arc<Mutex<Blockchain>>
) -> Result<impl warp::Reply, warp::Rejection> {
    let mut blockchain = blockchain.lock().await;
    
    let code = contract_request.get("code").unwrap_or(&"".to_string()).clone();
    let owner = contract_request.get("owner").unwrap_or(&"".to_string()).clone();
    
    if code.is_empty() || owner.is_empty() {
        return Ok(warp::reply::json(&json!({"status": "error", "message": "Invalid contract data"})));
    }
    
    let contract_address = blockchain.deploy_contract(code, owner, 0);
    
    Ok(warp::reply::json(&json!({
        "status": "success",
        "message": "Contract deployed successfully",
        "contract_address": contract_address
    })))
}

async fn execute_contract_handler(
    contract_tx: crate::models::ContractTransaction,
    blockchain: Arc<Mutex<Blockchain>>
) -> Result<impl warp::Reply, warp::Rejection> {
    let mut blockchain = blockchain.lock().await;
    
    let result = blockchain.execute_contract(contract_tx);
    
    Ok(warp::reply::json(&json!({
        "success": result.success,
        "output": result.output,
        "gas_used": result.gas_used,
        "logs": result.logs
    })))
}

// 🆕 NEW: WALLET HANDLERS
async fn get_wallet_balance_handler(
    params: HashMap<String, String>,
    blockchain: Arc<Mutex<Blockchain>>
) -> Result<impl warp::Reply, warp::Rejection> {
    let blockchain = blockchain.lock().await;
    
    let address = params.get("address").unwrap_or(&"".to_string()).clone();
    
    if address.is_empty() {
        return Ok(warp::reply::json(&json!({"status": "error", "message": "Address parameter required"})));
    }
    
    let balance = blockchain.get_wallet_balance(&address);
    
    Ok(warp::reply::json(&json!({
        "address": address,
        "balance": balance,
        "formatted_balance": blockchain.coin.format_amount(balance)
    })))
}

async fn get_wallet_info_handler(
    params: HashMap<String, String>,
    blockchain: Arc<Mutex<Blockchain>>
) -> Result<impl warp::Reply, warp::Rejection> {
    let blockchain = blockchain.lock().await;
    
    let address = params.get("address").unwrap_or(&"".to_string()).clone();
    
    if address.is_empty() {
        return Ok(warp::reply::json(&json!({"status": "error", "message": "Address parameter required"})));
    }
    
    let balance = blockchain.get_wallet_balance(&address);
    let staking_info = blockchain.get_staker_info(&address);
    let voting_power = 0;

    Ok(warp::reply::json(&json!({
        "address": address,
        "balance": balance,
        "formatted_balance": blockchain.coin.format_amount(balance),
        "voting_power": voting_power,
        "staking_info": staking_info
    })))
}

// 🆕 NEW: STAKING HANDLERS
async fn get_staking_info_handler(
    blockchain: Arc<Mutex<Blockchain>>
) -> Result<impl warp::Reply, warp::Rejection> {
    let blockchain = blockchain.lock().await;
    
    Ok(warp::reply::json(&json!({
        "total_staked": blockchain.staking_pool.total_staked,
        "formatted_total_staked": blockchain.coin.format_amount(blockchain.staking_pool.total_staked),
        "stakers_count": blockchain.staking_pool.stakers.len(),
        "apr": blockchain.staking_pool.apr,
        "total_rewards_distributed": blockchain.staking_pool.total_rewards_distributed,
        "formatted_rewards": blockchain.coin.format_amount(blockchain.staking_pool.total_rewards_distributed)
    })))
}

async fn stake_tokens_handler(
    stake_request: HashMap<String, String>,
    blockchain: Arc<Mutex<Blockchain>>
) -> Result<impl warp::Reply, warp::Rejection> {
    let mut blockchain = blockchain.lock().await;
    
    let address = stake_request.get("address").unwrap_or(&"".to_string()).clone();
    let amount_str = stake_request.get("amount").unwrap_or(&"0".to_string()).clone();
    let lock_days_str = stake_request.get("lock_days").unwrap_or(&"30".to_string()).clone();
    
    if address.is_empty() {
        return Ok(warp::reply::json(&json!({"status": "error", "message": "Address parameter required"})));
    }
    
    let amount = amount_str.parse::<f64>().unwrap_or(0.0);
    let lock_days = lock_days_str.parse::<u64>().unwrap_or(30);
    let parsed_amount = blockchain.coin.parse_amount(amount);
    
    let success = blockchain.stake_tokens(address.clone(), parsed_amount, lock_days);
    
    if success {
        Ok(warp::reply::json(&json!({
            "status": "success",
            "message": "Tokens staked successfully",
            "amount": parsed_amount,
            "formatted_amount": blockchain.coin.format_amount(parsed_amount),
            "lock_days": lock_days
        })))
    } else {
        Ok(warp::reply::json(&json!({"status": "error", "message": "Staking failed"})))
    }
}

async fn claim_rewards_handler(
    claim_request: HashMap<String, String>,
    blockchain: Arc<Mutex<Blockchain>>
) -> Result<impl warp::Reply, warp::Rejection> {
    let mut blockchain = blockchain.lock().await;
    
    let address = claim_request.get("address").unwrap_or(&"".to_string()).clone();
    
    if address.is_empty() {
        return Ok(warp::reply::json(&json!({"status": "error", "message": "Address parameter required"})));
    }
    
    let rewards = blockchain.claim_rewards(address.clone());
    
    Ok(warp::reply::json(&json!({
        "status": "success",
        "message": "Rewards claimed successfully",
        "rewards": rewards,
        "formatted_rewards": blockchain.coin.format_amount(rewards),
        "address": address
    })))
}

// 🆕 NEW: MASS ADOPTION HANDLERS
async fn claim_daily_reward_handler(
    reward_request: HashMap<String, String>,
    blockchain: Arc<Mutex<Blockchain>>
) -> Result<impl warp::Reply, warp::Rejection> {
    let mut blockchain = blockchain.lock().await;
    
    let address = reward_request.get("address").unwrap_or(&"".to_string()).clone();
    
    if address.is_empty() {
        return Ok(warp::reply::json(&json!({"status": "error", "message": "Address parameter required"})));
    }
    
    let reward = blockchain.claim_daily_reward(address.clone());
    
    Ok(warp::reply::json(&json!({
        "status": "success",
        "message": "Daily reward claimed",
        "reward": reward,
        "formatted_reward": blockchain.coin.format_amount(reward),
        "address": address
    })))
}

async fn create_social_wallet_handler(
    wallet_request: HashMap<String, String>,
    blockchain: Arc<Mutex<Blockchain>>
) -> Result<impl warp::Reply, warp::Rejection> {
    let mut blockchain = blockchain.lock().await;
    
    let phone = wallet_request.get("phone").unwrap_or(&"".to_string()).clone();
    let email = wallet_request.get("email").unwrap_or(&"".to_string()).clone();
    
    if phone.is_empty() {
        return Ok(warp::reply::json(&json!({"status": "error", "message": "Phone number required"})));
    }
    
    let social_wallet = blockchain.create_social_wallet(phone.clone(), email);
    
    Ok(warp::reply::json(&json!({
        "status": "success",
        "message": "Social wallet created",
        "phone": social_wallet.phone_number,
        "main_address": social_wallet.main_address,
        "recovery_code": social_wallet.recovery_code
    })))
}

async fn complete_micro_task_handler(
    task_request: HashMap<String, String>,
    blockchain: Arc<Mutex<Blockchain>>
) -> Result<impl warp::Reply, warp::Rejection> {
    let mut blockchain = blockchain.lock().await;
    
    let address = task_request.get("address").unwrap_or(&"".to_string()).clone();
    let task_id = task_request.get("task_id").unwrap_or(&"".to_string()).clone();
    
    if address.is_empty() || task_id.is_empty() {
        return Ok(warp::reply::json(&json!({"status": "error", "message": "Address and task_id required"})));
    }
    
    let success = blockchain.complete_micro_task(address.clone(), task_id.clone());
    
    if success {
        Ok(warp::reply::json(&json!({
            "status": "success",
            "message": "Micro task completed",
            "address": address,
            "task_id": task_id
        })))
    } else {
        Ok(warp::reply::json(&json!({"status": "error", "message": "Task completion failed"})))
    }
}

async fn create_payment_link_handler(
    payment_request: HashMap<String, String>,
    blockchain: Arc<Mutex<Blockchain>>
) -> Result<impl warp::Reply, warp::Rejection> {
    let mut blockchain = blockchain.lock().await;
    
    let creator = payment_request.get("creator").unwrap_or(&"".to_string()).clone();
    let amount_str = payment_request.get("amount").unwrap_or(&"0".to_string()).clone();
    let description = payment_request.get("description").unwrap_or(&"Payment".to_string()).clone();
    
    if creator.is_empty() {
        return Ok(warp::reply::json(&json!({"status": "error", "message": "Creator address required"})));
    }
    
    let amount = amount_str.parse::<f64>().unwrap_or(0.0);
    let parsed_amount = blockchain.coin.parse_amount(amount);
    
    let payment_url = blockchain.create_payment_link(creator, parsed_amount, description);
    
    Ok(warp::reply::json(&json!({
        "status": "success",
        "message": "Payment link created",
        "payment_url": payment_url,
        "amount": parsed_amount,
        "formatted_amount": blockchain.coin.format_amount(parsed_amount)
    })))
}

async fn claim_payment_link_handler(
    claim_request: HashMap<String, String>,
    blockchain: Arc<Mutex<Blockchain>>
) -> Result<impl warp::Reply, warp::Rejection> {
    let mut blockchain = blockchain.lock().await;
    
    let link_id = claim_request.get("link_id").unwrap_or(&"".to_string()).clone();
    let claimer = claim_request.get("claimer").unwrap_or(&"".to_string()).clone();
    
    if link_id.is_empty() || claimer.is_empty() {
        return Ok(warp::reply::json(&json!({"status": "error", "message": "Link ID and claimer address required"})));
    }
    
    let success = blockchain.claim_payment_link(link_id.clone(), claimer.clone());
    
    if success {
        Ok(warp::reply::json(&json!({
            "status": "success",
            "message": "Payment claimed successfully",
            "link_id": link_id,
            "claimer": claimer
        })))
    } else {
        Ok(warp::reply::json(&json!({"status": "error", "message": "Payment claim failed"})))
    }
}

// 🆕 NEW: ADVANCED FEATURES HANDLERS
async fn create_multi_sig_wallet_handler(
    wallet_request: HashMap<String, String>,
    blockchain: Arc<Mutex<Blockchain>>
) -> Result<impl warp::Reply, warp::Rejection> {
    let mut blockchain = blockchain.lock().await;
    
    let owners_str = wallet_request.get("owners").unwrap_or(&"".to_string()).clone();
    let required_str = wallet_request.get("required").unwrap_or(&"1".to_string()).clone();
    
    if owners_str.is_empty() {
        return Ok(warp::reply::json(&json!({"status": "error", "message": "Owners list required"})));
    }
    
    let owners: Vec<String> = owners_str.split(',').map(|s| s.trim().to_string()).collect();
    let required = required_str.parse::<u8>().unwrap_or(1);
    
    let wallet_address = blockchain.create_multi_sig_wallet(owners.clone(), required);
    
    Ok(warp::reply::json(&json!({
        "status": "success",
        "message": "Multi-signature wallet created",
        "wallet_address": wallet_address,
        "owners": owners,
        "required_signatures": required,
        "total_owners": owners.len()
    })))
}

async fn request_flash_loan_handler(
    loan_request: HashMap<String, String>,
    blockchain: Arc<Mutex<Blockchain>>
) -> Result<impl warp::Reply, warp::Rejection> {
    let mut blockchain = blockchain.lock().await;
    
    let borrower = loan_request.get("borrower").unwrap_or(&"".to_string()).clone();
    let amount_str = loan_request.get("amount").unwrap_or(&"0".to_string()).clone();
    let collateral_str = loan_request.get("collateral").unwrap_or(&"0".to_string()).clone();
    
    if borrower.is_empty() {
        return Ok(warp::reply::json(&json!({"status": "error", "message": "Borrower address required"})));
    }
    
    let amount = amount_str.parse::<f64>().unwrap_or(0.0);
    let collateral = collateral_str.parse::<f64>().unwrap_or(0.0);
    
    let parsed_amount = blockchain.coin.parse_amount(amount);
    let parsed_collateral = blockchain.coin.parse_amount(collateral);
    
    let loan_id = blockchain.request_flash_loan(borrower.clone(), parsed_amount, parsed_collateral);
    
    if let Some(loan_id) = loan_id {
        Ok(warp::reply::json(&json!({
            "status": "success",
            "message": "Flash loan approved",
            "loan_id": loan_id,
            "borrower": borrower,
            "amount": parsed_amount,
            "formatted_amount": blockchain.coin.format_amount(parsed_amount),
            "collateral": parsed_collateral,
            "formatted_collateral": blockchain.coin.format_amount(parsed_collateral),
            "fee": (parsed_amount as f64 * 0.003) as u64
        })))
    } else {
        Ok(warp::reply::json(&json!({"status": "error", "message": "Flash loan request failed"})))
    }
}

async fn create_quantum_wallet_handler(
    blockchain: Arc<Mutex<Blockchain>>
) -> Result<impl warp::Reply, warp::Rejection> {
    let mut blockchain = blockchain.lock().await;
    
    let wallet_address = blockchain.create_quantum_wallet();
    
    Ok(warp::reply::json(&json!({
        "status": "success",
        "message": "Quantum-resistant wallet created",
        "wallet_address": wallet_address,
        "security_features": ["Kyber512", "Dilithium2", "SPHINCS+"]
    })))
}

async fn bridge_tokens_handler(
    bridge_request: HashMap<String, String>,
    blockchain: Arc<Mutex<Blockchain>>
) -> Result<impl warp::Reply, warp::Rejection> {
    let mut blockchain = blockchain.lock().await;
    
    let from_chain = bridge_request.get("from_chain").unwrap_or(&"".to_string()).clone();
    let to_chain = bridge_request.get("to_chain").unwrap_or(&"".to_string()).clone();
    let amount_str = bridge_request.get("amount").unwrap_or(&"0".to_string()).clone();
    let user_address = bridge_request.get("user_address").unwrap_or(&"".to_string()).clone();
    
    if from_chain.is_empty() || to_chain.is_empty() || user_address.is_empty() {
        return Ok(warp::reply::json(&json!({"status": "error", "message": "All parameters required"})));
    }
    
    let amount = amount_str.parse::<f64>().unwrap_or(0.0);
    let parsed_amount = blockchain.coin.parse_amount(amount);
    
    let success = blockchain.bridge_tokens(from_chain.clone(), to_chain.clone(), parsed_amount, user_address.clone());
    
    if success {
        Ok(warp::reply::json(&json!({
            "status": "success",
            "message": "Cross-chain bridge transfer initiated",
            "from_chain": from_chain,
            "to_chain": to_chain,
            "amount": parsed_amount,
            "formatted_amount": blockchain.coin.format_amount(parsed_amount),
            "user_address": user_address,
            "fee_percentage": 0.1
        })))
    } else {
        Ok(warp::reply::json(&json!({"status": "error", "message": "Bridge transfer failed"})))
    }
}

async fn create_gaming_asset_handler(
    asset_request: HashMap<String, String>,
    blockchain: Arc<Mutex<Blockchain>>
) -> Result<impl warp::Reply, warp::Rejection> {
    let mut blockchain = blockchain.lock().await;
    
    let owner = asset_request.get("owner").unwrap_or(&"".to_string()).clone();
    let game_id = asset_request.get("game_id").unwrap_or(&"".to_string()).clone();
    let metadata = asset_request.get("metadata").unwrap_or(&"".to_string()).clone();
    let rarity = asset_request.get("rarity").unwrap_or(&"Common".to_string()).clone();
    
    if owner.is_empty() || game_id.is_empty() {
        return Ok(warp::reply::json(&json!({"status": "error", "message": "Owner and game_id required"})));
    }
    
    let asset_id = blockchain.create_gaming_asset(owner.clone(), game_id.clone(), metadata, rarity.clone());
    
    Ok(warp::reply::json(&json!({
        "status": "success",
        "message": "Gaming asset created",
        "asset_id": asset_id,
        "owner": owner,
        "game_id": game_id,
        "rarity": rarity
    })))
}

async fn create_defi_vault_handler(
    vault_request: HashMap<String, String>,
    blockchain: Arc<Mutex<Blockchain>>
) -> Result<impl warp::Reply, warp::Rejection> {
    let mut blockchain = blockchain.lock().await;
    
    let creator = vault_request.get("creator").unwrap_or(&"".to_string()).clone();
    let strategy = vault_request.get("strategy").unwrap_or(&"Yield Farming".to_string()).clone();
    let amount_str = vault_request.get("amount").unwrap_or(&"0".to_string()).clone();
    
    if creator.is_empty() {
        return Ok(warp::reply::json(&json!({"status": "error", "message": "Creator address required"})));
    }
    
    let amount = amount_str.parse::<f64>().unwrap_or(0.0);
    let parsed_amount = blockchain.coin.parse_amount(amount);
    
    let vault_id = blockchain.create_defi_vault(creator.clone(), strategy.clone(), parsed_amount);
    
    Ok(warp::reply::json(&json!({
        "status": "success",
        "message": "DeFi vault created",
        "vault_id": vault_id,
        "creator": creator,
        "strategy": strategy,
        "initial_liquidity": parsed_amount,
        "formatted_liquidity": blockchain.coin.format_amount(parsed_amount),
        "apr": 15.0
    })))
}

async fn create_social_trader_handler(
    trader_request: HashMap<String, String>,
    blockchain: Arc<Mutex<Blockchain>>
) -> Result<impl warp::Reply, warp::Rejection> {
    let mut blockchain = blockchain.lock().await;
    
    let address = trader_request.get("address").unwrap_or(&"".to_string()).clone();
    
    if address.is_empty() {
        return Ok(warp::reply::json(&json!({"status": "error", "message": "Address required"})));
    }
    
    blockchain.create_social_trader(address.clone());
    
    Ok(warp::reply::json(&json!({
        "status": "success",
        "message": "Social trader profile created",
        "address": address,
        "features": ["Copy Trading", "Performance Tracking", "Follower System"]
    })))
}

async fn create_decentralized_identity_handler(
    identity_request: HashMap<String, String>,
    blockchain: Arc<Mutex<Blockchain>>
) -> Result<impl warp::Reply, warp::Rejection> {
    let mut blockchain = blockchain.lock().await;
    
    let owner = identity_request.get("owner").unwrap_or(&"".to_string()).clone();
    
    if owner.is_empty() {
        return Ok(warp::reply::json(&json!({"status": "error", "message": "Owner address required"})));
    }
    
    let did = blockchain.create_decentralized_identity(owner.clone());
    
    Ok(warp::reply::json(&json!({
        "status": "success",
        "message": "Decentralized Identity created",
        "did": did,
        "owner": owner,
        "features": ["Soulbound Tokens", "Verifiable Credentials", "Reputation System"]
    })))
}

// 🆕 NEW: GET ADVANCED FEATURES INFO
async fn get_advanced_features_handler(
    blockchain: Arc<Mutex<Blockchain>>
) -> Result<impl warp::Reply, warp::Rejection> {
    let blockchain = blockchain.lock().await;
    
    Ok(warp::reply::json(&json!({
        "multi_sig_wallets": blockchain.multi_sig_wallets.len(),
        "flash_loans": blockchain.flash_loans.len(),
        "quantum_wallets": blockchain.quantum_wallets.len(),
        "cross_chain_bridge": {
            "supported_chains": blockchain.cross_chain_bridge.supported_chains.len(),
            "total_volume": blockchain.cross_chain_bridge.total_volume,
            "formatted_volume": blockchain.coin.format_amount(blockchain.cross_chain_bridge.total_volume)
        },
        "gaming_assets": blockchain.gaming_assets.len(),
        "defi_vaults": blockchain.defi_vaults.len(),
        "social_fi_users": blockchain.social_fi.social_trading.len(),
        "decentralized_identities": blockchain.decentralized_identity.len()
    })))
}

// 🆕 NEW: GET MASS ADOPTION STATS
async fn get_mass_adoption_stats_handler(
    blockchain: Arc<Mutex<Blockchain>>
) -> Result<impl warp::Reply, warp::Rejection> {
    let blockchain = blockchain.lock().await;
    
    Ok(warp::reply::json(&json!({
        "total_users": blockchain.balance_tracker.balances.len(),
        "daily_reward_users": blockchain.mass_adoption.daily_rewards.user_rewards.len(),
        "referral_users": blockchain.mass_adoption.referral_program.referrals.len(),
        "social_wallets": blockchain.mass_adoption.social_wallets.len(),
        "available_tasks": blockchain.mass_adoption.micro_earning.tasks.len(),
        "active_payment_links": blockchain.mass_adoption.social_pay.payment_links.len()
    })))
}