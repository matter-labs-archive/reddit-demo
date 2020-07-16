use crate::{
    database::Community,
    zksync::{Address, SubscriptionTx},
};
use serde_derive::{Deserialize, Serialize};

pub use community_oracle::requests::{GrantedTokensRequest, MintingSignatureRequest};

#[derive(Debug, Serialize, Deserialize)]
pub struct DeclareCommunityRequest {
    #[serde(flatten)]
    pub community: Community,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubscriptionCheckRequest {
    pub user: Address,
    pub community_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SetSubscriptionDataRequest {
    pub user: Address,
    pub community_name: String,
    pub subscription_wallet: Address,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AddSubscriptionTxsRequest {
    pub user: Address,
    pub community_name: String,
    pub txs: Vec<SubscriptionTx>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RelatedCommunitiesRequest {
    pub user: Address,
}
