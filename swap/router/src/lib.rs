use async_graphql::{Request, Response};
use linera_sdk::base::{ContractAbi, ServiceAbi};
use spec::swap::{RouterOperation, RouterResponse};

pub struct ApplicationAbi;

impl ContractAbi for ApplicationAbi {
    type Operation = RouterOperation;
    type Response = RouterResponse;
}

impl ServiceAbi for ApplicationAbi {
    type Query = Request;
    type QueryResponse = Response;
}
