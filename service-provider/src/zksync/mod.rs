use crate::{
    database::Subscription,
    zksync::{rest_client::RestApiClient, rpc_client::RpcClient},
};
use anyhow::{anyhow, Result};
use chrono::{DateTime, Duration, NaiveDateTime, Utc};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use zksync_models::node::tx::{FranklinTx, PackedEthSignature, Transfer, TransferFrom};

mod rest_client;
mod rpc_client;

// Public re-exports and type declarations to not tie the rest application to the actual zkSync types.
pub use zksync_models::node::Address;

// TODO: This should not be a hard-coded constant.
/// Cost of the subscription.
pub const SUBSCRIPTION_COST: u64 = 100;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubscriptionTx {
    transfer_to_sub: TransferFrom,
    burn_tx: Transfer,
    burn_tx_eth_signature: PackedEthSignature,
}

/// Extension trait for `SubscriptionTx` type adding convenient getters for the required functionality.
pub trait SubscriptionTxExt {
    /// Returns the `DateTime` object showing when this transaction can be executed.
    fn valid_from(&self) -> DateTime<Utc>;

    /// Returns the `DateTime` object showing when this transaction will not be valid anymore.
    fn valid_until(&self) -> DateTime<Utc>;
}

impl SubscriptionTxExt for SubscriptionTx {
    fn valid_from(&self) -> DateTime<Utc> {
        let time = NaiveDateTime::from_timestamp(self.transfer_to_sub.valid_from as i64, 0);

        DateTime::from_utc(time, Utc)
    }

    fn valid_until(&self) -> DateTime<Utc> {
        let time = NaiveDateTime::from_timestamp(self.transfer_to_sub.valid_until as i64, 0);

        DateTime::from_utc(time, Utc)
    }
}

#[derive(Debug)]
pub struct ZksyncApp {
    client: Client,
    rest_api_client: RestApiClient,
    rpc_client: RpcClient,
    burn_account_address: Address,
}

impl ZksyncApp {
    pub fn new(
        rest_api_addr: impl Into<String>,
        json_rpc_addr: impl Into<String>,
        burn_account_address: Address,
    ) -> Self {
        Self {
            client: Client::new(),
            rest_api_client: RestApiClient::new(rest_api_addr),
            rpc_client: RpcClient::new(json_rpc_addr),
            burn_account_address,
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
            // We may want to run a parallel thread which will invoke this method periodically for every subscription.
            self.send_subscription_tx(new_sub_tx).await?;

            // TODO: Should we remove sent tx from the database?

            return Ok(true);
        }

        // User is not subscribed, and there is no actual tx to send for sub to be refreshed.
        Ok(false)
    }

    pub async fn get_subscription_period(
        &self,
        subscription: Subscription,
    ) -> Result<(Option<DateTime<Utc>>, Option<DateTime<Utc>>)> {
        // Subscription starts at the time of execution of the very first subscription tx.
        // Here we assume that txs are executed at ~same time as the became valid.
        let started_at = subscription
            .pre_signed_txs
            .iter()
            .map(SubscriptionTxExt::valid_from)
            .min();

        // To find the end of subscription period, we take the last subscription tx and
        // add the length of the subscription period (31 days).
        let expires_at = subscription
            .pre_signed_txs
            .iter()
            .map(SubscriptionTxExt::valid_from)
            .max()
            .map(|date| date + Duration::days(31));

        Ok((started_at, expires_at))
    }

    pub async fn check_subscription_tx(
        &self,
        subscription: &Subscription,
        subscription_tx: &SubscriptionTx,
    ) -> Result<()> {
        if subscription_tx.transfer_to_sub.to != subscription.subscription_wallet {
            return Err(anyhow!(
                "`TransferFrom` recipient doesn't match known subscription wallet (expected {}, got {})",
                subscription.subscription_wallet,
                subscription_tx.transfer_to_sub.to
            ));
        }

        if subscription_tx.burn_tx.from != subscription.subscription_wallet {
            return Err(anyhow!(
                "Burn transaction `from` initiator incorrect (expected {}, got {})",
                subscription.subscription_wallet,
                subscription_tx.burn_tx.from
            ));
        }

        if subscription_tx.burn_tx.to != self.burn_account_address {
            return Err(anyhow!(
                "Burn transaction `to` recipient incorrect (expected {}, got {})",
                self.burn_account_address,
                subscription_tx.burn_tx.to
            ));
        }

        if subscription_tx.transfer_to_sub.amount != SUBSCRIPTION_COST.into() {
            return Err(anyhow!(
                "`TransferFrom` amount is not equal to the subscription cost (expected {}, got {})",
                SUBSCRIPTION_COST,
                subscription_tx.transfer_to_sub.amount
            ));
        }

        if subscription_tx.burn_tx.amount != SUBSCRIPTION_COST.into() {
            return Err(anyhow!(
                "Burn tx amount is not equal to the subscription cost (expected {}, got {})",
                SUBSCRIPTION_COST,
                subscription_tx.burn_tx.amount
            ));
        }

        if subscription_tx.burn_tx.nonce != (subscription_tx.transfer_to_sub.to_nonce + 1) {
            let burn_tx_nonce = subscription_tx.burn_tx.nonce;
            let transfer_from_nonce = subscription_tx.transfer_to_sub.to_nonce;

            return Err(anyhow!(
                "Burn tx nonce is expected to equal (transfer_from nonce + 1), but actually transfer_from nonce: {}; burn tx nonce: {}",
                burn_tx_nonce,
                transfer_from_nonce,
            ));
        }

        Ok(())
    }

