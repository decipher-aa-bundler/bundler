use crate::rpc::service::user_ops as user_ops_svc;
use crate::rpc::types::UserOps;
use crate::BundlerClient;
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
    user_ops_svc::estimate_user_ops_gas(user_ops, &ep_addr, client.eth_client.as_ref())
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(Response::ok())
}
