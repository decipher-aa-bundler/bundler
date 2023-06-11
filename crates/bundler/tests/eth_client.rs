#[cfg(test)]
mod eth_client_test {
    use async_trait::async_trait;
    use bundler::ethereum::types::EthClient;
    use bundler::ethereum::EthClientHandler;
    use ethers::types::{Address, Bytes};
    use std::str::FromStr;
    use test_context::{test_context, AsyncTestContext};

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
    async fn test_simulate_validation_gas(ctx: &EthClientContext) {

        // ctx.eth_client.simulate_validation_gas().await.unwrap();
    }
}
