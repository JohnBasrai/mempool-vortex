# mempool-vortex

[![Rust](https://img.shields.io/badge/Rust-Edition%202021-orange)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

🚀 A fast Rust pipeline for simulating MEV behavior via Ethereum mempool observation.
> An educational repo to simulate Ethereum MEV behavior using Rust, built for showcasing backend + low-latency system skills.

## Features

- Streams real-time pending Ethereum transactions
- Connects to Ethereum RPC via WebSocket (e.g., Sepolia on Alchemy)
- Logs sender, receiver, value, and gas price
- Per-transaction latency measurement and structured logging
- Highlights high-value transfers
- Address formatting options: short elided or full checksummed (via `--addr-style`)
- Formatted debug output for in-depth inspection of each transaction
- CLI flags for logging verbosity, RPC URL, and terminal color control
- Graceful fallback to `.env` for configuration


## Usage

```sh
cargo run --release -- [OPTIONS]
```
### ⚙️ Command Line Options

| Flag / Option                   | Description                                                                                   | Default        |
| ------------------------------- | --------------------------------------------------------------------------------------------- | -------------- |
| `--verbose`                     | Enable verbose logging (DEBUG level)                                                          | `false`        |
| `--simulate`                    | Enable simulation mode (no actual execution)                                                  | `false`        |
| `--max-tx <N>`                  | Stop after processing `N` transactions                                                        | Unlimited      |
| `--rpc-url <URL>`               | Ethereum RPC WebSocket endpoint (can also be set via `ETH_RPC_URL` env var)                   | `.env` or none |
| `--color <auto\|always\|never>` | Control ANSI color output in logs                                                             | `auto`         |
| `--addr-style <short\|full>`    | Address display style:<br>• `short`: checksummed, middle elided<br>• `full`: full checksummed | `short`        |
| `-h`, `--help`                  | Show help message                                                                             | —              |

### Example usage

```
cargo run -- --verbose --addr-style full --max-tx 20
```

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
⏱️ Processed tx latency_ms=73 from=0xabc…1234 to=0xdef…5678 value_eth="0.05" gas_price_gwei="10.2"
🔍 tx: from=0xabc…1234 → to=0xdef…5678, value=0.05 ETH, gas_price=10.2 gwei
✅ Processed 200 transactions. Exiting.
```

## License

MIT

---
