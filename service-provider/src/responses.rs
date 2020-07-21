use chrono::{DateTime, Utc};
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

#[derive(Debug, Serialize, Deserialize)]
pub struct SubscriptionCheckResponse {
    pub subscribed: bool,
    pub started_at: Option<DateTime<Utc>>,
    pub expires_at: Option<DateTime<Utc>>,
}
