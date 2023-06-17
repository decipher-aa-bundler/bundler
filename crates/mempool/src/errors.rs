use thiserror::Error;

#[derive(Debug, Error)]
pub enum MempoolError {
    #[error("failed to init db: {msg}")]
    DbInitError { msg: String },

    #[error("failed to insert value into db: {0}")]
    InsertError(String),

    #[error("failed to serialize value: {msg}")]
    SerializeError { msg: String },
}
