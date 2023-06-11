use crate::ethereum::EthClientHandler;
use crate::rpc::models::UserOps;
use crate::rpc::service::BundlerServiceHandler;
use async_trait::async_trait;
use bundler_types::user_operation::UserOperation;
use ethers::providers::Middleware;
use ethers::types::{Address, Bytes};
use eyre::Result;
use mempool::MempoolService;
use std::str::FromStr;
use std::sync::Arc;

pub struct BundlerService {
    pub eth_client: Box<dyn EthClientHandler>,
    pub mempool: Arc<dyn MempoolService>,
}

#[derive(Debug)]
pub struct GasOverhead {
    fixed: u64,
    per_user_op: u64,
    per_user_op_word: u64,
    zero_byte: u64,
    non_zero_byte: u64,
    bundle_size: u64,
    sig_size: u64,
}

impl Default for GasOverhead {
    fn default() -> Self {
        Self {
            fixed: 21000,
            per_user_op: 18300,
            per_user_op_word: 4,
            zero_byte: 4,
            non_zero_byte: 16,
            bundle_size: 1,
            sig_size: 65,
        }
    }
}

#[async_trait]
impl BundlerServiceHandler for BundlerService {
    async fn estimate_user_ops_gas(&self, user_ops: &UserOps, ep_addr: &str) -> Result<String> {
        let from = Address::from_str(&user_ops.sender)?;
        let to = Address::from_str(ep_addr)?;
        let call_data = Bytes::from_str(&user_ops.call_data)?;

        Ok(self
            .eth_client
            .estimate_gas(from, to, call_data)
            .await?
            .to_string())
    }

    async fn send_user_operation(&self, user_ops: &UserOps, ep_addr: &str) -> Result<String> {
        let ep_addr = Address::from_str(ep_addr)?;
        let sender = Address::from_str(&user_ops.sender)?;
        let user_ops: UserOperation = user_ops.try_into()?;

        self.mempool.add(ep_addr, user_ops.clone()).await?;

        let _pending_ops = self
            .mempool
            .get_op(&ep_addr, &sender, &user_ops.nonce)
            .await;
        Ok("".into())
    }

    async fn calc_pre_verification_gas(&self, user_ops: &UserOps) -> Result<String> {
        let user_operation: UserOperation = user_ops.try_into()?;
        let packed_user_operation = user_operation.pack();
        let bytes_user_ops = packed_user_operation.to_vec();
        let zeros = bytes_user_ops.iter().map(|i| *i == 0).count() as u64;
        let non_zero = bytes_user_ops.len() as u64 - zeros;
        let words = ((packed_user_operation.len() + 31) / 32) as u64;

        let gas_overhead = GasOverhead::default();

        Ok((zeros * gas_overhead.zero_byte
            + non_zero * gas_overhead.non_zero_byte
            + GAS_FIXED / gas_overhead.fixed
            + words * gas_overhead.per_user_op_word)
            .to_string())
    }
}
