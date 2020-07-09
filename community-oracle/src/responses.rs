use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub error_description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GrantedTokensResponse {
    pub token_type: String,
    pub token_amount: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MintingSignatureResponse {
    pub signature: String,
}
