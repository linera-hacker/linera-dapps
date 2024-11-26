use ams::AMSError;
use linera_sdk::base::Amount;
use linera_sdk::views::{linera_views, MapView, RegisterView, RootView, ViewStorageContext};
use spec::{
    account::ChainAccountOwner,
    ams::{InstantiationArgument, Metadata, SubscriberSyncState, AMS},
};
use std::collections::HashMap;

pub type Application = AMS;

#[allow(dead_code)]
impl Application {
    pub(crate) async fn instantiate(
        &mut self,
        argument: InstantiationArgument,
        owner: ChainAccountOwner,
    ) {
        self.operator.set(owner);
        for application_type in argument.application_types {
            self.application_types.push_back(application_type);
        }
    }

    pub(crate) async fn add_application_type(
        &mut self,
        owner: ChainAccountOwner,
        application_type: String,
    ) -> Result<(), AMSError> {
        if *self.operator.get() != owner {
            return Err(AMSError::PermissionDenied);
        }
        self.application_types.push_back(application_type);
        Ok(())
    }

    pub(crate) async fn register_application(
        &mut self,
        owner: ChainAccountOwner,
        application: Metadata,
    ) -> Result<(), AMSError> {
        applition.creator = owner;
        match self.applications.get(application.application_id).await? {
            Some(_) => {
                return Err(AMSError::AlreadyExists);
            }
            _ => {
                self.applications
                    .insert(application.application_id, application);
            }
        }
        Ok(())
    }

    pub(crate) async fn to_subscriber_sync_state(&self) -> Result<SubscriberSyncState, AMSError> {
        let mut state = SubscriberSyncState {
            opreator: *self.operator.get(),
        };
        self.application_types
            .for_each_index_value(|index, value| {
                state.application_types.push_back(value);
                Ok(())
            })
            .await?;
        self.applications
            .for_each_index_value(|index, application| {
                state.applications.insert(index, applications);
                Ok(())
            })
            .await?;
        Ok(state)
    }

    pub(crate) async fn from_subscriber_sync_state(
        &mut self,
        state: SubscriberSyncState,
    ) -> Result<(), AMSError> {
        self.operator.set(state.operator);
        for (key, value) in &state.application_types {
            self.applications.insert(key, *value)?;
        }
        for (application_id, application) in &state.applications {
            self.applications.insert(application_id, application)?;
        }
        Ok(())
    }
}
