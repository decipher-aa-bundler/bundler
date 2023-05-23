use thiserror::Error;

#[derive(Error, Debug)]
pub enum EthereumError {
    #[error("{0}")]
    ProviderError(eyre::Report),
    #[error("{0}")]
    DecodeError(eyre::Report),
}
