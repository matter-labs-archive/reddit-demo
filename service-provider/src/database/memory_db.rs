use super::{Community, DatabaseAccess};
use anyhow::Result;
use async_trait::async_trait;
use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

#[derive(Debug, Clone)]
pub struct MemoryDb {
    communities: Arc<RwLock<HashMap<String, Community>>>,
}

#[async_trait]
impl DatabaseAccess for MemoryDb {
    type DatabaseInitParams = ();

    fn init(_params: Self::DatabaseInitParams) -> Result<Self> {
        Ok(Self {
            communities: Default::default(),
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
}
