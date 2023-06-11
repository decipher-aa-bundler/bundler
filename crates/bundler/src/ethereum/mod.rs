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
    async fn simulate_validation_gas(
        &self,
        _user_ops: &UserOps,
        _ep_addr: &str,
    ) -> Result<U256, EthereumError>;
}
