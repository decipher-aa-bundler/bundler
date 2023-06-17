use actix_web::body::BoxBody;
use actix_web::http::StatusCode;
use actix_web::{HttpResponse, ResponseError};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum RpcError {
    #[error("{0}")]
    Error(String),
}

impl ResponseError for RpcError {
    fn status_code(&self) -> StatusCode {
        StatusCode::BAD_REQUEST
    }

    fn error_response(&self) -> HttpResponse<BoxBody> {
        HttpResponse::with_body(StatusCode::BAD_REQUEST, BoxBody::new(self.to_string()))
    }
}
