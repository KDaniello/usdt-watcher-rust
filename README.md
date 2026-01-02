# 🐋 USDT Whale Watcher

A production-ready, asynchronous Rust service that monitors Ethereum blockchain events in real-time. Designed for high availability and zero downtime.

It automatically detects large token transfers (Whales) and sends instant non-blocking alerts to Telegram.

## 🚀 Features

### 🛡️ Fault Tolerance & Resilience
Unlike simple scripts, this service implements a **Self-Healing Connection Loop**.
- Automatically detects WebSocket disconnects (e.g., node failures, network issues).
- Implements a reconnection strategy with delay to prevent API rate limiting.

### ⚡ Performance & Concurrency
- **Non-blocking Alerts:** Uses `tokio::spawn` to offload HTTP requests. Blockchain event processing is never blocked by Telegram API latency.
- **Node-Side Filtering:** Uses `eth_subscribe` filters to reduce bandwidth usage by 99%.

### 📊 Observability
- **Structured Logging:** Uses `tracing` crate for standardized logs (INFO/WARN/ERROR), making it easy to integrate with monitoring stacks (ELK, Grafana Loki)

## 🛠️ Tech Stack

- **Core:** Rust, Tokio (Async Runtime)
- **Web3:** Alloy (Modern Ethereum interaction)
- **Networking:** Reqwest (HTTP/2, Native TLS)
- **Logging:** Tracing & Tracing-Subscriber

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

# Contract to Watch (Default is USDT on Mainnet)
# You can change this to USDC, SHIB, or any ERC-20 token.
TARGET_CONTRACT=0xdac17f958d2ee523a2206206994597c13d831ec7
```

### 3. Run
```bash
cargo run --release
```

## 📸 Logs Output
```text
2026-01-02T15:29:54.533999Z  INFO usdt_watcher: 🎯 Target: 0xdac17f958d2ee523a2206206994597c13d831ec7
2026-01-02T15:29:54.534155Z  INFO usdt_watcher: 💰 Threshold: $50000
2026-01-02T15:29:54.534309Z  INFO usdt_watcher: Connecting to WebSocket...
2026-01-02T15:29:55.091613Z  INFO usdt_watcher: ✅ Connected! Listening for USDT transfers > 50000...

2026-01-02T15:30:00.005234Z  INFO usdt_watcher: 🐋 WHALE DETECTED: $200000.00 | Tx: 0x30e9a3596ec6de37276d79b04a0ee96a6a5ede2491a3f75878d65134f69d25ac
2026-01-02T15:30:00.281564Z  INFO usdt_watcher: ✅ Alert sent to Telegram
```

## Telegram Alert
```text
🚨 WHALE ALERT 🚨

💰 Amount: $50,000.00 USDT
📤 From: 0x123...abc
📥 To: 0x456...def

🔗 View Transaction [Link]
```

## 📜 Licence
MIT License.
