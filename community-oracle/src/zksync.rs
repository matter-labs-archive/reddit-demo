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

use zksync_models::node::tx::Transfer;
use zksync_testkit::zksync_account::ZksyncAccount;

// Public re-exports and type declarations to not tie the rest application to the actual zkSync types.
pub use ::zksync_models::node::Address;
pub type MintingTransaction = Transfer;

#[derive(Debug)]
pub struct MintingApi {
    mint_account: ZksyncAccount,
}

impl MintingApi {
    pub fn new() -> Self {
        let mint_account = ZksyncAccount::rand();

        // TODO: We have to set the actual account ID to this mint account.
        mint_account.set_account_id(Some(1));

        Self { mint_account }
    }

    pub fn is_minting_transaction_correct(
        &self,
        tx: &MintingTransaction,
        address: &Address,
    ) -> bool {
        tx.account_id == self.mint_account.get_account_id().unwrap()
            && tx.from == self.mint_account.address
            && tx.to == *address
    }
}
