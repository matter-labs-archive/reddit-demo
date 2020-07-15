use crate::zksync::{Address, MintingTransaction};
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GrantedTokensRequest {
    pub user: Address,
    pub community_name: String,
    pub auth: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MintingSignatureRequest {
    pub user: Address,
    pub community_name: String,
    pub minting_tx: MintingTransaction,
    pub auth: String,
}
