use crate::{database::Community, zksync::Address};
use serde_derive::{Deserialize, Serialize};

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
