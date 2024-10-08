#![cfg_attr(target_arch = "wasm32", no_main)]

mod state;

use std::sync::{Arc, Mutex};
use async_graphql::{Context, Schema, EmptySubscription, Object};
use self::state::Application;
use interfaces::erc20::{ERC20MutationRoot, ERC20QueryRoot};
use linera_sdk::{
    base::{Amount, WithServiceAbi, Account},
    views::View,
    Service, ServiceRuntime,
};

pub struct ApplicationService {
    state: Arc<Application>,
    runtime: Arc<Mutex<ServiceRuntime<Self>>>,
}

linera_sdk::service!(ApplicationService);

impl WithServiceAbi for ApplicationService {
    type Abi = erc20::ApplicationAbi;
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
impl ERC20QueryRoot for QueryRoot {
    async fn total_supply(&self) -> Amount {
        Amount::ZERO
    }
}

struct MutationRoot;

#[Object]
impl ERC20MutationRoot for MutationRoot {
    async fn transfer(&self, to: Account, amount: Amount) -> Vec<u8> {
        Vec::new()
    }
}
