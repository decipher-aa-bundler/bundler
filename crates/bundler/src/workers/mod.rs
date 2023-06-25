mod errors;
pub mod types;

use std::vec::Vec;

use async_trait::async_trait;
use bundler_types::user_operation::UserOperation;
use ethers::types::Address;

use crate::workers::errors::WorkerError;

use self::types::Reputation;

#[async_trait]
pub trait BundleManager: Send + Sync {
    async fn create_bundle(&self) -> Result<Vec<UserOperation>, WorkerError>;
    async fn send_bundle(
        &self,
        beneficiary: Address,
        bundle: Vec<UserOperation>,
    ) -> Result<(), WorkerError>;

    // async fn send_next_bundle(&self) -> Result<SendBundleResult, WorkerError>;
}

#[async_trait]
pub trait ReputationHandler: Send + Sync {
    fn update_whitelist(&self, addr: Address, is_whitelisted: bool);
    fn update_blacklist(&self, addr: Address, is_blacklisted: bool);
    fn check_reputation(&self, addr: Address) -> Option<Reputation>;
    fn crashed_user_ops(&self, addr: Address);
    fn register_address(&self, addr: Address);
}
