use spec::swap::RouterApplicationAbi;
use thiserror::Error;

pub type ApplicationAbi = RouterApplicationAbi;

#[derive(Debug, Error)]
#[allow(dead_code)]
pub enum RouterError {
    #[error("Invalid amount")]
    InvalidAmount,

    #[error("Create pool error")]
    CreatePoolError,

    #[error("Invalid pool")]
    InvalidPool,
}
