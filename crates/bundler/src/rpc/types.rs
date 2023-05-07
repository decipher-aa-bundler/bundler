use crate::ethereum::types::EthClient;
use crate::rpc::service::types::BundlerService;
use crate::rpc::service::BundlerServiceHandler;
use std::sync::Arc;

pub struct BundlerClient {
    pub bundler_service: Arc<dyn BundlerServiceHandler>,
}

impl BundlerClient {
    pub fn new() -> Result<BundlerClient, String> {
        Ok(BundlerClient {
            bundler_service: Arc::new(BundlerService {
                eth_client: EthClient::new().map_err(|e| e.to_string())?,
            }),
        })
    }
}
