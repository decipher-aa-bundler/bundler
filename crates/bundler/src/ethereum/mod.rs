pub mod errors;

use crate::ethereum::errors::EthereumError;
use async_trait::async_trait;
use ethers::providers::{Http, Provider};
use std::sync::Arc;

#[derive(Debug)]
pub struct EthClient {
    pub eth_provider: Arc<Provider<Http>>,
}

#[async_trait]
pub trait EthClientHandler {
    async fn calculate_gas(&self);
}

impl EthClient {
    pub fn new() -> Result<EthClient, EthereumError> {
        Ok(EthClient {
            // TODO: url 하드코딩 config로 빼기
            eth_provider: Arc::new(
                Provider::<Http>::try_from("https://goerli.blockpi.network/v1/rpc/public")
                    .map_err(|e| EthereumError::ProviderError { msg: e.to_string() })?,
            ),
        })
    }
}

#[async_trait]
impl EthClientHandler for EthClient {
    async fn calculate_gas(&self) {}
}
