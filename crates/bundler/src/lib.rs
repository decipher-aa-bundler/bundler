use crate::ethereum::EthClientHandler;

pub mod ethereum;
pub mod rpc;

pub struct BundlerClient {
    pub eth_client: Box<dyn EthClientHandler>,
}

impl BundlerClient {
    pub fn new(eth_client: impl EthClientHandler + 'static) -> BundlerClient {
        BundlerClient {
            eth_client: Box::new(eth_client),
        }
    }
}
