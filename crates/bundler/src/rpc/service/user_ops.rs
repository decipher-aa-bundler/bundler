use std::str::FromStr;
use ethers::types::Address;
use crate::ethereum::EthClientHandler;
use crate::rpc::errors::RpcError;
use crate::rpc::types::UserOps;

pub async fn estimate_user_ops_gas(
    _user_ops: UserOps,
    _ep_addr: &str,
    eth_client: &dyn EthClientHandler,
) -> Result<(), RpcError> {
    let user_ops = _user_ops.try_into()?;
    let ep_addr = Address::from_str(_ep_addr).unwrap_or("".parse().unwrap()); //TODO
    eth_client.calculate_gas(user_ops, ep_addr).await;
    Ok(())
}
