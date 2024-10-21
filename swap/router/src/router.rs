use crate::runtime::{
    receive_erc20_from_runtime_owner_to_application_creation,
    receive_token_from_runtime_owner_to_application_creation, runtime_owner,
    subscribe_erc20_application_creation, transfer_erc20,
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

    #[error("Invoke later")]
    InvokeLater,

    #[error("Invalid liquidity")]
    InvalidLiquidity,

    #[error("Invalid native token")]
    InvalidNativeToken,
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
            RouterOperation::Swap {
                token_0,
                token_1,
                amount_0_in,
                amount_1_in,
                amount_0_out_min,
                amount_1_out_min,
                to,
            } => {
                self.on_op_swap(
                    runtime,
                    state,
                    token_0,
                    token_1,
                    amount_0_in,
                    amount_1_in,
                    amount_0_out_min,
                    amount_1_out_min,
                    to,
                )
                .await
            }
            RouterOperation::CalculateSwapAmount {
                token_0,
                token_1,
                amount_1,
            } => {
                self.on_op_calculate_swap_amount(runtime, state, token_0, token_1, amount_1)
                    .await
            }
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
                block_timestamp,
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
                    block_timestamp,
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
                block_timestamp,
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
                    block_timestamp,
                )
                .await
            }
            RouterMessage::Swap {
                origin,
                token_0,
                token_1,
                amount_0_in,
                amount_1_in,
                amount_0_out_min,
                amount_1_out_min,
                to,
                block_timestamp,
            } => {
                self.on_msg_swap(
                    runtime,
                    state,
                    origin,
                    token_0,
                    token_1,
                    amount_0_in,
                    amount_1_in,
                    amount_0_out_min,
                    amount_1_out_min,
                    to,
                    block_timestamp,
                )
                .await
            }
            RouterMessage::SubscribeNewERC20Token { origin, token } => {
                self.on_msg_subscribe_new_erc20_token(runtime, state, origin, token)
            }
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

        let origin = runtime_owner(runtime);
        let to = to.unwrap_or(origin);

        Ok((
            RouterResponse::Liquidity((amount_0, amount_1, liquidity)),
            Some((
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
                    block_timestamp: runtime.system_time(),
                },
                true,
            )),
        ))
    }

    async fn mint_shares(
        &mut self,
        state: &mut SwapApplicationState,
        pool: Pool,
        liquidity: Amount,
        amount_0: Amount,
        amount_1: Amount,
        to: ChainAccountOwner,
        block_timestamp: Timestamp,
    ) -> Result<(), RouterError> {
        let balance_0 = pool.reserve_0.saturating_add(amount_0);
        let balance_1 = pool.reserve_0.saturating_add(amount_1);

        state.mint_fee(pool.id).await?;
        state.mint(pool.id, liquidity, to.clone()).await?;
        Ok(state
            .update(pool.id, balance_0, balance_1, block_timestamp)
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
        block_timestamp: Timestamp,
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
                        block_timestamp,
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
        self.mint_shares(
            state,
            pool,
            liquidity,
            amount_0,
            amount_1,
            to,
            block_timestamp,
        )
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
                block_timestamp,
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
                    block_timestamp: runtime.system_time(),
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
        block_timestamp: Timestamp,
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
            .update(pool.id, balance_0, balance_1, block_timestamp)
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
                block_timestamp,
            },
            true,
        )))
    }

    async fn on_op_swap<T: Contract>(
        &mut self,
        runtime: &mut ContractRuntime<T>,
        state: &mut SwapApplicationState,
        token_0: ApplicationId,
        token_1: Option<ApplicationId>,
        amount_0_in: Option<Amount>,
        amount_1_in: Option<Amount>,
        amount_0_out_min: Option<Amount>,
        amount_1_out_min: Option<Amount>,
        to: Option<ChainAccountOwner>,
    ) -> Result<(RouterResponse, Option<(RouterMessage, bool)>), RouterError> {
        let (pool, exchanged) = state.get_pool_exchangable(token_0, token_1).await?;
        let Some(pool) = pool else {
            return Err(RouterError::InvalidPool);
        };

        let token_0 = if exchanged { token_1.unwrap() } else { token_0 };
        let token_1 = if exchanged { Some(token_0) } else { token_1 };
        let amount_0_in = if exchanged { amount_1_in } else { amount_0_in };
        let amount_1_in = if exchanged { amount_0_in } else { amount_1_in };
        let amount_0_out_min = if exchanged {
            amount_1_out_min
        } else {
            amount_0_out_min
        };
        let amount_1_out_min = if exchanged {
            amount_0_out_min
        } else {
            amount_1_out_min
        };

        if let Some(_amount_0_out_min) = amount_0_out_min {
            if let Some(_amount_1_in) = amount_1_in {
                if pool.calculate_swap_amount_0(_amount_1_in)? < _amount_0_out_min {
                    return Err(RouterError::InvalidAmount);
                }
            }
        }
        if let Some(_amount_1_out_min) = amount_1_out_min {
            if let Some(_amount_0_in) = amount_0_in {
                if pool.calculate_swap_amount_1(_amount_0_in)? < _amount_1_out_min {
                    return Err(RouterError::InvalidAmount);
                }
            }
        }

        let origin = runtime_owner(runtime);
        let to = to.unwrap_or(origin);

        Ok((
            RouterResponse::Ok,
            Some((
                RouterMessage::Swap {
                    origin,
                    token_0,
                    token_1,
                    amount_0_in,
                    amount_1_in,
                    amount_0_out_min,
                    amount_1_out_min,
                    to,
                    block_timestamp: runtime.system_time(),
                },
                true,
            )),
        ))
    }

    async fn on_msg_swap<T: Contract>(
        &mut self,
        runtime: &mut ContractRuntime<T>,
        state: &mut SwapApplicationState,
        origin: ChainAccountOwner,
        token_0: ApplicationId,
        token_1: Option<ApplicationId>,
        amount_0_in: Option<Amount>,
        amount_1_in: Option<Amount>,
        amount_0_out_min: Option<Amount>,
        amount_1_out_min: Option<Amount>,
        to: ChainAccountOwner,
        block_timestamp: Timestamp,
    ) -> Result<Option<(RouterMessage, bool)>, RouterError> {
        let pool = state
            .get_pool_with_token_pair(token_0, token_1)
            .await?
            .expect("Invalid pool");

        let mut amount_0_out = Amount::ZERO;
        let mut amount_1_out = Amount::ZERO;

        if let Some(_amount_0_out_min) = amount_0_out_min {
            if let Some(_amount_1_in) = amount_1_in {
                amount_0_out = pool.calculate_swap_amount_0(_amount_1_in)?;
                if amount_0_out < _amount_0_out_min {
                    return Err(RouterError::InvalidAmount);
                }
                if amount_0_out == Amount::ZERO {
                    return Err(RouterError::InvalidAmount);
                }
            }
        }
        if let Some(_amount_1_out_min) = amount_1_out_min {
            if let Some(_amount_0_in) = amount_0_in {
                amount_1_out = pool.calculate_swap_amount_1(_amount_0_in)?;
                if amount_1_out < _amount_1_out_min {
                    return Err(RouterError::InvalidAmount);
                }
                if amount_1_out == Amount::ZERO {
                    return Err(RouterError::InvalidAmount);
                }
            }
        }

        let _token_1 = match token_1 {
            Some(__token_1) => __token_1,
            _ => match *state.wlinera_application_id.get() {
                Some(__token_1) => __token_1,
                _ => return Err(RouterError::InvalidPool),
            },
        };

        if origin.chain_id == runtime.chain_id() {
            if amount_0_out > Amount::ZERO {
                receive_erc20_from_runtime_owner_to_application_creation(
                    runtime,
                    _token_1,
                    amount_1_in.unwrap(),
                );
            }
            if amount_1_out > Amount::ZERO {
                receive_erc20_from_runtime_owner_to_application_creation(
                    runtime,
                    token_0,
                    amount_0_in.unwrap(),
                );
            }
        }

        if runtime.application_creator_chain_id() == runtime.chain_id() {
            if amount_0_out > Amount::ZERO {
                transfer_erc20(runtime, token_0, amount_0_out, to);
            }
            if amount_1_out > Amount::ZERO {
                transfer_erc20(runtime, _token_1, amount_1_out, to);
            }
        }

        let _ = pool.calculate_adjust_amount_pair(amount_0_out, amount_1_out)?;

        let balance_0 = pool.reserve_0.saturating_sub(amount_0_out);
        let balance_1 = pool.reserve_0.saturating_sub(amount_1_out);

        state
            .update(pool.id, balance_0, balance_1, block_timestamp)
            .await?;

        Ok(Some((
            RouterMessage::Swap {
                origin,
                token_0,
                token_1,
                amount_0_in,
                amount_1_in,
                amount_0_out_min,
                amount_1_out_min,
                to,
                block_timestamp,
            },
            true,
        )))
    }

    async fn on_op_calculate_swap_amount<T: Contract>(
        &mut self,
        runtime: &mut ContractRuntime<T>,
        state: &mut SwapApplicationState,
        token_0: ApplicationId,
        token_1: Option<ApplicationId>,
        amount_1: Amount,
    ) -> Result<(RouterResponse, Option<(RouterMessage, bool)>), RouterError> {
        Ok((
            RouterResponse::Amount(
                calculate_swap_amount(state, token_0, token_1, amount_1, runtime.system_time())
                    .await?,
            ),
            None,
        ))
    }

    fn on_msg_subscribe_new_erc20_token<T: Contract>(
        &mut self,
        runtime: &mut ContractRuntime<T>,
        _state: &mut SwapApplicationState,
        _origin: ChainAccountOwner,
        token: ApplicationId,
    ) -> Result<Option<(RouterMessage, bool)>, RouterError> {
        // Must not on creation chain, return None
        subscribe_erc20_application_creation(runtime, token);
        Ok(None)
    }
}

