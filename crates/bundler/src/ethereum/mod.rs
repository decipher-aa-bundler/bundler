pub mod errors;
pub mod models;
pub mod types;

use crate::ethereum::errors::EthereumError;

use bundler_types::user_operation::UserOperation;

use async_trait::async_trait;
use ethers::types::{Address, Bytes, U256};

use self::models::ValidationResult;

#[async_trait]
pub trait EthClientHandler: Send + Sync {
    async fn estimate_gas(
        &self,
        from: Address,
        to: Address,
        call_data: Bytes,
    ) -> Result<U256, EthereumError>;

    async fn calc_pre_verification_gas(
        &self,
        user_ops: &UserOperation,
    ) -> Result<U256, EthereumError>;

    async fn simulate_validation(
        &self,
        user_ops: UserOperation,
    ) -> Result<ValidationResult, EthereumError>;

    async fn get_balance(&self, address: Address) -> Result<U256, EthereumError>;

    async fn handle_ops(
        &self,
        ops: Vec<UserOperation>,
        beneficiary: Address,
    ) -> Option<EthereumError>;
}
