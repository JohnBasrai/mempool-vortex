use ethers::prelude::*;
use ethers::providers::{Provider, Ws};
use std::sync::Arc;
use tracing::info;

// ---

/// Connects to Ethereum node via WebSocket and listens for pending transactions.
/// Logs each incoming transaction hash (as a smoke test).
pub async fn listen_to_mempool() -> anyhow::Result<()> {
    // ---

    let rpc_url = std::env::var("ETH_RPC_URL").map_err(|_| {
        anyhow::anyhow!(
            "Missing ETH_RPC_URL.\n\
             👉 Set it in a `.env` file (ETH_RPC_URL=wss://...) \
             or export it in your shell before running the app."
        )
    })?;

    // Connect to WebSocket
    let ws = Ws::connect(rpc_url).await?;
    let provider = Provider::new(ws);
    let provider = Arc::new(provider);

    // Subscribe to pending transactions
    let mut stream = provider.subscribe_pending_txs().await?;

    info!("📡 Listening to pending transactions...");

    // Log the first 10 hashes as a smoke test
    for _ in 0..10 {
        if let Some(tx_hash) = stream.next().await {
            info!("🔍 Pending tx: {:?}", tx_hash);
        }
    }

    Ok(())
}

// ---
