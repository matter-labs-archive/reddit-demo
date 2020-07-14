use crate::{
    database::{Community, DatabaseAccess, Subscription},
    zksync::{Address, SubscriptionTx},
};
use anyhow::Result;
use async_trait::async_trait;
use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum MemoryDbError {
    #[error("User is not subscribed")]
    UserIsNotSubscribed,
}

#[derive(Debug, Clone)]
pub struct MemoryDb {
    communities: Arc<RwLock<HashMap<String, Community>>>,
    subscriptions: Arc<RwLock<HashMap<Address, Vec<Subscription>>>>,
}

impl MemoryDb {
    /// Finds a user subscription object given the user's address and community name, and
    /// applies the mutating function to this object.
    fn modify_subscription<F>(&self, address: Address, community: &str, f: F) -> Result<()>
    where
        F: FnOnce(&mut Subscription),
    {
        let mut existing_subscriptions = self.subscriptions.write().unwrap();

        let mut subscriptions = existing_subscriptions
            .get_mut(&address)
            .cloned()
            .unwrap_or_default();

        let subscription: &mut Subscription = subscriptions
            .iter_mut()
            .find(|sub| sub.service_name == community)
            .ok_or(MemoryDbError::UserIsNotSubscribed)?;

        f(subscription);

        Ok(())
    }
}

#[async_trait]
impl DatabaseAccess for MemoryDb {
    type DatabaseInitParams = ();

    fn init(_params: Self::DatabaseInitParams) -> Result<Self> {
        Ok(Self {
            communities: Default::default(),
            subscriptions: Default::default(),
        })
    }

    async fn declare_community(&self, community: Community) -> Result<()> {
        let mut communities = self.communities.write().unwrap();

        // TODO: Handle duplicates
        communities.insert(community.name.clone(), community);

        Ok(())
    }

    async fn get_community(&self, community_name: &str) -> Result<Option<Community>> {
        let communities = self.communities.read().unwrap();

        Ok(communities.get(community_name).cloned())
    }

    async fn add_subscription(&self, address: Address, subscription: Subscription) -> Result<()> {
        let mut existing_subscriptions = self.subscriptions.write().unwrap();

        // TODO: Handle duplicates
        let user_subscriptions = existing_subscriptions
            .entry(address)
            .or_insert_with(|| Vec::new());

        user_subscriptions.push(subscription);

        Ok(())
    }

    async fn add_subscription_txs(
        &self,
        address: Address,
        community: &str,
        txs: Vec<SubscriptionTx>,
    ) -> Result<()> {
        self.modify_subscription(address, community, |sub| sub.add_subscription_txs(txs))?;

        Ok(())
    }

    async fn get_user_subscriptions(&self, address: Address) -> Result<Vec<Subscription>> {
        let existing_subscriptions = self.subscriptions.read().unwrap();

        let user_subscriptions = existing_subscriptions
            .get(&address)
            .cloned()
            .unwrap_or_default();

        Ok(user_subscriptions)
    }

    async fn get_subscription(
        &self,
        address: Address,
        community: &str,
    ) -> Result<Option<Subscription>> {
        let subscriptions = self.get_user_subscriptions(address).await?;

        Ok(subscriptions
            .iter()
            .find(|sub| sub.service_name == community)
            .cloned())
    }
}
