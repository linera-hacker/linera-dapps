use linera_sdk::base::ParseAmountError;
use spec::swap::PoolApplicationAbi;
use thiserror::Error;

pub type ApplicationAbi = PoolApplicationAbi;

#[derive(Debug, Error)]
#[allow(dead_code)]
pub enum PoolError {
    #[error("Invalid initial amount")]
    InvalidInitialAmount,

    #[error(transparent)]
    ParseAmountError(#[from] ParseAmountError),

    #[error(transparent)]
    ViewError(#[from] linera_sdk::views::ViewError),

    #[error("Already exists")]
    AlreadyExists,

    #[error("Invalid pool")]
    InvalidPool,

    #[error("Permission denied")]
    PermissionDenied,

    #[error("Not supported")]
    NotSupported,

    #[error("Insufficient funds")]
    InsufficientFunds,

    #[error("Invalid amount")]
    InvalidAmount,

    #[error("Insufficient liquidity")]
    InsufficientLiquidity,

    #[error("Broken K")]
    BrokenK,
}
