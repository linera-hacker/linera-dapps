use linera_sdk::base::ParseAmountError;
use spec::erc20::ERC20ApplicationAbi;
use thiserror::Error;

pub type ApplicationAbi = ERC20ApplicationAbi;

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
