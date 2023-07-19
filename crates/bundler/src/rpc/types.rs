use crate::ethereum::types::EthClient;
use crate::rpc::service::types::BundlerService;
use crate::rpc::service::BundlerServiceHandler;

use crate::config::Config;
use crate::workers::types::{BundleWorker, ReputationChecker};
use mempool::MempoolService;

pub struct BundlerClient {
    pub bundler_service: Box<dyn BundlerServiceHandler>,
}

impl BundlerClient {
    pub fn new(config: &Config, mempool: Box<dyn MempoolService>) -> Result<BundlerClient, String> {
        Ok(BundlerClient {
            bundler_service: Box::new(BundlerService {
                eth_client: Box::new(
                    EthClient::new(
                        &config.eth_rpc,
                        &config.ep_addr,
                        &config.signer,
                        config.chain_id,
                    )
                    .map_err(|e| e.to_string())?,
                ),
                bundle_manager: Box::new(
                    BundleWorker::new(
                        mempool,
                        Box::new(
                            EthClient::new(
                                &config.eth_rpc,
                                &config.ep_addr,
                                &config.signer,
                                config.chain_id,
                            )
                            .map_err(|e| e.to_string())?,
                        ),
                        100000000,
                        &config.ep_addr,
                        &config.beneficiary,
                        Box::new(ReputationChecker::new(100, 10, 10)),
                    )
                    .map_err(|e| e.to_string())?,
                ),
            }),
        })
    }
}
