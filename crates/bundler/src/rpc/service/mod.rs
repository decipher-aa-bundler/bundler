pub mod types;

use crate::rpc::models::UserOps;
use async_trait::async_trait;
use eyre::Result;

#[async_trait]
pub trait BundlerServiceHandler: Send + Sync {
    async fn estimate_user_ops_gas(&self, user_ops: &UserOps, ep_addr: &str) -> Result<String>;
    async fn send_user_operation(&self, user_ops: &UserOps, ep_addr: &str) -> Result<String>;
    async fn calc_pre_verification_gas(&self, user_ops: &UserOps) -> Result<String>;
}
