use thiserror::Error;

#[derive(Error, Debug)]
pub enum EthereumError {
    #[error("{0}")]
    ProviderError(String),
    #[error("{0}")]
    DecodeError(String),
    #[error("{0}")]
    ValidateError(String),
}
