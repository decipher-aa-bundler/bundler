use crate::rpc::errors::RpcError;
use crate::rpc::types::UserOps;

pub async fn estimate_user_ops_gas(_user_ops: UserOps, _ep_addr: &str) -> Result<(), RpcError> {
    Ok(())
}
