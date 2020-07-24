use actix_web::{App, HttpServer};
use community_oracle::{community_oracle::CommunityOracle, config::AppConfig};
use std::path::PathBuf;
use structopt::StructOpt;

async fn run_server(config: AppConfig, bind_address: &str) -> std::io::Result<()> {
    let community_oracle = CommunityOracle::new(config);

    HttpServer::new(move || {
        let oracle = community_oracle.clone();
        let app = oracle.into_web_scope();
        App::new().service(app)
    })
    .bind(bind_address)?
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
    const BIND_ADDRES: &str = "0.0.0.0:4040";
    const CONFIG_PATH: &str = "config.json";

    let opt = CliArgs::from_args();

    env_logger::init();

    let config = AppConfig::load(opt.env_config, &PathBuf::from(CONFIG_PATH));

    run_server(config, BIND_ADDRES).await
}
