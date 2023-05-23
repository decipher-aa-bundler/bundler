use crate::ethereum::EthClientHandler;
use crate::rpc::models::UserOps;
use crate::rpc::service::BundlerServiceHandler;
use async_trait::async_trait;
use bundler_types::user_operation::UserOperation;
use ethers::types::{Address, Bytes};
use eyre::Result;
use mempool::MempoolService;
use std::str::FromStr;
use std::sync::Arc;
use ethers::providers::Middleware;

const GAS_FIXED: u64 = 21000;
const GAS_PER_USER_OP: u64 = 18300;
const GAS_PER_USER_OP_WORD: u64 = 4;
const GAS_ZERO_BYTE: u64 = 4;
const GAS_NON_ZERO_BYTE: u64 = 16;
const GAS_BUNDLE_SIZE: u64 = 1;
const GAS_SIG_SIZE: u64 = 65;

pub struct BundlerService {
    pub eth_client: Box<dyn EthClientHandler>,
    pub mempool: Arc<dyn MempoolService>,
}

#[derive(Debug)]
pub struct GasOverhead {
    fixed: u32,
    per_user_op: u32,
    per_user_op_word: u32,
    zero_byte: u32,
    non_zero_byte: u32,
    bundle_size: u32,
    sig_size: u32,
}

impl GasOverhead {
    pub fn new(
        fixed: u32,
        per_user_op: u32,
        per_user_op_word: u32,
        zero_byte: u32,
        non_zero_byte: u32,
        bundle_size: u32,
        sig_size: u32,
    ) -> Self {
        Self {
            fixed,
            per_user_op,
            per_user_op_word,
            zero_byte,
            non_zero_byte,
            bundle_size,
            sig_size,
        }
    }
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

        Ok((zeros * GAS_ZERO_BYTE
            + non_zero * GAS_NON_ZERO_BYTE
            + GAS_FIXED / GAS_BUNDLE_SIZE
            + words * GAS_PER_USER_OP_WORD)
            .to_string())
    }
}
