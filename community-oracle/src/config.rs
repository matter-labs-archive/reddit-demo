use crate::zksync::Address;
use serde_derive::Deserialize;
use std::env;
use std::path::PathBuf;

#[derive(Debug, Clone, Deserialize)]
pub struct AppConfig {
    pub genesis_account_id: u32,
    pub genesis_account_address: Address,
    pub genesis_account_private_key: String,
    pub genesis_account_eth_private_key: String,
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
            genesis_account_id: env::var("GENESIS_ACCOUNT_ID")
                .expect("GENESIS_ACCOUNT_ID")
                .parse()
                .unwrap(),
            genesis_account_address: env::var("GENESIS_ACCOUNT_ADDRESS")
                .expect("GENESIS_ACCOUNT_ADDRESS")
                .parse()
                .unwrap(),
            genesis_account_private_key: env::var("GENESIS_ACCOUNT_PRIVATE_KEY")
                .expect("GENESIS_ACCOUNT_PRIVATE_KEY"),
            genesis_account_eth_private_key: env::var("GENESIS_ACCOUNT_ETH_PRIVATE_KEY")
                .expect("GENESIS_ACCOUNT_ETH_PRIVATE_KEY"),
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
