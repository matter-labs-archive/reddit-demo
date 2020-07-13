use crate::{database::DatabaseAccess, requests::DeclareCommunityRequest};
use actix_web::{web, HttpResponse, Responder, Scope};
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct ServiceProvider<DB: DatabaseAccess> {
    db: Arc<DB>,
}

impl<DB: 'static + DatabaseAccess> ServiceProvider<DB> {
    pub fn new(db: DB) -> Self {
        Self { db: Arc::new(db) }
    }

    // TODO: Register community
    pub async fn declare_community(
        _provider: web::Data<Self>,
        _request: web::Json<DeclareCommunityRequest>,
    ) -> impl Responder {
        HttpResponse::Ok()
    }

    // TODO: Subscribe (manual)

    // TODO: Subscribe (pre-sign txs)

    // TODO: Unsubscribe (what should this method do? provide a "change pubkey" tx?) Alternative -- this is a fully client-side function, provider has nothing to do with it.

    // TODO: Request minting tx

    // TODO: Check subscription status

    pub fn into_web_scope(self) -> Scope {
        web::scope("api/v0.1/")
            .data(self)
            .service(web::resource("/declare_community").to(Self::declare_community))
    }
}
