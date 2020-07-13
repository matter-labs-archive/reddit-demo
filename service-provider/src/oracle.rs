use reqwest::Client;

#[derive(Debug, Clone)]
pub struct CommunityOracle {
    client: Client,
    oracle_addr: String,
}

impl CommunityOracle {
    pub fn new(oracle_addr: impl Into<String>) -> Self {
        Self {
            client: Client::new(),
            oracle_addr: oracle_addr.into(),
        }
    }
}
