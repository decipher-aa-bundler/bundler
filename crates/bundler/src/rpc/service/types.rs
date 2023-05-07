use crate::ethereum::EthClientHandler;
use crate::rpc::service::BundlerServiceHandler;
use async_trait::async_trait;

pub struct BundlerService {
    pub eth_client: Box<dyn EthClientHandler>,
}

#[async_trait]
impl BundlerServiceHandler for BundlerService {
    async fn estimate_user_ops_gas(&self) {
        self.eth_client.calculate_gas().await;
    }
}
