use crate::ethereum::EthClientHandler;
use crate::rpc::models::UserOps;
use crate::rpc::service::BundlerServiceHandler;
use async_trait::async_trait;
use bundler_types::user_operation::UserOperation;
use ethers::types::{Address, Bytes};
use eyre::Result;
use mempool::MempoolService;
use std::str::FromStr;
use std::sync::Arc;

pub struct BundlerService {
    pub eth_client: Box<dyn EthClientHandler>,
    pub mempool: Arc<dyn MempoolService>,
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

    async fn send_user_operation(&self, user_ops: &UserOps, ep_addr: &str) -> Result<String> {
        let ep_addr = Address::from_str(ep_addr)?;
        let sender = Address::from_str(&user_ops.sender)?;
        let user_ops: UserOperation = user_ops.try_into()?;

        self.mempool.add(ep_addr, user_ops.clone()).await?;

        let _pending_ops = self
            .mempool
            .get_op(&ep_addr, &sender, &user_ops.nonce)
            .await;
        Ok("".into())
    }
}
