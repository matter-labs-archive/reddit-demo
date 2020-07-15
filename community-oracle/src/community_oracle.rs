use crate::{
    requests::{GrantedTokensRequest, MintingSignatureRequest},
    responses::{ErrorResponse, GrantedTokensResponse, MintingSignatureResponse},
    zksync::MintingApi,
};
use actix_web::{web, HttpResponse, Responder, Scope};
use std::{collections::HashMap, sync::Arc};

const DEFAULT_TOKENS_AMOUNT: u64 = 100;

#[derive(Debug, Clone)]
pub struct CommunityInfo {
    token_id: u16,
    token_symbol: String,
}

impl CommunityInfo {
    pub fn new(token_id: u16, token_symbol: impl Into<String>) -> Self {
        Self {
            token_id,
            token_symbol: token_symbol.into(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct CommunityOracle {
    /// Mapping "community name" => "token symbol"
    known_communities: HashMap<String, CommunityInfo>,
    minter: Arc<MintingApi>,
    tokens_amount: u64,
}

impl CommunityOracle {
    pub fn new() -> Self {
        let known_communities = vec![("TestCommunity".to_string(), CommunityInfo::new(0, "ETH"))];

        CommunityOracle {
            known_communities: known_communities.into_iter().collect(),
            minter: Arc::new(MintingApi::new()),
            tokens_amount: DEFAULT_TOKENS_AMOUNT,
        }
    }

    pub async fn tokens_for_user(
        oracle: web::Data<Self>,
        request: web::Json<GrantedTokensRequest>,
    ) -> impl Responder {
        let request = request.into_inner();

        let community_info = match oracle.known_communities.get(&request.community_name) {
            Some(info) => info,
            None => {
                let error = ErrorResponse::error("Invalid community");
                return HttpResponse::BadRequest().json(error);
            }
        };

        let response = GrantedTokensResponse {
            token: community_info.token_symbol.clone(),
            amount: oracle.tokens_amount,
        };

        HttpResponse::Ok().json(response)
    }

    pub async fn sign_minting_tx(
        oracle: web::Data<Self>,
        request: web::Json<MintingSignatureRequest>,
    ) -> impl Responder {
        let request = request.into_inner();

        let community_info = match oracle.known_communities.get(&request.community_name) {
            Some(info) => info,
            None => {
                let error = ErrorResponse::error("Invalid community");
                return HttpResponse::BadRequest().json(error);
            }
        };

        if !oracle
            .minter
            .is_minting_transaction_correct(&request.minting_tx, &request.user)
        {
            let error = ErrorResponse::error("Incorrect minting tx");
            return HttpResponse::BadRequest().json(error);
        }

        let signature = oracle
            .minter
            .sign_minting_tx(request.minting_tx, &community_info.token_symbol);

        let response = MintingSignatureResponse { signature };

        HttpResponse::Ok().json(response)
    }

    pub fn into_web_scope(self) -> Scope {
        web::scope("api/v0.1/")
            .data(self)
            .service(web::resource("/granted_tokens").to(Self::tokens_for_user))
            .service(web::resource("/get_minting_signature").to(Self::sign_minting_tx))
    }
}
