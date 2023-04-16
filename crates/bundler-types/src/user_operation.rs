use crate::error::UserOpsError;

use std::str::FromStr;
use ethers::types::{Address, U256, Bytes};

#[derive(Debug, Clone)]
pub struct UserOperation {
    pub sender: Address,
    pub nonce: U256,
    pub init_code: Bytes,
    pub call_data: Bytes,
    pub verification_gas_limit: U256,
    pub pre_verification_gas: U256,
    pub max_fee_per_gas: U256,
    pub max_priority_fee_per_gas: U256,
    pub paymaster_and_data: Bytes,
    pub signature: Bytes,
}

impl UserOperation {
    fn new(sender: &str, nonce: &str, init_code: &str) -> Result<UserOperation, UserOpsError> {
        Ok(UserOperation {
            sender: Address::from_str(sender).map_err(|e| UserOpsError::ParseError { msg: e.to_string() })?,
            nonce: U256::from_str(nonce).map_err(|e| UserOpsError::ParseError { msg: e.to_string() })?,
            init_code: Bytes::from_str(init_code).map_err(|e| UserOpsError::ParseError { msg: e.to_string() })?,
            call_data: Default::default(),
            verification_gas_limit: Default::default(),
            pre_verification_gas: Default::default(),
            max_fee_per_gas: Default::default(),
            max_priority_fee_per_gas: Default::default(),
            paymaster_and_data: Default::default(),
            signature: Default::default(),
        })
    }
}