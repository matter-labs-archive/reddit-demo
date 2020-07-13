use crate::zksync::Address;
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
}
