use crate::runtime::{
    erc20_application_owner, receive_erc20_from_runtime_owner_to_application_creation,
    receive_token_from_runtime_owner_to_application_creation, runtime_owner,
    subscribe_erc20_application_creation,
};
use linera_sdk::{
    base::{Amount, ApplicationId, Timestamp},
    Contract, ContractRuntime,
};
use spec::{
    account::ChainAccountOwner,
    swap::{
        pool::{Pool, PoolMessage, PoolOperation, PoolResponse},
        state::{StateError, SwapApplicationState},
    },
};
use thiserror::Error;

pub struct PoolManager {}

#[derive(Debug, Error)]
#[allow(dead_code)]
pub enum PoolError {
    #[error(transparent)]
    StateError(#[from] StateError),

    #[error("Permission denied")]
    PermissionDenied,

    #[error("Invalid pool")]
    InvalidPool,
}

impl PoolManager {
    pub async fn new() -> Self {
        PoolManager {}
    }

    pub async fn execute_operation<T: Contract>(
        &mut self,
        runtime: &mut ContractRuntime<T>,
        state: &mut SwapApplicationState,
        operation: PoolOperation,
    ) -> Result<(PoolResponse, Option<(PoolMessage, bool)>), PoolError> {
        self.execute_pool_operation(runtime, state, operation).await
    }

    pub async fn execute_message<T: Contract>(
        &mut self,
        runtime: &mut ContractRuntime<T>,
        state: &mut SwapApplicationState,
        message: PoolMessage,
    ) -> Result<Option<(PoolMessage, bool)>, PoolError> {
        self.execute_pool_message(runtime, state, message).await
    }

    async fn execute_pool_operation<T: Contract>(
        &mut self,
        runtime: &mut ContractRuntime<T>,
        state: &mut SwapApplicationState,
        operation: PoolOperation,
    ) -> Result<(PoolResponse, Option<(PoolMessage, bool)>), PoolError> {
        match operation {
            PoolOperation::CreatePool {
                token_0,
                token_1,
                amount_0_initial,
                amount_1_initial,
                amount_0_virtual,
                amount_1_virtual,
            } => {
                self.on_op_create_pool(
                    runtime,
                    state,
                    token_0,
                    token_1,
                    amount_0_initial,
                    amount_1_initial,
                    amount_0_virtual,
                    amount_1_virtual,
                )
                .await
            }
            PoolOperation::SetFeeTo { pool_id, account } => {
                self.on_op_set_fee_to(runtime, state, pool_id, account)
                    .await
            }
            PoolOperation::SetFeeToSetter { pool_id, account } => {
                self.on_op_set_fee_to_setter(runtime, state, pool_id, account)
                    .await
            }
        }
    }

    async fn execute_pool_message<T: Contract>(
        &mut self,
        runtime: &mut ContractRuntime<T>,
        state: &mut SwapApplicationState,
        message: PoolMessage,
    ) -> Result<Option<(PoolMessage, bool)>, PoolError> {
        match message {
            PoolMessage::CreatePool {
                origin,
                token_0,
                token_1,
                amount_0_initial,
                amount_1_initial,
                amount_0_virtual,
                amount_1_virtual,
                block_timestamp,
            } => {
                self.on_msg_create_pool(
                    runtime,
                    state,
                    origin,
                    token_0,
                    token_1,
                    amount_0_initial,
                    amount_1_initial,
                    amount_0_virtual,
                    amount_1_virtual,
                    block_timestamp,
                )
                .await
            }
            PoolMessage::SetFeeTo {
                origin,
                pool_id,
                account,
            } => {
                self.on_msg_set_fee_to(runtime, state, origin, pool_id, account)
                    .await
            }
            PoolMessage::SetFeeToSetter {
                origin,
                pool_id,
                account,
            } => {
                self.on_msg_set_fee_to_setter(runtime, state, origin, pool_id, account)
                    .await
            }
        }
    }

