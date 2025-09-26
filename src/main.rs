//! mempool-vortex
//!
//! A fast Rust pipeline for simulating MEV behavior via Ethereum mempool observation.
//! Connects to an Ethereum node via WebSocket, listens for pending transactions,
//! and logs relevant details. Includes optional filtering, logging, and formatting controls.

use clap::Parser;
use dotenv::dotenv;
use tracing::{debug, info};
use tracing_subscriber;

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
    mempool::listen_to_mempool(&rpc_url, cli.max_tx).await?;
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
    about = "A fast Rust pipeline for simulating MEV behavior via Ethereum mempool observation."
)]
pub struct Args {
    // --
    /// Enable verbose (debug) logging
    #[arg(short, long)]
    pub verbose: bool,

    /// Run in simulation mode (no real bundle submission)
    #[arg(long)]
    pub simulate: bool,

    /// Ethereum RPC WebSocket URL (e.g., wss://...)
    ///
    /// Optional: If not provided, will be read from `.env` as `ETH_RPC_URL`.
    #[arg(
        long,
        help = "Ethereum RPC WebSocket URL (e.g., wss://...). Optional: can also be set via ETH_RPC_URL in .env"
    )]
    rpc_url: Option<String>,

    /// Maximum number of transactions to process before exiting.
    #[arg(
        long,
        default_value = "200",
        help = "Maximum number of transactions to process before exiting"
    )]
    pub max_tx: usize,

    /// Control colored log output for terminal compatibility.
    ///
    /// - auto: Detect terminal capabilities (default)
    /// - always: Force color output
    /// - never: Disable color output
    #[arg(long, value_enum, default_value = "auto")]
    pub color: ColorChoice,
}

// ---

/// Available options for controlling terminal log color output.
#[derive(clap::ValueEnum, Clone, Debug)]
pub enum ColorChoice {
    Auto,
    Always,
    Never,
}
