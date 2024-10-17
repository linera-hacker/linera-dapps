use spec::swap::abi::SwapApplicationAbi;
use spec::swap::state::StateError;
use swap_router::{pool_manager::PoolError, router::RouterError};
use thiserror::Error;

pub type ApplicationAbi = SwapApplicationAbi;

#[derive(Debug, Error)]
#[allow(dead_code)]
pub enum SwapError {
    #[error(transparent)]
    StateError(#[from] StateError),

    #[error(transparent)]
    PoolError(#[from] PoolError),

    #[error(transparent)]
    RouterError(#[from] RouterError),
}
