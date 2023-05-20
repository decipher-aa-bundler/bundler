use crate::rpc::errors::RpcError;
use crate::rpc::models::{EstimateUserOpsGasResponse, UserOps};
use crate::rpc::types::BundlerClient;

use actix_web::{post, web, HttpResponse, Responder};

#[post("/{ep_addr}")]
pub async fn estimate_user_ops_gas(
    body: web::Json<UserOps>,
    path: web::Path<String>,
    client: web::Data<BundlerClient>,
) -> Result<impl Responder, RpcError> {
    let user_ops = body.into_inner();
    let ep_addr = path.into_inner();

    let gas = client
        .bundler_service
        .estimate_user_ops_gas(&user_ops, &ep_addr)
        .await
        .map_err(RpcError::Error)?;

    let pre_verification_gas = client
        .bundler_service
        .calc_pre_verification_gas(&user_ops)
        .await
        .map_err(RpcError::Error)?;

    Ok(web::Json(EstimateUserOpsGasResponse::new(
        pre_verification_gas,
        gas,
        "".into(),
    )))
}

#[post("/{ep_addr}")]
pub async fn send_user_operation(
    body: web::Json<UserOps>,
    path: web::Path<String>,
    client: web::Data<BundlerClient>,
) -> Result<impl Responder, RpcError> {
    let user_ops = body.into_inner();
    let ep_addr = path.into_inner();

    client
        .bundler_service
        .send_user_operation(&user_ops, &ep_addr)
        .await
        .map_err(RpcError::Error)?;

    Ok(HttpResponse::Ok())
}
