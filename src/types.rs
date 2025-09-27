//! Shared types, configuration structs, and helper enums for mempool-vortex.
//!
//! This module contains common data structures used across the MEV pipeline,
//! including configuration management, MEV strategy parameters, and shared utilities.

use ethers::types::{Address, U256};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ---

/// Global configuration for the MEV pipeline.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    /// Ethereum RPC WebSocket URL
    pub eth_rpc_url: String,

    /// Private key for signing transactions (optional for simulation)
    pub private_key: Option<String>,

    /// MEV strategy configuration
    pub mev_config: MEVConfig,

    /// Relay endpoints configuration
    pub relay_config: RelayConfiguration,

    /// Gas price strategy settings
    pub gas_config: GasConfiguration,
}

/// MEV-specific configuration parameters.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MEVConfig {
    /// Minimum profit threshold in ETH to execute opportunities
    pub min_profit_eth: f64,

    /// Maximum gas price in gwei for profitable execution
    pub max_gas_price_gwei: u64,

    /// Arbitrage strategy settings
    pub arbitrage: ArbitrageConfig,

    /// Sandwich attack strategy settings  
    pub sandwich: SandwichConfig,

    /// Liquidation strategy settings
    pub liquidation: LiquidationConfig,
}

/// Arbitrage strategy configuration.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ArbitrageConfig {
    /// Minimum trade size in ETH to consider for arbitrage
    pub min_trade_size_eth: f64,

    /// Maximum slippage tolerance as percentage (0.0-100.0)
    pub max_slippage_percent: f64,

    /// Enabled DEX list for arbitrage detection
    pub enabled_dexs: Vec<String>,

    /// Token whitelist for arbitrage (empty = all tokens)
    pub token_whitelist: Vec<Address>,
}

/// Sandwich attack strategy configuration.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SandwichConfig {
    /// Minimum victim trade size in ETH to sandwich
    pub min_victim_size_eth: f64,

    /// Maximum frontrun amount as percentage of victim trade (0.0-100.0)
    pub max_frontrun_percent: f64,

    /// Gas price buffer above victim for frontrun priority
    pub gas_price_buffer_gwei: u64,

    /// Enabled for sandwich attacks
    pub enabled: bool,
}

/// Liquidation strategy configuration.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LiquidationConfig {
    /// Minimum liquidation bonus in ETH to execute
    pub min_bonus_eth: f64,

    /// Health factor threshold below which to attempt liquidation
    pub health_factor_threshold: f64,

    /// Enabled lending protocols for liquidation monitoring
    pub enabled_protocols: Vec<String>,

    /// Flash loan providers configuration
    pub flash_loan_providers: Vec<String>,
}

/// MEV relay configuration.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RelayConfiguration {
    /// Priority order for relay submission
    pub priority_order: Vec<String>,

    /// Individual relay settings
    pub relays: HashMap<String, RelaySettings>,

    /// Default timeout for relay submissions in seconds
    pub submission_timeout_secs: u64,
}

/// Individual relay endpoint settings.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RelaySettings {
    /// Relay endpoint URL
    pub endpoint: String,

    /// Authentication key/header for the relay
    pub auth_key: Option<String>,

    /// Whether this relay is enabled
    pub enabled: bool,

    /// Expected inclusion probability (0.0-1.0)
    pub inclusion_probability: f64,

    /// Average submission latency in milliseconds
    pub avg_latency_ms: u64,
}

/// Gas price strategy configuration.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GasConfiguration {
    /// Base gas price strategy
    pub strategy: GasStrategy,

    /// Priority fee strategy for EIP-1559
    pub priority_fee_strategy: PriorityFeeStrategy,

    /// Maximum gas price limit in gwei
    pub max_gas_price_gwei: u64,

    /// Gas limit multiplier for safety margin (e.g., 1.2 = 20% buffer)
    pub gas_limit_multiplier: f64,
}

/// Gas price calculation strategies.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum GasStrategy {
    /// Use fixed gas price in gwei
    Fixed(u64),

    /// Use network average + buffer
    NetworkAverage { buffer_gwei: u64 },

    /// Use percentile of recent transactions
    Percentile { percentile: u8 },

    /// Aggressive pricing for MEV competition
    Aggressive { multiplier: f64 },
}

