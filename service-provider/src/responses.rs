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
