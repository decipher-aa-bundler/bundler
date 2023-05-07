use crate::rpc::models::UserOps;
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

    client
        .bundler_service
        .estimate_user_ops_gas(user_ops, &ep_addr)
        .await;

    Ok(Response::ok())
}
