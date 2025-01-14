use async_graphql::SimpleObject;
use linera_sdk::base::{ContractAbi, ServiceAbi, Owner, BytecodeId};
use serde::{Deserialize, Serialize};

pub struct ProxyAbi;

impl ContractAbi for ProxyAbi {
    type Operation = ();
    type Response = ();
}

impl ServiceAbi for ProxyAbi {
    type Query = ();
    type QueryResponse = ();
}

#[derive(Clone, Debug, Deserialize, Serialize, SimpleObject)]
pub struct InstantiateArgument {
    operator: Option<Owner>,
    bytecode_id: BytecodeId,
    validators: Vec<Owner>,
}
