use crate::rpc::models::UserOps;
use crate::rpc::service;
use crate::rpc::types::BundlerClient;

use actix_web::dev::Response;
use actix_web::{post, web, Error, Responder};

#[post("/{ep_addr}")]
pub async fn estimate_user_ops_gas(
    body: web::Json<UserOps>,
    path: web::Path<String>,
    client: web::Data<BundlerClient>,
) -> Result<impl Responder, Error> {
    let user_ops = body.into_inner();
    let ep_addr = path.into_inner();

    service::user_ops::estimate_user_ops_gas(user_ops, &ep_addr, &client.bundler_service)
        .await
        .unwrap();

    Ok(Response::ok())
}
