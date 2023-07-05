#[cfg(test)]
mod bundle_worker_test {
    use async_trait::async_trait;
    use bundler::{
        ethereum::types::EthClient,
        rpc::models::UserOps,
        workers::{
            types::BundleWorker, types::ReputationChecker, BundleManager, ReputationHandler,
        },
    };
    use bundler_types::user_operation::UserOperation;
    use ethers::types::Address;
    use mempool::{Mempool, MempoolService};
    use std::str::FromStr;
    use test_context::{test_context, AsyncTestContext};

    struct BundleWorkerContext {
        bundle_worker: BundleWorker,
        user_ops: Vec<UserOperation>,
        ep_addr: String,
    }

    #[async_trait]
    impl AsyncTestContext for BundleWorkerContext {
        async fn setup() -> Self {
            let ep_addr = "0x7eb6D1C6a5C0c30b97668FC391EC9f0e5250a816";

            let private_key_hex =
                "ac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80";

            let eth_client =
                EthClient::new("http://127.0.0.1:8545/", ep_addr, private_key_hex, 5).unwrap();
            let mempool = Mempool::new(5);
            let reputation_checker = ReputationChecker::new(100, 10, 10);

            // Add user ops to mempool
            let user_ops: UserOperation = UserOps {
                sender: "0x40d574dd6068C3eDCDFF99C3A131304Ea0013C0A".to_string(),
                nonce: "3".to_string(),
                init_code: "0x".to_string(),
                call_data: "0xb61d27f6000000000000000000000000e59c5dfe380cccd122e16baf2379a5eed854073900000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000060000000000000000000000000000000000000000000000000000000000000000406661abd00000000000000000000000000000000000000000000000000000000".to_string(),
                call_gas_limit: "0xF4240".to_string(),
                verification_gas_limit: "0xF4240".to_string(),
                pre_verification_gas: "5208".to_string(),
                max_fee_per_gas: "AF9772145".to_string(),
                max_priority_fee_per_gas: "3B9ACA00".to_string(),
                paymaster_and_data: "0x".to_string(),
                signature: "0x8ff4bfc48e4641926eb4f80341f9b3c95f66311505db5844d89010f7286e51ef4695731ed9c01e2e3f378d6cf67712503fbc12264b29d465afe83b49109395bd1c".to_string(),
            }.try_into().unwrap();

            mempool
                .push(Address::from_str(ep_addr).unwrap(), user_ops.clone())
                .await;
            reputation_checker.register_address(
                Address::from_str("0xAFA2355A2035b394D24FC18bD88eD5821B81e3b7").unwrap(),
            );
            // reputation_checker.register_address(user_ops.get_factory_addr().unwrap());

            BundleWorkerContext {
                bundle_worker: BundleWorker::new(
                    Box::new(mempool),
                    Box::new(eth_client),
                    5000000, //max_gas
                    ep_addr,
                    "0x7eA231E8C3b21ca5086cb2ed6647C1B851029Cc7", //beneficiary
                    Box::new(reputation_checker),
                )
                .unwrap(),
                user_ops: vec![user_ops],
                ep_addr: ep_addr.into(),
            }
        }
    }

    #[test_context(BundleWorkerContext)]
    #[tokio::test]
    async fn test_create_bundle(ctx: &BundleWorkerContext) {
        let res = ctx.bundle_worker.create_bundle().await;

        assert!(res.is_ok());
        assert!(res.unwrap()[0] == ctx.user_ops[0]);
    }

    #[test_context(BundleWorkerContext)]
    #[tokio::test]
    async fn test_send_bundle(ctx: &BundleWorkerContext) {
        let bundle = ctx.bundle_worker.create_bundle().await.unwrap();

        let res = ctx
            .bundle_worker
            .send_bundle(
                Address::from_str(ctx.ep_addr.as_str()).unwrap(),
                bundle.clone(),
            )
            .await;

        assert!(res.is_ok());
    }
}
