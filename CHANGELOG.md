# Changelog

## [Unreleased]

### Added
- Latency tracking for transaction processing using `Instant`
- Per-transaction debug logs with sender, receiver, ETH value, gas price, and processing time
- Address rendering options via `--addr-style` CLI flag:
  - `short` (default): displays checksummed address with middle elided (`0x12Ab…c34F`)
  - `full`: displays full EIP-55 checksummed address
- Rich `--help` output with examples and inline documentation

### Changed
- Structured debug and info log formatting for clarity and traceability
- Transaction fetch tasks now use `tokio::JoinSet` for coordinated async processing

## [0.1.0] - 2025-09-27

### Added

- Ethereum mempool listener via WebSocket
- CLI with logging and color control
- `.env` fallback support
- Basic searcher: detect and log high-value ETH transactions
- Structured doc comments for main and core module
- Latency tracking for transaction processing using `Instant`
- Per-transaction debug logs with sender, receiver, ETH value, gas price, and processing time
