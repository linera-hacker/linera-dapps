#![cfg_attr(target_arch = "wasm32", no_main)]

mod state;

use self::state::Application;
use async_graphql::{EmptySubscription, Object, Schema};
use linera_sdk::{
    base::{ApplicationId, Timestamp, WithServiceAbi},
    views::View,
    Service, ServiceRuntime,
};
use spec::ams::{AMSQueryRoot, Metadata};
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
        limit: usize,
    ) -> Vec<Metadata> {
        let mut values = Vec::new();
        self.state
            .applications
            .for_each_index_value_while(|_, value| {
                if created_before.is_some() && value.created_at > created_before.unwrap() {
                    return Ok(true);
                }
                if created_after.is_some() && value.created_at > created_after.unwrap() {
                    return Ok(true);
                }
                if let Some(application_type) = application_type.clone() {
                    if value.application_type != application_type {
                        return Ok(true);
                    }
                }
                values.push(value);
                Ok(values.len() < limit)
            })
            .await
            .expect("Failed get applications");
        values
    }

    async fn application(&self, application_id: ApplicationId) -> Option<Metadata> {
        self.state
            .applications
            .get(&application_id)
            .await
            .expect("Failed get application")
    }
}

struct MutationRoot {}

#[Object]
impl MutationRoot {
    async fn do_nothing(&self) -> Vec<u8> {
        Vec::new()
    }
}
