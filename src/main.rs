//! mempool-vortex
//!
//! A fast Rust pipeline for simulating MEV behavior via Ethereum mempool observation.
//! Connects to an Ethereum node via WebSocket, listens for pending transactions,
//! and logs relevant details. Includes optional filtering, logging, and formatting controls.

use clap::Parser;
use dotenv::dotenv;
use tracing::{debug, info};
//e tracing_subscriber;

mod bundler;
mod mempool;
mod searcher;
mod types;

// ---

/// Application entry point.
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // ---

    dotenv().ok();

    let cli = Args::parse();
    let log_level = if cli.verbose { "debug" } else { "info" };

    // ---

    // Initialize tracing with smart colorization
    let use_color = match cli.color {
        ColorChoice::Always => true,
        ColorChoice::Never => false,
        ColorChoice::Auto => {
            // Check if stdout is a terminal and not being redirected
            std::io::IsTerminal::is_terminal(&std::io::stdout())
        }
    };

    tracing_subscriber::fmt()
        .with_env_filter(log_level)
        .with_ansi(use_color)
        .init();

    info!("🚀 mempool-vortex starting...");
    debug!("CLI args: {:?}", cli);

    // Final RPC URL, use command line if available else fallback to .env
    let rpc_url = cli
        .rpc_url
        .clone()
        .or_else(|| std::env::var("ETH_RPC_URL").ok())
        .ok_or_else(|| {
            anyhow::anyhow!(
                "Missing Ethereum RPC URL: provide via --rpc-url or set ETH_RPC_URL in .env"
            )
        })?;

    // ---

    // Placeholder for pipeline
    mempool::listen_to_mempool(&rpc_url, cli.max_tx, cli.addr_style).await?;
    // searcher::evaluate_opportunity();
    // bundler::send_bundle().await?;

    Ok(())
}

// ---

/// Command-line arguments for mempool-vortex.
#[derive(Parser, Debug)]
#[command(
    name = "mempool-vortex",
    version,
    about = "Observe Ethereum mempool and simulate MEV-style processing.",
    long_about = "Observe Ethereum mempool via WebSocket and simulate MEV-style processing.\n\
                  Streams pending transactions, logs key fields, and measures per-tx latency.\n\
                  Addresses render as short/Full checksummed formats for readable logs.",
    after_long_help = "Examples:\n  \
        mempool-vortex --max-tx 200\n  \
        mempool-vortex --rpc-url wss://eth-sepolia.g.alchemy.com/v2/KEY --verbose\n  \
        ETH_RPC_URL=wss://eth-sepolia.g.alchemy.com/v2/KEY mempool-vortex --addr-style full"
)]
pub struct Args {
    // --
    /// Enable verbose (debug) logging
    #[arg(short, long)]
    pub verbose: bool,

    /// Run in simulation mode (no real bundle submission)
    #[arg(long)]
    pub simulate: bool,

    /// Ethereum RPC WebSocket URL to connect to.
    ///
    /// Optional: can also be provided via the ETH_RPC_URL environment variable
    /// (dotenv is supported).
    #[arg(
        long,
        value_name = "WSS_URL",
        env = "ETH_RPC_URL" // clap reads from env (dotenv already loaded in main)
    )]
    rpc_url: Option<String>,

    /// Maximum number of transactions to process before exiting.
    #[arg(
        long,
        value_name = "N",
        default_value = "200",
        help = "Maximum number of transactions to process before exiting"
    )]
    pub max_tx: usize,

    /// Control colored log output for terminal compatibility.
    #[arg(long, value_enum, value_name = "MODE", default_value = "auto")]
    pub color: ColorChoice,

    /// Controls how Ethereum addresses are rendered in logs.
    ///
    /// Use `short` for compact logs or `full` when debugging exact addresses.
    #[arg(
        long,
        value_enum,
        value_name = "STYLE",
        default_value = "short",
        long_help = "Controls how Ethereum addresses are rendered in logs.\n\
                     • short: checksummed with middle elided (e.g., 0x12Abcd…90ef)\n\
                     • full:  full EIP-55 checksummed address"
    )]
    pub addr_style: AddrStyle,
}

// ---

/// Available options for controlling terminal log color output.
#[derive(clap::ValueEnum, Clone, Debug)]
pub enum ColorChoice {
    Auto,
    Always,
    Never,
}

/// How to render Ethereum addresses in logs.
#[derive(clap::ValueEnum, Clone, Debug)]
pub enum AddrStyle {
    // ---
    /// Checksummed address with the middle elided for compact logs.
    Short,

    /// Full EIP-55 checksummed address with no elision.
    Full,
}
