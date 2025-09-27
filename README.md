# mempool-vortex

[![Rust](https://img.shields.io/badge/Rust-Edition%202021-orange)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

🚀 A modular Rust pipeline for simulating MEV strategies via Ethereum mempool observation.

> Built for educational, research, and prototyping use cases — showcasing high-performance mempool processing, strategy detection, and bundle orchestration.

---

## ✨ Features

- 📡 Listens to real-time Ethereum mempool (pending txs) via WebSocket
- 🧠 Detects **MEV opportunities**:
  - Arbitrage across AMMs (Uniswap V2/V3)
  - Sandwich attacks
  - Liquidation opportunities
- 📦 Builds and simulates bundles for submission to MEV relays (Flashbots, bloXroute, Eden)
- 🧪 Simulation-first: run full strategy logic without executing on-chain
- 🖥️ Rich CLI with colorized logs, address formatting, and tx limits
- 🛠️ Extensible design for new strategies, relays, and config systems

---

## 🚀 Usage

```bash
cargo run --release -- [OPTIONS]
```

### ⚙️ Command Line Options

| Flag / Option                   | Description                                                                                   | Default        |
| ------------------------------- | --------------------------------------------------------------------------------------------- | -------------- |
| `--verbose`                     | Enable verbose logging (DEBUG level)                                                          | `false`        |
| `--simulate`                    | Enable simulation mode (no actual relay submission)                                           | `false`        |
| `--max-tx <N>`                  | Stop after processing `N` transactions                                                        | Unlimited      |
| `--rpc-url <URL>`               | Ethereum RPC WebSocket endpoint (`ETH_RPC_URL` env fallback)                                 | `.env` or none |
| `--color <auto\|always\|never>` | Control ANSI color output in logs                                                             | `auto`         |
| `--addr-style <short\|full>`    | Address display:<br>• `short`: checksummed, middle elided<br>• `full`: full checksummed       | `short`        |
| `-h`, `--help`                  | Show help message                                                                             | —              |

### 🧪 Example: Simulated Run

```bash
cargo run -- --simulate --verbose --addr-style full --max-tx 25
```

---

## ⚙️ Setup

1. Get a free WebSocket RPC URL from [Alchemy](https://alchemy.com) or [Infura](https://infura.io).
2. Create a `.env` file:

   ```env
   ETH_RPC_URL=wss://eth-sepolia.g.alchemy.com/v2/your_api_key
   ```

3. Run the MEV pipeline:

   ```bash
   cargo run --release -- --simulate
   ```

---

## 🧪 Example Output (Simulation Mode)

```text
📡 Listening to pending transactions...
🔍 tx: from=0xabc…1234 → to=0xdef…5678, value=0.42 ETH, gas_price=10.2 gwei
📦 [Sim] MEV opportunity detected: arbitrage between UniswapV2 <-> UniswapV3
⏱️ Bundle created: UUID=123e4567-e89b-12d3-a456-426614174000
✅ Simulated submission to relays [Flashbots, bloXroute]
```

---

## 🔭 Roadmap

Planned for future versions:
- ✅ Integration test harness for strategy simulation
- 📊 Metrics system with Prometheus-compatible `MEVMetrics`
- ⚙️ Config loading from JSON / `.env`
- 🛠️ Live relay auth and real bundle submission
- 🧠 Strategy evaluation cost/profit/risk thresholds

---

## 📜 License

MIT

---
