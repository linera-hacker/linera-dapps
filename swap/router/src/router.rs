use linera_sdk::{Contract, ContractRuntime};
use spec::swap::{
    router::{RouterMessage, RouterOperation, RouterResponse},
    state::SwapApplicationState,
};
use std::sync::Arc;
use thiserror::Error;

#[derive(Debug, Error)]
#[allow(dead_code)]
pub enum RouterError {}

pub struct Router {}

impl Router {
    pub async fn new() -> Self {
        Router {}
    }

    pub async fn execute_operation<T: Contract>(
        &mut self,
        runtime: &mut ContractRuntime<T>,
        state: &mut SwapApplicationState,
        operation: RouterOperation,
    ) -> Result<RouterResponse, RouterError> {
        self.execute_router_operation(runtime, state, operation)
            .await
    }

    pub async fn execute_message<T: Contract>(
        &mut self,
        runtime: &mut ContractRuntime<T>,
        state: &mut SwapApplicationState,
        message: RouterMessage,
    ) -> Result<(), RouterError> {
        self.execute_router_message(runtime, state, message).await
    }

    async fn execute_router_operation<T: Contract>(
        &mut self,
        runtime: &mut ContractRuntime<T>,
        state: &mut SwapApplicationState,
        operation: RouterOperation,
    ) -> Result<RouterResponse, RouterError> {
        Ok(RouterResponse::Ok)
    }

    async fn execute_router_message<T: Contract>(
        &mut self,
        runtime: &mut ContractRuntime<T>,
        state: &mut SwapApplicationState,
        message: RouterMessage,
    ) -> Result<(), RouterError> {
        Ok(())
    }
}
