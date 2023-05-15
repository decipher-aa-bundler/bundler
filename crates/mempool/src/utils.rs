use ethers::types::{Address, U256};

pub fn get_unique_key(ep_addr: &Address, sender: &Address, nonce: &U256) -> Vec<u8> {
    let mut key = Vec::new();
    key.extend_from_slice(ep_addr.as_bytes());
    key.extend_from_slice(sender.as_bytes());
    key.extend_from_slice(nonce.to_string().as_bytes());
    key
}
