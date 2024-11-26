#![cfg_attr(target_arch = "wasm32", no_main)]

mod state;

use self::state::Application;
use async_graphql::{EmptySubscription, Object, Schema};
use linera_sdk::{
    base::{Amount, WithServiceAbi},
    views::View,
    Service, ServiceRuntime,
};
use spec::{
    account::ChainAccountOwner,
    ams::{AMSOperation, AMSQueryRoot, Metadata},
    base::BaseOperation,
};
use std::sync::Arc;

pub struct ApplicationService {
    state: Arc<Application>,
}

linera_sdk::service!(ApplicationService);

impl WithServiceAbi for ApplicationService {
    type Abi = ams::ApplicationAbi;
}

impl Service for ApplicationService {
    type Parameters = ();

    async fn new(runtime: ServiceRuntime<Self>) -> Self {
        let state = Application::load(runtime.root_view_storage_context())
            .await
            .expect("Failed to load state");
        ApplicationService {
            state: Arc::new(state),
        }
    }

    async fn handle_query(&self, query: Self::Query) -> Self::QueryResponse {
        let schema = Schema::build(
            QueryRoot {
                state: self.state.clone(),
            },
            MutationRoot {},
            EmptySubscription,
        )
        .finish();
        schema.execute(query).await
    }
}

struct QueryRoot {
    state: Arc<Application>,
}

#[Object]
impl AMSQueryRoot for QueryRoot {
    async fn applications(
        &self,
        created_before: Option<Timestamp>,
        created_after: Option<Timestamp>,
        application_type: Option<String>,
    ) -> Vec<Metadata> {
        Vec::new()
    }

    async fn application(&self) -> Option<Option<Metadata>> {
        None
    }
}

struct MutationRoot {}

#[Object]
impl MutationRoot {
    async fn do_nothing(&self) -> Vec<u8> {
        Vec::new()
    }
}
