pub mod errors;
mod types;

use std::collections::BinaryHeap;
use std::sync::{Arc, RwLock};

use crate::types::UserOpsWithEpAddr;
use async_trait::async_trait;
use bundler_types::user_operation::UserOperation;
use ethers::types::{Address, Bytes, U256};
use moka::sync::Cache;
use std::time::Duration;

#[async_trait]
pub trait MempoolService: Send + Sync {
    async fn push(&self, ep_addr: Address, user_ops: UserOperation);
    async fn pop(&self) -> Option<UserOperation>;
    async fn get_by_hash(&self, hash: Bytes) -> Option<UserOperation>;
    async fn get_mempool_size(&self) -> usize;
}

#[derive(Clone)]
pub struct Mempool {
    chain_id: U256,
    queue: Arc<RwLock<BinaryHeap<UserOpsWithEpAddr>>>,
    db: Arc<RwLock<Cache<Bytes, UserOpsWithEpAddr>>>,
}

#[allow(clippy::new_ret_no_self)]
impl Mempool {
    pub fn new(chain_id: u32) -> Mempool {
        Mempool {
            chain_id: U256::from(chain_id),
            queue: Arc::new(RwLock::new(BinaryHeap::new())),
            db: Arc::new(RwLock::new(
                Cache::builder()
                    // cache lives for 3 hour after insert
                    .time_to_live(Duration::from_secs(3 * 60 * 60))
                    .build(),
            )),
        }
    }
}

#[async_trait]
impl MempoolService for Mempool {
    async fn push(&self, ep_addr: Address, user_ops: UserOperation) {
        let user_ops_with_ep_addr =
            UserOpsWithEpAddr::from_user_ops(&ep_addr, &user_ops, &self.chain_id);
        self.queue
            .write()
            .unwrap()
            .push(user_ops_with_ep_addr.clone());

        let hash = user_ops.hash(&ep_addr, &self.chain_id);
        self.db.write().unwrap().insert(hash, user_ops_with_ep_addr);
    }

    async fn pop(&self) -> Option<UserOperation> {
        let user_ops = self.queue.write().unwrap().pop();
        if let Some(user_ops) = &user_ops {
            self.db.write().unwrap().remove(&user_ops.hash);
        }

        user_ops.map(|u| u.user_ops)
    }

    async fn get_by_hash(&self, hash: Bytes) -> Option<UserOperation> {
        self.db.read().unwrap().get(&hash).map(|u| u.user_ops)
    }

    async fn get_mempool_size(&self) -> usize {
        self.queue.read().unwrap().len()
    }
}
