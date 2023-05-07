use crate::ethereum::EthClientHandler;
use crate::rpc::models::UserOps;
use crate::rpc::service::BundlerServiceHandler;
use async_trait::async_trait;

pub struct BundlerService {
    pub eth_client: Box<dyn EthClientHandler>,
}

#[async_trait]
impl BundlerServiceHandler for BundlerService {
    async fn estimate_user_ops_gas(&self, _user_ops: UserOps, _ep_addr: &str) {
        self.eth_client.calculate_gas().await;
    }
}
