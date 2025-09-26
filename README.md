# mempool-vortex

A minimal, high-performance Rust pipeline to explore Ethereum mempool activity and toy MEV searcher logic.

## 🚀 Features

- Connects to Ethereum testnet mempool via WebSocket
- Simulates arbitrage logic between mock DEX pools
- Modular architecture for low-latency event-driven flow
- Built with `tokio`, `ethers-rs`, and clean async structure

## 🧪 Goals

This is not a production MEV bot. It's a focused repo to:
- Demonstrate systems thinking applied to MEV
- Explore real-time blockchain data flow
- Build credibility in performance blockchain infrastructure

## 🛠️ Run Locally

```bash
cp .env.example .env
cargo run
```

## 📦 Structure

- `mempool.rs` – Connect and stream pending transactions
- `searcher.rs` – Analyze toy arbitrage conditions
- `bundler.rs` – (Optional) Submit bundles
- `types.rs` – Shared config/types

## 🔐 .env Setup

To run `mempool-vortex`, you need to create a `.env` file in the project root:

```
cp .env.example .env
```
Then edit `.env` and set your Ethereum RPC URL:

```
ETH_RPC_URL=wss://sepolia.infura.io/ws/v3/YOUR_KEY
PRIVATE_KEY=0xyourprivatekeyhere
```

---
Built for learning, exploration, and sharp systems design ✌️
