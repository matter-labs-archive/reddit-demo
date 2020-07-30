//! Type definitions for the API requests of the Service Provider.
//! Note that some of types are re-exported from the `community-oracle` crate.
//! See the `community_oracle::requests` module for their definitions.

use crate::{
    database::Community,
    zksync::{Address, SubscriptionTx},
};
use serde_derive::{Deserialize, Serialize};

pub use community_oracle::requests::{
    GrantedTokensRequest, MintingSignatureRequest, RelatedCommunitiesRequest,
};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeclareCommunityRequest {
    #[serde(flatten)]
    pub community: Community,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubscriptionCheckRequest {
    pub user: Address,
    pub community_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubscribeRequest {
    pub user: Address,
    pub community_name: String,
    pub subscription_wallet: Address,
    pub txs: Vec<SubscriptionTx>,
}