    async fn mint_shares(
        &mut self,
        state: &mut SwapApplicationState,
        pool: Pool,
        amount_0: Amount,
        amount_1: Amount,
        to: ChainAccountOwner,
        block_timestamp: Timestamp,
    ) -> Result<(), PoolError> {
        let balance_0 = pool.reserve_0.saturating_add(amount_0);
        let balance_1 = pool.reserve_0.saturating_add(amount_1);

        state.mint_fee(pool.id).await?;
        let liquidity = pool.calculate_liquidity(amount_0, amount_1);
        state.mint(pool.id, liquidity, to.clone()).await?;
        Ok(state
            .update(pool.id, balance_0, balance_1, block_timestamp)
            .await?)
    }

    async fn create_pool<T: Contract>(
        &mut self,
        runtime: &mut ContractRuntime<T>,
        state: &mut SwapApplicationState,
        token_0: ApplicationId,
        token_1: Option<ApplicationId>,
        amount_0_initial: Amount,
        amount_1_initial: Amount,
        amount_0_virtual: Amount,
        amount_1_virtual: Amount,
        block_timestamp: Timestamp,
    ) -> Result<Pool, PoolError> {
        // Check exists
        if let (Some(pool), _) = state.get_pool_exchangable(token_0, token_1).await? {
            return Ok(pool);
        }
        // Create pool if it's not exists
        let creator = runtime_owner(runtime);
        let pool = state
            .create_pool(
                token_0,
                token_1,
                amount_0_initial,
                amount_1_initial,
                amount_0_virtual,
                amount_1_virtual,
                creator,
                runtime.system_time(),
            )
            .await?;
        // If initial liquidity is not virtual, mint shares to creator
        if !pool.virtual_initial_liquidity {
            self.mint_shares(
                state,
                pool.clone(),
                amount_0_initial,
                amount_1_initial,
                creator,
                block_timestamp,
            )
            .await?
        }
        Ok(pool)
    }

    async fn on_op_create_pool<T: Contract>(
        &mut self,
        runtime: &mut ContractRuntime<T>,
        state: &mut SwapApplicationState,
        token_0: ApplicationId,
        token_1: Option<ApplicationId>,
        amount_0_initial: Amount,
        amount_1_initial: Amount,
        amount_0_virtual: Amount,
        amount_1_virtual: Amount,
    ) -> Result<(PoolResponse, Option<(PoolMessage, bool)>), PoolError> {
        // Receive tokens
        if amount_0_initial > Amount::ZERO {
            receive_erc20_from_runtime_owner_to_application_creation(
                runtime,
                token_0,
                amount_0_initial,
            );
        }
        if amount_1_initial > Amount::ZERO {
            receive_token_from_runtime_owner_to_application_creation(
                runtime,
                token_1,
                amount_1_initial,
            );
        }

        let origin = runtime_owner(runtime);
        let block_timestamp = runtime.system_time();

        // Create pool if it's not exists
        let pool = self
            .create_pool(
                runtime,
                state,
                token_0,
                token_1,
                amount_0_initial,
                amount_1_initial,
                amount_0_virtual,
                amount_1_virtual,
                block_timestamp,
            )
            .await?;

        // Only creator can create virtual pool
        if pool.virtual_initial_liquidity {
            let mut owner = erc20_application_owner(runtime, token_0);
            if owner != origin {
                if let Some(_token_1) = token_1 {
                    owner = erc20_application_owner(runtime, _token_1);
                    if owner != origin {
                        return Err(PoolError::PermissionDenied);
                    }
                } else {
                    return Err(PoolError::PermissionDenied);
                }
            }
        }

        // Broadcast pool creation
        Ok((
            PoolResponse::Ok,
            Some((
                PoolMessage::CreatePool {
                    origin,
                    token_0,
                    token_1,
                    amount_0_initial,
                    amount_1_initial,
                    amount_0_virtual,
                    amount_1_virtual,
                    block_timestamp,
                },
                true,
            )),
        ))
    }

