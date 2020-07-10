use anyhow::Result;

pub use self::memory_db::MemoryDb;

pub mod memory_db;

pub trait DatabaseAccess: Sized {
    type DatabaseInitParams;

    fn init(params: Self::DatabaseInitParams) -> Result<Self>;
}
