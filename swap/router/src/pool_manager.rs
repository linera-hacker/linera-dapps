use linera_sdk::base::ParseAmountError;
use linera_sdk::{Contract, ContractRuntime};
use spec::swap::{
    abi::SwapApplicationState,
    pool::{PoolMessage, PoolOperation, PoolResponse},
};
use thiserror::Error;

pub struct PoolManager<T: Contract> {
    state: SwapApplicationState,
    runtime: ContractRuntime<T>,
}

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

impl<T: Contract> PoolManager<T> {
    pub async fn new(runtime: ContractRuntime<T>, state: SwapApplicationState) -> Self {
        PoolManager { state, runtime }
    }

    pub async fn execute_operation(&mut self, operation: PoolOperation) -> PoolResponse {
        self.execute_pool_operation(operation)
            .await
            .expect("Fail OP: pool")
    }

    pub async fn execute_message(&mut self, message: PoolMessage) {
        self.execute_pool_message(message)
            .await
            .expect("Fail MSG: pool")
    }

    async fn execute_pool_operation(
        &mut self,
        operation: PoolOperation,
    ) -> Result<PoolResponse, PoolError> {
        Ok(PoolResponse::Ok)
    }

    async fn execute_pool_message(&mut self, message: PoolMessage) -> Result<(), PoolError> {
        Ok(())
    }
}
