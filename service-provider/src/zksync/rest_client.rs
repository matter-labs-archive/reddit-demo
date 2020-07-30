//! REST API client for the zkSync server.

// Built-in imports
// External uses
use anyhow::{anyhow, Result};
use chrono::NaiveDateTime;
use serde_derive::{Deserialize, Serialize};
// Workspace uses
// Local uses
use crate::zksync::Address;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct TransactionsHistoryItem {
    pub tx_id: String,
    pub hash: Option<String>,
    pub eth_block: Option<i64>,
    pub pq_id: Option<i64>,
    pub tx: serde_json::Value,
    pub success: Option<bool>,
    pub fail_reason: Option<String>,
    pub commited: bool,
    pub verified: bool,
    pub created_at: NaiveDateTime,
}

/// `RpcClient` is capable of interacting with the ZKSync node via its
/// JSON RPC interface.
#[derive(Debug, Clone)]
pub struct RestApiClient {
    api_addr: String,
    client: reqwest::Client,
}

impl RestApiClient {
    /// Creates a new `RestApiClient` object.
    pub fn new(api_addr: impl Into<String>) -> Self {
        Self {
            api_addr: api_addr.into(),
            client: reqwest::Client::new(),
        }
    }

    pub async fn get_transactions_history(
        &self,
        address: Address,
    ) -> Result<Vec<TransactionsHistoryItem>> {
        let address = format!("0x{}", hex::encode(address.as_ref()));

        let formatted_postfix = format!(
            "/account/{address}/history/{offset}/{limit}",
            address = address,
            offset = 0,
            limit = 40
        );
        let endpoint = self.endpoint(&formatted_postfix);

        let response = self.client.get(&endpoint).send().await?;

        let json_data: Vec<TransactionsHistoryItem> = match response.json().await {
            Ok(json) => json,
            Err(error) => {
                log::error!("zkSync server returned incorrect JSON: {}", error);
                log::error!("request path: {}", &endpoint);
                return Err(anyhow!("Unable to decode response from the zkSync server",));
            }
        };

        Ok(json_data)
    }

    fn endpoint(&self, postfix: &str) -> String {
        format!("{}/api/v0.1{}", self.api_addr, postfix)
    }
}
