#![cfg_attr(target_arch = "wasm32", no_main)]

mod state;

use self::state::Application;
use interfaces::erc20::{ERC20MutationRoot, ERC20QueryRoot};
use linera_sdk::{
    base::{Amount, WithServiceAbi},
    views::View,
    Service, ServiceRuntime,
};

pub struct ApplicationService {
    state: Application,
    runtime: ServiceRuntime<Self>,
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
        ApplicationService { state, runtime }
    }

    async fn handle_query(&self, query: Self::Query) -> Self::QueryResponse {
        let runtime = self.rumtime.clone();
        let schema = Schema::build(QueryRoot, MutationRoot, EmptySubscription).finish();
        schema.execute(query).await
    }
}

struct QueryRoot;

impl ERC20QueryRoot for QueryRoot {
    fn total_supply(&self) -> Amount {
        Amount::ZERO
    }
}

struct MutationRoot;

impl ERC20MutationRoot for MutationRoot {}
