#[cfg(test)]
mod eth_client_test {
    use async_trait::async_trait;
    use bundler::ethereum::types::EthClient;
    use bundler::ethereum::EthClientHandler;
    use bundler::rpc::models::UserOps;
    use bundler_types::user_operation::UserOperation;
    use ethers::types::{Address, Bytes};
    use std::str::FromStr;
    use test_context::{test_context, AsyncTestContext};

    struct EthClientContext {
        eth_client: EthClient,
        user_ops: UserOperation,
    }

    #[async_trait]
    impl AsyncTestContext for EthClientContext {
        async fn setup() -> Self {
            let private_key_hex =
                "b01525a6e3d4b5804aa22dec67b9797de5430c27dc7b64a00762c51219f2bc63";
            let private_key = hex::decode(private_key_hex).unwrap();

            EthClientContext {
                eth_client: EthClient::new(
                    "0x7eA231E8C3b21ca5086cb2ed6647C1B851029Cc7",
                    private_key.as_slice(),
                )
                .unwrap(),
                user_ops: UserOps {
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
                }.try_into().unwrap()
            }
        }
    }

    #[test_context(EthClientContext)]
    #[tokio::test]
    async fn test_eth_estimate_gas(ctx: &EthClientContext) {
        let from = Address::from_str("0x7eA231E8C3b21ca5086cb2ed6647C1B851029Cc7").unwrap();
        let to = Address::from_str("0xAFA2355A2035b394D24FC18bD88eD5821B81e3b7").unwrap();
        let call_data = Bytes::from("0x".as_bytes().to_vec());

        let res = ctx.eth_client.estimate_gas(from, to, call_data).await;

        assert!(res.is_ok());
    }

    #[test_context(EthClientContext)]
    #[tokio::test]
    async fn test_calc_pre_verification_gas(ctx: &EthClientContext) {
        let res = ctx
            .eth_client
            .calc_pre_verification_gas(&ctx.user_ops.clone())
            .await;

        assert!(res.is_ok());
        assert_eq!(42732, res.unwrap().as_u64());
    }

    #[test_context(EthClientContext)]
    #[tokio::test]
    async fn test_calc_verification_gas(ctx: &EthClientContext) {
        let res = ctx
            .eth_client
            .simulate_validation(ctx.user_ops.clone())
            .await;
        assert!(res.is_ok());
        assert_eq!(334203, res.unwrap().as_u64())
    }
}
