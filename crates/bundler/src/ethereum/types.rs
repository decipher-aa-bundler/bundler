use std::str::FromStr;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};
use crate::ethereum::errors::EthereumError;
use crate::ethereum::EthClientHandler;
use async_trait::async_trait;
use ethers::abi::AbiDecode;
use ethers::middleware::SignerMiddleware;
use ethers::prelude::{ContractError};
use ethers::prelude::gas_oracle::MiddlewareError;
use ethers::providers::{Http, Middleware, Provider};
use ethers::signers::{LocalWallet, Signer};
use ethers::types::transaction::eip2718::TypedTransaction;
use ethers::types::{Address, Bytes, U256};
use eyre::eyre;
use bundler_types::user_operation::UserOperation;
use contracts::bindings::abi::entry_point;
use contracts::bindings::abi::entry_point::{IEntryPoint, IEntryPointErrors};
use crate::rpc::errors::RpcError;
use crate::rpc::models::UserOps;

#[derive(Debug)]
pub struct EthClient {
    pub eth_provider: Provider<Http>,
}

#[allow(clippy::new_ret_no_self)]
impl EthClient {
    pub fn new() -> Result<Box<dyn EthClientHandler>, EthereumError> {
        Ok(Box::new(EthClient {
            // TODO: url 하드코딩 config로 빼기
            eth_provider: Provider::<Http>::try_from(
                "https://goerli.blockpi.network/v1/rpc/public",
            )
            .map_err(|e| EthereumError::ProviderError(eyre!(e)))?,
        }))
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
            .map_err(|e| EthereumError::ProviderError(eyre!(e)))
    }

    async fn simulate_validation_gas(&self, _user_ops: &UserOps, _ep_addr: &str) -> Result<U256, EthereumError> {
        let user_ops: UserOperation = _user_ops.try_into().map_err(|e: RpcError| EthereumError::ProviderError(eyre!(e)))?;
        let user_operation = entry_point::UserOperation::new(user_ops).map_err(|e| EthereumError::ProviderError(eyre!(e)))?;
        let entry_point = self.get_entry_point(_ep_addr)?;

        let result = entry_point.simulate_validation(user_operation).send().await;

        match result {
            Ok(_) => Err(EthereumError::ProviderError(eyre!("simulation must be reverted"))),
            Err(contract_error) => {
                let revert_msg = self.decode_revert_msg(contract_error)?;
                match revert_msg {
                    Err(IEntryPointErrors::ValidationResult(e)) => {
                        //check signature
                        let sig_failed = e.return_info.2;
                        if sig_failed {
                            return Err(EthereumError::ProviderError(eyre!("signature failed")))
                        }

                        //check allowance
                        let valid_after = e.return_info.3;
                        let valid_until = e.return_info.4;
                        let now: u64 = SystemTime::now().duration_since(UNIX_EPOCH).into()?;
                        if now < valid_after || now > valid_until {
                            return Err(EthereumError::ProviderError(eyre!("paymaster allowance is expired")))
                        }

                        //check stake info
                        if e.factory_info.1 <= U256::from(0) || e.sender_info.1 <= U256::from(0)  || e.paymaster_info.1 <= U256::from(0)  {
                            return Err(EthereumError::ProviderError(eyre!("no staking info")))
                        }
                        //return preOpGas
                        Ok(e.return_info.0)

                        //사용하지 않은 값들 -> prefund, paymasterContext. 사용할 필요가 있는지?
                        //리턴값에 더 필요한 정보는 없는지?
                    }
                    Err(IEntryPointErrors::ValidationResultWithAggregation(e)) => {
                        //TODO: 위 로직과 거의 비슷한데, 어떻게 합칠 수 있을지?

                        Ok(e.return_info.0)
                    }
                    _ => {
                        return Err(EthereumError::ProviderError(eyre!("Error"))); //TODO
                    }
                }
            }
        }
    }

    fn decode_revert_msg(&self, error: ContractError<SignerMiddleware<M, S>>) -> Result<IEntryPointErrors, EthereumError> {
        match error {
            ContractError::Revert(e) => AbiDecode::decode(e).map_err(|e| EthereumError::DecodeError(eyre!(e))),
            ContractError::ProviderError { e } => Err(EthereumError::DecodeError(eyre!("Provider error {:?}", e))),
            ContractError::MiddlewareError { e } => Err(EthereumError::DecodeError(eyre!("Middleware error {:?}", e))),
            ContractError::DecodingError(e) => Err(EthereumError::DecodeError(eyre!("Decode error {:?}", e.into()))),
            ContractError::AbiError(e) => Err(EthereumError::DecodeError(eyre!("Abi error {:?}", e.into()))),
            _ => Err(EthereumError::DecodeError(eyre!("failed to decode contract error, cannot infer the reason")))
        }
    }

    fn get_entry_point(&self, _ep_addr: &str) -> Result<IEntryPoint<M>, EthereumError> {
        //////////TODO: config, sign 가능한 상태로 두는게 맞는지
        let signer = "3bda47bf6e810ccccc595cd6fd7d7895bbcdf008d1f3fc3b89c1503a2c73300a".parse::<LocalWallet>().unwrap(); // address 0x8017484dE221AE05Fe3069D6972919b6eb1228d7
        let chain_id : u64 = 5;
        ////////
        let client = Arc::new(SignerMiddleware::new(&self.eth_provider, signer.with_chain_id(chain_id)));
        let ep_addr = Address::from_str(_ep_addr).map_err(|e| EthereumError::ProviderError(eyre!(e)))?;
        Ok(IEntryPoint::new(ep_addr, client))
    }
}
