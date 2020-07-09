use crate::zksync::{Address, MintingTransaction};
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GrantedTokensRequest {
    pub user_address: Address,
    pub community_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MintingSignatureRequest {
    pub user_address: Address,
    pub community_name: String,
    pub minting_tx: MintingTransaction,
}