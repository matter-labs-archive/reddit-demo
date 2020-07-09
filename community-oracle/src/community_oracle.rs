use actix_web::{web, HttpResponse, Responder, Scope};

#[derive(Debug, Clone)]
pub struct CommunityOracle;

impl CommunityOracle {
    pub fn new() -> Self {
        CommunityOracle
    }

    pub async fn sample_endpoint(
        _data: web::Data<Self>,
        info: web::Path<(String, u32)>,
    ) -> impl Responder {
        let (name, id) = info.into_inner();

        let response = serde_json::json!({
            "name": name,
            "id": id,
        });

        HttpResponse::Ok().json(response)
    }

    pub fn into_web_scope(self) -> Scope {
        web::scope("api/v0.1/")
            .data(self)
            .service(web::resource("/{name}/{id}").to(Self::sample_endpoint))
    }
}
