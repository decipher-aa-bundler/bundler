pub mod errors;
pub mod types;

use std::vec::Vec;

use async_trait::async_trait;
use bundler_types::user_operation::UserOperation;
use ethers::types::{Address, Bytes, TxHash};

use crate::workers::errors::WorkerError;

use self::types::Reputation;

#[async_trait]
pub trait BundleManager: Send + Sync {
    async fn add_user_ops(
        &self,
        user_ops: UserOperation,
        ep_addr: Address,
    ) -> Result<Bytes, WorkerError>;
    async fn attempt_bunlde(&self, force: bool) -> Result<TxHash, WorkerError>;
    async fn create_bundle(&self) -> Result<Vec<UserOperation>, WorkerError>;
    async fn send_bundle(
        &self,
        beneficiary: Address,
        bundle: Vec<UserOperation>,
    ) -> Result<TxHash, WorkerError>;

    // async fn send_next_bundle(&self) -> Result<SendBundleResult, WorkerError>;
}

#[async_trait]
pub trait ReputationHandler: Send + Sync {
    fn update_whitelist(&self, addr: Address, is_whitelisted: bool);
    fn update_blacklist(&self, addr: Address, is_blacklisted: bool);
    fn check_reputation(&self, addr: Address) -> Option<Reputation>;
    fn crashed_user_ops(&self, addr: Address);
    fn success_user_ops(&self, addr: Address);
    fn success_bundle(&self, bundle: Vec<UserOperation>);
    fn register_address(&self, addr: Address);
}
