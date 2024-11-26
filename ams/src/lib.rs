use spec::ams::AMSApplicationAbi;
use thiserror::Error;

pub type ApplicationAbi = AMSApplicationAbi;

#[derive(Debug, Error)]
#[allow(dead_code)]
pub enum AMSError {
    #[error(transparent)]
    ViewError(#[from] linera_sdk::views::ViewError),

    #[error("Already exists")]
    AlreadyExists,

    #[error("Permission denied")]
    PermissionDenied,

    #[error("Not implemented")]
    NotImplemented,
}
