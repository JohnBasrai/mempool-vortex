# mempool-vortex

[![Rust](https://img.shields.io/badge/Rust-Edition%202021-orange)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

🚀 A fast Rust pipeline for simulating MEV behavior via Ethereum mempool observation.
> An educational repo to simulate Ethereum MEV behavior using Rust, built for showcasing backend + low-latency system skills.

## Features

- Connects to Ethereum RPC via WebSocket (e.g., Sepolia on Alchemy)
- Listens to pending transactions in real-time
- Logs sender, receiver, value, and gas price
- Highlights high-value transfers
- CLI flags for logging, color control, and RPC URL
- Graceful fallback to `.env` for configuration

## Usage

```sh
cargo run --release -- [OPTIONS]
```

### Options

|  Flag / Option                  |  Description                                      |
|:--------------------------------|:--------------------------------------------------|
| `-v`, `--verbose`               | Enable debug logging                              |
| `--rpc-url <RPC_URL>`           | Ethereum WebSocket URL (can use `.env`)           |
| `--simulate`                    | Run in simulation mode (stub)                     |
| `--max-tx <MAX_TX>`             | Number of transactions to process (default: 200)  |
| `--color <auto\|always\|never>` | Control ANSI color in log output                  |

## Setup

1. Create a free account at [Alchemy](https://alchemy.com) and get a Sepolia WebSocket URL.
2. Create a `.env` file:

   ```env
   ETH_RPC_URL=wss://eth-sepolia.g.alchemy.com/v2/your_api_key
   ```

3. Run:

   ```sh
   cargo run --release
   ```

## Example Output

```
📡 Listening to pending transactions...
🔍 tx: from=0xabc…1234 → to=0xdef…5678, value=0.05 ETH, gas_price=10.2 gwei
🚨 High-value tx detected: 1.25 ETH
✅ Processed 200 transactions. Exiting.
```

## License

MIT

---
