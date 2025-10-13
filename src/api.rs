// api.rs
use warp::Filter;
use std::sync::Arc;
use tokio::sync::Mutex;
use serde_json::json;

use crate::{Blockchain, Transaction, Block, coin::Coin};

// ✅ SIMPLE WORKING VERSION
pub async fn start_api_server(blockchain: Arc<Mutex<Blockchain>>, port: u16) {
    let blockchain_filter = warp::any().map(move || Arc::clone(&blockchain));

    // GET /blocks - Get all blocks
    let get_blocks = warp::path("blocks")
        .and(warp::get())
        .and(blockchain_filter.clone())
        .and_then(get_all_blocks);

    // GET /blocks/latest - Get latest block
    let get_latest_block = warp::path("blocks")
        .and(warp::path("latest"))
        .and(warp::get())
        .and(blockchain_filter.clone())
        .and_then(get_latest_block_handler);

    // POST /transactions - Add new transaction
    let add_transaction = warp::path("transactions")
        .and(warp::post())
        .and(warp::body::json())
        .and(blockchain_filter.clone())
        .and_then(add_transaction_handler);

    // POST /mine - Mine a new block
    let mine_block = warp::path("mine")
        .and(warp::post())
        .and(blockchain_filter.clone())
        .and_then(mine_block_handler);

    // GET /status - Get blockchain status
    let get_status = warp::path("status")
        .and(warp::get())
        .and(blockchain_filter.clone())
        .and_then(get_blockchain_status);

    // GET /explorer - Block explorer
    let block_explorer = warp::path("explorer")
        .and(warp::get())
        .and(blockchain_filter.clone())
        .and_then(block_explorer_handler);

    let routes = get_blocks
        .or(get_latest_block)
        .or(add_transaction)
        .or(mine_block)
        .or(get_status)
        .or(block_explorer)
        .with(warp::cors().allow_any_origin());

    println!("🌐 REST API Server starting on port {}...", port);
    
    // ✅ SIMPLE & WORKING VERSION
    warp::serve(routes)
        .run(([127, 0, 0, 1], port))
        .await;
}

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
    transaction: Transaction,
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
        "pending_transactions": blockchain.pending_transactions.len(),
        "difficulty": blockchain.difficulty,
        "coin_supply": blockchain.get_coin_info(),
        "balance_info": blockchain.get_balance_info()
    });
    Ok(warp::reply::json(&status))
}

// ✅ Block Explorer
async fn block_explorer_handler(
    blockchain: Arc<Mutex<Blockchain>>
) -> Result<impl warp::Reply, warp::Rejection> {
    let blockchain = blockchain.lock().await;
    
    let html = format!(r#"
    <!DOCTYPE html>
    <html>
    <head>
        <title>Nova Chain Block Explorer</title>
        <style>
            body {{ font-family: Arial, sans-serif; margin: 40px; background: #f5f5f5; }}
            .container {{ max-width: 1200px; margin: 0 auto; }}
            .header {{ background: #2c3e50; color: white; padding: 20px; border-radius: 10px; }}
            .block {{ background: white; border: 1px solid #ddd; padding: 20px; margin: 10px 0; border-radius: 8px; box-shadow: 0 2px 4px rgba(0,0,0,0.1); }}
            .transaction {{ background: #f9f9f9; padding: 10px; margin: 5px 0; border-left: 4px solid #3498db; }}
            .hash {{ font-family: monospace; color: #666; font-size: 12px; }}
            .success {{ color: #27ae60; }}
            .info {{ color: #3498db; }}
        </style>
    </head>
    <body>
        <div class="container">
            <div class="header">
                <h1>🌐 Nova Chain Block Explorer</h1>
                <p>Total Blocks: <strong>{}</strong> | Circulating Supply: <strong>{} NOVA</strong></p>
            </div>
            
            <h2>📦 Latest Blocks</h2>
            {}
        </div>
    </body>
    </html>
    "#, 
    blockchain.chain.len(),
    blockchain.coin.format_amount(blockchain.coin.circulating_supply),
    render_blocks(&blockchain.chain, &blockchain.coin)
    );
    
    Ok(warp::reply::html(html))
}

fn render_blocks(blocks: &Vec<Block>, coin: &Coin) -> String {
    blocks.iter().rev().take(10).map(|block| {
        let transactions_html: String = block.transactions.iter().map(|tx| {
            format!(r#"
            <div class="transaction">
                <strong>{} → {}:</strong> {} NOVA
                <br><small class="hash">Tx Hash: {}</small>
            </div>
            "#, 
            if tx.sender == "MINING_REWARD" { "⛏️ MINING" } else { &tx.sender },
            &tx.receiver,
            coin.format_amount(tx.amount),
            if tx.calculate_hash().len() >= 16 {
                format!("{}...", &tx.calculate_hash()[..16])
            } else {
                tx.calculate_hash()
            }
            )
        }).collect();
        
        format!(r#"
        <div class="block">
            <h3>Block #{} <span class="success">✅</span></h3>
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