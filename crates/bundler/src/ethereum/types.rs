use crate::ethereum::errors::EthereumError;
use crate::ethereum::EthClientHandler;
use crate::rpc::models::UserOps;
use async_trait::async_trait;
use bundler_types::user_operation::UserOperation;
use contracts::bindings::abi::entry_point::IEntryPoint;
use ethers::middleware::SignerMiddleware;
use ethers::providers::{Http, Middleware, Provider};
use ethers::signers::LocalWallet;
use ethers::types::transaction::eip2718::TypedTransaction;
use ethers::types::{Address, Bytes, U256};
use std::str::FromStr;
use std::sync::Arc;

#[derive(Debug)]
pub struct EthClient {
    pub eth_provider: Arc<Provider<Http>>,
    pub entry_point: IEntryPoint<SignerMiddleware<Provider<Http>, LocalWallet>>,
}

#[allow(clippy::new_ret_no_self)]
impl EthClient {
    pub fn new(ep_addr: &str, signer: &str) -> Result<Box<dyn EthClientHandler>, EthereumError> {
        let eth_provider = Provider::<Http>::try_from(
            // TODO: url 하드코딩 config로 빼기
            "https://goerli.blockpi.network/v1/rpc/public",
        )
        .map_err(|e| EthereumError::ProviderError(e.to_string()))?;
        let ep_addr =
            Address::from_str(ep_addr).map_err(|e| EthereumError::DecodeError(e.to_string()))?;
        let signer =
            LocalWallet::from_str(signer).map_err(|e| EthereumError::DecodeError(e.to_string()))?;

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
            .map_err(|e| EthereumError::ProviderError(e.to_string()))
    }

    async fn simulate_validation_gas(
        &self,
        user_ops: &UserOps,
        _ep_addr: &str,
    ) -> Result<U256, EthereumError> {
        let user_ops = UserOperation::try_from(user_ops)
            .map_err(|e| EthereumError::DecodeError(e.to_string()))?;
        let _simulation_result = self
            .entry_point
            .simulate_validation(user_ops.into())
            .call()
            .await;

        todo!("simulation result parse")
    }
}
