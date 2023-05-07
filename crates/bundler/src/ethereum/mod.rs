pub mod errors;
pub mod types;

use async_trait::async_trait;

#[async_trait]
pub trait EthClientHandler: Send + Sync {
    async fn calculate_gas(&self);
}
