use async_graphql::SimpleObject;
use linera_sdk::{
    base::{BytecodeId, ContractAbi, Owner, ServiceAbi},
    graphql::GraphQLMutationRoot,
};
use serde::{Deserialize, Serialize};
use spec::erc20::{ERC20Parameters, InstantiationArgument as ERC20InstantiationArgument};

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
pub struct InstantiationArgument {
    operator: Option<Owner>,
    bytecode_id: BytecodeId,
    validators: Vec<Owner>,
}

#[derive(Debug, Deserialize, Serialize, GraphQLMutationRoot)]
pub enum Operation {
    SetOperator {
        owner: Owner,
    },
    SetBytecodeId {
        bytecode_id: BytecodeId,
    },
    RegisterValidator {
        owner: Owner,
    },
    DeregisterValidator {
        owner: Owner,
    },
    CreateApplication {
        instantiation_argument: ERC20InstantiationArgument,
        parameters: ERC20Parameters,
    },
}

#[derive(Debug, Deserialize, Serialize)]
pub enum Message {
    SetOperator {
        owner: Owner,
    },
    SetBytecodeId {
        bytecode_id: BytecodeId,
    },
    RegisterValidator {
        owner: Owner,
    },
    DeregisterValidator {
        owner: Owner,
    },
    CreateApplication {
        instantiation_argument: ERC20InstantiationArgument,
        parameters: ERC20Parameters,
    },
}
