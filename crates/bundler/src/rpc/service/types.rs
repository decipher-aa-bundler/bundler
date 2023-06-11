use crate::ethereum::EthClientHandler;
use crate::rpc::models::{EstimateUserOpsGasResponse, UserOps};
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
    async fn estimate_user_ops_gas(
        &self,
        user_ops: &UserOps,
        ep_addr: &str,
    ) -> Result<EstimateUserOpsGasResponse> {
        let from = Address::from_str(&user_ops.sender)?;
        let to = Address::from_str(ep_addr)?;
        let call_data = Bytes::from_str(&user_ops.call_data)?;

        let call_gas_limit = self
            .eth_client
            .estimate_gas(from, to, call_data)
            .await?
            .to_string();

        let pre_verification_gas = self
            .eth_client
            .calc_pre_verification_gas(user_ops)
            .await?
            .to_string();

        Ok(EstimateUserOpsGasResponse {
            pre_verification_gas,
            verification_gas_limit: "".to_string(),
            call_gas_limit,
        })
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
