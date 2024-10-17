use crate::swap::pool::Pool;
use async_graphql::SimpleObject;
use linera_sdk::{
    base::ApplicationId,
    views::{linera_views, MapView, RegisterView, RootView, ViewStorageContext},
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use thiserror::Error;

#[derive(Debug, Error)]
#[allow(dead_code)]
pub enum StateError {
    #[error(transparent)]
    ViewError(#[from] linera_sdk::views::ViewError),
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct SubscriberSyncState {
    pub erc20_erc20_pools: HashMap<ApplicationId, HashMap<ApplicationId, Pool>>,
    pub erc20_native_pools: HashMap<ApplicationId, Pool>,
    pub pool_id: u64,
    pub pool_erc20_erc20s: HashMap<u64, Vec<ApplicationId>>,
    pub pool_erc20_natives: HashMap<u64, ApplicationId>,
}

#[derive(RootView, SimpleObject)]
#[view(context = "ViewStorageContext")]
pub struct SwapApplicationState {
    pub erc20_erc20_pools: MapView<ApplicationId, HashMap<ApplicationId, Pool>>,
    pub erc20_native_pools: MapView<ApplicationId, Pool>,
    pub pool_id: RegisterView<u64>,
    pub pool_erc20_erc20s: MapView<u64, Vec<ApplicationId>>,
    pub pool_erc20_natives: MapView<u64, ApplicationId>,
}

impl SwapApplicationState {
    pub async fn to_subscriber_sync_state(&self) -> Result<SubscriberSyncState, StateError> {
        let mut state = SubscriberSyncState {
            erc20_erc20_pools: HashMap::new(),
            erc20_native_pools: HashMap::new(),
            pool_id: *self.pool_id.get(),
            pool_erc20_erc20s: HashMap::new(),
            pool_erc20_natives: HashMap::new(),
        };
        self.erc20_erc20_pools
            .for_each_index_value(|index, pools| {
                state.erc20_erc20_pools.insert(index, pools);
                Ok(())
            })
            .await?;
        self.erc20_native_pools
            .for_each_index_value(|index, pool| {
                state.erc20_native_pools.insert(index, pool);
                Ok(())
            })
            .await?;
        self.pool_erc20_erc20s
            .for_each_index_value(|index, tokens| {
                state.pool_erc20_erc20s.insert(index, tokens);
                Ok(())
            })
            .await?;
        self.pool_erc20_natives
            .for_each_index_value(|index, token| {
                state.pool_erc20_natives.insert(index, token);
                Ok(())
            })
            .await?;
        Ok(state)
    }

    pub async fn from_subscriber_sync_state(
        &mut self,
        state: SubscriberSyncState,
    ) -> Result<(), StateError> {
        self.pool_id.set(state.pool_id);
        for (key, pools) in &state.erc20_erc20_pools {
            self.erc20_erc20_pools.insert(key, pools.clone())?;
        }
        for (key, pool) in &state.erc20_native_pools {
            self.erc20_native_pools.insert(key, pool.clone())?;
        }
        for (key, tokens) in &state.pool_erc20_erc20s {
            self.pool_erc20_erc20s.insert(key, tokens.clone())?;
        }
        for (key, token) in &state.pool_erc20_natives {
            self.pool_erc20_natives.insert(key, *token)?;
        }
        Ok(())
    }
}
