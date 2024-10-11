use async_graphql::{Request, Response};
use linera_sdk::base::{ContractAbi, ServiceAbi};
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
}