pub async fn calculate_swap_amount(
    state: &SwapApplicationState,
    token_0: ApplicationId,
    token_1: Option<ApplicationId>,
    amount_1: Amount,
    block_timestamp: Timestamp,
) -> Result<Amount, RouterError> {
    let token_1 = match token_1 {
        Some(__token_1) => __token_1,
        _ => match *state.wlinera_application_id.get() {
            Some(__token_1) => __token_1,
            _ => return Err(RouterError::InvalidNativeToken),
        },
    };
    let (pool, exchanged) = state.get_pool_exchangable(token_0, Some(token_1)).await?;
    let Some(pool) = pool else {
        return Err(RouterError::InvalidPool);
    };
    let time_elapsed = u128::from(
        block_timestamp
            .delta_since(pool.block_timestamp)
            .as_micros(),
    );
    if time_elapsed == 0 {
        return Err(RouterError::InvokeLater);
    }
    if pool.reserve_0 == Amount::ZERO || pool.reserve_1 == Amount::ZERO {
        return Err(RouterError::InvalidLiquidity);
    }
    let (price_0_cumulative, price_1_cumulative) =
        pool.calculate_price_cumulative_pair(time_elapsed);
    let (price_0_cumulative, _price_1_cumulative) = if exchanged {
        (price_1_cumulative, price_0_cumulative)
    } else {
        (price_0_cumulative, price_1_cumulative)
    };
    let amount_0: Amount = price_0_cumulative
        .sub(pool.price_0_cumulative)
        .mul(amount_1.into())
        .div(time_elapsed.into())
        .into();
    Ok(amount_0)
}
