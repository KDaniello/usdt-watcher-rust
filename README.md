# 🐋 USDT Whale Watcher

A high-performance, asynchronous Rust application that monitors the Ethereum blockchain in real-time for large USDT transfers ("Whale Movements") and sends instant alerts to Telegram.

Unlike simple scripts, this watcher uses a **non-blocking architecture**: blockchain event processing continues uninterrupted even while network requests to Telegram are being sent.

## Features

- **Real-time Monitoring:** Connects to Ethereum nodes via WebSocket (WSS) using the modern `alloy-rs` library.
- **Node-Side Filtering:** Efficiently filters logs on the RPC node (saves bandwidth and CPU).
- **Non-blocking Alerts:** Uses `tokio::spawn` to send HTTP requests to Telegram asynchronously without blocking the WebSocket stream.
- **Configurable Threshold:** Alert sensitivity can be adjusted via environment variables (e.g., $10k, $50k).
- **Smart Decoding:** Decodes raw binary EVM logs using the `sol!` macro.

## 🛠️ Tech Stack

- **Language:** Rust
- **Blockchain Client:** [Alloy]
- **Async Runtime:** Tokio
- **HTTP Client:** Reqwest (with native-tls and http2)
- **Serialization:** Serde

## 📦 Installation & Setup

### 1. Clone the repository
```bash
git clone https://github.com/KDaniello/rust-usdt-watcher.git
cd usdt_watcher
```

### 2. Configure Environment
Create a .env file in the root directory:

```INI
# Your Ethereum Node WSS URL (Infura, Alchemy, QuickNode, or public node)
RPC_URL=wss://mainnet.infura.io/ws/v3/YOUR_API_KEY

# Telegram Bot Config
TELEGRAM_BOT_TOKEN=123456789:ABCdefGHIjklMNOpqrsTUVwxyz
TELEGRAM_CHAT_ID=123456789

# Minimum amount to alert (in USD). Default: 10000
WHALE_THRESHOLD=50000
```

### 3. Run
```bash
cargo run --release
```

## 📸 Example Alert

```text
🚨 WHALE ALERT 🚨

💰 Amount: $50,000.00 USDT
📤 From: 0x123...abc
📥 To: 0x456...def

🔗 View Transaction [Link]
```

## Licence
This project is open-source and available under the MIT License.
