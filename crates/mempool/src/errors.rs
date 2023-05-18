use thiserror::Error;

#[derive(Debug, Error)]
pub enum MempoolError {
    #[error("failed to init db: {msg}")]
    DbInitError { msg: String },

    #[error("failed to insert value into db: {msg}")]
    InsertError { msg: String },

    #[error("failed to serialize value: {msg}")]
    SerializeError { msg: String },
}
