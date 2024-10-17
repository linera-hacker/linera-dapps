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
pub enum PoolError {}

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
