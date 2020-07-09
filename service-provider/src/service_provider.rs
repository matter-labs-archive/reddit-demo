use actix_web::{web, Scope};

#[derive(Debug, Clone)]
pub struct ServiceProvider {}

impl ServiceProvider {
    pub fn new() -> Self {
        Self {}
    }

    pub fn into_web_scope(self) -> Scope {
        web::scope("api/v0.1/").data(self)
    }
}
