pub mod types;

use crate::rpc::models::UserOps;
use async_trait::async_trait;

#[async_trait]
pub trait BundlerServiceHandler: Send + Sync {
    async fn estimate_user_ops_gas(&self, _user_ops: UserOps, _ep_addr: &str);
}
