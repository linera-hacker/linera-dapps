#![cfg_attr(target_arch = "wasm32", no_main)]

mod state;

use self::state::Application;
use async_graphql::{Context, EmptySubscription, Object, Schema};
use linera_sdk::{
    base::{Amount, ApplicationId, Timestamp, WithServiceAbi},
    views::View,
    Service, ServiceRuntime,
};
use spec::{
    account::ChainAccountOwner,
    base::BaseOperation,
    swap::{
        abi::{SwapMutationRoot, SwapOperation, SwapQueryRoot},
        pool::{Pool, PoolOperation},
        router::RouterOperation,
        state::Transaction,
    },
};
use std::sync::{Arc, Mutex};
use swap_router::router::calculate_swap_amount;

struct SwapContext {
    state: Arc<Application>,
    runtime: Mutex<ServiceRuntime<ApplicationService>>,
}

pub struct ApplicationService {
    context: Arc<SwapContext>,
}

linera_sdk::service!(ApplicationService);

impl WithServiceAbi for ApplicationService {
    type Abi = swap::ApplicationAbi;
}

impl Service for ApplicationService {
    type Parameters = ();

    async fn new(runtime: ServiceRuntime<Self>) -> Self {
        let state = Application::load(runtime.root_view_storage_context())
            .await
            .expect("Failed to load state");
        ApplicationService {
            context: Arc::new(SwapContext {
                state: Arc::new(state),
                runtime: Mutex::new(runtime),
            }),
        }
    }

    async fn handle_query(&self, query: Self::Query) -> Self::QueryResponse {
        let schema = Schema::build(QueryRoot, MutationRoot, EmptySubscription)
            .data(self.context.clone())
            .finish();
        schema.execute(query).await
    }
}

struct QueryRoot;

#[Object]
impl SwapQueryRoot for QueryRoot {
    async fn calculate_swap_amount(
        &self,
        ctx: &Context<'_>,
        token_0: ApplicationId,
        token_1: Option<ApplicationId>,
        amount_1: Amount,
    ) -> Amount {
        let context = ctx.data::<Arc<SwapContext>>().unwrap();
        let block_timestamp = context.runtime.lock().unwrap().system_time();
        match calculate_swap_amount(&context.state, token_0, token_1, amount_1, block_timestamp)
            .await
        {
            Ok(amount) => amount,
            _ => Amount::ZERO,
        }
    }

    async fn get_pool(&self, ctx: &Context<'_>, pool_id: u64) -> Option<Pool> {
        let context = ctx.data::<Arc<SwapContext>>().unwrap();
        match context.state.get_pool(pool_id).await {
            Ok(pool) => pool,
            _ => None,
        }
    }

    async fn get_fee_to(&self, ctx: &Context<'_>, pool_id: u64) -> Option<ChainAccountOwner> {
        let context = ctx.data::<Arc<SwapContext>>().unwrap();
        match context.state.get_pool(pool_id).await {
            Ok(pool) => Some(pool.unwrap().fee_to),
            _ => None,
        }
    }

    async fn get_pools(&self, ctx: &Context<'_>) -> Vec<Pool> {
        let context = ctx.data::<Arc<SwapContext>>().unwrap();
        let mut pools = Vec::<Pool>::new();
        context
            .state
            .erc20_erc20_pools
            .for_each_index_value(|_index, values| {
                for value in values.values() {
                    pools.push(value.clone());
                }
                Ok(())
            })
            .await
            .expect("Fail get erc20 pools");
        context
            .state
            .erc20_native_pools
            .for_each_index_value(|_index, value| {
                pools.push(value);
                Ok(())
            })
            .await
            .expect("Fail get native pools");
        pools
    }

    async fn get_transactions(
        &self,
        ctx: &Context<'_>,
        pool_id: Option<u64>,
        start_id: Option<u64>,
        start_timestamp: Option<Timestamp>,
    ) -> Vec<Transaction> {
        let context = ctx.data::<Arc<SwapContext>>().unwrap();
        context
            .state
            .last_transactions
            .elements()
            .await
            .expect("Fail get transactions")
            .into_iter()
            .filter(|transaction| {
                let mut ok = true;
                if let Some(pool_id) = pool_id {
                    ok = ok && transaction.pool_id == pool_id;
                }
                if let Some(start_id) = start_id {
                    ok = ok && transaction.transaction_id >= start_id;
                }
                if let Some(start_timestamp) = start_timestamp {
                    ok = ok && transaction.timestamp >= start_timestamp;
                }
                ok
            })
            .collect()
    }
}

