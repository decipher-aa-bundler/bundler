use crate::ethereum::errors::EthereumError;
use crate::ethereum::EthClientHandler;
use crate::rpc::models::UserOps;

use async_trait::async_trait;
use bundler_types::user_operation::UserOperation;
use contracts::bindings::abi::entry_point::IEntryPoint;
use ethers::middleware::SignerMiddleware;
use ethers::providers::{Http, Middleware, Provider};
use ethers::signers::LocalWallet;
use ethers::types::transaction::eip2718::TypedTransaction;
use ethers::types::{Address, Bytes, U256};
use std::str::FromStr;
use std::sync::Arc;

#[derive(Debug)]
pub struct EthClient {
    eth_provider: Arc<Provider<Http>>,
    entry_point: IEntryPoint<SignerMiddleware<Provider<Http>, LocalWallet>>,
}

#[derive(Debug)]
pub struct GasOverhead {
    fixed: u64,
    per_user_op: u64,
    per_user_op_word: u64,
    zero_byte: u64,
    non_zero_byte: u64,
    bundle_size: u64,
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
        }
    }
}

impl EthClient {
    pub fn new(ep_addr: &str, signer: &[u8]) -> Result<EthClient, EthereumError> {
        let eth_provider = Provider::<Http>::try_from(
            // TODO: url 하드코딩 config로 빼기
            "https://ethereum-goerli.publicnode.com",
        )
        .map_err(|e| EthereumError::ProviderError(e.to_string()))?;
        let ep_addr = Address::from_str(ep_addr)
            .map_err(|e| EthereumError::DecodeError(format!("ep_addr decode failed: {}", e)))?;

        let signer = LocalWallet::from_bytes(signer).map_err(|e| {
            EthereumError::DecodeError(format!("failed to create signer key: {}", e))
        })?;

        Ok(EthClient {
            eth_provider: Arc::new(eth_provider.clone()),
            entry_point: IEntryPoint::new(
                ep_addr,
                Arc::new(SignerMiddleware::new(eth_provider, signer)),
            ),
        })
    }
}

#[async_trait]
impl EthClientHandler for EthClient {
    async fn estimate_gas(
        &self,
        from: Address,
        to: Address,
        call_data: Bytes,
    ) -> Result<U256, EthereumError> {
        let mut tx = TypedTransaction::default();
        tx.set_from(from).set_to(to).set_data(call_data);

        self.eth_provider
            .estimate_gas(&tx, None)
            .await
            .map_err(|e| EthereumError::ProviderError(e.to_string()))
    }

    async fn calc_pre_verification_gas(&self, user_ops: &UserOps) -> Result<U256, EthereumError> {
        let mut user_operation = UserOperation::try_from(user_ops)
            .map_err(|e| EthereumError::DecodeError(e.to_string()))?;

        let gas_overhead = GasOverhead::default();
        // dummy value
        user_operation.pre_verification_gas = gas_overhead.fixed.into();
        // dummy signature
        user_operation.signature = Bytes::from_str("0x").unwrap();

        let packed_user_operation = user_operation.pack();
        let bytes_user_ops = packed_user_operation.to_vec();
        let zeros = bytes_user_ops.iter().filter(|i| **i == 0).count() as u64;
        let non_zero = bytes_user_ops.len() as u64 - zeros;
        let words = ((packed_user_operation.len() + 31) / 32) as u64;

        Ok((zeros * gas_overhead.zero_byte
            + non_zero * gas_overhead.non_zero_byte
            + gas_overhead.fixed / gas_overhead.bundle_size
            + gas_overhead.per_user_op
            + words * gas_overhead.per_user_op_word)
            .into())
    }
}
