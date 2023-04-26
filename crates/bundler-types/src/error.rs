use thiserror::Error;

#[derive(Debug, Error)]
pub enum BundlerTypeError {
    #[error("failed to parse value: {msg}")]
    ParseError { msg: String },
}