struct MutationRoot;

#[Object]
impl SwapMutationRoot for MutationRoot {
    // Return pair token amount and liquidity
    async fn add_liquidity(
        &self,
        token_0: ApplicationId,
        token_1: Option<ApplicationId>,
        amount_0_desired: Amount,
        amount_1_desired: Amount,
        amount_0_min: Amount,
        amount_1_min: Amount,
        to: Option<ChainAccountOwner>,
        deadline: Timestamp,
    ) -> Vec<u8> {
        bcs::to_bytes(&SwapOperation::RouterOperation(
            RouterOperation::AddLiquidity {
                token_0,
                token_1,
                amount_0_desired,
                amount_1_desired,
                amount_0_min,
                amount_1_min,
                to,
                deadline,
            },
        ))
        .unwrap()
    }

    // Return pair token amount
    async fn remove_liquidity(
        &self,
        token_0: ApplicationId,
        token_1: Option<ApplicationId>,
        liquidity: Amount,
        amount_0_min: Amount,
        amount_1_min: Amount,
        to: Option<ChainAccountOwner>,
        deadline: Timestamp,
    ) -> Vec<u8> {
        bcs::to_bytes(&SwapOperation::RouterOperation(
            RouterOperation::RemoveLiquidity {
                token_0,
                token_1,
                liquidity,
                amount_0_min,
                amount_1_min,
                to,
                deadline,
            },
        ))
        .unwrap()
    }

    async fn swap(
        &self,
        token_0: ApplicationId,
        token_1: Option<ApplicationId>,
        amount_0_in: Option<Amount>,
        amount_1_in: Option<Amount>,
        amount_0_out_min: Option<Amount>,
        amount_1_out_min: Option<Amount>,
        to: Option<ChainAccountOwner>,
    ) -> Vec<u8> {
        bcs::to_bytes(&SwapOperation::RouterOperation(RouterOperation::Swap {
            token_0,
            token_1,
            amount_0_in,
            amount_1_in,
            amount_0_out_min,
            amount_1_out_min,
            to,
        }))
        .unwrap()
    }

    async fn subscribe_creator_chain(&self) -> Vec<u8> {
        bcs::to_bytes(&SwapOperation::BaseOperation(
            BaseOperation::SubscribeCreatorChain,
        ))
        .unwrap()
    }

    // Just put all liquidity pool in one application
    async fn create_pool(
        &self,
        token_0: ApplicationId,
        token_1: Option<ApplicationId>,
        amount_0_initial: Amount,
        amount_1_initial: Amount,
        amount_0_virtual: Amount,
        amount_1_virtual: Amount,
    ) -> Vec<u8> {
        bcs::to_bytes(&SwapOperation::PoolOperation(PoolOperation::CreatePool {
            token_0,
            token_1,
            amount_0_initial,
            amount_1_initial,
            amount_0_virtual,
            amount_1_virtual,
        }))
        .unwrap()
    }

    async fn set_fee_to(&self, pool_id: u64, account: ChainAccountOwner) -> Vec<u8> {
        bcs::to_bytes(&SwapOperation::PoolOperation(PoolOperation::SetFeeTo {
            pool_id,
            account,
        }))
        .unwrap()
    }

    async fn set_fee_to_setter(&self, pool_id: u64, account: ChainAccountOwner) -> Vec<u8> {
        bcs::to_bytes(&SwapOperation::PoolOperation(
            PoolOperation::SetFeeToSetter { pool_id, account },
        ))
        .unwrap()
    }

    // Return minted liquidity
    async fn mint(
        &self,
        _pool_id: u64,
        _amount_0: Amount,
        _amount_1: Amount,
        _to: ChainAccountOwner,
    ) -> Vec<u8> {
        // Invoked by router
        Vec::new()
    }

    // Return pair token amount
    async fn burn(&self, _pool_id: u64, _liquidity: Amount) -> Vec<u8> {
        // Invoked by router
        Vec::new()
    }

    async fn swap_with_pool(
        &self,
        _pool_id: u64,
        _amount_0_out: Amount,
        _amount_1_out: Amount,
        _to: ChainAccountOwner,
    ) -> Vec<u8> {
        // Invoked by router
        Vec::new()
    }
}
