use std::str::FromStr;
use ethers::types::Address;
use bundler_types::user_operation::UserOperation;
use crate::BundlerClient;
use crate::rpc::types::{EthClient, UserOps, EthClientHandler};

pub async fn estimate_user_ops_gas(user_ops: UserOps, ep_addr: &str, client: &BundlerClient) -> Result<(), ()>{
    let user_ops: UserOperation = user_ops.try_into()?;
    let ep_address = Address::from_str(ep_addr)?;

    client.eth_client.calculate_gas()
}
