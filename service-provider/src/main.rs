use crate::{
    database::{DatabaseAccess, MemoryDb},
    service_provider::ServiceProvider,
    subscription_keeper::SubscriptionKeeper,
};
use actix_web::{App, HttpServer};

mod config;
mod database;
mod oracle;
mod requests;
mod responses;
mod service_provider;
mod subscription_keeper;
mod utils;
mod zksync;

async fn run_server(db: MemoryDb, bind_address: &str) -> std::io::Result<()> {
    let service_provider = ServiceProvider::new(db);

    HttpServer::new(move || {
        let provider = service_provider.clone();
        let app = provider.into_web_scope();
        App::new().service(app)
    })
    .bind(bind_address)?
    .run()
    .await
}

async fn run_sub_keeper(db: MemoryDb) -> anyhow::Result<()> {
    let subscription_keeper = SubscriptionKeeper::new(db);

    subscription_keeper.run().await
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    const BIND_ADDRES: &str = "127.0.0.1:8081";
    let memory_db = MemoryDb::init(()).unwrap();

    tokio::spawn(run_sub_keeper(memory_db.clone()));

    run_server(memory_db, BIND_ADDRES).await
}
