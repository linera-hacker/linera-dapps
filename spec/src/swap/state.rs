use crate::{account::ChainAccountOwner, base, erc20::ERC20, swap::pool::Pool};
use async_graphql::SimpleObject;
use linera_sdk::{
    base::{Amount, ApplicationId, ParseAmountError, Timestamp},
    views::{linera_views, MapView, RegisterView, RootView, ViewStorageContext},
};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, str::FromStr};
use thiserror::Error;

#[derive(Debug, Error)]
#[allow(dead_code)]
pub enum StateError {
    #[error("Invalid initial amount")]
    InvalidInitialAmount,

    #[error(transparent)]
    ParseAmountError(#[from] ParseAmountError),

    #[error(transparent)]
    ViewError(#[from] linera_sdk::views::ViewError),

    #[error("Already exists")]
    AlreadyExists,

    #[error("Invalid pool")]
    InvalidPool,

    #[error("Permission denied")]
    PermissionDenied,

    #[error("Not supported")]
    NotSupported,

    #[error("Insufficient funds")]
    InsufficientFunds,

    #[error("Invalid amount")]
    InvalidAmount,

    #[error("Insufficient liquidity")]
    InsufficientLiquidity,

    #[error("Broken K")]
    BrokenK,
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

    async fn _insert_erc20_erc20(
        &mut self,
        pool: Pool,
        required: bool,
    ) -> Result<Pool, StateError> {
        let token_1 = pool.token_1.unwrap();
        let mut token_pools = self
            .erc20_erc20_pools
            .get(&pool.token_0)
            .await?
            .unwrap_or(HashMap::new());
        if let Some(_pool) = token_pools.get(&token_1) {
            if required {
                return Err(StateError::AlreadyExists);
            }
        }
        token_pools.insert(token_1, pool.clone());
        self.erc20_erc20_pools.insert(&pool.token_0, token_pools)?;
        self.pool_erc20_erc20s
            .insert(&pool.id, [pool.token_0, token_1].to_vec())?;
        Ok(pool)
    }

    async fn _insert_erc20_native(
        &mut self,
        pool: Pool,
        required: bool,
    ) -> Result<Pool, StateError> {
        if let Some(_pool) = self.erc20_native_pools.get(&pool.token_0).await? {
            if required {
                return Err(StateError::AlreadyExists);
            }
            return Ok(_pool);
        }
        self.erc20_native_pools
            .insert(&pool.token_0, pool.clone())?;
        self.pool_erc20_natives.insert(&pool.id, pool.token_0)?;
        Ok(pool)
    }

    async fn insert_pool(&mut self, pool: Pool, required: bool) -> Result<Pool, StateError> {
        match pool.token_1 {
            Some(_) => self._insert_erc20_erc20(pool, required).await,
            None => self._insert_erc20_native(pool, required).await,
        }
    }

    pub async fn _create_pool(
        &mut self,
        token_0: ApplicationId,
        token_1: Option<ApplicationId>,
        amount_0_initial: Amount,
        amount_1_initial: Amount,
        amount_0_virtual: Amount,
        amount_1_virtual: Amount,
        creator: ChainAccountOwner,
        block_timestamp: Timestamp,
        required: bool,
    ) -> Result<Pool, StateError> {
        if amount_0_initial != Amount::ZERO && amount_0_initial != amount_0_virtual {
            return Err(StateError::InvalidInitialAmount);
        }
        if amount_1_initial != Amount::ZERO && amount_1_initial != amount_1_virtual {
            return Err(StateError::InvalidInitialAmount);
        }

        let pool_id = *self.pool_id.get();
        let amount_0_initial = if amount_0_initial != Amount::ZERO {
            amount_0_initial
        } else {
            amount_0_virtual
        };
        let amount_1_initial = if amount_1_initial != Amount::ZERO {
            amount_1_initial
        } else {
            amount_1_virtual
        };

        let pool = Pool {
            id: pool_id,
            token_0,
            token_1,
            virtual_initial_liquidity: amount_0_initial != amount_0_virtual
                || amount_1_initial != amount_1_virtual,
            amount_0_initial,
            amount_1_initial,
            reserve_0: Amount::ZERO,
            reserve_1: Amount::ZERO,
            pool_fee_rate: Amount::from_str("0.3")?,
            protocol_fee_rate: Amount::from_str("0.05")?,
            erc20: ERC20::default(),
            fee_to: creator.clone(),
            fee_to_setter: creator.clone(),
            price_0_cumulative: Amount::ZERO,
            price_1_cumulative: Amount::ZERO,
            k_last: Amount::ZERO,
            block_timestamp: block_timestamp,
        };

        self.pool_id.set(pool_id + 1);
        self.insert_pool(pool, required).await
    }

    pub async fn create_pool(
        &mut self,
        token_0: ApplicationId,
        token_1: Option<ApplicationId>,
        amount_0_initial: Amount,
        amount_1_initial: Amount,
        amount_0_virtual: Amount,
        amount_1_virtual: Amount,
        creator: ChainAccountOwner,
        block_timestamp: Timestamp,
    ) -> Result<Pool, StateError> {
        self._create_pool(
            token_0,
            token_1,
            amount_0_initial,
            amount_1_initial,
            amount_0_virtual,
            amount_1_virtual,
            creator,
            block_timestamp,
            false,
        )
        .await
    }

    pub async fn require_create_pool(
        &mut self,
        token_0: ApplicationId,
        token_1: Option<ApplicationId>,
        amount_0_initial: Amount,
        amount_1_initial: Amount,
        amount_0_virtual: Amount,
        amount_1_virtual: Amount,
        creator: ChainAccountOwner,
        block_timestamp: Timestamp,
    ) -> Result<Pool, StateError> {
        self._create_pool(
            token_0,
            token_1,
            amount_0_initial,
            amount_1_initial,
            amount_0_virtual,
            amount_1_virtual,
            creator,
            block_timestamp,
            true,
        )
        .await
    }

    pub async fn get_pool(&self, pool_id: u64) -> Result<Option<Pool>, StateError> {
        match self.pool_erc20_erc20s.get(&pool_id).await? {
            Some(tokens) => Ok(self
                .erc20_erc20_pools
                .get(&tokens[0])
                .await?
                .unwrap_or(HashMap::new())
                .get(&tokens[1])
                .cloned()),
            None => match self.pool_erc20_natives.get(&pool_id).await? {
                Some(token) => Ok(self.erc20_native_pools.get(&token).await?),
                None => Ok(None),
            },
        }
    }

    pub async fn update_pool(&mut self, pool: Pool) -> Result<(), StateError> {
        let Some(_pool) = self
            .get_pool_with_token_pair(pool.token_0, pool.token_1)
            .await?
        else {
            return Err(StateError::InvalidPool);
        };
        if _pool.id != pool.id {
            return Err(StateError::InvalidPool);
        }

        match pool.token_1 {
            Some(token_1) => {
                let mut pools = self.erc20_erc20_pools.get(&pool.token_0).await?.unwrap();
                pools.insert(token_1, pool.clone());
                Ok(self.erc20_erc20_pools.insert(&pool.token_0, pools)?)
            }
            _ => Ok(self
                .erc20_native_pools
                .insert(&pool.token_0.clone(), pool)?),
        }
    }

    pub async fn get_pool_with_token_pair(
        &self,
        token_0: ApplicationId,
        token_1: Option<ApplicationId>,
    ) -> Result<Option<Pool>, StateError> {
        match token_1 {
            Some(_token_1) => match self
                .erc20_erc20_pools
                .get(&token_0)
                .await?
                .unwrap_or(HashMap::new())
                .get(&_token_1)
            {
                Some(pool) => Ok(Some(pool.clone())),
                _ => Ok(None),
            },
            _ => Ok(self.erc20_native_pools.get(&token_0).await?),
        }
    }

    pub async fn set_fee_to(
        &mut self,
        pool_id: u64,
        account: ChainAccountOwner,
        setter: ChainAccountOwner,
    ) -> Result<(), StateError> {
        let mut pool = self.get_pool(pool_id).await?.expect("Invalid pool");
        if pool.fee_to_setter != setter {
            return Err(StateError::PermissionDenied);
        }
        pool.fee_to = account;
        self.update_pool(pool).await?;
        Ok(())
    }

    pub async fn set_fee_to_setter(
        &mut self,
        pool_id: u64,
        account: ChainAccountOwner,
        setter: ChainAccountOwner,
    ) -> Result<(), StateError> {
        let mut pool = self.get_pool(pool_id).await?.expect("Invalid pool");
        if pool.fee_to_setter != setter {
            return Err(StateError::PermissionDenied);
        }
        pool.fee_to_setter = account;
        self.update_pool(pool).await?;
        Ok(())
    }

    pub async fn mint(
        &mut self,
        pool_id: u64,
        liquidity: Amount,
        to: ChainAccountOwner,
    ) -> Result<(), StateError> {
        let mut pool = self.get_pool(pool_id).await?.expect("Invalid pool");
        pool.erc20._mint(to, liquidity);
        self.update_pool(pool).await?;
        Ok(())
    }

    pub async fn burn(
        &mut self,
        pool_id: u64,
        liquidity: Amount,
        from: ChainAccountOwner,
    ) -> Result<(), StateError> {
        let mut pool = self.get_pool(pool_id).await?.expect("Invalid pool");
        pool.erc20._burn(from, liquidity);
        self.update_pool(pool).await?;
        Ok(())
    }

    pub async fn update(
        &mut self,
        pool_id: u64,
        balance_0: Amount,
        balance_1: Amount,
        block_timestamp: Timestamp,
    ) -> Result<(), StateError> {
        let mut pool = self.get_pool(pool_id).await?.expect("Invalid pool");

        let time_elapsed = u128::from(
            block_timestamp
                .delta_since(pool.block_timestamp)
                .as_micros(),
        );
        if time_elapsed > 0 && pool.reserve_0 > Amount::ZERO && pool.reserve_1 > Amount::ZERO {
            pool.price_0_cumulative = Amount::from_attos(
                pool.reserve_1
                    .saturating_div(pool.reserve_0)
                    .saturating_mul(time_elapsed),
            );
            pool.price_1_cumulative = Amount::from_attos(
                pool.reserve_0
                    .saturating_div(pool.reserve_1)
                    .saturating_mul(time_elapsed),
            );
        }

        pool.reserve_0 = balance_0;
        pool.reserve_1 = balance_1;
        pool.block_timestamp = block_timestamp;
        pool.k_last = pool.reserve_0.saturating_mul(pool.reserve_1.into());

        self.update_pool(pool).await?;

        Ok(())
    }

    pub async fn mint_fee(&mut self, pool_id: u64) -> Result<(), StateError> {
        let mut pool = self.get_pool(pool_id).await?.expect("Invalid pool");

        if pool.k_last != Amount::ZERO {
            let root_k = base::sqrt(pool.reserve_0.saturating_mul(pool.reserve_1.into()));
            let root_k_last = base::sqrt(pool.k_last);
            if root_k > root_k_last {
                let numerator = pool
                    .erc20
                    .total_supply
                    .saturating_mul(root_k.saturating_sub(root_k_last.into()).into());
                let denominator = root_k.saturating_mul(5).saturating_add(root_k_last.into());
                let liquidity = Amount::from_attos(numerator.saturating_div(denominator));
                if liquidity > Amount::ZERO {
                    pool.erc20._mint(pool.fee_to, liquidity);
                    self.update_pool(pool).await?;
                }
            }
        }

        Ok(())
    }
}
