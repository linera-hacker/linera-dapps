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
    swap::{PoolMutationRoot, PoolQueryRoot},
};
use std::sync::{Arc, Mutex};

pub struct ApplicationService {
    state: Arc<Application>,
    runtime: Arc<Mutex<ServiceRuntime<Self>>>,
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
impl PoolQueryRoot for QueryRoot {
    async fn get_pool(&self, token_0: ApplicationId, token_1: ApplicationId) -> Option<u64> {
        None
    }

    async fn get_fee_to(&self) -> Option<ChainAccountOwner> {
        None
    }
}

struct MutationRoot;

#[Object]
impl PoolMutationRoot for MutationRoot {
    // Just put all liquidity pool in one application
    async fn create_pool(&self, token_0: ApplicationId, token_1: ApplicationId) -> Vec<u8> {
        Vec::new()
    }

    async fn set_fee_to(&self, account: ChainAccountOwner) -> Vec<u8> {
        Vec::new()
    }

    async fn set_fee_to_setter(&self, account: ChainAccountOwner) -> Vec<u8> {
        Vec::new()
    }

    // Return minted liquidity
    async fn mint(&self, to: ChainAccountOwner) -> Vec<u8> {
        Vec::new()
    }

    // Return pair token amount
    async fn burn(&self, to: ChainAccountOwner) -> Vec<u8> {
        Vec::new()
    }

    async fn swap(
        &self,
        amount_0_out: Amount,
        amount_1_out: Amount,
        to: ChainAccountOwner,
    ) -> Vec<u8> {
        Vec::new()
    }
}
