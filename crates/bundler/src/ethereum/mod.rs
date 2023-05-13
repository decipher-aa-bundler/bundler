pub mod errors;
pub mod types;

use crate::ethereum::errors::EthereumError;
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
}
