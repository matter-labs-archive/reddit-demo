use crate::{
    config::AppConfig,
    database::{DatabaseAccess, MemoryDb},
    service_provider::ServiceProvider,
};
use actix_web::{App, HttpServer};
use std::path::PathBuf;
use structopt::StructOpt;

mod config;
mod database;
mod oracle;
mod requests;
mod responses;
mod service_provider;
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

#[derive(Debug, StructOpt)]
#[structopt(name = "service_provider", about = "A Reddit Service Provider.")]
pub struct CliArgs {
    /// Load config from env (rather than a config file)
    #[structopt(short, long)]
    pub env_config: bool,
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    const CONFIG_PATH: &str = "config.json";

    let opt = CliArgs::from_args();

    env_logger::init();

    let config = AppConfig::load(opt.env_config, &PathBuf::from(CONFIG_PATH));
    let memory_db = MemoryDb::init(()).unwrap();

    run_server(memory_db, config).await
}
