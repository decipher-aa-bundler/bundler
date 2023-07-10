use async_trait::async_trait;
use bundler_types::user_operation::UserOperation;

use ethers::types::{Address, Bytes, TxHash, U256};
use mempool::MempoolService;
use std::{
    collections::{HashMap, HashSet},
    ops::{Add, Sub},
    str::FromStr,
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
    beneficiary: Address,
    reputation_checker: Box<dyn ReputationHandler>,
}

impl BundleWorker {
    pub fn new(
        mempool: Box<dyn MempoolService>,
        eth_client: Box<dyn EthClientHandler>,
        max_gas: u64,
        ep_addr: &str,
        beneficiary: &str,
        reputation_checker: Box<dyn ReputationHandler>,
    ) -> Result<BundleWorker, WorkerError> {
        Ok(BundleWorker {
            mempool,
            eth_client,
            max_gas,
            ep_addr: Address::from_str(ep_addr)
                .map_err(|e| WorkerError::DecodeError(e.to_string()))?,
            beneficiary: Address::from_str(beneficiary)
                .map_err(|e| WorkerError::DecodeError(e.to_string()))?,
            reputation_checker,
        })
    }
}

#[async_trait]
impl BundleManager for BundleWorker {
    async fn add_user_ops(
        &self,
        user_ops: UserOperation,
        ep_addr: Address,
    ) -> Result<Bytes, WorkerError> {
        self.eth_client
            .simulate_validation(user_ops.clone())
            .await
            .map_err(|e| WorkerError::EthClientError(e.to_string()))?;

        self.reputation_checker.register_address(user_ops.sender);
        if let Some(payamster) = user_ops.get_paymaster_addr() {
            self.reputation_checker.register_address(payamster);
        }
        if let Some(factory) = user_ops.get_factory_addr() {
            self.reputation_checker.register_address(factory);
        }

        return Ok(self.mempool.push(ep_addr, user_ops).await);
    }

    async fn attempt_bunlde(&self, force: bool) -> Result<TxHash, WorkerError> {
        if !force && self.mempool.get_mempool_size().await < 2 {
            //TODO 하드코딩 삭제
            return Err(WorkerError::AttmeptError(
                "mempool size is too small".to_string(),
            ));
        }
        let bundle = self.create_bundle().await?;
        if bundle.is_empty() {
            return Err(WorkerError::AttmeptError("bundle is empty".to_string()));
        }
        self.send_bundle(self.beneficiary, bundle).await
    }

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
        println!("created bundle : {:?}", bundle);
        Ok(bundle)
    }

    async fn send_bundle(
        &self,
        beneficiary: Address,
        bundle: Vec<UserOperation>,
    ) -> Result<TxHash, WorkerError> {
        // send bundle
        let result = self
            .eth_client
            .handle_ops(bundle.clone(), beneficiary)
            .await;

        if result.is_ok() {
            return Ok(result.unwrap());
        }
        println!("bundle failed with error : {:?}", result);
        println!("failed bundle : {:?}", bundle);

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

        for op in returning_bundle {
            self.mempool.push(self.ep_addr, op).await;
        }

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
    pub fn new(
        op_seen_denominator: u64,
        banned_threshold: u64,
        throttled_threshold: u64,
    ) -> ReputationChecker {
        ReputationChecker {
            address_info: Arc::new(RwLock::new(HashMap::new())),
            white_list: Arc::new(RwLock::new(HashSet::new())),
            black_list: Arc::new(RwLock::new(HashSet::new())),
            reputation_params: ReputationParams {
                op_seen_denominator,
                banned_threshold,
                throttled_threshold,
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
        if !self.address_info.read().unwrap().contains_key(&addr) {
            return None;
        }

        let entry = self.address_info.read().unwrap()[&addr].clone();
        let min_op_added = entry.op_seen / self.reputation_params.op_seen_denominator;

        if min_op_added <= entry.op_included + self.reputation_params.throttled_threshold {
            Some(Reputation::OK)
        } else if min_op_added <= entry.op_included + self.reputation_params.banned_threshold {
            Some(Reputation::THROTTLED)
        } else {
            Some(Reputation::BANNED)
        }
    }

    fn crashed_user_ops(&self, addr: Address) {
        // Initialize by (100, 0)
        self.address_info.write().unwrap().insert(
            addr,
            ReputationEntry {
                op_seen: 100,
                op_included: 0,
            },
        );
    }

    fn success_user_ops(&self, addr: Address) {
        if !self.address_info.read().unwrap().contains_key(&addr) {
            return;
        }
        let prev_reputation = self.address_info.read().unwrap()[&addr].clone();

        self.address_info.write().unwrap().insert(
            addr,
            ReputationEntry {
                op_seen: prev_reputation.op_seen,
                op_included: prev_reputation.op_included + 1,
            },
        );
    }

    fn success_bundle(&self, bundle: Vec<UserOperation>) {
        for user_ops in bundle {
            if let Some(addr) = user_ops.get_paymaster_addr() {
                self.success_user_ops(addr);
            }
            if let Some(addr) = user_ops.get_factory_addr() {
                self.success_user_ops(addr);
            }
            self.success_user_ops(user_ops.sender);
        }
    }

    fn register_address(&self, addr: Address) {
        let mut new_reputation = ReputationEntry {
            op_seen: 0,
            op_included: 0,
        };
        if self.address_info.read().unwrap().contains_key(&addr) {
            let prev_reputation = self.address_info.read().unwrap()[&addr].clone();

            new_reputation = ReputationEntry {
                op_seen: prev_reputation.op_seen + 1,
                op_included: prev_reputation.op_included,
            };
        }

        self.address_info
            .write()
            .unwrap()
            .insert(addr, new_reputation);
    }
}
