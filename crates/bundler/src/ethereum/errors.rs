use thiserror::Error;

#[derive(Error, Debug)]
pub enum EthereumError {
    #[error("cannot init eth provider: {msg}")]
    ProviderError { msg: String },
}
