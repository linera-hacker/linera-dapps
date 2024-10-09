use async_graphql::{Request, Response};
use linera_sdk::base::{ContractAbi, ServiceAbi};
use spec::swap::PoolOperation;

pub struct ApplicationAbi;

impl ContractAbi for ApplicationAbi {
    type Operation = PoolOperation;
    type Response = ();
}

impl ServiceAbi for ApplicationAbi {
    type Query = Request;
    type QueryResponse = Response;
}
