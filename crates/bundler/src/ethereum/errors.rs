use thiserror::Error;

#[derive(Error, Debug)]
pub enum EthereumError {
    #[error("eth client error: {msg}")]
    ProviderError { msg: String },
}
