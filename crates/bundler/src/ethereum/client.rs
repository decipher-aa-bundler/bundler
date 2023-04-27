use crate::ethereum::{EthClient, EthClientHandler};
use async_trait::async_trait;

#[async_trait]
impl EthClientHandler for EthClient {
    async fn calculate_gas(&self) {}
}
