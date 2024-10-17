use linera_sdk::base::ParseAmountError;
use linera_sdk::{Contract, ContractRuntime};
use spec::swap::{
    pool::{PoolMessage, PoolOperation, PoolResponse},
    state::SwapApplicationState,
};
use thiserror::Error;

pub struct PoolManager {}

#[derive(Debug, Error)]
#[allow(dead_code)]
pub enum PoolError {}

impl PoolManager {
    pub async fn new() -> Self {
        PoolManager {}
    }

    pub async fn execute_operation<T: Contract>(
        &mut self,
        runtime: ContractRuntime<T>,
        state: SwapApplicationState,
        operation: PoolOperation,
    ) -> PoolResponse {
        self.execute_pool_operation(runtime, state, operation)
            .await
            .expect("Fail OP: pool")
    }

    pub async fn execute_message<T: Contract>(
        &mut self,
        runtime: ContractRuntime<T>,
        state: SwapApplicationState,
        message: PoolMessage,
    ) {
        self.execute_pool_message(runtime, state, message)
            .await
            .expect("Fail MSG: pool")
    }

    async fn execute_pool_operation<T: Contract>(
        &mut self,
        runtime: ContractRuntime<T>,
        state: SwapApplicationState,
        operation: PoolOperation,
    ) -> Result<PoolResponse, PoolError> {
        Ok(PoolResponse::Ok)
    }

    async fn execute_pool_message<T: Contract>(
        &mut self,
        runtime: ContractRuntime<T>,
        state: SwapApplicationState,
        message: PoolMessage,
    ) -> Result<(), PoolError> {
        Ok(())
    }
}
