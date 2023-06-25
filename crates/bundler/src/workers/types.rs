use async_trait::async_trait;
use bundler_types::user_operation::UserOperation;

use ethers::types::{Address, U256};
use mempool::MempoolService;
use std::{
    collections::{HashMap, HashSet},
    ops::{Add, Sub},
    sync::{Arc, RwLock},
};

use crate::{
    ethereum::{errors::EthereumError, EthClientHandler},
    workers::BundleManager,
};

use super::{errors::WorkerError, ReputationHandler};

pub struct BundleWorker {
    mempool: Box<dyn MempoolService>,
    eth_client: Box<dyn EthClientHandler>,
    max_gas: u64,
    ep_addr: Address,
    reputation_checker: Box<dyn ReputationHandler>,
}

impl BundleWorker {
    pub fn new(
        mempool: Box<dyn MempoolService>,
        eth_client: Box<dyn EthClientHandler>,
        max_gas: u64,
        ep_addr: Address,
        reputation_checker: Box<dyn ReputationHandler>,
    ) -> Self {
        BundleWorker {
            mempool,
            eth_client,
            max_gas,
            ep_addr,
            reputation_checker,
        }
    }
}

#[async_trait]
impl BundleManager for BundleWorker {
    async fn create_bundle(&self) -> Result<Vec<UserOperation>, WorkerError> {
        let mut senders: HashSet<Address> = HashSet::new();
        let mut total_gas: u64 = 0;
        let mut paymaster_deposit: HashMap<Address, U256> = HashMap::new();
        let mut staked_entity_count: HashMap<Address, u16> = HashMap::new();

        let mut bundle: Vec<UserOperation> = Vec::new();

        while self.mempool.get_mempool_size().await > 0 {
            let pop_result = self.mempool.pop().await;

            if pop_result.is_none() {
                break;
            }

            let user_ops = pop_result.unwrap();
            let paymaster = user_ops.get_paymaster_addr();
            let factory = user_ops.get_factory_addr();

            // check paymaster & factory : staked count, reputation
            if let Some(paymaster) = paymaster {
                let reputation = self.reputation_checker.check_reputation(paymaster);
                match reputation {
                    None => {
                        return Err(WorkerError::ReputationError(
                            "Reputation does not exist".to_string(),
                        ))
                    }
                    Some(Reputation::THROTTLED) => {
                        // skip
                        if staked_entity_count[&paymaster] > 1 {
                            self.mempool.push(self.ep_addr, user_ops).await;
                            continue;
                        }
                    }
                    Some(Reputation::BANNED) => {
                        // remove
                        continue;
                    }
                    _ => {}
                }
            }

            if let Some(factory) = factory {
                let reputation = self.reputation_checker.check_reputation(factory);
                match reputation {
                    None => {
                        return Err(WorkerError::ReputationError(
                            "Reputation does not exist".to_string(),
                        ))
                    }
                    Some(Reputation::THROTTLED) => {
                        // skip
                        if staked_entity_count[&factory] > 1 {
                            self.mempool.push(self.ep_addr, user_ops).await;
                            continue;
                        }
                    }
                    Some(Reputation::BANNED) => {
                        // remove
                        continue;
                    }
                    _ => {}
                }
            }

            // check if sender already has its user operation in the bundle
            if senders.contains(&user_ops.sender) {
                self.mempool.push(self.ep_addr, user_ops).await;
                continue;
            }

            // 2nd validation
            let validation_result = self
                .eth_client
                .simulate_validation(user_ops.clone())
                .await
                .map_err(|e| WorkerError::SecondValidationError(e.to_string()))?;

            // check maximum gas
            let user_ops_gas: u64 = user_ops
                .call_gas_limit
                .add(validation_result.return_info.pre_op_gas)
                .as_u64();
            let new_total_gas = total_gas.add(user_ops_gas);

            if new_total_gas > self.max_gas {
                self.mempool.push(self.ep_addr, user_ops).await;
                break;
            }

            // check paymaster deposit
            if let Some(paymaster) = paymaster {
                if paymaster_deposit[&paymaster] == U256::from(0) {
                    let balance = self
                        .eth_client
                        .get_balance(paymaster)
                        .await
                        .map_err(|e| WorkerError::EthClientError(e.to_string()))?;
                    paymaster_deposit
                        .entry(paymaster)
                        .and_modify(|deposit| *deposit = balance);
                }
                if paymaster_deposit[&paymaster] < validation_result.return_info.prefund {
                    self.mempool.push(self.ep_addr, user_ops).await;
                    continue;
                }

                staked_entity_count.entry(paymaster).and_modify(|count| {
                    *count = count.add(1);
                });
                paymaster_deposit.entry(paymaster).and_modify(|deposit| {
                    *deposit = deposit.sub(validation_result.return_info.prefund);
                });
            }

            if let Some(factory) = factory {
                staked_entity_count.entry(factory).and_modify(|count| {
                    *count = count.add(1);
                });
            }

            senders.insert(user_ops.sender);
            bundle.push(user_ops);
            total_gas = new_total_gas;
        }

        Ok(bundle)
    }

