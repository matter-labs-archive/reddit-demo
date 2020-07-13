use actix_web::{web, Scope};

#[derive(Debug, Clone)]
pub struct ServiceProvider {
    // TODO: Database Access
}

impl ServiceProvider {
    pub fn new() -> Self {
        Self {}
    }

    // TODO: Register community

    // TODO: Subscribe (manual)

    // TODO: Subscribe (pre-sign txs)

    // TODO: Unsubscribe (what should this method do? provide a "close account" tx?)

    // TODO: Request minting tx

    // TODO: Check subscription status

    pub fn into_web_scope(self) -> Scope {
        web::scope("api/v0.1/").data(self)
    }
}
