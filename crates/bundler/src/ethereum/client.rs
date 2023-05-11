use crate::ethereum::{EthClient, EthClientHandler};
use async_trait::async_trait;
use ethers::types::Address;

use bundler_types::user_operation::UserOperation;
use contracts::bindings::abi::entry_point;

#[async_trait]
impl EthClientHandler for EthClient {
    async fn calculate_gas(&self, _user_ops: UserOperation, _ep: Address) {
        let client = &self.eth_provider;
        let ep = entry_point::IEntryPoint::new(_ep, client.clone()); //TODO: 그냥 막 하다보니 됐는데 왜 된 건지 잘 모르겠음
        let user_ops = entry_point::UserOperation::new(_user_ops).unwrap_or_default();
        let res = ep.get_user_op_hash(user_ops.clone()).await;

        //TODO: Check banned Opcode

        //1. getSimulationValidation
        let sim = ep.simulate_validation(user_ops.clone()).await;

        //2. getCallGasEstimate
        let call_gas = ep.simulate_handle_op(user_ops.clone(), _ep, user_ops.call_data.clone());

        //3. calculate pre-verification gas

    }

    async fn calcPreVerificationGas(_user_ops: UserOperation) {

    }
}
