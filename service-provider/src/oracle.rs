use crate::{
    requests::{GrantedTokensRequest, MintingSignatureRequest, RelatedCommunitiesRequest},
    responses::ErrorResponse,
};
use actix_web::HttpResponse;
use anyhow::Result;
use reqwest::{Client, StatusCode};

#[derive(Debug, Clone)]
pub struct CommunityOracle {
    client: Client,
    oracle_addr: String,
}

impl CommunityOracle {
    pub fn new(oracle_addr: impl Into<String>) -> Self {
        Self {
            client: Client::new(),
            oracle_addr: oracle_addr.into(),
        }
    }

    pub async fn tokens_for_user(&self, request: GrantedTokensRequest) -> Result<HttpResponse> {
        let reqwest_response = self
            .client
            .post(&self.tokens_for_user_endpoint())
            .json(&request)
            .send()
            .await?;

        Ok(Self::convert_response(reqwest_response).await)
    }

    pub async fn sign_minting_tx(&self, request: MintingSignatureRequest) -> Result<HttpResponse> {
        let reqwest_response = self
            .client
            .post(&self.sign_minting_tx_endpoint())
            .json(&request)
            .send()
            .await?;

        Ok(Self::convert_response(reqwest_response).await)
    }

    pub async fn related_communities(
        &self,
        request: RelatedCommunitiesRequest,
    ) -> Result<HttpResponse> {
        let reqwest_response = self
            .client
            .post(&self.related_communities_endpoint())
            .json(&request)
            .send()
            .await?;

        Ok(Self::convert_response(reqwest_response).await)
    }

    fn tokens_for_user_endpoint(&self) -> String {
        format!("{}/api/v0.1/granted_tokens", &self.oracle_addr)
    }

    fn sign_minting_tx_endpoint(&self) -> String {
        format!("{}/api/v0.1/sign_minting_tx", &self.oracle_addr)
    }

    fn related_communities_endpoint(&self) -> String {
        format!("{}/api/v0.1/related_communities", &self.oracle_addr)
    }

    /// Transforms the `reqwest` response type into `actix_web::HttpResponse`.
    async fn convert_response(response: reqwest::Response) -> HttpResponse {
        let mut response_builder = match response.status() {
            StatusCode::OK => HttpResponse::Ok(),
            StatusCode::BAD_REQUEST => HttpResponse::BadRequest(),
            StatusCode::INTERNAL_SERVER_ERROR => HttpResponse::InternalServerError(),
            _ => {
                log::error!(
                    "Community oracle returned unexpected response: {:?}",
                    response
                );
                return HttpResponse::InternalServerError().json(ErrorResponse::error(
                    "Unexpected response from the Community Oracle",
                ));
            }
        };

        let json_data: serde_json::Value = match response.json().await {
            Ok(json) => json,
            Err(error) => {
                log::error!("Community oracle returned incorrect JSON: {}", error);
                return HttpResponse::InternalServerError().json(ErrorResponse::error(
                    "Unable to decode response from the Community Oracle",
                ));
            }
        };

        response_builder.json(json_data)
    }
}
