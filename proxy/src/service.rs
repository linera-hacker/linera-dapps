#![cfg_attr(target_arch = "wasm32", no_main)]

mod state;

use self::state::ProxyState;
use linera_sdk::{
    base::WithServiceAbi,
    views::{View, ViewStorageContext},
    Service, ServiceRuntime,
};

pub struct ProxyService {
    state: ProxyState,
    runtime: ServiceRuntime<Self>,
}

linera_sdk::service!(ProxyService);

impl WithServiceAbi for ProxyService {
    type Abi = proxy::ProxyAbi;
}

impl Service for ProxyService {
    type Parameters = ();

    async fn new(runtime: ServiceRuntime<Self>) -> Self {
        let state = ProxyState::load(runtime.root_view_storage_context())
            .await
            .expect("Failed to load state");
        ProxyService { state, runtime }
    }

    async fn handle_query(&self, _query: Self::Query) -> Self::QueryResponse {
        panic!("Queries not supported by application");
    }
}
