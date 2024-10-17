use spec::swap::abi::SwapApplicationAbi;
use spec::swap::state::StateError;
use thiserror::Error;

pub type ApplicationAbi = SwapApplicationAbi;

#[derive(Debug, Error)]
#[allow(dead_code)]
pub enum SwapError {
    #[error(transparent)]
    StateError(#[from] StateError),
}