    pub async fn send_subscription_tx(&self, subscription_tx: &SubscriptionTx) -> Result<()> {
        let subscription_tx = subscription_tx.clone();

        let transfer_from = FranklinTx::TransferFrom(Box::new(subscription_tx.transfer_to_sub));
        let burn = FranklinTx::Transfer(Box::new(subscription_tx.burn_tx));

        let txs = vec![
            (transfer_from, None),
            (burn, Some(subscription_tx.burn_tx_eth_signature)),
        ];

        self.rpc_client.send_txs_batch(txs).await?;

        Ok(())
    }

    pub async fn last_subscription_tx(
        &self,
        subscription_address: Address,
    ) -> Result<Option<DateTime<Utc>>> {
        // Note: Here we must check not only for the executed txs, but for pending as well.
        // However, if the latest tx was executed and failed, it must not be taken into account.
        // We should the latest of either pending or successfully executed tx for a wallet.
        let tx_history = self
            .rest_api_client
            .get_transactions_history(subscription_address)
            .await?;

        // Logic of actions below:
        // 1. Filter out the `TransferFrom` transactions for the subscription wallet which have
        //    the subscription cost amount.
        // 2. Filter out the burn `Transfer` transactions which have the "burn" address recipient
        //    and the subscription cost amount.
        // 3. Sort the `TransferFrom` operations from newest to oldest.
        // 4. Find such a `TransferFrom` which has a corresponding "burn" `Transfer` with nonce
        //    next to the `TransferFrom` operation.
        // 5. Return its timestamp.
        // 6. If no `TransferFrom` was found, return `None`: we assume that subscription wallet
        //    is only used for subscription payments and will not have other transactions, thus
        //    (`TransferFrom` / "burn" `Transfer`) pair will always be in the list of latest txs
        //    if subscription was paid at least once.

        let mut transfer_from_txs: Vec<(TransferFrom, DateTime<Utc>)> = tx_history
            .iter()
            .filter_map(|tx| {
                if tx.success == Some(false) {
                    // Failed transactions aren't taken into account.
                    return None;
                }

                if tx.tx_id == "TransferFrom" {
                    let transfer_from: TransferFrom = serde_json::from_value(tx.tx.clone())
                        .unwrap_or_else(|err| {
                            panic!(
                                "zkSync provided incorrect transaction history: {:?}. Error: {}",
                                tx_history, err
                            )
                        });

                    if transfer_from.amount == SUBSCRIPTION_COST.into() {
                        Some((transfer_from, tx.created_at))
                    } else {
                        // Obviously not a subscription tx.
                        None
                    }
                } else {
                    None
                }
            })
            .collect();

        // Sort transfers by their timestamp from newest to the oldest.
        transfer_from_txs.sort_unstable_by(|tx_a, tx_b| tx_b.1.cmp(&tx_a.1));

        let burn_txs: Vec<Transfer> = tx_history
            .iter()
            .filter_map(|tx| {
                if tx.tx_id == "Transfer" {
                    let transfer: Transfer =
                        serde_json::from_value(tx.tx.clone()).unwrap_or_else(|err| {
                            panic!(
                                "zkSync provided incorrect transaction history: {:?}. Error: {}",
                                tx_history, err
                            )
                        });

                    // Only consider burn transactions which burn the subscription cost.
                    if transfer.to == self.burn_account_address
                        && transfer.amount == SUBSCRIPTION_COST.into()
                    {
                        Some(transfer)
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .collect();

        for (transfer_from, created_at) in transfer_from_txs {
            let transfer_from_nonce = transfer_from.to_nonce;
            let expected_burn_nonce = transfer_from_nonce + 1;

            // Find a corresponding burn tx (it must have the nonce next to the `TransferFrom` operation).
            for burn_tx in &burn_txs {
                if burn_tx.nonce == expected_burn_nonce {
                    return Ok(Some(created_at));
                }
            }
        }

        Ok(None)
    }

    /// Checks whether provided tx can be sent as the next subscription tx.
    fn is_next_sub_tx(&self, subscription_tx: &SubscriptionTx) -> bool {
        let current_time = Utc::now();

        subscription_tx.valid_from() <= current_time
            && subscription_tx.valid_until() >= current_time
    }

    /// Checks whether user subscription is expired. Currently the subscription duration is set to be
    /// exactly 31 days, thus we simply check that the last tx on the subscription wallet is not older
    /// than 31 days.
    fn check_subscription_status(&self, last_tx_timestamp: DateTime<Utc>) -> bool {
        let current_time = Utc::now();
        current_time <= last_tx_timestamp + Duration::days(31)
    }
}
