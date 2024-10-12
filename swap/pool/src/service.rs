#![cfg_attr(target_arch = "wasm32", no_main)]

mod state;

use self::state::Application;
use async_graphql::{Context, EmptySubscription, Object, Schema};
use linera_sdk::{
    base::{Amount, ApplicationId, WithServiceAbi},
    views::View,
    Service, ServiceRuntime,
};
use spec::{
    account::ChainAccountOwner,
    swap::{Pool, PoolMutationRoot, PoolOperation, PoolQueryRoot},
};
use std::sync::Arc;

struct PoolContext {
    state: Arc<Application>,
}

pub struct ApplicationService {
    pool_context: Arc<PoolContext>,
}

linera_sdk::service!(ApplicationService);

impl WithServiceAbi for ApplicationService {
    type Abi = swap_pool::ApplicationAbi;
}

impl Service for ApplicationService {
    type Parameters = ();

    async fn new(runtime: ServiceRuntime<Self>) -> Self {
        let state = Application::load(runtime.root_view_storage_context())
            .await
            .expect("Failed to load state");
        ApplicationService {
            pool_context: Arc::new(PoolContext {
                state: Arc::new(state),
            }),
        }
    }

    async fn handle_query(&self, query: Self::Query) -> Self::QueryResponse {
        let schema = Schema::build(QueryRoot, MutationRoot, EmptySubscription)
            .data(self.pool_context.clone())
            .finish();
        schema.execute(query).await
    }
}

struct QueryRoot;

#[Object]
impl PoolQueryRoot for QueryRoot {
    async fn get_pool(&self, ctx: &Context<'_>, pool_id: u64) -> Option<Pool> {
        let pool_context = ctx.data::<Arc<PoolContext>>().unwrap();
        match pool_context.state.get_pool(pool_id).await {
            Ok(pool) => pool,
            _ => None,
        }
    }

    async fn get_fee_to(&self, ctx: &Context<'_>, pool_id: u64) -> Option<ChainAccountOwner> {
        let pool_context = ctx.data::<Arc<PoolContext>>().unwrap();
        match pool_context.state.get_pool(pool_id).await {
            Ok(pool) => Some(pool.unwrap().fee_to),
            _ => None,
        }
    }
}

struct MutationRoot;

#[Object]
impl PoolMutationRoot for MutationRoot {
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
        bcs::to_bytes(&PoolOperation::CreatePool {
            token_0,
            token_1,
            amount_0_initial,
            amount_1_initial,
            amount_0_virtual,
            amount_1_virtual,
        })
        .unwrap()
    }

    async fn set_fee_to(&self, pool_id: u64, account: ChainAccountOwner) -> Vec<u8> {
        bcs::to_bytes(&PoolOperation::SetFeeTo { pool_id, account }).unwrap()
    }

    async fn set_fee_to_setter(&self, pool_id: u64, account: ChainAccountOwner) -> Vec<u8> {
        bcs::to_bytes(&PoolOperation::SetFeeToSetter { pool_id, account }).unwrap()
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

    async fn swap(
        &self,
        pool_id: u64,
        amount_0_out: Amount,
        amount_1_out: Amount,
        to: ChainAccountOwner,
    ) -> Vec<u8> {
        // Invoked by router
        Vec::new()
    }
}
