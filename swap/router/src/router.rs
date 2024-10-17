use linera_sdk::{
    base::{Amount, ApplicationId, Timestamp},
    Contract, ContractRuntime,
};
use spec::{
    swap::{
    router::{RouterMessage, RouterOperation, RouterResponse},
    state::{SwapApplicationState, StateError},
    pool::{Pool, PoolError},
},
account::ChainAccountOwner,
};
use std::sync::Arc;
use thiserror::Error;
use crate::runtime::runtime_owner;

#[derive(Debug, Error)]
#[allow(dead_code)]
pub enum RouterError {
    #[error(transparent)]
    StateError(#[from] StateError),

    #[error(transparent)]
    PoolError(#[from] PoolError),
}

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
    ) -> Result<(RouterResponse, Option<(RouterMessage, bool)>), RouterError> {
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
    ) -> Result<(RouterResponse, Option<(RouterMessage, bool)>), RouterError> {
        match operation {
            RouterOperation::AddLiquidity {
                token_0,
                token_1,
                amount_0_desired,
                amount_1_desired,
                amount_0_min,
                amount_1_min,
                to,
                deadline,
            } => self.on_op_add_liquidity(
                runtime,
                state,
                token_0,
                token_1,
                amount_0_desired,
                amount_1_desired,
                amount_0_min,
                amount_1_min,
                to,
                deadline,
            ).await,
            _ => todo!(),
        }
    }

    async fn execute_router_message<T: Contract>(
        &mut self,
        runtime: &mut ContractRuntime<T>,
        state: &mut SwapApplicationState,
        message: RouterMessage,
    ) -> Result<(), RouterError> {
        Ok(())
    }

    async fn on_op_add_liquidity<T: Contract>(
        &mut self,
        runtime: &mut ContractRuntime<T>,
        state: &mut SwapApplicationState,
        token_0: ApplicationId,
        token_1: Option<ApplicationId>,
        amount_0_desired: Amount,
        amount_1_desired: Amount,
        amount_0_min: Amount,
        amount_1_min: Amount,
        to: Option<ChainAccountOwner>,
        deadline: Timestamp,
    ) -> Result<(RouterResponse, Option<(RouterMessage, bool)>), RouterError> {
        let (pool, exchanged) = state.get_pool_exchangable(token_0, token_1).await?;

        let token_0 = if exchanged { token_1.unwrap() } else { token_0 };
        let token_1 = if exchanged { Some(token_0) } else { token_1 };
        let amount_0_desired = if exchanged {
            amount_1_desired
        } else {
            amount_0_desired
        };
        let amount_1_desired = if exchanged {
            amount_0_desired
        } else {
            amount_1_desired
        };
        let amount_0_min = if exchanged {
            amount_1_min
        } else {
            amount_0_min
        };
        let amount_1_min = if exchanged {
            amount_0_min
        } else {
            amount_1_min
        };

        let (amount_0, amount_1) = if let Some(_pool) = pool.clone() {
            _pool.calculate_swap_amount_pair(amount_0_desired, amount_1_desired, amount_0_min, amount_1_min)?
        } else {
            (amount_0_desired, amount_1_desired)
        };

        let liquidity = if let Some(mut _pool) = pool {
            _pool.reserve_0.saturating_add_assign(amount_0);
            _pool.reserve_0.saturating_add_assign(amount_1);
            _pool.calculate_liquidity(amount_0, amount_1)
        } else {
            Pool::calculate_initial_liquidity(amount_0, amount_1)
        };

        Ok((
                RouterResponse::Liquidity((amount_0, amount_1, liquidity)),
                Some((RouterMessage::AddLiquidity {
                    origin: runtime_owner(runtime),
                    token_0,
                    token_1,
                    amount_0_desired,
                    amount_1_desired,
                    amount_0_min,
                    amount_1_min,
                    to: runtime_owner(runtime),
                    deadline,
                },
                true,
            )),
        ))
    }
}
