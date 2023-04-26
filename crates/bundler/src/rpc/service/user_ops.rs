use actix_web::{Error, post, Responder};
use actix_web::dev::Response;

#[post("")]
pub async fn handle() -> Result<impl Responder, Error>{
    Ok(Response::ok())
}