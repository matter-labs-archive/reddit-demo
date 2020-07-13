use anyhow::Result;
use async_trait::async_trait;

pub use self::{memory_db::MemoryDb, types::Community};

pub mod memory_db;
pub mod types;

#[async_trait]
pub trait DatabaseAccess: Sized {
    type DatabaseInitParams;

    fn init(params: Self::DatabaseInitParams) -> Result<Self>;

    async fn declare_community(&self, community: Community) -> Result<()>;

    async fn get_community(&self, community_name: &str) -> Result<Option<Community>>;
}
