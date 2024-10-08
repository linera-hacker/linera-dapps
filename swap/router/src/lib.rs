use linera_sdk::base::{ContractAbi, ServiceAbi};
use async_graphql::{Request, Response};

pub struct ApplicationAbi;

impl ContractAbi for ApplicationAbi {
    type Operation = ();
    type Response = ();
}

impl ServiceAbi for ApplicationAbi {
    type Query = Request;
    type QueryResponse = Response;
}
