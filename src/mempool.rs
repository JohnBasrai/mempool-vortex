//! Ethereum mempool listener module for mempool-vortex.
//!
//! Provides functionality to connect to an Ethereum node over WebSocket,
//! subscribe to pending transactions, decode their metadata, analyze them
//! for MEV opportunities, and execute profitable strategies via bundle submission.

use super::AddrStyle;
use crate::{bundler, searcher};
use ethers::providers::{Middleware, Provider, StreamExt, Ws};
use ethers::types::{Address, Transaction};
use ethers::utils::to_checksum;
use std::sync::Arc;
use std::time::Instant;
use tracing::{debug, error, info, warn};

// ---

/// Starts listening to the Ethereum mempool for pending transactions with full MEV pipeline.
///
/// Connects to the given WebSocket RPC URL, subscribes to pending transaction hashes,
/// fetches each transaction, analyzes it for MEV opportunities, and executes profitable
/// strategies. Exits after processing `max_tx` transactions or when the stream terminates.
///
/// # Arguments
///
/// * `rpc_url` - Ethereum WebSocket endpoint (e.g., wss://eth-sepolia.g.alchemy.com/v2/...).
/// * `max_tx` - Maximum number of transactions to process before exiting.
/// * `addr_style` - Address rendering mode used when logging transactions
///                  (`short` elides the middle; `full` prints full EIP-55).
/// * `simulate` - Whether to simulate MEV execution without actual bundle submission.
///
/// # Errors
///
/// Returns an error if the WebSocket connection fails or transaction fetch fails.
pub async fn listen_to_mempool(
    rpc_url: &str,
    max_tx: usize,
    addr_style: AddrStyle,
    simulate: bool,
) -> anyhow::Result<()> {
    // ---

    let provider = Arc::new(Provider::<Ws>::connect(rpc_url).await?);
    let mut stream = provider.subscribe_pending_txs().await?;

    info!("📡 Listening to pending transactions with MEV analysis...");

    if simulate {
        info!(
            "🧪 Running in simulation mode - MEV opportunities will be detected but not executed"
        );
    }

    let mut join_set = tokio::task::JoinSet::new();
    let mut count = 0;
    let mut opportunities_found = 0;

    while let Some(tx_hash) = stream.next().await {
        // ---

        let provider = provider.clone();
        let addr_style = addr_style.clone();

        join_set.spawn(async move {
            // ---
            let start = Instant::now();

            match provider.get_transaction(tx_hash).await {
                Ok(Some(tx)) => {
                    // Log basic transaction details
                    log_transaction(&tx, start, addr_style);

                    // Analyze for MEV opportunities
                    if let Some(opportunity) = searcher::evaluate_opportunity(&tx).await {
                        info!("🎯 MEV opportunity detected: {:?}",
                              std::mem::discriminant(&opportunity));

                        // Execute the opportunity (create and submit bundle)
                        match bundler::create_and_send_bundle(opportunity, simulate).await {
                            Ok(result) => {
                                info!("📦 Bundle submission result: {:?}", result.status);
                                if !simulate {
                                    info!("💰 Bundle {} submitted to {} with {:.1}% inclusion probability",
                                          result.bundle_hash,
                                          result.relay,
                                          result.inclusion_probability.unwrap_or(0.0) * 100.0);
                                }
                                1 // Return count of opportunities found
                            }
                            Err(e) => {
                                error!("❌ Failed to create/submit bundle: {}", e);
                                0
                            }
                        }
                    } else {
                        0 // No opportunity found
                    }
                }
                Ok(None) => {
                    debug!("Transaction {} not found", tx_hash);
                    0
                }
                Err(e) => {
                    warn!("Failed to fetch transaction {}: {}", tx_hash, e);
                    0
                }
            }
        });

        count += 1;
        if count >= max_tx {
            break;
        }
    }

    // Wait for all spawned tasks to complete and count opportunities
    while let Some(res) = join_set.join_next().await {
        if let Ok(found) = res {
            opportunities_found += found;
        }
    }

    info!(
        "✅ Processed {} transactions, found {} MEV opportunities",
        count, opportunities_found
    );
    info!("🏁 Reached max_tx ({}). Exiting.", max_tx);

    Ok(())
}

