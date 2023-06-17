use std::fmt::Formatter;

#[derive(Debug)]
pub struct BundlerTypeError {
    pub msg: String,
    pub code: ErrorCode,
}

impl BundlerTypeError {
    pub fn invalid_argument(msg: String) -> BundlerTypeError {
        BundlerTypeError {
            msg,
            code: ErrorCode::InvalidArgument,
        }
    }
}

impl std::fmt::Display for BundlerTypeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format_args!("{}: {:?}", self.msg, self.code))
    }
}

#[derive(Debug)]
pub enum ErrorCode {
    InvalidArgument = 32602,
}
