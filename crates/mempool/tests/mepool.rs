#[cfg(test)]
mod test_mempool {
    use bundler_types::user_operation::UserOperation;
    use ethers::types::{Address, U256};
    use log::info;
    use mempool::utils::get_unique_key;
    use mempool::Mempool;
    use std::str::FromStr;

    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    fn test_get_unique_key() {
        init();
        let ep_addr = Address::from_str("0x0576a174d229e3cfa37253523e645a78a0c91b57").unwrap();
        let sender = Address::from_str("0x9c98b1528c26cf36e78527308c1b21d89baed700").unwrap();
        let nonce = U256::from_str("1").unwrap();

        let _key = get_unique_key(&ep_addr, &sender, &nonce);
        info!("=== test ===");
    }

    #[tokio::test]
    async fn test_insert_cache() {
        init();
        let ep_addr = Address::from_str("0x0576a174d229e3cfa37253523e645a78a0c91b57").unwrap();

        let user_ops = UserOperation::new(
            "0x9c98b1528c26cf36e78527308c1b21d89baed700",
            "8",
            "0x",
            "0x940d3c600000000000000000000000004648a43b2c14da09fdf82b161150d3f634f40491000000000000000000000000000000000000000000000000002386f26fc100000000000000000000000000000000000000000000000000000000000000000080000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000002843593564c000000000000000000000000000000000000000000000000000000000000006000000000000000000000000000000000000000000000000000000000000000a0000000000000000000000000000000000000000000000000000000006449438400000000000000000000000000000000000000000000000000000000000000020b000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000002000000000000000000000000000000000000000000000000000000000000004000000000000000000000000000000000000000000000000000000000000000a000000000000000000000000000000000000000000000000000000000000000400000000000000000000000000000000000000000000000000000000000000002000000000000000000000000000000000000000000000000002386f26fc1000000000000000000000000000000000000000000000000000000000000000001000000000000000000000000000000000000000000000000000000000000000001000000000000000000000000000000000000000000000000002386f26fc100000000000000000000000000000000000000000000000000001396d8984a3aa6af00000000000000000000000000000000000000000000000000000000000000a00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000002bb4fbf271143f4fbf7b91a5ded31805e42b2208d60001f41f9840a85d5af5bf1d1762f925bdaddc4201f98400000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000",
            "356496",
            "100000",
            "55904",
            "131674005360",
            "1500000000",
            "0x3b912be0270b59143985cc5c6aab452d99e2b4bb0000000000000000000000000000000000000000000000000000000064493ee50000000000000000000000000000000000000000000000000000000000000000864797bf1ae6b7e936b0af45f5f17ffda6c0fbc5189b6a19d0fb44ff0f37c15730d47efbff0079bd629f4d3dd09bf136226db30e11c58843cebb4b20f024de101b",
            "0x981c39ab076c5400c830a57e5e711443221085978dea4d343b413b50a1c841c7244c5a10295c89367c7d4acbe7451ab916d537fc61a1ac1ccda26b61e0137b761b",
        ).unwrap();

        let mempool = Mempool::new().unwrap();
        mempool.add(ep_addr, user_ops.clone()).await.unwrap();

        let _data = mempool
            .get_op(&ep_addr, &user_ops.sender, &U256::from_str("8").unwrap())
            .await;
    }
}