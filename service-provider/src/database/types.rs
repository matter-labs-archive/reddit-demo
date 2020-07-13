use crate::zksync::Address;
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Community {
    pub name: String,
    pub erc20_token_name: String,
    pub erc20_token_address: Address,
}
