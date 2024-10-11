#![cfg_attr(target_arch = "wasm32", no_main)]

mod state;

use self::state::Application;
use async_graphql::{EmptySubscription, Object, Schema};
use linera_sdk::{
    base::{Amount, ApplicationId, Timestamp, WithServiceAbi},
    views::View,
    Service, ServiceRuntime,
};
use spec::{
    account::ChainAccountOwner,
    swap::{RouterMutationRoot, RouterOperation, RouterQueryRoot},
};
use std::sync::Arc;

pub struct ApplicationService {
    _state: Arc<Application>,
}

linera_sdk::service!(ApplicationService);

impl WithServiceAbi for ApplicationService {
    type Abi = swap_router::ApplicationAbi;
}

impl Service for ApplicationService {
    type Parameters = ();

    async fn new(runtime: ServiceRuntime<Self>) -> Self {
        let state = Application::load(runtime.root_view_storage_context())
            .await
            .expect("Failed to load state");
        ApplicationService {
            _state: Arc::new(state),
        }
    }

    async fn handle_query(&self, query: Self::Query) -> Self::QueryResponse {
        let schema = Schema::build(QueryRoot, MutationRoot, EmptySubscription).finish();
        schema.execute(query).await
    }
}

struct QueryRoot;

#[Object]
impl RouterQueryRoot for QueryRoot {
    async fn calculate_swap_amount(
        &self,
        _token_0: ApplicationId,
        _token_1: Option<ApplicationId>,
        _amount_1: Amount,
    ) -> Amount {
        Amount::ZERO
    }
}

struct MutationRoot;

#[Object]
impl RouterMutationRoot for MutationRoot {
    // Return pair token amount and liquidity
    async fn add_liquidity(
        &self,
        token_0: ApplicationId,
        token_1: Option<ApplicationId>,
        amount_0_desired: Amount,
        amount_1_desired: Amount,
        amount_0_min: Amount,
        amount_1_min: Amount,
        to: ChainAccountOwner,
        deadline: Timestamp,
    ) -> Vec<u8> {
        bcs::to_bytes(&RouterOperation::AddLiquidity {
            token_0,
            token_1,
            amount_0_desired,
            amount_1_desired,
            amount_0_min,
            amount_1_min,
            to,
            deadline,
        })
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
        to: ChainAccountOwner,
        deadline: Timestamp,
    ) -> Vec<u8> {
        bcs::to_bytes(&RouterOperation::RemoveLiquidity {
            token_0,
            token_1,
            liquidity,
            amount_0_min,
            amount_1_min,
            to,
            deadline,
        })
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
        to: ChainAccountOwner,
    ) -> Vec<u8> {
        bcs::to_bytes(&RouterOperation::Swap {
            token_0,
            token_1,
            amount_0_in,
            amount_1_in,
            amount_0_out_min,
            amount_1_out_min,
            to,
        })
        .unwrap()
    }
}
