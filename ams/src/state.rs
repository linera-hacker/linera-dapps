use ams::AMSError;
use async_graphql::SimpleObject;
use linera_sdk::{
    base::ApplicationId,
    views::{linera_views, MapView, QueueView, RegisterView, RootView, ViewStorageContext},
};
use spec::{
    account::ChainAccountOwner,
    ams::{InstantiationArgument, Metadata, SubscriberSyncState},
};
use std::collections::HashMap;

#[derive(RootView, SimpleObject)]
#[view(context = "ViewStorageContext")]
pub struct Application {
    pub application_types: QueueView<String>,
    pub applications: MapView<ApplicationId, Metadata>,
    pub operator: RegisterView<Option<ChainAccountOwner>>,
    pub subscribed_creator_chain: RegisterView<bool>,
}

#[allow(dead_code)]
impl Application {
    pub(crate) async fn instantiate(
        &mut self,
        argument: InstantiationArgument,
        owner: ChainAccountOwner,
    ) {
        self.operator.set(Some(owner));
        for application_type in argument.application_types {
            self.application_types.push_back(application_type);
        }
    }

    pub(crate) async fn add_application_type(
        &mut self,
        owner: ChainAccountOwner,
        application_type: String,
    ) -> Result<(), AMSError> {
        if self.operator.get().unwrap() != owner {
            return Err(AMSError::PermissionDenied);
        }
        self.application_types.push_back(application_type);
        Ok(())
    }

    pub(crate) async fn register_application(
        &mut self,
        application: Metadata,
    ) -> Result<(), AMSError> {
        let application_id = application.application_id;
        Ok(self.applications.insert(&application_id, application)?)
    }

    pub(crate) async fn to_subscriber_sync_state(&self) -> Result<SubscriberSyncState, AMSError> {
        let mut state = SubscriberSyncState {
            operator: *self.operator.get(),
            applications: HashMap::new(),
            application_types: self.application_types.elements().await?,
        };
        self.applications
            .for_each_index_value(|index, application| {
                state.applications.insert(index, application);
                Ok(())
            })
            .await?;
        Ok(state)
    }

    pub(crate) async fn from_subscriber_sync_state(
        &mut self,
        state: SubscriberSyncState,
    ) -> Result<(), AMSError> {
        if *self.subscribed_creator_chain.get() {
            return Ok(());
        }
        self.operator.set(state.operator);
        for application_type in state.application_types {
            self.application_types.push_back(application_type);
        }
        for (application_id, application) in state.applications {
            self.applications.insert(&application_id, application)?;
        }
        self.subscribed_creator_chain.set(true);
        Ok(())
    }
}
