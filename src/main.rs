use alloy::{
    primitives::{Address, U256, address}, 
    providers::{Provider, ProviderBuilder, WsConnect}, 
    rpc::types::Filter,
    sol, 
    sol_types::SolEvent
};
use eyre::Result;
use futures_util::StreamExt;
use std::{env, thread::spawn};
use std::collections::HashMap;
use std::sync::Arc;

sol! {
    event Transfer(address indexed from, address indexed to, uint256 value);
}

const USDT_ADDRESS: Address = address!("dac17f958d2ee523a2206206994597c13d831ec7");

// Telegram structure
#[derive(Clone)]
struct TelegramConfig {
    token: String,
    chat_id: String,
}

// Func: spawn Telegram task (not req .await)
fn spawn_telegram_alert(config: TelegramConfig, message: String) {
    tokio::spawn(async move {
        let url = format!("https://api.telegram.org/bot{}/sendMessage", config.token);

        let mut params = HashMap::new();
        params.insert("chat_id", config.chat_id);
        params.insert("text", message);
        params.insert("parse_mode", "HTML".to_string());

        let client = reqwest::Client::new();

        match client.post(&url).json(&params).send().await {
            Ok(_) => {},
            Err(e) => eprintln!("❌ Telegram Error: {:?}", e),
        }
    });
    
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();

    let rpc_url = env::var("RPC_URL").expect("RPC_URL must be set in .env");
    let tg_config = TelegramConfig {
        token: env::var("TELEGRAM_BOT_TOKEN").expect("TELEGRAM_BOT_TOKEN must be set"),
        chat_id: env::var("TELEGRAM_CHAT_ID").expect("TELEGRAM_CHAT_ID must be set"),
    };

    let threshold_env = env::var("WHALE_THRESHOLD").unwrap_or_else(|_| "10000".to_string());
    let whale_threshold_u128: u128 = threshold_env.parse().expect("Invalid WHALE_THRESHOLD number");
    let whale_threshold_decimals = whale_threshold_u128 * 1_000_000;

    println!("🐋 USDT Watcher Started");
    println!("   Threshold: ${:?}", whale_threshold_u128);
    println!("   Connecting via WS...");

    let ws = WsConnect::new(rpc_url);
    let provider = ProviderBuilder::new().connect_ws(ws).await?;

    println!("Connected! Listening for USDT transfers > {}...\n", threshold_env);

    let filter = Filter::new()
        .address(USDT_ADDRESS)
        .event_signature(Transfer::SIGNATURE_HASH);

    let sub = provider.subscribe_logs(&filter).await?;
    let mut stream = sub.into_stream();

    while let Some(log) = stream.next().await {
        if let Ok(decoded) = log.log_decode::<Transfer>() {
            let transfer = decoded.inner.data;
            let amount_u128 = transfer.value.saturating_to::<u128>();

            if amount_u128 >= whale_threshold_decimals {
                let amount_formatted = amount_u128 as f64 / 1_000_000.0;
                let tx_hash = log.transaction_hash.unwrap_or_default();

                let message = format!(
                    "🚨 <b>WHALE ALERT</b> 🚨\n\n💰 <b>Amount:</b> ${:.2} USDT\n📤 <b>From:</b> <code>{}</code>\n📥 <b>To:</b> <code>{}</code>\n\n🔗 <a href=\"https://etherscan.io/tx/{}\">View Transaction</a>",
                    amount_formatted,
                    transfer.from,
                    transfer.to,
                    tx_hash
                );

                println!(
                    "🐋 WHALE ALERT! Moved: ${:.2} USDT (Tx: {:?})",
                    amount_formatted, tx_hash
                );

                spawn_telegram_alert(tg_config.clone(), message);
            }
        }
    }

    Ok(())
}