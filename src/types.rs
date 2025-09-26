// Shared types, config structs, helper enums
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    // ---
    pub eth_rpc_url: String,
    pub private_key: String,
}
