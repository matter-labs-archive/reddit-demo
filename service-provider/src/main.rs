use crate::service_provider::ServiceProvider;
use actix_web::{App, HttpServer};

mod requests;
mod responses;
mod service_provider;
mod zksync;

async fn run_server(bind_address: &str) -> std::io::Result<()> {
    let service_provider = ServiceProvider::new();

    HttpServer::new(move || {
        let provider = service_provider.clone();
        let app = provider.into_web_scope();
        App::new().service(app)
    })
    .bind(bind_address)?
    .run()
    .await
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    const BIND_ADDRES: &str = "127.0.0.1:8081";

    run_server(BIND_ADDRES).await
}
