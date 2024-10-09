use async_graphql::{Request, Response};
use linera_sdk::base::{ContractAbi, ServiceAbi};
use thiserror::Error;
use spec::erc20::{ERC20Operation, ERC20Response};

pub struct ApplicationAbi;

impl ContractAbi for ApplicationAbi {
    type Operation = ERC20Operation;
    type Response = ERC20Response;
}

impl ServiceAbi for ApplicationAbi {
    type Query = Request;
    type QueryResponse = Response;
}

#[derive(Debug, Error)]
#[allow(dead_code)]
pub enum ERC20Error {}
