use crate::{
    database::Community,
    zksync::{Address, SubscriptionTx},
};
use serde_derive::{Deserialize, Serialize};

pub use community_oracle::requests::{GrantedTokensRequest, MintingSignatureRequest};

/// Type representing the authorization data for the user. In real application, it may be
/// the authorization token or something similar, currently it's just string that is not checked,
/// as it's just a demo.
pub type AuthData = String;

#[derive(Debug, Serialize, Deserialize)]
pub struct DeclareCommunityRequest {
    #[serde(flatten)]
    pub community: Community,
    pub auth: AuthData,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubscriptionCheckRequest {
    pub user: Address,
    pub community_name: String,
    pub auth: AuthData,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SetSubscriptionDataRequest {
    pub user: Address,
    pub community_name: String,
    pub subscription_wallet: Address,
    pub auth: AuthData,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AddSubscriptionTxsRequest {
    pub user: Address,
    pub community_name: String,
    pub txs: Vec<SubscriptionTx>,
    pub auth: AuthData,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RelatedCommunitiesRequest {
    pub user: Address,
    pub auth: AuthData,
}
