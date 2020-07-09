use crate::community_oracle::CommunityOracle;
use actix_web::{App, HttpServer};

mod community_oracle;

async fn run_server(bind_address: &str) -> std::io::Result<()> {
    let community_oracle = CommunityOracle::new();

    HttpServer::new(move || {
        let oracle = community_oracle.clone();
        let app = oracle.into_web_scope();
        App::new().service(app)
    })
    .bind(bind_address)?
    .run()
    .await
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    const BIND_ADDRES: &str = "127.0.0.1:8080";

    run_server(BIND_ADDRES).await
}
