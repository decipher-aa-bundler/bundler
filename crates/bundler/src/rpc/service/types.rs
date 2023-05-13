use crate::ethereum::EthClientHandler;
use crate::rpc::models::UserOps;
use crate::rpc::service::BundlerServiceHandler;
use async_trait::async_trait;
use ethers::types::{Address, Bytes};
use eyre::Result;
use std::str::FromStr;

pub struct BundlerService {
    pub eth_client: Box<dyn EthClientHandler>,
}

#[async_trait]
impl BundlerServiceHandler for BundlerService {
    async fn estimate_user_ops_gas(&self, user_ops: &UserOps, ep_addr: &str) -> Result<String> {
        let from = Address::from_str(&user_ops.sender)?;
        let to = Address::from_str(ep_addr)?;
        let call_data = Bytes::from_str(&user_ops.call_data)?;

        Ok(self
            .eth_client
            .estimate_gas(from, to, call_data)
            .await?
            .to_string())
    }
}
