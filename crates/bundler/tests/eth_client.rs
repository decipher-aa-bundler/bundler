#[cfg(test)]
mod eth_client_test {
    use async_trait::async_trait;
    use bundler::ethereum::types::EthClient;
    use bundler::ethereum::EthClientHandler;
    use ethers::types::{Address, Bytes};
    use std::str::FromStr;
    use test_context::{test_context, AsyncTestContext};
    use bundler::rpc::models::UserOps;

    struct EthClientContext {
        eth_client: EthClient,
    }

    #[async_trait]
    impl AsyncTestContext for EthClientContext {
        async fn setup() -> Self {
            let private_key_hex =
                "35640e441bf700a1afcdb33eee4cf795013d23195f450cf5f9d80274617b72ec";
            let private_key = hex::decode(private_key_hex).unwrap();

            EthClientContext {
                eth_client: EthClient::new(
                    "0x5FF137D4b0FDCD49DcA30c7CF57E578a026d2789",
                    private_key.as_slice(),
                )
                .unwrap(),
            }
        }
    }

    #[test_context(EthClientContext)]
    #[tokio::test]
    async fn test_eth_estimate_gas(ctx: &EthClientContext) {
        let from = Address::from_str("0x5FF137D4b0FDCD49DcA30c7CF57E578a026d2789").unwrap();
        let to = Address::from_str("0xfFec17D1920455f6AD90E578269832d5c442D59C").unwrap();
        let call_data = Bytes::from("0x0565bb6700000000000000000000000017a62ab5da63f3570179e87cb8711de28a0f8412000000000000000000000000000000000000000000000000016345785d8a000000000000000000000000000000000000000000000000000000000000000000600000000000000000000000000000000000000000000000000000000000000000".as_bytes().to_vec());

        let res = ctx.eth_client.estimate_gas(from, to, call_data).await;
        assert!(res.is_ok());
    }

    #[test_context(EthClientContext)]
    #[tokio::test]
    async fn test_calc_pre_verification_gas(ctx: &EthClientContext) {
        let user_ops = UserOps {
            sender: "0xfFec17D1920455f6AD90E578269832d5c442D59C".to_string(),
            nonce: "36".to_string(),
            init_code: "0x".to_string(),
            call_data: "0x0565bb6700000000000000000000000017a62ab5da63f3570179e87cb8711de28a0f8412000000000000000000000000000000000000000000000000016345785d8a000000000000000000000000000000000000000000000000000000000000000000600000000000000000000000000000000000000000000000000000000000000000".to_string(),
            call_gas_limit: "1500000".to_string(),
            verification_gas_limit: "1500000".to_string(),
            pre_verification_gas: "65000".to_string(),
            max_fee_per_gas: "1500000026".to_string(),
            max_priority_fee_per_gas: "1500000000".to_string(),
            paymaster_and_data: "0x".to_string(),
            signature: "0x561a6befd401331cd263d1db2e717a0176945d5496f8844aecc5dd37959233a359b141e6e1662decd56f65248de4197f794ed82a7401fcdef2c2b2a42f359fe61b".to_string(),
        };

        let res = ctx.eth_client.calc_pre_verification_gas(
            &user_ops
        ).await.unwrap();

        println!("{:?}", res);
    }
}
