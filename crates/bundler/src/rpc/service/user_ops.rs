use crate::rpc::errors::RpcError;
use crate::rpc::models::UserOps;
use crate::rpc::service::BundlerServiceHandler;

pub async fn estimate_user_ops_gas(
    _user_ops: UserOps,
    _ep_addr: &str,
    bundler_service: &Box<dyn BundlerServiceHandler>,
) -> Result<(), RpcError> {
    bundler_service.estimate_user_ops_gas().await;
    Ok(())
}
