use crate::zksync::{Address, SubscriptionTx};
use anyhow::Result;
use async_trait::async_trait;

pub use self::{
    memory_db::MemoryDb,
    types::{Community, Subscription},
};

pub mod memory_db;
pub mod types;

#[async_trait]
pub trait DatabaseAccess: Sized {
    type DatabaseInitParams;

    fn init(params: Self::DatabaseInitParams) -> Result<Self>;

    async fn declare_community(&self, community: Community) -> Result<()>;

    async fn get_community(&self, community_name: &str) -> Result<Option<Community>>;

    async fn add_subscription(&self, address: Address, subscription: Subscription) -> Result<()>;

    async fn add_subscription_txs(
        &self,
        address: Address,
        community: &str,
        txs: Vec<SubscriptionTx>,
    ) -> Result<()>;

    async fn get_user_subscriptions(&self, address: Address) -> Result<Vec<Subscription>>;

    async fn get_subscription(
        &self,
        address: Address,
        community: &str,
    ) -> Result<Option<Subscription>>;

    // async fn get_expired_subscriptions(&self)
}
