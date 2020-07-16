use serde_derive::Deserialize;
use std::path::PathBuf;

#[derive(Debug, Clone, Deserialize)]
pub struct AppConfig {
    pub app_bind_address: String,
    pub zksync_rest_api_address: String,
    pub zksync_json_rpc_address: String,
    pub community_oracle_address: String,
    // TODO: Database params
}

impl AppConfig {
    /// Loads the spec from the file given its path.
    pub fn load(filepath: &PathBuf) -> Self {
        load_json(filepath)
    }
}

fn load_json<T: serde::de::DeserializeOwned>(filepath: &PathBuf) -> T {
    let buffer = std::fs::read_to_string(filepath).expect("Failed to read the test spec file");
    serde_json::from_str(&buffer).expect(
        "Failed to parse config file. Ensure that you provided \
             the correct path for the type of test you're about to run",
    )
}
