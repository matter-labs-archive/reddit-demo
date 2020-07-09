use crate::{
    requests::{GrantedTokensRequest, MintingSignatureRequest},
    responses::{GrantedTokensResponse, MintingSignatureResponse},
    signer::Signer,
};
use actix_web::{web, HttpResponse, Responder, Scope};

const DEFAULT_TOKENS_AMOUNT: u64 = 100;

#[derive(Debug, Clone)]
pub struct CommunityOracle {
    signer: Signer,
    tokens_amount: u64,
}

impl CommunityOracle {
    pub fn new() -> Self {
        CommunityOracle {
            signer: Signer::new(),
            tokens_amount: DEFAULT_TOKENS_AMOUNT,
        }
    }

    pub async fn tokens_for_user(
        oracle: web::Data<Self>,
        request: web::Json<GrantedTokensRequest>,
    ) -> impl Responder {
        let _request = request.into_inner();

        let response = GrantedTokensResponse {
            token_type: "ETH".into(),
            token_amount: oracle.tokens_amount,
        };

        HttpResponse::Ok().json(response)
    }

    pub async fn sign_minting_tx(
        _oracle: web::Data<Self>,
        request: web::Json<MintingSignatureRequest>,
    ) -> impl Responder {
        let _request = request.into_inner();

        let response = MintingSignatureResponse {
            signature: "NO_SIGNATURE_YET".into(),
        };

        HttpResponse::Ok().json(response)
    }

    pub fn into_web_scope(self) -> Scope {
        web::scope("api/v0.1/")
            .data(self)
            .service(web::resource("/granted_tokens").to(Self::tokens_for_user))
            .service(web::resource("/get_minting_signature").to(Self::sign_minting_tx))
    }
}
