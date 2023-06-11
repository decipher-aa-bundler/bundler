use crate::ethereum::errors::EthereumError;
use crate::ethereum::models::{ValidateSimulation, ValidationResult};
use crate::ethereum::EthClientHandler;
use crate::rpc::errors::RpcError;
use crate::rpc::models::UserOps;
use async_trait::async_trait;
use bundler_types::user_operation::UserOperation;
use contracts::bindings::abi::entry_point;
use contracts::bindings::abi::entry_point::{IEntryPoint, IEntryPointErrors};
use ethers::abi::AbiDecode;
use ethers::middleware::SignerMiddleware;
use ethers::prelude::gas_oracle::MiddlewareError;
use ethers::prelude::ContractError;
use ethers::prelude::ContractError::AbiError;
use ethers::providers::{Http, Middleware, Provider};
use ethers::signers::{LocalWallet, Signer, Wallet};
use ethers::types::transaction::eip2718::TypedTransaction;
use ethers::types::{Address, Bytes, U256};
use eyre::eyre;
use std::str::FromStr;
use std::sync::Arc;
use std::time::{Instant, SystemTime, UNIX_EPOCH};

#[derive(Debug)]
pub struct EthClient {
    pub eth_provider: Arc<Provider<Http>>,
    pub entry_point: IEntryPoint<SignerMiddleware<Provider<Http>, LocalWallet>>,
}

#[allow(clippy::new_ret_no_self)]
impl EthClient {
    pub fn new(
        ep_addr: impl Into<Address>,
        signer: impl Into<LocalWallet>,
    ) -> Result<Box<dyn EthClientHandler>, EthereumError> {
        let eth_provider = Provider::<Http>::try_from(
            // TODO: url 하드코딩 config로 빼기
            "https://goerli.blockpi.network/v1/rpc/public",
        )
        .map_err(|e| EthereumError::ProviderError(eyre!(e)))?;

        Ok(Box::new(EthClient {
            eth_provider: Arc::new(eth_provider.clone()),
            entry_point: IEntryPoint::new(
                ep_addr,
                Arc::new(SignerMiddleware::new(eth_provider, signer)),
            ),
        }))
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
            .map_err(|e| EthereumError::ProviderError(eyre!(e)))
    }

    async fn simulate_validation_gas(
        &self,
        _user_ops: &UserOps,
        _ep_addr: &str,
    ) -> Result<U256, EthereumError> {
        let user_ops: UserOperation = _user_ops
            .try_into()
            .map_err(|e| EthereumError::ProviderError(eyre!(e)))?;
        let simulation_result = self
            .entry_point
            .simulate_validation(user_ops.into())
            .call()
            .await;

        match simulation_result {
            Err(ContractError::Revert(err)) => {
                let revert_msg: IEntryPointErrors =
                    AbiDecode::decode(err).map_err(|e| EthereumError::ProviderError(eyre!(e)))?;
                match revert_msg {
                    IEntryPointErrors::ValidationResult(error) => {
                        let validation_result: ValidationResult = error.into();
                        validation_result.validate()?;
                        //return preOpGas
                        Ok(validation_result.return_info.pre_op_gas)
                    }
                    IEntryPointErrors::ValidationResultWithAggregation(error) => {}
                    _ => EthereumError::ProviderError(eyre!("simulation must be reverted")),
                }
            }
            _ => EthereumError::ProviderError(eyre!("simulation must be reverted")),
        }
    }
}
