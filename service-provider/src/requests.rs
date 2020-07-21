use crate::{
    database::Community,
    zksync::{Address, SubscriptionTx},
};
use serde_derive::{Deserialize, Serialize};

pub use community_oracle::requests::{
    GrantedTokensRequest, MintingSignatureRequest, RelatedCommunitiesRequest,
};

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
pub struct SubscribeRequest {
    pub user: Address,
    pub community_name: String,
    pub subscription_wallet: Address,
    pub txs: Vec<SubscriptionTx>,
}
