use anyhow::Result;
use chrono::{DateTime, Utc};

// Public re-exports and type declarations to not tie the rest application to the actual zkSync types.
pub use zksync_models::node::Address;

#[derive(Debug)]
pub struct ZksyncApp {
    rest_api_addr: String,
    json_rpc_addr: String,
}

impl ZksyncApp {
    pub fn new(rest_api_addr: impl Into<String>, json_rpc_addr: impl Into<String>) -> Self {
        Self {
            rest_api_addr: rest_api_addr.into(),
            json_rpc_addr: json_rpc_addr.into(),
        }
    }

    pub async fn last_subscription_tx(
        &self,
        _subscription_address: Address,
    ) -> Result<Option<DateTime<Utc>>> {
        // TODO: Stub
        Ok(Some(Utc::now()))
    }
}
