## [0.2.0] - 2025-09-27

### Added
- 🚀 **MEV pipeline integration**:
  - `searcher.rs`: detects arbitrage, sandwich, and liquidation opportunities from live txs
  - `bundler.rs`: builds MEV bundles and simulates submission to Flashbots, bloXroute, Eden
  - `mempool.rs`: orchestrates full detection-to-execution pipeline
- 🧪 `--simulate` mode for dry-run MEV testing (no actual bundle submission)
- 🎨 CLI enhancements:
  - `--addr-style short|full` for pretty address formatting
  - `--color` control for ANSI log output
  - `--max-tx` to cap number of txs processed
- 📦 Bundle tracking with UUIDs, gas estimates, and simulated inclusion probabilities
- 🔍 Transaction classification and decoding (Uniswap V2/V3, ERC20 transfers, etc.)
- 🧰 Scaffolding for configuration (`types.rs::Config`) and performance metrics (`MEVMetrics`)
- 🧱 Base structure for adding new DEXes, strategies, and real relay submission

### Fixed
- 🐛 Compilation failure due to `AddrStyle` being moved in async logger
- 🧼 Clean startup flow with dotenv fallback and tracing initialization

### Notes
- This release establishes the foundation for real MEV simulation and bundle orchestration.
- Future versions will include:
  - Integration test harness
  - Config + metrics usage
  - Live RPC interaction with relay APIs
  - Risk + profitability constraints
