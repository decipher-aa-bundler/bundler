pub mod types;
pub mod user_ops;
use async_trait::async_trait;

#[async_trait]
pub trait BundlerServiceHandler: Send + Sync {
    async fn estimate_user_ops_gas(&self);
}
