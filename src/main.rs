use alloy::{
    primitives::{Address, U256, address},
    providers::{Provider, ProviderBuilder, WsConnect},
    rpc::types::Filter,
    sol, sol_types::SolEvent,
};
use eyre::Result;
use futures_util::StreamExt;
use std::env;

sol! {
    event Transfer(address indexed from, address indexed to, uint256 value);
}

const USDT_ADDRESS: Address = address!("dac17f958d2ee523a2206206994597c13d831ec7");

const WHALE_THRESHOLD: u128 = 10_000*1_000_000;

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

                println!(
                    "🐋 WHALE ALERT! Moved: ${:.2} USDT\n   From: {}\n   To:   {}\n   Tx:   {:?}\n",
                    amount_formatted,
                    transfer.from,
                    transfer.to,
                    log.transaction_hash.unwrap_or_default()
                );
            }
        }
    }

    Ok(())
}