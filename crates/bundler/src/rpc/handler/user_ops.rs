use crate::rpc::types::UserOps;
use actix_web::dev::Response;
use actix_web::{post, web, Error, Responder};

#[post("")]
pub async fn request(body: web::Json<UserOps>) -> Result<impl Responder, Error> {
    println!("{:?}", body);
    Ok(Response::ok())
}
