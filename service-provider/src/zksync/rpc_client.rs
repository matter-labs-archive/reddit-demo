//! RPC client for the zkSync server.

// Built-in imports
// External uses
use anyhow::{anyhow, Result};
use jsonrpc_core::types::response::Output;
// Workspace uses
use zksync_models::node::tx::{FranklinTx, PackedEthSignature, TxHash};
// use server::api_server::rpc_server::AccountInfoResp;
// Local uses
use self::messages::JsonRpcRequest;

/// State of the ZKSync operation.
#[derive(Debug)]
pub struct OperationState {
    pub executed: bool,
    pub verified: bool,
}

/// `RpcClient` is capable of interacting with the ZKSync node via its
/// JSON RPC interface.
#[derive(Debug, Clone)]
pub struct RpcClient {
    rpc_addr: String,
    client: reqwest::Client,
}

impl RpcClient {
    /// Creates a new `RpcClient` object.
    pub fn new(rpc_addr: impl Into<String>) -> Self {
        Self {
            rpc_addr: rpc_addr.into(),
            client: reqwest::Client::new(),
        }
    }

    pub async fn send_txs_batch(
        &self,
        txs: Vec<(FranklinTx, Option<PackedEthSignature>)>,
    ) -> Result<Vec<TxHash>> {
        let msg = JsonRpcRequest::submit_txs_batch(txs);

        let ret = self.post(&msg).await?;
        let tx_hashes =
            serde_json::from_value(ret).expect("failed to parse `send_txs_batch` response");
        Ok(tx_hashes)
    }

    /// Performs a POST query to the JSON RPC endpoint,
    /// and decodes the response, returning the decoded `serde_json::Value`.
    /// `Ok` is returned only for successful calls, for any kind of error
    /// the `Err` variant is returned (including the failed RPC method
    /// execution response).
    async fn post(&self, message: impl serde::Serialize) -> Result<serde_json::Value> {
        let reply: Output = self.post_raw(message).await?;

        let ret = match reply {
            Output::Success(v) => v.result,
            Output::Failure(v) => return Err(anyhow!("RPC error: {}", v.error)),
        };

        Ok(ret)
    }

    /// Performs a POST query to the JSON RPC endpoint,
    /// and decodes the response, returning the decoded `serde_json::Value`.
    /// `Ok` is returned only for successful calls, for any kind of error
    /// the `Err` variant is returned (including the failed RPC method
    /// execution response).
    async fn post_raw(&self, message: impl serde::Serialize) -> Result<Output> {
        let res = self
            .client
            .post(&self.rpc_addr)
            .json(&message)
            .send()
            .await?;
        if res.status() != reqwest::StatusCode::OK {
            anyhow!(
                "Post query responded with a non-OK response: {}",
                res.status()
            );
        }
        let reply: Output = res.json().await.unwrap();

        Ok(reply)
    }
}

/// Structures representing the RPC request messages.
mod messages {
    use serde_derive::Serialize;
    use zksync_models::node::tx::{FranklinTx, PackedEthSignature, TxEthSignature};

    #[derive(Debug, Serialize)]
    pub struct JsonRpcRequest {
        pub id: String,
        pub method: String,
        pub jsonrpc: String,
        pub params: Vec<serde_json::Value>,
    }

    #[derive(Debug, Serialize)]
    #[serde(rename_all = "camelCase")]
    pub struct TxWithSignature {
        tx: FranklinTx,
        signature: Option<TxEthSignature>,
    }

    impl JsonRpcRequest {
        fn create(method: impl ToString, params: Vec<serde_json::Value>) -> Self {
            Self {
                id: "1".to_owned(),
                jsonrpc: "2.0".to_owned(),
                method: method.to_string(),
                params,
            }
        }

        pub fn submit_txs_batch(txs: Vec<(FranklinTx, Option<PackedEthSignature>)>) -> Self {
            let txs: Vec<_> = txs
                .into_iter()
                .map(|(tx, signature)| TxWithSignature {
                    tx,
                    signature: signature.map(TxEthSignature::EthereumSignature),
                })
                .collect();

            let mut params = Vec::new();
            params.push(serde_json::to_value(txs).expect("serialization fail"));
            Self::create("submit_txs_batch", params)
        }
    }
}
