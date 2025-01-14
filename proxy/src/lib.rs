use linera_sdk::base::{ContractAbi, ServiceAbi};

pub struct ProxyAbi;

impl ContractAbi for ProxyAbi {
    type Operation = ();
    type Response = ();
}

impl ServiceAbi for ProxyAbi {
    type Query = ();
    type QueryResponse = ();
}
