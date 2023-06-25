use serde::Deserialize;
use std::sync::RwLock;

lazy_static::lazy_static! {
    pub static ref CONFIGS: RwLock<Config> = RwLock::new(Config::default());
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct Config {
    pub chain_id: u64,
    pub eth_rpc: String,
    pub ep_addr: String,
    pub signer: String,
}

impl Config {
    pub fn set(config_path: String) {
        let config_file = std::fs::read_to_string(config_path).expect("file not found");
        let config: Config = toml::from_str(&config_file).expect("invalid config file");

        let mut c = CONFIGS.write().unwrap();
        c.chain_id = config.chain_id;
        c.eth_rpc = config.eth_rpc;
        c.ep_addr = config.ep_addr;
        c.signer = config.signer;
    }

    pub fn get() -> Config {
        CONFIGS.read().unwrap().clone()
    }
}