// ---

/// Logs a summary of a pending transaction, including addresses, ETH value, gas price,
/// and processing latency.
///
/// Also highlights transactions above a value threshold with a high-value alert.
///
/// # Arguments
///
/// * `tx` - A pending Ethereum transaction to inspect and log.
/// * `start_time` - Time when processing of this transaction began.
/// * `addr_style` - How to format addresses in the output.
fn log_transaction(tx: &Transaction, start_time: Instant, addr_style: AddrStyle) {
    // ---

    let from = format_addr(&tx.from, addr_style.clone());
    let to = tx.to.unwrap_or_default();
    let to_formatted = format_addr(&to, addr_style.clone());
    let value_eth = ethers::utils::format_ether(tx.value);
    let gas_price_gwei = tx
        .gas_price
        .map(|gp| ethers::utils::format_units(gp, "gwei").unwrap_or_default())
        .unwrap_or_else(|| "N/A".into());

    let duration = start_time.elapsed();

    debug!(
        latency_ms = %duration.as_millis(),
        from = %&from,
        to = %&to_formatted,
        value_eth,
        gas_price_gwei,
        "⏱️ Processed tx"
    );

    info!(
        "🔍 tx: from={} → to={}, value={} ETH, gas_price={} gwei",
        &from, &to_formatted, value_eth, gas_price_gwei
    );

    // High-value transaction alert
    if tx.value > ethers::utils::parse_ether(0.5).unwrap_or_default() {
        info!("🚨 High-value tx detected: {} ETH", value_eth);
    }

    // Large gas price alert (potential MEV competition)
    if let Some(gas_price) = tx.gas_price {
        let gas_price_gwei_num: f64 = gas_price.as_u64() as f64 / 1_000_000_000.0;
        if gas_price_gwei_num > 100.0 {
            info!(
                "⚡ High gas price detected: {:.1} gwei (potential MEV competition)",
                gas_price_gwei_num
            );
        }
    }
}

/// Format an Ethereum address as a shortened string: `0x1234…abcd`.
/// Always use on raw Address values, never on already-formatted or shortened strings.
///
/// Output format: `0x1234abcd…cdef`
///
/// # Arguments
/// * `addr` - The Ethereum address to format.
///
/// # Returns
/// A shortened string representation suitable for human-readable logs.
fn format_addr_short(addr: &Address) -> String {
    // ---

    // Always generate a fresh ASCII checksummed string (no prior elision)
    let full = to_checksum(addr, None); // e.g. "0x12Ab34…"; ASCII hex, no Unicode except we add it
                                        // Elide by *characters* to avoid UTF-8 boundary issues.
    let prefix: String = full.chars().take(8).collect(); // "0x" + 6 hex
    let suffix: String = full
        .chars()
        .rev()
        .take(4)
        .collect::<String>()
        .chars()
        .rev()
        .collect();
    format!("{prefix}…{suffix}")
}

/// Formats an `ethers::types::Address` according to the selected `AddrStyle`.
///
/// Always derives a fresh EIP-55 checksummed string and applies the chosen
/// presentation. `Short` elides the middle (e.g., `0x12Abcd…90ef`) using
/// character-safe slicing to avoid UTF-8 boundary panics. This is purely
/// a log-presentation helper; the address value is unchanged.
///
/// # Examples
/// - `AddrStyle::Short` → `0x12Abcd…90ef`
/// - `AddrStyle::Full`  → `0x12Abcd34Ef...90ef`
///
/// This is a **presentation helper** only; it does not mutate or reinterpret
/// the underlying address value.
fn format_addr(addr: &ethers::types::Address, style: AddrStyle) -> String {
    // ---
    match style {
        AddrStyle::Full => to_checksum(addr, None),
        AddrStyle::Short => format_addr_short(&addr),
    }
}
