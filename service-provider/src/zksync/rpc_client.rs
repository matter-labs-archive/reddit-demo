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

    /// Requests and returns a tuple `(executed, verified)` (as `OperationState`) for
    /// a transaction given its hash`.
    pub async fn tx_info(&self, tx_hash: TxHash) -> Result<OperationState> {
        let msg = JsonRpcRequest::tx_info(tx_hash);

        let ret = self.post(&msg).await?;
        let obj = ret.as_object().unwrap();
        let executed = obj["executed"].as_bool().unwrap();
        let verified = if executed {
            let block = obj["block"].as_object().unwrap();
            block["verified"].as_bool().unwrap()
        } else {
            false
        };
        Ok(OperationState { executed, verified })
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
    use zksync_models::node::{
        tx::{FranklinTx, PackedEthSignature, TxEthSignature, TxHash},
        Address,
    };

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

        pub fn ethop_info(serial_id: u64) -> Self {
            let mut params = Vec::new();
            params.push(serde_json::to_value(serial_id).expect("serialization fail"));
            Self::create("ethop_info", params)
        }

        pub fn tx_info(tx_hash: TxHash) -> Self {
            let mut params = Vec::new();
            params.push(serde_json::to_value(tx_hash).expect("serialization fail"));
            Self::create("tx_info", params)
        }

        pub fn get_tx_fee(tx_type: &str, address: Address, token_symbol: &str) -> Self {
            let mut params = Vec::new();
            params.push(serde_json::to_value(tx_type).expect("serialization fail"));
            params.push(serde_json::to_value(address).expect("serialization fail"));
            params.push(serde_json::to_value(token_symbol).expect("serialization fail"));
            Self::create("get_tx_fee", params)
        }
    }
}
