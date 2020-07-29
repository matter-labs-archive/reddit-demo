//! Wrapper around zkSync abstractions.
//!
//! Currently this module has two significant things that has to be kept in mind:
//!
//! 1. As a gateway, this module uses `testkit` package from the zkSync core. This package
//!   is only used because currently there is no official zkSync client library in Rust.
//!   Note that the main purpose of `testkit` crate is to provide the *testing* infrastructure
//!   rather than a full-fledged client experience.
//! 2. Until zkSync server implements minting/burning API, it is assumed that minting is done
//!   via transfers from a rich ("genesis") account.

use crate::config::AppConfig;
use serde_derive::{Deserialize, Serialize};
use zksync_models::node::tx::{TransferFrom, TxSignature};
use zksync_testkit::zksync_account::ZksyncAccount;

// Public re-exports and type declarations to not tie the rest application to the actual zkSync types.
pub use zksync_models::node::{Address, PrivateKey, H256};
pub type MintingTransaction = TransferFrom;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MintingSignature {
    zksync_signature: TxSignature,
}

#[derive(Debug)]
pub struct MintingApi {
    mint_account: ZksyncAccount,
}

impl MintingApi {
    pub fn new(config: AppConfig) -> Self {
        let private_key_bytes =
            hex::decode(config.genesis_account_private_key.trim_start_matches("0x"))
                .expect("Incorrect private key for genesis account");
        let zk_private_key = PrivateKey::read(&private_key_bytes[..])
            .expect("Incorrect private key for genesis account");
        let eth_private_key = H256::from_slice(&private_key_bytes);

        let mint_account = ZksyncAccount::new(
            zk_private_key,
            0,
            config.genesis_account_address,
            eth_private_key,
        );

        // Set the account ID (required to sign transactions)
        mint_account.set_account_id(Some(config.genesis_account_id));

        Self { mint_account }
    }

    pub fn is_minting_transaction_correct(
        &self,
        tx: &MintingTransaction,
        address: &Address,
    ) -> bool {
        tx.from == self.mint_account.address
            && tx.to == *address
            && tx.amount == crate::community_oracle::DEFAULT_TOKENS_AMOUNT.into()
    }

    pub fn sign_minting_tx(&self, tx: MintingTransaction) -> MintingSignature {
        let from_signature =
            TxSignature::sign_musig(&self.mint_account.private_key, &tx.get_bytes());

        MintingSignature {
            zksync_signature: from_signature,
        }
    }
}
