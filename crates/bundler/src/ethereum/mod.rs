pub mod errors;
pub mod models;
pub mod types;

use crate::ethereum::errors::EthereumError;
use crate::rpc::models::UserOps;
use async_trait::async_trait;
use contracts::bindings::abi::entry_point;
use contracts::bindings::abi::entry_point::{IEntryPoint, IEntryPointErrors};
use ethers::abi::AbiDecode;
use ethers::contract::ContractError;
use ethers::prelude::{Middleware, Provider};
use ethers::types::{Address, Bytes, U256};
use eyre::eyre;
use std::println;
use std::sync::Arc;

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
