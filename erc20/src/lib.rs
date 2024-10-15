use async_graphql::{InputObject, SimpleObject};
use linera_sdk::base::ParseAmountError;
use serde::{Deserialize, Serialize};
use spec::{account::ChainAccountOwner, erc20::ERC20ApplicationAbi};
use thiserror::Error;

pub type ApplicationAbi = ERC20ApplicationAbi;

#[derive(Serialize, Deserialize, Debug, Clone, SimpleObject, InputObject)]
pub struct AllowanceKey {
    pub owner: ChainAccountOwner,
    pub spender: ChainAccountOwner,
}

impl AllowanceKey {
    pub fn new(owner: ChainAccountOwner, spender: ChainAccountOwner) -> Self {
        Self { owner, spender }
    }
}

#[derive(Debug, Error)]
#[allow(dead_code)]
pub enum ERC20Error {
    #[error("Insufficient funds")]
    InsufficientFunds,

    #[error(transparent)]
    ParseAmountError(#[from] ParseAmountError),

    #[error(transparent)]
    ViewError(#[from] linera_sdk::views::ViewError),

    #[error("Already exists")]
    AlreadyExists,

    #[error("Permission denied")]
    PermissionDenied,

    #[error("Not supported")]
    NotSupported,
}
