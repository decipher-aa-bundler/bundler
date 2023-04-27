pub mod errors;

use std::sync::Arc;
use ethers::providers::{Http, Provider};
use crate::ethereum::errors::EthereumError;


#[derive(Debug)]
pub struct EthClient {
    pub eth_provider: Arc<Provider<Http>>,
}

pub trait EthClientHandler {
    async fn calculate_gas(&self) {}
}

impl EthClient {
    pub fn new() -> Result<EthClient, EthereumError> {
        Ok(EthClient {
            // TODO: url 하드코딩 config로 빼기
            eth_provider: Arc::new(Provider::<Http>::try_from("https://goerli.blockpi.network/v1/rpc/public")
                .map_err(|e| EthereumError::ProviderError { msg: e.to_string() })?)
        })
    }
}

impl EthClientHandler for EthClient {
    async fn calculate_gas(&self) {}
}