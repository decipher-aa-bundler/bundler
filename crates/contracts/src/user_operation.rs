use crate::bindings::abi::entry_point::UserOperation;
use bundler_types::user_operation::UserOperation as bundlerUserOperation;
use bundler_types::error::BundlerTypeError;

impl UserOperation {
    pub fn new(
        user_ops: bundlerUserOperation
    ) -> Result<UserOperation, BundlerTypeError> {
        Ok(UserOperation {
            sender: user_ops.sender,
            nonce: user_ops.nonce,
            init_code: user_ops.init_code,
            call_data: user_ops.call_data,
            call_gas_limit: user_ops.call_gas_limit,
            verification_gas_limit: user_ops.verification_gas_limit,
            pre_verification_gas: user_ops.pre_verification_gas,
            max_fee_per_gas: user_ops.max_fee_per_gas,
            max_priority_fee_per_gas: user_ops.max_priority_fee_per_gas,
            paymaster_and_data: user_ops.paymaster_and_data,
            signature: user_ops.signature,
        })
    }
}
