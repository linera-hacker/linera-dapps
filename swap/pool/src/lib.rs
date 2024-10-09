use async_graphql::{Request, Response};
use linera_sdk::base::{ContractAbi, ServiceAbi};
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
pub enum PoolError {}
