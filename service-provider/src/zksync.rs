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

        // TODO: If subscription is outdated, but there are some not sent pre-signed txs yet, we must
        // say that user is subscribed and immediately send the subscription tx to zkSync.
        // User must be considered quasi-subscribed until tx is executed.
        // This is required to ensure that there are no short periods of "not subscribed" state between
        // subscription periods.

        Ok(self.check_subscription_status(last_subscription_tx_on))
    }

    pub async fn check_subscription_tx(&self, _subscription_tx: &SubscriptionTx) -> Result<()> {
        // TODO: Stub
        Ok(())
    }

    pub async fn send_subscription_tx(&self, _subscription_tx: &SubscriptionTx) -> Result<()> {
        // TODO: Stub
        Ok(())
    }

    pub async fn last_subscription_tx(
        &self,
        _subscription_address: Address,
    ) -> Result<Option<DateTime<Utc>>> {
        // TODO: Stub
        Ok(Some(Utc::now()))
    }

    /// Checks whether user subscription is expired. Currently the subscription duration is set to be
    /// exactly 30 days, thus we simply check that the last tx on the subscription wallet is not older
    /// than 30 days.
    fn check_subscription_status(&self, last_tx_timestamp: DateTime<Utc>) -> bool {
        let current_time = Utc::now();
        current_time <= last_tx_timestamp + Duration::days(30)
    }
}
