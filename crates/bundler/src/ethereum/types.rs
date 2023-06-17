use crate::ethereum::{errors::EthereumError, models::ValidationResult, EthClientHandler};

use async_trait::async_trait;
use bundler_types::user_operation::UserOperation;
use contracts::bindings::abi::entry_point::{IEntryPoint, IEntryPointErrors};

use ethers::{
    contract::ContractError,
    middleware::SignerMiddleware,
    providers::{Http, Middleware, Provider},
    signers::LocalWallet,
    types::{transaction::eip2718::TypedTransaction, Address, Bytes, U256},
};

use ethers::abi::AbiDecode;
use std::{str::FromStr, sync::Arc};

#[derive(Debug)]
pub struct EthClient {
    eth_provider: Arc<Provider<Http>>,
    pub entry_point: IEntryPoint<SignerMiddleware<Provider<Http>, LocalWallet>>,
}

#[derive(Debug)]
pub struct GasOverhead {
    fixed: u64,
    per_user_op: u64,
    per_user_op_word: u64,
    zero_byte: u64,
    non_zero_byte: u64,
    bundle_size: u64,
}

impl Default for GasOverhead {
    fn default() -> Self {
        Self {
            fixed: 21000,
            per_user_op: 18300,
            per_user_op_word: 4,
            zero_byte: 4,
            non_zero_byte: 16,
            bundle_size: 1,
        }
    }
}

impl EthClient {
    pub fn new(eth_rpc: &str, ep_addr: &str, signer: &str) -> Result<EthClient, EthereumError> {
        let eth_provider = Provider::<Http>::try_from(eth_rpc)
            .map_err(|e| EthereumError::ProviderError(e.to_string()))?;
        let ep_addr = Address::from_str(ep_addr)
            .map_err(|e| EthereumError::DecodeError(format!("ep_addr decode failed: {}", e)))?;

        let signer = hex::decode(signer).map_err(|e| {
            EthereumError::DecodeError(format!("failed to decode signer private key: {}", e))
        })?;

        let signer = LocalWallet::from_bytes(&signer).map_err(|e| {
            EthereumError::DecodeError(format!("failed to create signer key: {}", e))
        })?;

        Ok(EthClient {
            eth_provider: Arc::new(eth_provider.clone()),
            entry_point: IEntryPoint::new(
                ep_addr,
                Arc::new(SignerMiddleware::new(eth_provider, signer)),
            ),
        })
    }
}

#[async_trait]
impl EthClientHandler for EthClient {
    async fn estimate_gas(
        &self,
        from: Address,
        to: Address,
        call_data: Bytes,
    ) -> Result<U256, EthereumError> {
        let mut tx = TypedTransaction::default();
        tx.set_from(from).set_to(to).set_data(call_data);

        self.eth_provider
            .estimate_gas(&tx, None)
            .await
            .map_err(|e| EthereumError::ProviderError(e.to_string()))
    }

    async fn calc_pre_verification_gas(
        &self,
        user_ops: &UserOperation,
    ) -> Result<U256, EthereumError> {
        let gas_overhead = GasOverhead::default();
        let mut user_operation = user_ops.clone();
        // dummy value
        user_operation.pre_verification_gas = gas_overhead.fixed.into();
        // dummy signature
        user_operation.signature = Bytes::from_str("0x").unwrap();

        let packed_user_operation = user_operation.pack();
        let bytes_user_ops = packed_user_operation.to_vec();
        let zeros = bytes_user_ops.iter().filter(|i| **i == 0).count() as u64;
        let non_zero = bytes_user_ops.len() as u64 - zeros;
        let words = ((packed_user_operation.len() + 31) / 32) as u64;

        Ok((zeros * gas_overhead.zero_byte
            + non_zero * gas_overhead.non_zero_byte
            + gas_overhead.fixed / gas_overhead.bundle_size
            + gas_overhead.per_user_op
            + words * gas_overhead.per_user_op_word)
            .into())
    }

    async fn simulate_validation(&self, user_ops: UserOperation) -> Result<U256, EthereumError> {
        let validation_call = self.entry_point.simulate_validation(user_ops.into());
        let res = validation_call.call().await;
        if res.is_ok() {
            return Err(EthereumError::ValidateError(String::from(
                "simulate_validation must be reverted",
            )));
        }

        // safe: already check res is err above
        let revert_msg = match res.err().unwrap() {
            ContractError::Revert(msg) => msg,
            other => {
                return Err(EthereumError::ValidateError(format!(
                    "error is not reverted: {:?}",
                    other
                )))
            }
        };

        match IEntryPointErrors::decode(revert_msg.as_ref())
            .map_err(|e| EthereumError::DecodeError(e.to_string()))?
        {
            IEntryPointErrors::ValidationResult(v) => {
                let validation_result: ValidationResult = v.into();
                // check validation result is ok
                // uncomment below when needed

                // validation_result.validate()?;
                Ok(validation_result.return_info.pre_op_gas)
            }

            other => Err(EthereumError::DecodeError(format!(
                "expected validation error: {}",
                other
            ))),
        }
    }
}
