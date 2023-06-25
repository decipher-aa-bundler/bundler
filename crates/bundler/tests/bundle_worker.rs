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
    }

    #[async_trait]
    impl AsyncTestContext for BundleWorkerContext {
        async fn setup() -> Self {
            let ep_addr = Address::from_str("0x7eA231E8C3b21ca5086cb2ed6647C1B851029Cc7").unwrap();

            let private_key_hex =
                "b01525a6e3d4b5804aa22dec67b9797de5430c27dc7b64a00762c51219f2bc63";

            let eth_client = EthClient::new(
                "https://ethereum-goerli.publicnode.com",
                "0x7eA231E8C3b21ca5086cb2ed6647C1B851029Cc7",
                private_key_hex,
            )
            .unwrap();
            let mempool = Mempool::new(5);
            let reputation_checker = ReputationChecker::new();

            // Add user ops to mempool
            let user_ops: UserOperation = UserOps {
                sender: "0xAFA2355A2035b394D24FC18bD88eD5821B81e3b7".to_string(),
                nonce: "0".to_string(),
                init_code: "0xaf0e7f80a3be9250d21792348c10e7240822e97b5fbfb9cf0000000000000000000000005339b0823cfac4a66de077f9ad42ce2ca17a0adf0000000000000000000000000000000000000000000000000000000000000001".to_string(),
                call_data: "0x".to_string(),
                call_gas_limit: "0xF4240".to_string(),
                verification_gas_limit: "0xF4240".to_string(),
                pre_verification_gas: "5208".to_string(),
                max_fee_per_gas: "3B9ACA22".to_string(),
                max_priority_fee_per_gas: "3B9ACA00".to_string(),
                paymaster_and_data: "0x".to_string(),
                signature: "0x90a6341a1755e2f8b0ae8a4e9b1befacf5cc06925bfe3c94481193b64c7b9fa308eaa8dd3c517acf6dff6f24f1b1db0c6712466d040a7d791bdb79ad9445de611b".to_string(),
            }.try_into().unwrap();

            mempool.push(ep_addr, user_ops.clone()).await;
            reputation_checker.register_address(
                Address::from_str("0xAFA2355A2035b394D24FC18bD88eD5821B81e3b7").unwrap(),
            );
            reputation_checker.register_address(user_ops.get_factory_addr().unwrap());

            BundleWorkerContext {
                bundle_worker: BundleWorker::new(
                    Box::new(mempool),
                    Box::new(eth_client),
                    5000000,
                    ep_addr,
                    Box::new(reputation_checker),
                ),
                user_ops: vec![user_ops],
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
        let res = ctx
            .bundle_worker
            .send_bundle(
                Address::from_str("0x7eA231E8C3b21ca5086cb2ed6647C1B851029Cc7").unwrap(),
                ctx.user_ops.clone(),
            )
            .await;

        print!("{:?}", res);
        assert!(res.is_ok());
    }
}
