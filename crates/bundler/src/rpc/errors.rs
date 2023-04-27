use bundler_types::error::BundlerTypeError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum RpcError {
    #[error(transparent)]
    BundlerTypeError(#[from] BundlerTypeError),
}