use std::env;
use std::path::PathBuf;
use ethers::contract::{abigen, EthCall};
use ethers::prelude::{Abigen, ConfigurableArtifacts, ProjectPathsConfig};
use ethers::solc::Solc;

fn main() {
    let contract_base = "/Users/sangyun/Documents/workspace/aa-bundler/ext";
    let contract_name = "EntryPoint";

    let contracts = Solc::default()
        // .with_base_path(".")
        .compile_source(format!("{}/contracts/core/EntryPoint.sol", contract_base)).unwrap();

    let c = contracts.get(&format!("{}/contracts/core/EntryPoint.sol", contract_base), "EntryPoint").unwrap().unwrap();
    println!("{:?}", c.abi);

    println!("{}", &env!("CARGO_MANIFEST_DIR"))
    // let contract = contracts.get("EntryPoint.sol", contract_name).unwrap();
}