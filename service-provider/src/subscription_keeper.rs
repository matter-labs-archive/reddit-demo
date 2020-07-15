//! Subscription keeper is a module responsible for keeping the user's subscriptions active.
//! It loads the subscription txs from the database and sends them to the zkSync network
//! according to the subscription schedule.

use crate::database::DatabaseAccess;
use anyhow::Result;

pub struct SubscriptionKeeper<DB: DatabaseAccess> {
    db: DB,
}

impl<DB: DatabaseAccess> SubscriptionKeeper<DB> {
    pub fn new(db: DB) -> Self {
        Self { db }
    }

    pub async fn run(self) -> Result<()> {
        Ok(())
    }
}
