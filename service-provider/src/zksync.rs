use anyhow::Result;
use chrono::{DateTime, Duration, Utc};
use reqwest::Client;
use zksync_models::node::tx::Transfer;

// Public re-exports and type declarations to not tie the rest application to the actual zkSync types.
pub use zksync_models::node::Address;

pub type SubscriptionTx = Transfer;

#[derive(Debug)]
pub struct ZksyncApp {
    client: Client,
    rest_api_addr: String,
    json_rpc_addr: String,
}

impl ZksyncApp {
    pub fn new(rest_api_addr: impl Into<String>, json_rpc_addr: impl Into<String>) -> Self {
        Self {
            client: Client::new(),
            rest_api_addr: rest_api_addr.into(),
            json_rpc_addr: json_rpc_addr.into(),
        }
    }

    pub async fn is_user_subscribed(&self, subscription_address: Address) -> Result<bool> {
        let last_subscription_tx_on = match self.last_subscription_tx(subscription_address).await? {
            Some(datetime) => datetime,
            None => return Ok(false),
        };

        let current_time = Utc::now();

        // TODO imprecise calculation.
        let subscribed = current_time <= last_subscription_tx_on + Duration::days(30);

        Ok(subscribed)
    }

    pub async fn last_subscription_tx(
        &self,
        _subscription_address: Address,
    ) -> Result<Option<DateTime<Utc>>> {
        // TODO: Stub
        Ok(Some(Utc::now()))
    }
}
