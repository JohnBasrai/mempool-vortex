use ethers::prelude::*;
use ethers::providers::{Provider, Ws};
use std::sync::Arc;
use tracing::{info, warn};

// ---

/// Connects to Ethereum node via WebSocket and listens for pending transactions.
/// Logs each incoming transaction hash (as a smoke test).

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

fn short(addr: &ethers::types::Address) -> String {
    let hex = format!("{:?}", addr);
    format!("{}…{}", &hex[0..8], &hex[hex.len() - 4..])
}
