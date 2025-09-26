//! Ethereum mempool listener module for mempool-vortex.
//!
//! Provides functionality to connect to an Ethereum node over WebSocket,
//! subscribe to pending transactions, decode their metadata,
//! and invoke basic searcher logic such as high-value transaction alerts.
use ethers::prelude::*;
use ethers::providers::{Provider, Ws};
use std::sync::Arc;
use tracing::{info, warn};

// ---

/// Starts listening to the Ethereum mempool for pending transactions.
///
/// Connects to the given WebSocket RPC URL, subscribes to new pending transaction hashes,
/// fetches the full transaction data, and logs key fields.  Exits after processing a
/// maximum number of transactions.
///
/// # Arguments
///
/// * `rpc_url`  - Ethereum WebSocket endpoint (e.g., wss://eth-sepolia.g.alchemy.com/v2/...).
/// * `simulate` - Whether to simulate (no-op for now).
/// * `max_tx`   - Maximum number of transactions to process before exiting.
///
/// # Errors
///
/// Returns an error if the WebSocket connection fails or transaction fetch fails.
pub async fn listen_to_mempool(rpc_url: &str, max_tx: usize) -> anyhow::Result<()> {
    // ---

    let provider = Arc::new(Provider::<Ws>::connect(rpc_url).await?);
    let mut stream = provider.subscribe_pending_txs().await?;

    info!("📡 Listening to pending transactions...");

    let mut count = 0;
    while let Some(tx_hash) = stream.next().await {
        // ---

        let provider = provider.clone();
        tokio::spawn(async move {
            // ---
            match provider.get_transaction(tx_hash).await {
                Ok(Some(tx)) => log_transaction(&tx),
                Ok(None) => {
                    warn!("⏳ Tx not found yet: {tx_hash:?}");
                }
                Err(e) => {
                    warn!("⚠️ Failed to fetch tx {tx_hash:?}: {e}");
                }
            }
        });

        count += 1;
        if count >= max_tx {
            break;
        }
    }

    info!("✅ Reached max_tx ({max_tx}). Exiting.");

    Ok(())
}

// ---

/// Logs a summary of a pending transaction, including addresses, ETH value, and gas price.
///
/// Also highlights transactions above a value threshold with a high-value alert.
///
/// # Arguments
///
/// * `tx` - A pending Ethereum transaction to inspect and log.
fn log_transaction(tx: &Transaction) {
    // ---

    let from = tx.from;
    let to = tx.to.unwrap_or_default(); // Handle Option<Address>
    let value_eth = ethers::utils::format_ether(tx.value);
    let gas_price_gwei = tx
        .gas_price
        .map(|gp| ethers::utils::format_units(gp, "gwei").unwrap_or_default())
        .unwrap_or_else(|| "N/A".into());

    info!(
        "🔍 tx: from={} → to={}, value={} ETH, gas_price={} gwei",
        short(&from),
        short(&to),
        value_eth,
        gas_price_gwei
    );

    if tx.value > ethers::utils::parse_ether(0.5).unwrap_or_default() {
        info!("🚨 High-value tx detected: {} ETH", value_eth);
    }
}

/// Formats an Ethereum address for log display by truncating the middle.
///
/// Outputs format like: `0x1234abcd…cdef`
///
/// # Arguments
///
/// * `addr` - A reference to an Ethereum address.
///
/// # Returns
///
/// A shortened hexadecimal string suitable for human-readable logs.
fn short(addr: &ethers::types::Address) -> String {
    let hex = format!("{:?}", addr);
    format!("{}…{}", &hex[0..8], &hex[hex.len() - 4..])
}