/// Priority fee strategies for EIP-1559 transactions.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum PriorityFeeStrategy {
    /// Fixed priority fee in gwei
    Fixed(u64),

    /// Dynamic based on network congestion
    Dynamic { base_fee_multiplier: f64 },

    /// Competitive MEV pricing
    Competitive { min_priority_gwei: u64 },
}

/// Performance metrics for MEV operations.
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct MEVMetrics {
    /// Total transactions analyzed
    pub transactions_analyzed: u64,

    /// Total opportunities detected
    pub opportunities_detected: u64,

    /// Total bundles submitted
    pub bundles_submitted: u64,

    /// Total bundles included on-chain
    pub bundles_included: u64,

    /// Total profit realized in ETH
    pub total_profit_eth: f64,

    /// Total gas costs in ETH
    pub total_gas_costs_eth: f64,

    /// Net profit in ETH
    pub net_profit_eth: f64,

    /// Opportunity type breakdown
    pub arbitrage_count: u64,
    pub sandwich_count: u64,
    pub liquidation_count: u64,

    /// Average processing latency in milliseconds
    pub avg_processing_latency_ms: f64,

    /// Success rate (included bundles / submitted bundles)
    pub success_rate: f64,
}

/// Token metadata for MEV analysis.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TokenInfo {
    /// Token contract address
    pub address: Address,

    /// Token symbol (e.g., "USDC", "WETH")
    pub symbol: String,

    /// Token name
    pub name: String,

    /// Number of decimals
    pub decimals: u8,

    /// Whether this token is actively traded
    pub is_active: bool,

    /// Liquidity score (0.0-1.0)
    pub liquidity_score: f64,

    /// Average daily volume in USD
    pub avg_daily_volume_usd: f64,
}

/// DEX pool information for arbitrage calculations.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PoolInfo {
    /// Pool contract address
    pub address: Address,

    /// DEX protocol name
    pub dex: String,

    /// Token A in the pair
    pub token_a: Address,

    /// Token B in the pair
    pub token_b: Address,

    /// Current reserves of token A
    pub reserve_a: U256,

    /// Current reserves of token B
    pub reserve_b: U256,

    /// Pool fee (basis points, e.g., 30 = 0.3%)
    pub fee_bps: u16,

    /// Total liquidity in USD
    pub liquidity_usd: f64,

    /// Last updated timestamp
    pub last_updated: u64,
}

/// Risk parameters for MEV strategies.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RiskParameters {
    /// Maximum position size in ETH
    pub max_position_size_eth: f64,

    /// Maximum number of concurrent opportunities
    pub max_concurrent_opportunities: u8,

    /// Blacklisted tokens (avoid due to risk)
    pub token_blacklist: Vec<Address>,

    /// Blacklisted addresses (suspicious activity)
    pub address_blacklist: Vec<Address>,

    /// Minimum confirmations before considering transaction final
    pub min_confirmations: u8,

    /// Circuit breaker: max losses before stopping (ETH)
    pub max_daily_loss_eth: f64,
}

// ---

impl Default for Config {
    fn default() -> Self {
        Self {
            eth_rpc_url: "wss://eth-mainnet.g.alchemy.com/v2/your_api_key".to_string(),
            private_key: None,
            mev_config: MEVConfig::default(),
            relay_config: RelayConfiguration::default(),
            gas_config: GasConfiguration::default(),
        }
    }
}

impl Default for MEVConfig {
    fn default() -> Self {
        Self {
            min_profit_eth: 0.01,    // 0.01 ETH minimum profit
            max_gas_price_gwei: 200, // 200 gwei max
            arbitrage: ArbitrageConfig::default(),
            sandwich: SandwichConfig::default(),
            liquidation: LiquidationConfig::default(),
        }
    }
}

impl Default for ArbitrageConfig {
    fn default() -> Self {
        Self {
            min_trade_size_eth: 1.0,   // 1 ETH minimum
            max_slippage_percent: 2.0, // 2% max slippage
            enabled_dexs: vec![
                "uniswap_v2".to_string(),
                "uniswap_v3".to_string(),
                "sushiswap".to_string(),
            ],
            token_whitelist: Vec::new(), // All tokens allowed by default
        }
    }
}

impl Default for SandwichConfig {
    fn default() -> Self {
        Self {
            min_victim_size_eth: 5.0,   // 5 ETH minimum victim trade
            max_frontrun_percent: 15.0, // 15% max frontrun size
            gas_price_buffer_gwei: 5,   // 5 gwei buffer above victim
            enabled: false,             // Disabled by default (more risky)
        }
    }
}