    async fn send_bundle(
        &self,
        beneficiary: Address,
        bundle: Vec<UserOperation>,
    ) -> Result<(), WorkerError> {
        // send bundle
        let result = self
            .eth_client
            .handle_ops(bundle.clone(), beneficiary)
            .await;

        if result.is_ok() {
            return Ok(());
        }

        let mut returning_bundle: Vec<UserOperation> = bundle.clone();

        if let EthereumError::FailedOpError(failed_op_index, revert_msg) =
            result.as_ref().unwrap_err()
        {
            let failed_user_ops = &bundle[*failed_op_index as usize];

            if revert_msg.to_string().contains("AA3") {
                if let Some(addr) = failed_user_ops.get_paymaster_addr() {
                    self.reputation_checker.crashed_user_ops(addr);
                }
            } else if revert_msg.to_string().contains("AA2") {
                self.reputation_checker
                    .crashed_user_ops(failed_user_ops.sender);
            } else if revert_msg.to_string().contains("AA1") {
                if let Some(addr) = failed_user_ops.get_factory_addr() {
                    self.reputation_checker.crashed_user_ops(addr);
                }
            } else {
                // failed op
                returning_bundle.remove((*failed_op_index).try_into().unwrap());
            }
        }

        returning_bundle.iter().for_each(|op| {
            let _ = self.mempool.push(self.ep_addr, op.clone());
        });

        return result.map_err(|e| WorkerError::EthClientError(e.to_string()));
    }
}

#[derive(PartialEq)]
pub enum Reputation {
    BANNED,
    THROTTLED,
    OK,
}

#[derive(Clone)]
pub struct ReputationEntry {
    op_seen: u64,
    op_included: u64,
}

pub struct ReputationParams {
    op_seen_denominator: u64,
    banned_threshold: u64,
    throttled_threshold: u64,
}

pub struct ReputationChecker {
    address_info: Arc<RwLock<HashMap<Address, ReputationEntry>>>,
    white_list: Arc<RwLock<HashSet<Address>>>,
    black_list: Arc<RwLock<HashSet<Address>>>,
    reputation_params: ReputationParams,
}

impl ReputationChecker {
    pub fn new() -> ReputationChecker {
        ReputationChecker {
            address_info: Arc::new(RwLock::new(HashMap::new())),
            white_list: Arc::new(RwLock::new(HashSet::new())),
            black_list: Arc::new(RwLock::new(HashSet::new())),
            reputation_params: ReputationParams {
                op_seen_denominator: 100,
                banned_threshold: 10,
                throttled_threshold: 10,
            },
        }
    }
}

impl ReputationHandler for ReputationChecker {
    fn update_whitelist(&self, addr: Address, is_whitelisted: bool) {
        if is_whitelisted {
            self.white_list.write().unwrap().insert(addr);
        } else {
            self.white_list.write().unwrap().remove(&addr);
        }
    }

    fn update_blacklist(&self, addr: Address, is_blacklisted: bool) {
        if is_blacklisted {
            self.black_list.write().unwrap().insert(addr);
        } else {
            self.black_list.write().unwrap().remove(&addr);
        }
    }

    fn check_reputation(&self, addr: Address) -> Option<Reputation> {
        if self.address_info.read().unwrap().contains_key(&addr) {
            return None;
        }

        let entry = self.address_info.read().unwrap()[&addr].clone();
        let min_op_added = entry.op_seen / self.reputation_params.op_seen_denominator;

        if min_op_added <= entry.op_included + self.reputation_params.throttled_threshold {
            return Some(Reputation::OK);
        } else if min_op_added <= entry.op_included + self.reputation_params.banned_threshold {
            return Some(Reputation::THROTTLED);
        } else {
            return Some(Reputation::BANNED);
        }
    }

    fn crashed_user_ops(&self, addr: Address) {
        // Increase op_seen[addr] by 100
        self.address_info.write().unwrap().insert(
            addr,
            ReputationEntry {
                op_seen: 100,
                op_included: 0,
            },
        );
    }

    fn register_address(&self, addr: Address) {
        self.address_info.write().unwrap().insert(
            addr,
            ReputationEntry {
                op_seen: 0,
                op_included: 0,
            },
        );
    }
}
