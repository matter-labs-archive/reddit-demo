use crate::responses::ErrorResponse;
use actix_web::HttpResponse;
use anyhow::Error;

/// Converts the `anyhow::Result` into `HttpResponse`.
/// Since not handled error is uncommon situation, we assume it to be an internal server error.
pub fn response_from_error(error: Error) -> HttpResponse {
    log::error!("Request failed with the following error: {}", error);
    // TODO: Should we really return the error text to the user?
    HttpResponse::InternalServerError().json(ErrorResponse::error(&error.to_string()))
}
