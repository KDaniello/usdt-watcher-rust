# 🐋 USDT Whale Watcher

A high-performance, asynchronous Rust application that monitors the Ethereum blockchain in real-time for large USDT transfers (>10000) and sends instant alerts to Telegram.

Built with the **Alloy** stack for Ethereum interaction.

## Features

- **Real-time Monitoring:** Connects to Ethereum nodes via WebSocket (WSS).
- **Event Filtering:** Efficiently filters logs on the node side (saves bandwidth).
- **Smart Decoding:** Decodes raw binary ABI data using `sol!` macro.
- **Telegram Integration:** Sends formatted HTML alerts with Etherscan links.
- **Async Runtime:** Fully non-blocking architecture powered by `tokio`.

## 🛠️ Tech Stack

- **Language:** Rust
- **Blockchain Client:** [Alloy]
- **Async Runtime:** Tokio
- **HTTP Client:** Reqwest (with native-tls)
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
```

### 3. Run
```bash
cargo run --release
```

## 📸 Example Alert

🚨 WHALE ALERT 🚨

💰 Amount: $50,000.00 USDT
📤 From: 0x123...abc
📥 To: 0x456...def

🔗 View Transaction [Link]