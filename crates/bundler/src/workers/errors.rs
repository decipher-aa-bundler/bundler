use thiserror::Error;

#[derive(Error, Debug)]
pub enum WorkerError {
    #[error("{0}")]
    ReputationError(String),
    #[error("{0}")]
    SecondValidationError(String),
    #[error("{0}")]
    EthClientError(String),
}
