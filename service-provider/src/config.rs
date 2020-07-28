use crate::zksync::Address;
use serde_derive::Deserialize;
use std::env;
use std::path::PathBuf;

#[derive(Debug, Clone, Deserialize)]
pub struct AppConfig {
    pub app_bind_address: String,
    pub zksync_rest_api_address: String,
    pub zksync_json_rpc_address: String,
    pub community_oracle_address: String,
    pub burn_account_address: Address,
    // TODO: Database params
}

impl AppConfig {
    /// Loads the spec from the file given its path.
    pub fn load(env_config: bool, filepath: &PathBuf) -> Self {
        if env_config {
            Self::load_from_env()
        } else {
            load_json(filepath)
        }
    }

    /// Loads config from env
    fn load_from_env() -> Self {
        Self {
            app_bind_address: env::var("APP_BIND_ADDRESS").expect("APP_BIND_ADDRESS"),
            zksync_rest_api_address: env::var("ZKSYNC_REST_API_ADDRESS")
                .expect("ZKSYNC_REST_API_ADDRESS"),
            zksync_json_rpc_address: env::var("ZKSYNC_JSON_RPC_ADDRESS")
                .expect("ZKSYNC_JSON_RPC_ADDRESS"),
            community_oracle_address: env::var("COMMUNITY_ORACLE_ADDRESS")
                .expect("COMMUNITY_ORACLE_ADDRESS"),
            burn_account_address: env::var("BURN_ACCOUNT_ADDRESS")
                .expect("BURN_ACCOUNT_ADDRESS")
                .parse()
                .expect("Can't decode burn account address"),
        }
    }
}

fn load_json<T: serde::de::DeserializeOwned>(filepath: &PathBuf) -> T {
    let buffer = std::fs::read_to_string(filepath).expect("Failed to read the test spec file");
    serde_json::from_str(&buffer).expect(
        "Failed to parse config file. Ensure that you provided \
             the correct path for the type of test you're about to run",
    )
}