impl Default for LiquidationConfig {
    fn default() -> Self {
        Self {
            min_bonus_eth: 0.05,          // 0.05 ETH minimum bonus
            health_factor_threshold: 1.0, // Below 1.0 health factor
            enabled_protocols: vec!["aave".to_string(), "compound".to_string()],
            flash_loan_providers: vec!["aave".to_string(), "dydx".to_string()],
        }
    }
}

impl Default for RelayConfiguration {
    fn default() -> Self {
        let mut relays = HashMap::new();

        relays.insert(
            "flashbots".to_string(),
            RelaySettings {
                endpoint: "https://relay.flashbots.net".to_string(),
                auth_key: None,
                enabled: true,
                inclusion_probability: 0.85,
                avg_latency_ms: 150,
            },
        );

        relays.insert(
            "bloXroute".to_string(),
            RelaySettings {
                endpoint: "https://mev.api.blxrbdn.com".to_string(),
                auth_key: None,
                enabled: true,
                inclusion_probability: 0.75,
                avg_latency_ms: 120,
            },
        );

        Self {
            priority_order: vec!["flashbots".to_string(), "bloXroute".to_string()],
            relays,
            submission_timeout_secs: 10,
        }
    }
}

impl Default for GasConfiguration {
    fn default() -> Self {
        Self {
            strategy: GasStrategy::NetworkAverage { buffer_gwei: 10 },
            priority_fee_strategy: PriorityFeeStrategy::Dynamic {
                base_fee_multiplier: 1.5,
            },
            max_gas_price_gwei: 300,
            gas_limit_multiplier: 1.2,
        }
    }
}

impl MEVMetrics {
    /// Updates metrics after processing a transaction.
    pub fn record_transaction(&mut self) {
        self.transactions_analyzed += 1;
    }

    /// Records a detected MEV opportunity.
    pub fn record_opportunity(&mut self, opportunity_type: &str) {
        self.opportunities_detected += 1;
        match opportunity_type {
            "arbitrage" => self.arbitrage_count += 1,
            "sandwich" => self.sandwich_count += 1,
            "liquidation" => self.liquidation_count += 1,
            _ => {}
        }
    }

    /// Records a bundle submission.
    pub fn record_bundle_submission(&mut self) {
        self.bundles_submitted += 1;
    }

    /// Records a successful bundle inclusion.
    pub fn record_bundle_inclusion(&mut self, profit_eth: f64, gas_cost_eth: f64) {
        self.bundles_included += 1;
        self.total_profit_eth += profit_eth;
        self.total_gas_costs_eth += gas_cost_eth;
        self.net_profit_eth = self.total_profit_eth - self.total_gas_costs_eth;
        self.success_rate = self.bundles_included as f64 / self.bundles_submitted as f64;
    }
}

/// Utility functions for configuration management.
impl Config {
    /// Loads configuration from environment variables and config files.
    pub fn from_env() -> anyhow::Result<Self> {
        let mut config = Config::default();

        // Override with environment variables
        if let Ok(rpc_url) = std::env::var("ETH_RPC_URL") {
            config.eth_rpc_url = rpc_url;
        }

        if let Ok(private_key) = std::env::var("PRIVATE_KEY") {
            config.private_key = Some(private_key);
        }

        // Load additional config from file if exists
        if let Ok(config_str) = std::fs::read_to_string("mev_config.json") {
            if let Ok(file_config) = serde_json::from_str::<Config>(&config_str) {
                config = file_config;
            }
        }

        Ok(config)
    }

    /// Validates the configuration for completeness and correctness.
    pub fn validate(&self) -> anyhow::Result<()> {
        if self.eth_rpc_url.is_empty() {
            anyhow::bail!("ETH_RPC_URL cannot be empty");
        }

        if !self.eth_rpc_url.starts_with("wss://") {
            anyhow::bail!("ETH_RPC_URL must be a WebSocket URL (wss://)");
        }

        if self.mev_config.min_profit_eth <= 0.0 {
            anyhow::bail!("Minimum profit must be positive");
        }

        if self.mev_config.max_gas_price_gwei == 0 {
            anyhow::bail!("Maximum gas price must be positive");
        }

        Ok(())
    }
}
