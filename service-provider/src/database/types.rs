use crate::zksync::{Address, SubscriptionTx};
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Community {
    pub name: String,
    pub erc20_token_name: String,
    pub erc20_token_address: Address,
}

#[derive(Debug, Clone)]
pub struct Subscription {
    pub service_name: String,
    pub subscription_wallet: Address,
    pub pre_signed_txs: Vec<SubscriptionTx>,
}

impl Subscription {
    pub fn new(service_name: impl Into<String>, subscription_wallet: Address) -> Self {
        Self {
            service_name: service_name.into(),
            subscription_wallet,
            pre_signed_txs: Vec::new(),
        }
    }

    pub fn add_subscription_txs(&mut self, mut new_txs: Vec<SubscriptionTx>) {
        self.pre_signed_txs.append(&mut new_txs);
    }
}
