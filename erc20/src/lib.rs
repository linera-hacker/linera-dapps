use async_graphql::{Request, Response};
use linera_sdk::base::{ContractAbi, ParseAmountError, ServiceAbi};
use spec::erc20::{ERC20Operation, ERC20Response};
use thiserror::Error;

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
pub enum ERC20Error {
    #[error("Invalid initial amount")]
    InvalidInitialAmount,

    #[error(transparent)]
    ParseAmountError(#[from] ParseAmountError),

    #[error(transparent)]
    ViewError(#[from] linera_sdk::views::ViewError),

    #[error("Already exists")]
    AlreadyExists,

    #[error("Permission denied")]
    PermissionDenied,
}
