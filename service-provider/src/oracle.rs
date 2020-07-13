#[derive(Debug, Clone)]
pub struct CommunityOracle {
    oracle_addr: String,
}

impl CommunityOracle {
    pub fn new(oracle_addr: impl Into<String>) -> Self {
        Self {
            oracle_addr: oracle_addr.into(),
        }
    }
}
