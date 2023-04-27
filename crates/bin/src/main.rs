use std::sync::Arc;
use ethers::core::types::Address;
use ethers::core::types::Bytes;
use ethers::core::types::U256;
use ethers::middleware::SignerMiddleware;
use bundler::rpc::server::new_server;
use contracts::ethereum::entry_point;
use contracts::ethereum::entry_point::UserOperation;
use ethers_providers::{Http, Provider};
use ethers_signers::LocalWallet;
use ethers_signers::Signer;

#[tokio::main]
async fn main() {
    new_server().await;
}

async fn get_entry_point() {
    let provider = Provider::<Http>::try_from("https://goerli.blockpi.network/v1/rpc/public").unwrap();
    let address = "0x0576a174D229E3cFA37253523E645A78A0C91B57".parse::<Address>().unwrap();

    let signer = "3bda47bf6e810ccccc595cd6fd7d7895bbcdf008d1f3fc3b89c1503a2c73300a".parse::<LocalWallet>().unwrap();
    let signer_address = ("0x8017484dE221AE05Fe3069D6972919b6eb1228d7").parse::<Address>().unwrap();

    let holder = "0xaBA64332AD3d9aD206eDddA16280b5F73E6770Ea".parse::<Address>().unwrap();
    let chain_id : u64 = 5;
    let client = Arc::new(SignerMiddleware::new(provider, signer.with_chain_id(chain_id)));

    let entry_point = entry_point::IEntryPoint::new(address, client);
    let mut balance = entry_point.balance_of(holder).call().await.unwrap();
    println!("{:?}", balance);

    let call = entry_point.deposit_to(holder).value(200);
    let result = call.send().await;
    println!("{:?}", result.unwrap());
    balance = entry_point.balance_of(holder).call().await.unwrap();
    println!("{:?}", balance);
}
