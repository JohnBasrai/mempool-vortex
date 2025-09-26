use clap::Parser;
use dotenv::dotenv;
use tracing::{debug, info};
use tracing_subscriber;

mod bundler;
mod mempool;
mod searcher;
mod types;

// ---

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // ---

    dotenv().ok();

    let args = Args::parse();
    let log_level = if args.verbose { "debug" } else { "info" };

    // ---

    // Initialize tracing with smart colorization
    let use_color = match args.color {
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
    debug!("CLI args: {:?}", args);

    // ---

    // Placeholder for pipeline
    mempool::listen_to_mempool().await?;
    // searcher::evaluate_opportunity();
    // bundler::send_bundle().await?;

    Ok(())
}

// ---

#[derive(Parser, Debug)]
#[command(
    name = "mempool-vortex",
    version,
    about = "A fast Rust pipeline for simulating MEV behavior via Ethereum mempool observation."
)]
pub struct Args {
    /// Enable verbose (debug) logging
    #[arg(short, long)]
    pub verbose: bool,

    /// Run in simulation mode (no real bundle submission)
    #[arg(long)]
    pub simulate: bool,

    /// Control colored log output for terminal compatibility
    #[arg(long, value_enum, default_value = "auto")]
    pub color: ColorChoice,
}

// ---

/// Controls colored output in logs
#[derive(clap::ValueEnum, Clone, Debug)]
pub enum ColorChoice {
    Auto,
    Always,
    Never,
}
