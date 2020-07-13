use crate::{
    database::DatabaseAccess,
    requests::{DeclareCommunityRequest, SubscriptionCheckRequest},
    responses::{ErrorResponse, SubscriptionCheckResponse},
    zksync::ZksyncApp,
};
use actix_web::{web, HttpResponse, Responder, Scope};
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct ServiceProvider<DB: DatabaseAccess> {
    db: Arc<DB>,
    zksync: Arc<ZksyncApp>,
}

impl<DB: 'static + DatabaseAccess> ServiceProvider<DB> {
    pub fn new(db: DB) -> Self {
        let zksync = ZksyncApp::new("incorrect_addr", "incorrect_addr");

        Self {
            db: Arc::new(db),
            zksync: Arc::new(zksync),
        }
    }

    pub async fn declare_community(
        provider: web::Data<Self>,
        request: web::Json<DeclareCommunityRequest>,
    ) -> impl Responder {
        let request = request.into_inner();

        match provider.db.declare_community(request.community).await {
            Ok(()) => HttpResponse::Ok().json(()),
            Err(err) => {
                HttpResponse::InternalServerError().json(ErrorResponse::error(&err.to_string()))
            }
        }
    }

    // TODO: Subscribe (manual)

    // TODO: Subscribe (pre-sign txs)

    // TODO: Unsubscribe (what should this method do? provide a "change pubkey" tx?) Alternative -- this is a fully client-side function, provider has nothing to do with it.

    // TODO: Check amount of tokens granted to user

    // TODO: Request minting tx

    pub async fn is_user_subscribed(
        provider: web::Data<Self>,
        request: web::Json<SubscriptionCheckRequest>,
    ) -> impl Responder {
        let request = request.into_inner();

        let sub = match provider
            .db
            .get_subscription(request.user, &request.community_name)
            .await
        {
            Ok(Some(community)) => community,
            Ok(None) => {
                return HttpResponse::Ok().json(SubscriptionCheckResponse { subscribed: false })
            }
            Err(error) => {
                return HttpResponse::InternalServerError()
                    .json(ErrorResponse::error(&error.to_string()))
            }
        };

        let subscribed = match provider
            .zksync
            .is_user_subscribed(sub.subscription_wallet)
            .await
        {
            Ok(subscribed) => subscribed,
            Err(error) => {
                return HttpResponse::InternalServerError()
                    .json(ErrorResponse::error(&error.to_string()))
            }
        };

        HttpResponse::Ok().json(SubscriptionCheckResponse { subscribed })
    }

    pub fn into_web_scope(self) -> Scope {
        web::scope("api/v0.1/")
            .data(self)
            .service(web::resource("/declare_community").to(Self::declare_community))
            .service(web::resource("/is_user_subscribed").to(Self::is_user_subscribed))
    }
}
