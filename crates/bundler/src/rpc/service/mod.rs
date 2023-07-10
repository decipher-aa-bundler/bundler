pub mod types;

use crate::rpc::models::{EstimateUserOpsGasResponse, UserOps};
use async_trait::async_trait;
use ethers::types::TxHash;
use eyre::Result;

#[async_trait]
pub trait BundlerServiceHandler: Send + Sync {
    async fn estimate_user_ops_gas(
        &self,
        user_ops: &UserOps,
        ep_addr: &str,
    ) -> Result<EstimateUserOpsGasResponse>;
    async fn send_user_operation(&self, user_ops: &UserOps, ep_addr: &str) -> Result<TxHash>;
}
