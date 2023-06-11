pub mod errors;
pub mod models;
pub mod types;

use crate::ethereum::errors::EthereumError;
use crate::rpc::models::UserOps;

use async_trait::async_trait;
use ethers::types::{Address, Bytes, U256};

#[async_trait]
pub trait EthClientHandler: Send + Sync {
    async fn estimate_gas(
        &self,
        from: Address,
        to: Address,
        call_data: Bytes,
    ) -> Result<U256, EthereumError>;

    async fn calc_pre_verification_gas(&self, user_ops: &UserOps) -> Result<U256, EthereumError>;
}
