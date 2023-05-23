pub mod errors;
pub mod types;
pub mod entry_point_call;

use std::println;
use std::sync::Arc;
use crate::ethereum::errors::EthereumError;
use async_trait::async_trait;
use ethers::abi::AbiDecode;
use ethers::contract::ContractError;
use ethers::prelude::{Middleware, Provider};
use ethers::types::{Address, Bytes, U256};
use eyre::eyre;
use contracts::bindings::abi::entry_point;
use contracts::bindings::abi::entry_point::{IEntryPoint, IEntryPointErrors};
use crate::rpc::models::UserOps;

#[async_trait]
pub trait EthClientHandler: Send + Sync {
    async fn estimate_gas(
        &self,
        from: Address,
        to: Address,
        call_data: Bytes,
    ) -> Result<U256, EthereumError>;
    async fn simulate_validation_gas(&self, _user_ops: &UserOps, _ep_addr: &str) -> Result<U256, EthereumError> ;
    fn decode_revert_msg(&self, error: ContractError<M>) -> Result<IEntryPointErrors, EthereumError>;
    fn get_entry_point(&self, _ep_addr: &str) -> Result<IEntryPoint<M>, EthereumError>;
}
