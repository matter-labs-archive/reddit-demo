//! Type definitions for the API responses of the Service Provider.
//! Note that some of types are re-exported from the `community-oracle` crate.
//! See the `community_oracle::responses` module for their definitions.

use chrono::{DateTime, Utc};
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
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

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubscriptionCheckResponse {
    pub subscribed: bool,
    pub started_at: Option<DateTime<Utc>>,
    pub expires_at: Option<DateTime<Utc>>,
}
