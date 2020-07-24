use crate::database::Subscription;
use anyhow::Result;
use chrono::{DateTime, Duration, Utc};
use reqwest::Client;
use zksync_models::node::tx::Transfer;

// Public re-exports and type declarations to not tie the rest application to the actual zkSync types.
pub use zksync_models::node::Address;

pub type SubscriptionTx = Transfer;

/// Extension trait for `SubscriptionTx` type adding convenient getters for the required functionality.
pub trait SubscriptionTxExt {
    /// Returns the `DateTime` object showing when this transaction can be executed.
    fn active_on(&self) -> DateTime<Utc>;

    /// Returns the `DateTime` object showing when this transaction will not be valid anymore.
    fn active_until(&self) -> DateTime<Utc>;
}

impl SubscriptionTxExt for SubscriptionTx {
    fn active_on(&self) -> DateTime<Utc> {
        // TODO: Stub
        Utc::now() - Duration::hours(1)
    }

    fn active_until(&self) -> DateTime<Utc> {
        // TODO: Stub
        Utc::now() + Duration::hours(11)
    }
}

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

    pub async fn is_user_subscribed(&self, subscription: Subscription) -> Result<bool> {
        let last_subscription_tx_on = match self
            .last_subscription_tx(subscription.subscription_wallet)
            .await?
        {
            Some(datetime) => datetime,
            None => return Ok(false),
        };

        // If subscription is outdated, but there are some not sent pre-signed txs yet, we must
        // say that user is subscribed and immediately send the subscription tx to zkSync.
        // User must be considered quasi-subscribed until tx is executed.
        // This is required to ensure that there are no short periods of "not subscribed" state between
        // subscription periods.
        let subscribed = self.check_subscription_status(last_subscription_tx_on);

        if subscribed {
            // User is subscribed, no further actions required.
            return Ok(true);
        }

        // User is not subscribed, we have to check if we can send the next subscription tx.
        let new_sub_tx = subscription
            .pre_signed_txs
            .iter()
            .find(|tx| self.is_next_sub_tx(tx));

        if let Some(new_sub_tx) = new_sub_tx {
            // TODO: This is a workaround which assumes that this endpoint is invoked somewhat consistently.
            // It may work incorrect if invoked rarely, thus it has to be replaced with a routine which schedules
            // sending txs right after the subscription has expired.
            self.send_subscription_tx(new_sub_tx).await?;

            // TODO: Remove sent tx from the database.

            return Ok(true);
        }

        // User is not subscribed, and there is no actual tx to send for sub to be refreshed.
        Ok(false)
    }

    pub async fn get_subscription_period(
        &self,
        _subscription: Subscription,
    ) -> Result<(DateTime<Utc>, DateTime<Utc>)> {
        // TODO: Stub

        let today = Utc::now();
        let end = today + Duration::days(31);

        Ok((today, end))
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
        // Note: Here we must check not only for the executed txs, but for pending as well.
        // However, if the latest tx was executed and failed, it must not be taken into account.
        // We should the latest of either pending or successfully executed tx for a wallet.
        Ok(Some(Utc::now()))
    }

    /// Checks whether provided tx can be sent as the next subscription tx.
    fn is_next_sub_tx(&self, subscription_tx: &SubscriptionTx) -> bool {
        let current_time = Utc::now();

        subscription_tx.active_on() <= current_time
            && subscription_tx.active_until() >= current_time
    }

    /// Checks whether user subscription is expired. Currently the subscription duration is set to be
    /// exactly 31 days, thus we simply check that the last tx on the subscription wallet is not older
    /// than 31 days.
    fn check_subscription_status(&self, last_tx_timestamp: DateTime<Utc>) -> bool {
        let current_time = Utc::now();
        current_time <= last_tx_timestamp + Duration::days(31)
    }
}
