use crate::zksync::MintingSignature;
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub error_description: String,
}

impl ErrorResponse {
    pub fn error(message: &str) -> Self {
        Self {
            error_description: message.into(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GrantedTokensResponse {
    pub token: String,
    pub amount: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MintingSignatureResponse {
    pub signature: MintingSignature,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RelatedCommunitiesResponse {
    pub communities: Vec<String>,
}
