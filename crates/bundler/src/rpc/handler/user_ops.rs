use crate::rpc::types::{EthClient, UserOps};
use actix_web::dev::Response;
use actix_web::{post, web, Error, Responder};
use crate::BundlerClient;
use crate::rpc::service::user_ops as user_ops_svc;

#[post("/{ep_addr}")]
pub async fn estimate_user_ops_gas(body: web::Json<UserOps>, path: web::Path<String>, client: &mut web::Data<BundlerClient>) -> Result<impl Responder, Error> {
    let user_ops = body.into_inner();
    let ep_addr = path.into_inner();
    user_ops_svc::estimate_user_ops_gas(user_ops, &ep_addr, client);

    Ok(Response::ok())
}
