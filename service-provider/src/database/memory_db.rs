use super::DatabaseAccess;
use anyhow::Result;

pub struct MemoryDb {}

impl DatabaseAccess for MemoryDb {
    type DatabaseInitParams = ();

    fn init(_params: Self::DatabaseInitParams) -> Result<Self> {
        Ok(Self {})
    }
}
