use crate::ethereum::EthClientHandler;
use crate::rpc::models::{EstimateUserOpsGasResponse, UserOps};
use crate::rpc::service::BundlerServiceHandler;
use crate::workers::BundleManager;

use async_trait::async_trait;
use bundler_types::user_operation::UserOperation;
use ethers::types::{Address, Bytes, TxHash};
use eyre::Result;
use log::{info, warn};
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
        let from = Address::from_str(ep_addr)?;
        let to = Address::from_str(&user_ops.sender)?;
        let call_data = Bytes::from_str(&user_ops.call_data)?;

        let user_operation = UserOperation::try_from(user_ops)?;

        let call_gas_limit = self
            .eth_client
            .estimate_gas(from, to, call_data)
            .await
            .map_err(|e| {
                warn!("Failed to estimate call gas limit : {:?}", e);
                e
            })?
            .to_string();

        let pre_verification_gas = self
            .eth_client
            .calc_pre_verification_gas(&user_operation)
            .await
            .map_err(|e| {
                warn!("Failed to calculate pre verification gas : {:?}", e);
                e
            })?
            .to_string();

        let verification_gas = self
            .eth_client
            .simulate_validation(user_operation)
            .await
            .map_err(|e| {
                warn!("Failed to get verification gas : {:?}", e);
                e
            })?
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

        let user_ops_hash = self
            .bundle_manager
            .add_user_ops(user_ops, ep_addr)
            .await
            .map_err(|e| {
                warn!("Failed to add user ops : {:?}", e);
                e
            })?;
        info!("Added user ops. UserOpHash : {:?}", user_ops_hash);

        let tx_hash = self
            .bundle_manager
            .attempt_bunlde(true)
            .await
            .map_err(|e| {
                warn!("Attempt Failed: {:?}", e);
                e
            })?;
        info!("Bundle submitted. TxHash : {:?}", tx_hash);

        Ok(tx_hash)
    }
}
