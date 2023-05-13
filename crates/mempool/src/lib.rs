pub mod errors;

use crate::errors::MempoolError;
use async_trait::async_trait;
use contracts::ethereum::abi::entry_point::UserOperation;
use ethers::types::Address;

#[async_trait]
pub trait MempoolService: Send + Sync {
    async fn add(&self) -> Result<(), MempoolError>;
    async fn get(&self, ep_addr: Address, sender: Address) -> Option<Vec<UserOperation>>;
}

pub struct Mempool {}

#[allow(clippy::new_ret_no_self)]
impl Mempool {
    pub fn new() -> Result<Box<dyn MempoolService>, MempoolError> {
        Ok(Box::new(Mempool {}))
    }
}

#[async_trait]
impl MempoolService for Mempool {
    async fn add(&self) -> Result<(), MempoolError> {
        // TODO: implement me
        Ok(())
    }

    async fn get(&self, _ep_addr: Address, _sender: Address) -> Option<Vec<UserOperation>> {
        // TODO: implement me
        None
    }
}
