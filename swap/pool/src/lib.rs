use async_graphql::{Request, Response};
use linera_sdk::base::{ContractAbi, ParseAmountError, ServiceAbi};
use spec::swap::{PoolOperation, PoolResponse};
use thiserror::Error;

pub struct ApplicationAbi;

impl ContractAbi for ApplicationAbi {
    type Operation = PoolOperation;
    type Response = PoolResponse;
}

impl ServiceAbi for ApplicationAbi {
    type Query = Request;
    type QueryResponse = Response;
}

#[derive(Debug, Error)]
#[allow(dead_code)]
pub enum PoolError {
    #[error("Invalid initial amount")]
    InvalidInitialAmount,

    #[error(transparent)]
    ParseAmountError(#[from] ParseAmountError),

    #[error(transparent)]
    ViewError(#[from] linera_sdk::views::ViewError),

    #[error("Already exists")]
    AlreadyExists,
}
