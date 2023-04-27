use bundler_types::error::BundlerTypeError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum RpcError {
    #[error(transparent)] //thiserror : 가진 에러를 그대로 string형태로 나타낼 수 있는 매크로 / macro에도 종류가 있다.
    BundlerTypeError(#[from] BundlerTypeError), //from을 구현해 준다 (Error derive하기 때문에~~
}
