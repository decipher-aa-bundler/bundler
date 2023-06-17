use crate::ethereum::types::EthClient;
use crate::rpc::service::types::BundlerService;
use crate::rpc::service::BundlerServiceHandler;

use mempool::MempoolService;

pub struct BundlerClient {
    pub bundler_service: Box<dyn BundlerServiceHandler>,
}

impl BundlerClient {
    pub fn new(
        ep_addr: &str,
        signer: &str,
        mempool: Box<dyn MempoolService>,
    ) -> Result<BundlerClient, String> {
        Ok(BundlerClient {
            bundler_service: Box::new(BundlerService {
                eth_client: Box::new(
                    EthClient::new(ep_addr, signer.as_bytes()).map_err(|e| e.to_string())?,
                ),
                mempool,
            }),
        })
    }
}
