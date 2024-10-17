use crate::runtime::{
    receive_erc20_from_runtime_owner_to_application_creation,
    receive_token_from_runtime_owner_to_application_creation, runtime_owner,
};
use linera_sdk::{
    base::{Amount, ApplicationId, ParseAmountError, Timestamp},
    Contract, ContractRuntime,
};
use spec::{
    account::ChainAccountOwner,
    swap::{
        abi::SwapMessage,
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
            _ => todo!(),
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
                )
                .await
            }
            _ => todo!(),
        }
    }

    async fn mint_shares<T: Contract>(
        &mut self,
        runtime: &mut ContractRuntime<T>,
        state: &mut SwapApplicationState,
        pool: Pool,
        amount_0: Amount,
        amount_1: Amount,
        to: ChainAccountOwner,
    ) -> Result<(), PoolError> {
        let balance_0 = pool.reserve_0.saturating_add(amount_0);
        let balance_1 = pool.reserve_0.saturating_add(amount_1);

        state.mint_fee(pool.id).await?;
        let liquidity = pool.calculate_liquidity(amount_0, amount_1);
        state.mint(pool.id, liquidity, to.clone()).await?;
        Ok(state
            .update(pool.id, balance_0, balance_1, runtime.system_time())
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
    ) -> Result<(), PoolError> {
        // Check exists
        if let (Some(poo), _) = state.get_pool_exchangable(token_0, token_1).await? {
            return Ok(());
        }
        // TODO: check if called by token creator
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
                runtime,
                state,
                pool,
                amount_0_initial,
                amount_1_initial,
                creator,
            )
            .await?
        }
        Ok(())
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
        // Create pool if it's not exists
        self.create_pool(
            runtime,
            state,
            token_0,
            token_1,
            amount_0_initial,
            amount_1_initial,
            amount_0_virtual,
            amount_1_virtual,
        )
        .await?;
        // Broadcast pool creation
        Ok((
            PoolResponse::Ok,
            Some((
                PoolMessage::CreatePool {
                    origin: runtime_owner(runtime),
                    token_0,
                    token_1,
                    amount_0_initial,
                    amount_1_initial,
                    amount_0_virtual,
                    amount_1_virtual,
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
    ) -> Result<Option<(PoolMessage, bool)>, PoolError> {
        if origin.chain_id != runtime.chain_id() {
            self.create_pool(
                runtime,
                state,
                token_0,
                token_1,
                amount_0_initial,
                amount_1_initial,
                amount_0_virtual,
                amount_1_virtual,
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
            },
            true,
        )))
    }
}
