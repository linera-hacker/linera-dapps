use crate::runtime::{
    balance_of_erc20_of_runtime_application_creation,
    receive_erc20_from_runtime_owner_to_application_creation,
    receive_token_from_runtime_owner_to_application_creation, runtime_owner, transfer_erc20,
};
use linera_sdk::{
    base::{Amount, ApplicationId, Timestamp},
    Contract, ContractRuntime,
};
use spec::{
    account::ChainAccountOwner,
    swap::{
        pool::{Pool, PoolError},
        router::{RouterMessage, RouterOperation, RouterResponse},
        state::{StateError, SwapApplicationState},
    },
};
use std::sync::Arc;
use thiserror::Error;

#[derive(Debug, Error)]
#[allow(dead_code)]
pub enum RouterError {
    #[error(transparent)]
    StateError(#[from] StateError),

    #[error(transparent)]
    PoolError(#[from] PoolError),

    #[error("Invalid amount")]
    InvalidAmount,

    #[error("Invalid pool")]
    InvalidPool,
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
    ) -> Result<Option<(RouterMessage, bool)>, RouterError> {
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
            } => {
                self.on_op_add_liquidity(
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
                )
                .await
            }
            RouterOperation::RemoveLiquidity {
                token_0,
                token_1,
                liquidity,
                amount_0_min,
                amount_1_min,
                to,
                deadline,
            } => {
                self.on_op_remove_liquidity(
                    runtime,
                    state,
                    token_0,
                    token_1,
                    liquidity,
                    amount_0_min,
                    amount_1_min,
                    to,
                    deadline,
                )
                .await
            }
            _ => todo!(),
        }
    }

    async fn execute_router_message<T: Contract>(
        &mut self,
        runtime: &mut ContractRuntime<T>,
        state: &mut SwapApplicationState,
        message: RouterMessage,
    ) -> Result<Option<(RouterMessage, bool)>, RouterError> {
        match message {
            RouterMessage::AddLiquidity {
                origin,
                token_0,
                token_1,
                amount_0_desired,
                amount_1_desired,
                amount_0_min,
                amount_1_min,
                to,
                deadline,
            } => {
                self.on_msg_add_liquidity(
                    runtime,
                    state,
                    origin,
                    token_0,
                    token_1,
                    amount_0_desired,
                    amount_1_desired,
                    amount_0_min,
                    amount_1_min,
                    to,
                    deadline,
                )
                .await
            }
            RouterMessage::RemoveLiquidity {
                origin,
                token_0,
                token_1,
                liquidity,
                amount_0_min,
                amount_1_min,
                to,
                deadline,
            } => {
                self.on_msg_remove_liquidity(
                    runtime,
                    state,
                    origin,
                    token_0,
                    token_1,
                    liquidity,
                    amount_0_min,
                    amount_1_min,
                    to,
                    deadline,
                )
                .await
            }
            _ => todo!(),
        }
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
            _pool.calculate_swap_amount_pair(
                amount_0_desired,
                amount_1_desired,
                amount_0_min,
                amount_1_min,
            )?
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
            Some((
                RouterMessage::AddLiquidity {
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

    async fn mint_shares<T: Contract>(
        &mut self,
        runtime: &mut ContractRuntime<T>,
        state: &mut SwapApplicationState,
        pool: Pool,
        amount_0: Amount,
        amount_1: Amount,
        to: ChainAccountOwner,
    ) -> Result<(), RouterError> {
        let balance_0 = pool.reserve_0.saturating_add(amount_0);
        let balance_1 = pool.reserve_0.saturating_add(amount_1);

        state.mint_fee(pool.id).await?;
        let liquidity = pool.calculate_liquidity(amount_0, amount_1);
        state.mint(pool.id, liquidity, to.clone()).await?;
        Ok(state
            .update(pool.id, balance_0, balance_1, runtime.system_time())
            .await?)
    }

    async fn on_msg_add_liquidity<T: Contract>(
        &mut self,
        runtime: &mut ContractRuntime<T>,
        state: &mut SwapApplicationState,
        origin: ChainAccountOwner,
        token_0: ApplicationId,
        token_1: Option<ApplicationId>,
        amount_0_desired: Amount,
        amount_1_desired: Amount,
        amount_0_min: Amount,
        amount_1_min: Amount,
        to: ChainAccountOwner,
        deadline: Timestamp,
    ) -> Result<Option<(RouterMessage, bool)>, RouterError> {
        let (pool, exchanged) = state.get_pool_exchangable(token_0, token_1).await?;
        let (pool, created) = match pool {
            Some(_pool) => (_pool, false),
            None => (
                state
                    .create_pool(
                        token_0,
                        token_1,
                        amount_0_desired,
                        amount_1_desired,
                        amount_0_desired,
                        amount_1_desired,
                        runtime_owner(runtime),
                        runtime.system_time(),
                    )
                    .await?,
                true,
            ),
        };
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

        let (amount_0, amount_1) = if created {
            (amount_0_desired, amount_1_desired)
        } else {
            pool.calculate_swap_amount_pair(
                amount_0_desired,
                amount_1_desired,
                amount_0_min,
                amount_1_min,
            )?
        };

        let liquidity = if created {
            Pool::calculate_initial_liquidity(amount_0, amount_1)
        } else {
            let mut _pool = pool.clone();
            _pool.reserve_0.saturating_add_assign(amount_0);
            _pool.reserve_0.saturating_add_assign(amount_1);
            _pool.calculate_liquidity(amount_0, amount_1)
        };

        if origin.chain_id == runtime.chain_id() {
            if amount_0 > Amount::ZERO {
                receive_erc20_from_runtime_owner_to_application_creation(
                    runtime, token_0, amount_0,
                );
            }
            if amount_1 > Amount::ZERO {
                receive_token_from_runtime_owner_to_application_creation(
                    runtime, token_1, amount_1,
                );
            }
        }
        self.mint_shares(runtime, state, pool, amount_0, amount_1, to)
            .await?;

        Ok(Some((
            RouterMessage::AddLiquidity {
                origin,
                token_0,
                token_1,
                amount_0_desired,
                amount_1_desired,
                amount_0_min,
                amount_1_min,
                to,
                deadline,
            },
            true,
        )))
    }

    async fn on_op_remove_liquidity<T: Contract>(
        &mut self,
        runtime: &mut ContractRuntime<T>,
        state: &mut SwapApplicationState,
        token_0: ApplicationId,
        token_1: Option<ApplicationId>,
        liquidity: Amount,
        amount_0_min: Amount,
        amount_1_min: Amount,
        to: Option<ChainAccountOwner>,
        deadline: Timestamp,
    ) -> Result<(RouterResponse, Option<(RouterMessage, bool)>), RouterError> {
        let (pool, exchanged) = state.get_pool_exchangable(token_0, token_1).await?;
        let Some(pool) = pool else {
            return Err(RouterError::InvalidPool);
        };

        let token_0 = if exchanged { token_1.unwrap() } else { token_0 };
        let token_1 = if exchanged { Some(token_0) } else { token_1 };
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

        // TODO: after we can lock native tokens to application, it should use native token
        // directly
        let _token_1 = match token_1 {
            Some(__token_1) => __token_1,
            _ => match *state.wlinera_application_id.get() {
                Some(__token_1) => __token_1,
                _ => return Err(RouterError::InvalidPool),
            },
        };

        // TODO: may need to check balance here

        let (amount_0, amount_1) =
            pool.calculate_liquidity_amount_pair(liquidity, pool.reserve_0, pool.reserve_1)?;
        if amount_0 < amount_0_min || amount_1 < amount_1_min {
            return Err(RouterError::InvalidAmount);
        }

        let origin = runtime_owner(runtime);
        let to = to.unwrap_or(origin);

        Ok((
            RouterResponse::AmountPair((amount_0, amount_1)),
            Some((
                RouterMessage::RemoveLiquidity {
                    origin,
                    token_0,
                    token_1,
                    liquidity,
                    amount_0_min,
                    amount_1_min,
                    to,
                    deadline,
                },
                true,
            )),
        ))
    }

    async fn on_msg_remove_liquidity<T: Contract>(
        &mut self,
        runtime: &mut ContractRuntime<T>,
        state: &mut SwapApplicationState,
        origin: ChainAccountOwner,
        token_0: ApplicationId,
        token_1: Option<ApplicationId>,
        liquidity: Amount,
        amount_0_min: Amount,
        amount_1_min: Amount,
        to: ChainAccountOwner,
        deadline: Timestamp,
    ) -> Result<Option<(RouterMessage, bool)>, RouterError> {
        let pool = state
            .get_pool_with_token_pair(token_0, token_1)
            .await?
            .expect("Invalid pool");

        let _token_1 = match token_1 {
            Some(__token_1) => __token_1,
            _ => match *state.wlinera_application_id.get() {
                Some(__token_1) => __token_1,
                _ => return Err(RouterError::InvalidPool),
            },
        };

        let (amount_0, amount_1) =
            pool.calculate_liquidity_amount_pair(liquidity, pool.reserve_0, pool.reserve_1)?;
        if amount_0 < amount_0_min || amount_1 < amount_1_min {
            return Err(RouterError::InvalidAmount);
        }

        // The assets is stored on application creation chain, so we can only transfered there
        if runtime.application_creator_chain_id() == runtime.chain_id() {
            transfer_erc20(runtime, token_0, amount_0, to);
            transfer_erc20(runtime, _token_1, amount_1, to);
        }

        state.burn(pool.id, liquidity, to).await?;

        let balance_0 = pool.reserve_0.saturating_sub(amount_0);
        let balance_1 = pool.reserve_0.saturating_sub(amount_1);

        state
            .update(pool.id, balance_0, balance_1, runtime.system_time())
            .await?;

        Ok(Some((
            RouterMessage::RemoveLiquidity {
                origin,
                token_0,
                token_1,
                liquidity,
                amount_0_min,
                amount_1_min,
                to,
                deadline,
            },
            true,
        )))
    }
}
