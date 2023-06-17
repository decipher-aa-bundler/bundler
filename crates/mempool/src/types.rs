use bundler_types::user_operation::UserOperation;
use ethers::types::{Address, Bytes, U256};
use std::cmp::Ordering;

#[derive(Debug, Clone, Eq, PartialEq)]
pub(crate) struct UserOpsWithEpAddr {
    pub ep_addr: Address,
    pub hash: Bytes,
    pub user_ops: UserOperation,
}

impl UserOpsWithEpAddr {
    pub fn from_user_ops(
        ep_addr: &Address,
        user_ops: &UserOperation,
        chain_id: &U256,
    ) -> UserOpsWithEpAddr {
        UserOpsWithEpAddr {
            ep_addr: *ep_addr,
            hash: user_ops.hash(ep_addr, chain_id),
            user_ops: user_ops.clone(),
        }
    }
}

impl PartialOrd<Self> for UserOpsWithEpAddr {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.user_ops
            .max_priority_fee_per_gas
            .partial_cmp(&other.user_ops.max_priority_fee_per_gas)
    }
}

impl Ord for UserOpsWithEpAddr {
    fn cmp(&self, other: &Self) -> Ordering {
        self.user_ops
            .max_priority_fee_per_gas
            .cmp(&other.user_ops.max_priority_fee_per_gas)
    }
}
