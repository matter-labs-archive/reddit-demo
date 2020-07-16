use crate::{
    config::AppConfig,
    database::{DatabaseAccess, MemoryDb},
    service_provider::ServiceProvider,
    subscription_keeper::SubscriptionKeeper,
};
use actix_web::{App, HttpServer};
use std::path::PathBuf;

mod config;
mod database;
mod oracle;
mod requests;
mod responses;
mod service_provider;
mod subscription_keeper;
mod utils;
mod zksync;

async fn run_server(db: MemoryDb, config: AppConfig) -> std::io::Result<()> {
    let service_provider = ServiceProvider::new(db, config.clone());

    HttpServer::new(move || {
        let provider = service_provider.clone();
        let app = provider.into_web_scope();
        App::new().service(app)
    })
    .bind(config.app_bind_address)?
    .run()
    .await
}

async fn run_sub_keeper(db: MemoryDb) -> anyhow::Result<()> {
    let subscription_keeper = SubscriptionKeeper::new(db);

    subscription_keeper.run().await
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    const CONFIG_PATH: &str = "config.json";

    let config = AppConfig::load(&PathBuf::from(CONFIG_PATH));
    let memory_db = MemoryDb::init(()).unwrap();

    tokio::spawn(run_sub_keeper(memory_db.clone()));

    run_server(memory_db, config).await
}
