use alloy::{
    primitives::{Address}, 
    providers::{Provider, ProviderBuilder, WsConnect}, 
    rpc::types::Filter,
    sol, 
    sol_types::SolEvent
};
use eyre::Result;
use futures_util::StreamExt;
use std::env;
use std::collections::HashMap;
use std::str::FromStr;
use std::time::Duration;
use tracing::{error, info, warn};

sol! {
    event Transfer(address indexed from, address indexed to, uint256 value);
}

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
            Ok(_) => info!("✅ Alert sent to Telegram"),
            Err(e) => error!("❌ Failed to send alert: {:?}", e),
        }
    });
    
}

#[tokio::main]
async fn main() -> Result<()> {

    // Init logger & env
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();
    dotenv::dotenv().ok();

    // Load Config
    let rpc_url = env::var("RPC_URL").expect("RPC_URL must be set in .env");
    let tg_config = TelegramConfig {
        token: env::var("TELEGRAM_BOT_TOKEN").expect("TELEGRAM_BOT_TOKEN must be set"),
        chat_id: env::var("TELEGRAM_CHAT_ID").expect("TELEGRAM_CHAT_ID must be set"),
    };

    let threshold_env = env::var("WHALE_THRESHOLD").unwrap_or_else(|_| "10000".to_string());
    let threshold_u128: u128 = threshold_env.parse().expect("Invalid WHALE_THRESHOLD number");
    let threshold_decimals = threshold_u128 * 1_000_000;

    let contract_str = env::var("TARGET_CONTRACT").unwrap_or_else(|_| "0xdac17f958d2ee523a2206206994597c13d831ec7".to_string());
    let contract_address = Address::from_str(&contract_str).expect("Invalid contract address");

    info!("🚀 Watcher Started");
    info!("🎯 Target: {:?}", contract_address);
    info!("💰 Threshold: ${}", threshold_u128);

    // Loop reconnect
    loop {
        info!("Connecting to WebSocket...");

        match run_watcher(&rpc_url, contract_address, threshold_decimals, tg_config.clone()).await {
            Ok(_) => {
                warn!("⚠️ Stream ended unexpectedly. Reconnecting in 5s...");
            }
            Err(e) => {
                error!("❌ Connection error: {:?}. Reconnecting in 5s...", e);
            }
        }

        tokio::time::sleep(Duration::from_secs(5)).await;
    }
}

async fn run_watcher(rpc_url: &str, target: Address, threshold: u128, tg_config: TelegramConfig) -> Result<()> {
    let ws = WsConnect::new(rpc_url);
    let provider = ProviderBuilder::new().connect_ws(ws).await?;

    let display_threshold = threshold / 1_000_000;
    info!("✅ Connected! Listening for USDT transfers > {}...\n", display_threshold);

    let filter = Filter::new()
        .address(target)
        .event_signature(Transfer::SIGNATURE_HASH);

    let sub = provider.subscribe_logs(&filter).await?;
    let mut stream = sub.into_stream();

    while let Some(log) = stream.next().await {
        if let Ok(decoded) = log.log_decode::<Transfer>() {
            let transfer = decoded.inner.data;
            let amount_u128 = transfer.value.saturating_to::<u128>();

            if amount_u128 >= threshold {
                let amount_formatted = amount_u128 as f64 / 1_000_000.0;
                let tx_hash = log.transaction_hash.unwrap_or_default();

                info!(
                    "🐋 WHALE DETECTED: ${:.2} | Tx: {:?}",
                    amount_formatted, tx_hash
                );

                let message = format!(
                    "🚨 <b>WHALE ALERT</b> 🚨\n\n💰 <b>Amount:</b> ${:.2} USDT\n📤 <b>From:</b> <code>{}</code>\n📥 <b>To:</b> <code>{}</code>\n\n🔗 <a href=\"https://etherscan.io/tx/{}\">View Transaction</a>",
                    amount_formatted,
                    transfer.from,
                    transfer.to,
                    tx_hash
                );

                spawn_telegram_alert(tg_config.clone(), message);
            }
        }
    }

    Ok(())
}