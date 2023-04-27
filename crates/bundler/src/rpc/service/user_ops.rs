use crate::ethereum::EthClientHandler;
use crate::rpc::errors::RpcError;
use crate::rpc::types::UserOps;

pub async fn estimate_user_ops_gas(
    _user_ops: UserOps,
    _ep_addr: &str,
    eth_client: &dyn EthClientHandler,
) -> Result<(), RpcError> {
    eth_client.calculate_gas().await;
    Ok(())
}
