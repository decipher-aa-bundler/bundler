use crate::ethereum::errors::EthereumError;
use crate::ethereum::EthClientHandler;
use async_trait::async_trait;
use ethers::providers::{Http, Middleware, Provider};
use ethers::types::transaction::eip2718::TypedTransaction;
use ethers::types::{Address, Bytes, U256};

#[derive(Debug)]
pub struct EthClient {
    pub eth_provider: Provider<Http>,
}

#[allow(clippy::new_ret_no_self)]
impl EthClient {
    pub fn new() -> Result<Box<dyn EthClientHandler>, EthereumError> {
        Ok(Box::new(EthClient {
            // TODO: url 하드코딩 config로 빼기
            eth_provider: Provider::<Http>::try_from(
                "https://goerli.blockpi.network/v1/rpc/public",
            )
            .map_err(|e| EthereumError::ProviderError { msg: e.to_string() })?,
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
            .map_err(|e| EthereumError::ProviderError { msg: e.to_string() })
    }
}
