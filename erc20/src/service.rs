#![cfg_attr(target_arch = "wasm32", no_main)]

mod state;

use self::state::Application;
use interfaces::erc20::ERC20;
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

    async fn handle_query(&self, _query: Self::Query) -> Self::QueryResponse {
        panic!("Queries not supported by application");
    }
}

impl ERC20 for ApplicationService {
    fn total_supply(&self) -> Amount {
        Amount::ZERO
    }
}
