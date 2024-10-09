use async_graphql::{Request, Response};
use linera_sdk::base::{ContractAbi, ServiceAbi};

pub struct ApplicationAbi;

impl ContractAbi for ApplicationAbi {
    type Operation = ();
    type Response = ();
}

impl ServiceAbi for ApplicationAbi {
    type Query = Request;
    type QueryResponse = Response;
}
