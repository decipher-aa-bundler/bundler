use crate::ethereum::EthClientHandler;

pub mod rpc;
pub mod ethereum;

#[derive(Debug)]
pub struct BundlerClient {
    pub eth_client: Box<dyn EthClientHandler>,
}

impl BundlerClient {
    pub fn new(eth_client: impl EthClientHandler) -> BundlerClient {
        BundlerClient {
            eth_client
        }
    }
}