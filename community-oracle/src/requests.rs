use crate::zksync::{Address, MintingTransaction};
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GrantedTokensRequest {
    pub user: Address,
    pub community_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MintingSignatureRequest {
    pub user: Address,
    pub community_name: String,
    pub minting_tx: MintingTransaction,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RelatedCommunitiesRequest {
    pub user: Address,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RelatedCommunitiesResponse {
    pub communities: Vec<String>,
}
