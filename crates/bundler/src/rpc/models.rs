use crate::rpc::errors::RpcError;

use bundler_types::user_operation::UserOperation;
use serde::{Deserialize, Serialize};

/// All fields must be set as hex values
/// Empty bytes block must be set to 0x
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UserOps {
    pub sender: String,
    pub nonce: String,
    pub init_code: String,
    pub call_data: String,
    pub call_gas_limit: String,
    pub verification_gas_limit: String,
    pub pre_verification_gas: String,
    pub max_fee_per_gas: String,
    pub max_priority_fee_per_gas: String,
    pub paymaster_and_data: String,
    pub signature: String,
}

impl TryFrom<UserOps> for UserOperation {
    type Error = RpcError;

    fn try_from(value: UserOps) -> Result<Self, Self::Error> {
        UserOperation::new(
            &value.sender,
            &value.nonce,
            &value.init_code,
            &value.call_data,
            &value.call_gas_limit,
            &value.verification_gas_limit,
            &value.pre_verification_gas,
            &value.max_fee_per_gas,
            &value.max_priority_fee_per_gas,
            &value.paymaster_and_data,
            &value.signature,
        )
        .map_err(|e| RpcError::Error(e.to_string()))
    }
}

impl TryFrom<&UserOps> for UserOperation {
    type Error = RpcError;

    fn try_from(value: &UserOps) -> Result<Self, Self::Error> {
        UserOperation::new(
            &value.sender,
            &value.nonce,
            &value.init_code,
            &value.call_data,
            &value.call_gas_limit,
            &value.verification_gas_limit,
            &value.pre_verification_gas,
            &value.max_fee_per_gas,
            &value.max_priority_fee_per_gas,
            &value.paymaster_and_data,
            &value.signature,
        )
        .map_err(|e| RpcError::Error(e.to_string()))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EstimateUserOpsGasResponse {
    pub call_gas_limit: String,
    pub pre_verification_gas: String,
    pub verification_gas: String,
}

impl EstimateUserOpsGasResponse {
    pub fn new(
        call_gas_limit: String,
        pre_verification_gas: String,
        verification_gas: String,
    ) -> Self {
        EstimateUserOpsGasResponse {
            call_gas_limit,
            pre_verification_gas,
            verification_gas,
        }
    }
}
