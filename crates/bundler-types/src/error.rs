use thiserror::Error;

#[derive(Debug, Error)]
pub enum UserOpsError {
    #[error("failed to parse value: {msg}")]
    ParseError {
        msg: String
    }
}