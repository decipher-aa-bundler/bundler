pub mod errors;
pub mod utils;

use crate::errors::MempoolError;
use crate::utils::get_unique_key;
use async_trait::async_trait;
use bundler_types::user_operation::UserOperation;
use ethers::types::{Address, U256};
use log::info;
use moka::sync::Cache;
use rocksdb::{Options, DB};
use std::sync::Arc;
use std::time::Duration;

#[async_trait]
pub trait MempoolService: Send + Sync {
    async fn add(&self, ep_addr: Address, user_ops: UserOperation) -> Result<(), MempoolError>;
    async fn get_op(
        &self,
        ep_addr: &Address,
        sender: &Address,
        nonce: &U256,
    ) -> Option<UserOperation>;
}

pub struct Mempool {
    cache: Cache<Vec<u8>, UserOperation>,
    db: DB,
}

#[allow(clippy::new_ret_no_self)]
impl Mempool {
    pub fn new() -> Result<Arc<dyn MempoolService>, MempoolError> {
        let mut db_options = Options::default();
        db_options.create_if_missing(true);

        Ok(Arc::new(Mempool {
            cache: Cache::builder()
                // cache lives for 1 hour after insert
                .time_to_live(Duration::from_secs(60 * 60))
                .build(),
            // TODO: config bundler db directory
            db: DB::open(&db_options, "./db")
                .map_err(|e| MempoolError::DbInitError { msg: e.to_string() })?,
        }))
    }
}

#[async_trait]
impl MempoolService for Mempool {
    async fn add(&self, ep_addr: Address, user_ops: UserOperation) -> Result<(), MempoolError> {
        let key = get_unique_key(&ep_addr, &user_ops.sender, &user_ops.nonce);
        let value = user_ops
            .try_serialize()
            .map_err(|e| MempoolError::SerializeError { msg: e.to_string() })?;

        self.cache.insert(key.clone(), user_ops);
        self.db
            .put(key, value)
            .map_err(|e| MempoolError::InsertError { msg: e.to_string() })?;

        Ok(())
    }

    async fn get_op(
        &self,
        ep_addr: &Address,
        sender: &Address,
        nonce: &U256,
    ) -> Option<UserOperation> {
        let key = get_unique_key(ep_addr, sender, nonce);
        if self.cache.contains_key(&key) {
            info!("cache hit");
            self.cache.get(&key)
        } else {
            let data = self.db.get(&key).ok()??;
            serde_json::from_slice::<UserOperation>(&data).ok()
        }
    }
}
