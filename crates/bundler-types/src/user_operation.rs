use crate::error::BundlerTypeError;
use ethers::abi::AbiEncode;
use ethers::contract::{EthAbiCodec, EthAbiType};
use ethers::types::{Address, Bytes, U256};
use serde::{Deserialize, Serialize};
use std::str::FromStr;
macro_rules! parse_value {
    ($t:ty, $value: expr) => {
        <$t>::from_str($value).map_err(|e| BundlerTypeError::ParseError { msg: e.to_string() })
    };
}

#[derive(Debug, Clone, Serialize, Deserialize, EthAbiType, EthAbiCodec)]
pub struct UserOperation {
    pub sender: Address,
    pub nonce: U256,
    pub init_code: Bytes,
    pub call_data: Bytes,
    pub call_gas_limit: U256,
    pub verification_gas_limit: U256,
    pub pre_verification_gas: U256,
    pub max_fee_per_gas: U256,
    pub max_priority_fee_per_gas: U256,
    pub paymaster_and_data: Bytes,
    pub signature: Bytes,
}

#[allow(clippy::too_many_arguments)]
impl UserOperation {
    pub fn new(
        sender: &str,
        nonce: &str,
        init_code: &str,
        call_data: &str,
        call_gas_limit: &str,
        verification_gas_limit: &str,
        pre_verification_gas: &str,
        max_fee_per_gas: &str,
        max_priority_fee_per_gas: &str,
        paymaster_and_data: &str,
        signature: &str,
    ) -> Result<UserOperation, BundlerTypeError> {
        Ok(UserOperation {
            sender: parse_value!(Address, sender)?,
            nonce: parse_value!(U256, nonce)?,
            init_code: parse_value!(Bytes, init_code)?,
            call_data: parse_value!(Bytes, call_data)?,
            call_gas_limit: parse_value!(U256, call_gas_limit)?,
            verification_gas_limit: parse_value!(U256, verification_gas_limit)?,
            pre_verification_gas: parse_value!(U256, pre_verification_gas)?,
            max_fee_per_gas: parse_value!(U256, max_fee_per_gas)?,
            max_priority_fee_per_gas: parse_value!(U256, max_priority_fee_per_gas)?,
            paymaster_and_data: parse_value!(Bytes, paymaster_and_data)?,
            signature: parse_value!(Bytes, signature)?,
        })
    }

    pub fn try_serialize(&self) -> Result<String, BundlerTypeError> {
        serde_json::to_string(self)
            .map_err(|e| BundlerTypeError::SerializeError { msg: e.to_string() })
    }

    pub fn pack(&self) -> Bytes {
        self.clone().encode().into()
    }
}