    async fn on_msg_create_pool<T: Contract>(
        &mut self,
        runtime: &mut ContractRuntime<T>,
        state: &mut SwapApplicationState,
        origin: ChainAccountOwner,
        token_0: ApplicationId,
        token_1: Option<ApplicationId>,
        amount_0_initial: Amount,
        amount_1_initial: Amount,
        amount_0_virtual: Amount,
        amount_1_virtual: Amount,
        block_timestamp: Timestamp,
    ) -> Result<Option<(PoolMessage, bool)>, PoolError> {
        subscribe_erc20_application_creation(runtime, token_0);
        let _token_1 = match token_1 {
            Some(__token_1) => __token_1,
            _ => match *state.wlinera_application_id.get() {
                Some(__token_1) => __token_1,
                _ => return Err(PoolError::InvalidPool),
            },
        };
        subscribe_erc20_application_creation(runtime, _token_1);

        if origin.chain_id != runtime.chain_id() {
            let _ = self
                .create_pool(
                    runtime,
                    state,
                    token_0,
                    token_1,
                    amount_0_initial,
                    amount_1_initial,
                    amount_0_virtual,
                    amount_1_virtual,
                    block_timestamp,
                )
                .await?;
        }
        Ok(Some((
            PoolMessage::CreatePool {
                origin,
                token_0,
                token_1,
                amount_0_initial,
                amount_1_initial,
                amount_0_virtual,
                amount_1_virtual,
                block_timestamp,
            },
            true,
        )))
    }

    async fn on_op_set_fee_to<T: Contract>(
        &mut self,
        runtime: &mut ContractRuntime<T>,
        _state: &mut SwapApplicationState,
        pool_id: u64,
        account: ChainAccountOwner,
    ) -> Result<(PoolResponse, Option<(PoolMessage, bool)>), PoolError> {
        Ok((
            PoolResponse::Ok,
            Some((
                PoolMessage::SetFeeTo {
                    origin: runtime_owner(runtime),
                    pool_id,
                    account,
                },
                true,
            )),
        ))
    }

    async fn on_msg_set_fee_to<T: Contract>(
        &mut self,
        _runtime: &mut ContractRuntime<T>,
        state: &mut SwapApplicationState,
        origin: ChainAccountOwner,
        pool_id: u64,
        account: ChainAccountOwner,
    ) -> Result<Option<(PoolMessage, bool)>, PoolError> {
        state.set_fee_to(pool_id, account, origin).await?;
        Ok(Some((
            PoolMessage::SetFeeTo {
                origin,
                pool_id,
                account,
            },
            true,
        )))
    }

    async fn on_op_set_fee_to_setter<T: Contract>(
        &mut self,
        runtime: &mut ContractRuntime<T>,
        _state: &mut SwapApplicationState,
        pool_id: u64,
        account: ChainAccountOwner,
    ) -> Result<(PoolResponse, Option<(PoolMessage, bool)>), PoolError> {
        Ok((
            PoolResponse::Ok,
            Some((
                PoolMessage::SetFeeToSetter {
                    origin: runtime_owner(runtime),
                    pool_id,
                    account,
                },
                true,
            )),
        ))
    }

    async fn on_msg_set_fee_to_setter<T: Contract>(
        &mut self,
        _runtime: &mut ContractRuntime<T>,
        state: &mut SwapApplicationState,
        origin: ChainAccountOwner,
        pool_id: u64,
        account: ChainAccountOwner,
    ) -> Result<Option<(PoolMessage, bool)>, PoolError> {
        state.set_fee_to_setter(pool_id, account, origin).await?;
        Ok(Some((
            PoolMessage::SetFeeToSetter {
                origin,
                pool_id,
                account,
            },
            true,
        )))
    }
}
