use crate::responses::ErrorResponse;
use actix_web::HttpResponse;
use anyhow::Error;

pub fn response_from_error(error: Error) -> HttpResponse {
    HttpResponse::InternalServerError().json(ErrorResponse::error(&error.to_string()))
}
