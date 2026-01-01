use alloy::{
    primitives::{Address, U256, address},
    providers::{Provider, ProviderBuilder, WsConnect},
    rpc::types::Filter,
    sol, sol_types::SolEvent,
};
use eyre::Result;
use futures_util::StreamExt;
use std::env;
use std::collections::HashMap;

sol! {
    event Transfer(address indexed from, address indexed to, uint256 value);
}

const USDT_ADDRESS: Address = address!("dac17f958d2ee523a2206206994597c13d831ec7");
const WHALE_THRESHOLD: u128 = 10_000*1_000_000;

// Func: Send to telegram
async fn send_telegram_alert(message: &str) -> Result<()> {

    let token = env::var("TELEGRAM_BOT_TOKEN").expect("TELEGRAM_BOT_TOKEN not set");
    let chat_id = env::var("TELEGRAM_CHAT_ID").expect("TELEGRAM_CHAT_ID not set");

    let url = format!("https://api.telegram.org/bot{}/sendMessage", token);

    let mut params = HashMap::new();
    params.insert("chat_id", chat_id);
    params.insert("text", message.to_string());
    params.insert("parse_mode", "HTML".to_string());

    let client = reqwest::Client::new();
    let res = client.post(&url).json(&params).send().await;

    match res {
        Ok(_) => println!("✅ Alert sent to Telegram"),
        Err(e) => println!("❌ Failed to send alert: {:?}", e),
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();

    let rpc_url = env::var("RPC_URL").expect("RPC_URL must be set in .env");

    println!("Connecting to Ethereum via WS...");

    let ws = WsConnect::new(rpc_url);
    let provider = ProviderBuilder::new().connect_ws(ws).await?;

    println!("Connected! Listening for USDT transfers > $10,000...\n");

    let filter = Filter::new()
        .address(USDT_ADDRESS)
        .event_signature(Transfer::SIGNATURE_HASH);

    let sub = provider.subscribe_logs(&filter).await?;
    let mut stream = sub.into_stream();

    while let Some(log) = stream.next().await {
        if let Ok(decoded) = log.log_decode::<Transfer>() {
            let transfer = decoded.inner.data;
            let amount_u128 = transfer.value.saturating_to::<u128>();

            if amount_u128 >= WHALE_THRESHOLD {
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

                if let Err(e) = send_telegram_alert(&message).await {
                    eprint!("Telegram error: {:?}", e);
                }
            }
        }
    }

    Ok(())
}