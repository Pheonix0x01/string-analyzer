use actix_web::{HttpResponse, http::StatusCode};
use serde::Serialize;

#[derive(Serialize)]
pub struct ErrorResponse {
    pub error: String,
}

pub fn success_response<T: Serialize>(data: T, status: StatusCode) -> HttpResponse {
    HttpResponse::build(status).json(data)
}

pub fn error_response(message: &str, status: StatusCode) -> HttpResponse {
    HttpResponse::build(status).json(ErrorResponse {
        error: message.to_string(),
    })
}