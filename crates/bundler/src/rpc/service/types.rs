use crate::ethereum::EthClientHandler;
use crate::rpc::models::{EstimateUserOpsGasResponse, UserOps};
use crate::rpc::service::BundlerServiceHandler;
use crate::workers::BundleManager;

use async_trait::async_trait;
use bundler_types::user_operation::UserOperation;
use ethers::types::{Address, Bytes, TxHash};
use eyre::Result;
use std::str::FromStr;

pub struct BundlerService {
    pub eth_client: Box<dyn EthClientHandler>,
    pub bundle_manager: Box<dyn BundleManager>,
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

        let user_operation = UserOperation::try_from(user_ops)?;

        let call_gas_limit = self
            .eth_client
            .estimate_gas(from, to, call_data)
            .await?
            .to_string();

        let pre_verification_gas = self
            .eth_client
            .calc_pre_verification_gas(&user_operation)
            .await?
            .to_string();

        let verification_gas = self
            .eth_client
            .simulate_validation(user_operation)
            .await?
            .return_info
            .pre_op_gas
            .to_string();

        Ok(EstimateUserOpsGasResponse::new(
            call_gas_limit,
            pre_verification_gas,
            verification_gas,
        ))
    }

    async fn send_user_operation(&self, user_ops: &UserOps, ep_addr: &str) -> Result<TxHash> {
        let ep_addr = Address::from_str(ep_addr)?;
        let user_ops: UserOperation = user_ops.try_into()?;

        self.bundle_manager.add_user_ops(user_ops, ep_addr).await?;
        self.bundle_manager
            .attempt_bunlde(true)
            .await
            .map_err(|e| e.into())
    }
}
