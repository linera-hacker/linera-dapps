#![cfg_attr(target_arch = "wasm32", no_main)]

mod state;

use self::state::Application;
use async_graphql::{EmptySubscription, Object, Schema};
use linera_sdk::{
    base::{Amount, ApplicationId, Timestamp, WithServiceAbi},
    views::{View, ViewStorageContext},
    Service, ServiceRuntime,
};
use spec::{
    account::ChainAccountOwner,
    swap::{RouterMutationRoot, RouterQueryRoot},
};
use std::sync::{Arc, Mutex};

pub struct ApplicationService {
    state: Arc<Application>,
    runtime: Arc<Mutex<ServiceRuntime<Self>>>,
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
            state: Arc::new(state),
            runtime: Arc::new(Mutex::new(runtime)),
        }
    }

    async fn handle_query(&self, query: Self::Query) -> Self::QueryResponse {
        let runtime = self.runtime.clone();
        let schema = Schema::build(QueryRoot, MutationRoot, EmptySubscription).finish();
        schema.execute(query).await
    }
}

struct QueryRoot;

#[Object]
impl RouterQueryRoot for QueryRoot {
    async fn calculate_swap_amount(
        &self,
        token_0: ApplicationId,
        token_1: ApplicationId,
        amount_1: Amount,
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
        token_1: ApplicationId,
        amount_0_desired: Amount,
        amount_1_desired: Amount,
        amount_0_min: Amount,
        amount_1_min: Amount,
        to: ChainAccountOwner,
        deadline: Timestamp,
    ) -> Vec<u8> {
        Vec::new()
    }

    // Return pair token amount
    async fn remove_liquidity(
        &self,
        token_0: ApplicationId,
        token_1: ApplicationId,
        liquidity: Amount,
        amount_0_min: Amount,
        amount_1_min: Amount,
        to: ChainAccountOwner,
        deadline: Timestamp,
    ) -> Vec<u8> {
        Vec::new()
    }
}
