use linera_sdk::{
    base::{Amount, ApplicationId, Timestamp},
    views::{linera_views, MapView, RegisterView, RootView, ViewStorageContext},
};
use num_bigint::BigUint;
use num_traits::cast::ToPrimitive;
use num_traits::FromPrimitive;
use spec::{account::ChainAccountOwner, base, erc20::ERC20, swap::Pool};
use std::{collections::HashMap, str::FromStr};
use swap_pool::PoolError;

#[derive(RootView, async_graphql::SimpleObject)]
#[view(context = "ViewStorageContext")]
pub struct Application {
    pub erc20_erc20_pools: MapView<ApplicationId, HashMap<ApplicationId, Pool>>,
    pub erc20_native_pools: MapView<ApplicationId, Pool>,
    pub pool_id: RegisterView<u64>,
    pub pool_erc20_erc20s: MapView<u64, Vec<ApplicationId>>,
    pub pool_erc20_natives: MapView<u64, ApplicationId>,
    pub router_application_id: RegisterView<Option<ApplicationId>>,
}

#[allow(dead_code)]
impl Application {
    pub(crate) async fn set_router_application_id(&mut self, application_id: ApplicationId) {
        self.router_application_id.set(Some(application_id));
    }

    pub(crate) fn get_router_application_id(&self) -> Option<ApplicationId> {
        *self.router_application_id.get()
    }

    async fn _insert_erc20_erc20(&mut self, pool: Pool, required: bool) -> Result<Pool, PoolError> {
        let token_1 = pool.token_1.unwrap();
        let mut token_pools = self
            .erc20_erc20_pools
            .get(&pool.token_0)
            .await?
            .unwrap_or(HashMap::new());
        if let Some(_pool) = token_pools.get(&token_1) {
            if required {
                return Err(PoolError::AlreadyExists);
            }
            return Ok(_pool.clone());
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
    ) -> Result<Pool, PoolError> {
        if let Some(_pool) = self.erc20_native_pools.get(&pool.token_0).await? {
            if required {
                return Err(PoolError::AlreadyExists);
            }
            return Ok(_pool);
        }
        self.erc20_native_pools
            .insert(&pool.token_0, pool.clone())?;
        self.pool_erc20_natives.insert(&pool.id, pool.token_0)?;
        Ok(pool)
    }

    async fn insert_pool(&mut self, pool: Pool, required: bool) -> Result<Pool, PoolError> {
        match pool.token_1 {
            Some(_) => self._insert_erc20_erc20(pool, required).await,
            None => self._insert_erc20_native(pool, required).await,
        }
    }

    pub(crate) async fn _create_pool(
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
    ) -> Result<Pool, PoolError> {
        if amount_0_initial != Amount::ZERO && amount_0_initial != amount_0_virtual {
            return Err(PoolError::InvalidInitialAmount);
        }
        if amount_1_initial != Amount::ZERO && amount_1_initial != amount_1_virtual {
            return Err(PoolError::InvalidInitialAmount);
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

        let mut pool = Pool {
            id: pool_id,
            token_0,
            token_1,
            virtual_initial_liquidity: amount_0_initial == amount_0_virtual
                && amount_1_initial == amount_1_virtual,
            amount_0_initial,
            amount_1_initial,
            reserve_0: amount_0_initial,
            reserve_1: amount_1_initial,
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

        if !pool.virtual_initial_liquidity {
            let amount_0_bigint =
                BigUint::from_u128(u128::from(amount_0_initial)).expect("Couldn't convert amount");
            let amount_1_bigint =
                BigUint::from_u128(u128::from(amount_1_initial)).expect("Couldn't convert amount");
            let amount_mul_bigint = amount_0_bigint * amount_1_bigint;
            let liquidity = Amount::from_attos(
                BigUint::sqrt(&amount_mul_bigint)
                    .to_u128()
                    .expect("Couldn't convert BigUint to u128"),
            );
            pool.erc20._mint(creator, liquidity);
        }

        self.pool_id.set(pool_id + 1);
        self.insert_pool(pool, required).await
    }

    pub(crate) async fn create_pool(
        &mut self,
        token_0: ApplicationId,
        token_1: Option<ApplicationId>,
        amount_0_initial: Amount,
        amount_1_initial: Amount,
        amount_0_virtual: Amount,
        amount_1_virtual: Amount,
        creator: ChainAccountOwner,
        block_timestamp: Timestamp,
    ) -> Result<Pool, PoolError> {
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

    pub(crate) async fn require_create_pool(
        &mut self,
        token_0: ApplicationId,
        token_1: Option<ApplicationId>,
        amount_0_initial: Amount,
        amount_1_initial: Amount,
        amount_0_virtual: Amount,
        amount_1_virtual: Amount,
        creator: ChainAccountOwner,
        block_timestamp: Timestamp,
    ) -> Result<Pool, PoolError> {
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

    pub(crate) async fn get_pool(&self, pool_id: u64) -> Result<Option<Pool>, PoolError> {
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

    pub(crate) async fn get_pool_with_token_pair(
        &self,
        token_0: ApplicationId,
        token_1: Option<ApplicationId>,
    ) -> Result<Option<Pool>, PoolError> {
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

    pub(crate) async fn set_fee_to(
        &mut self,
        pool_id: u64,
        account: ChainAccountOwner,
        setter: ChainAccountOwner,
    ) -> Result<(), PoolError> {
        let mut pool = self.get_pool(pool_id).await?.expect("Invalid pool");
        if pool.fee_to_setter != setter {
            return Err(PoolError::PermissionDenied);
        }
        pool.fee_to = account;
        // TODO: test if we need to insert again
        Ok(())
    }

    pub(crate) async fn set_fee_to_setter(
        &mut self,
        pool_id: u64,
        account: ChainAccountOwner,
        setter: ChainAccountOwner,
    ) -> Result<(), PoolError> {
        let mut pool = self.get_pool(pool_id).await?.expect("Invalid pool");
        if pool.fee_to_setter != setter {
            return Err(PoolError::PermissionDenied);
        }
        pool.fee_to_setter = account;
        // TODO: test if we need to insert again
        Ok(())
    }

    pub(crate) async fn mint(
        &mut self,
        pool_id: u64,
        liquidity: Amount,
        to: ChainAccountOwner,
    ) -> Result<(), PoolError> {
        let mut pool = self.get_pool(pool_id).await?.expect("Invalid pool");
        pool.erc20._mint(to, liquidity);
        // TODO: test if we need to insert again
        Ok(())
    }

    pub(crate) async fn burn(
        &mut self,
        pool_id: u64,
        liquidity: Amount,
        from: ChainAccountOwner,
    ) -> Result<(), PoolError> {
        let mut pool = self.get_pool(pool_id).await?.expect("Invalid pool");
        pool.erc20._burn(from, liquidity);
        Ok(())
    }

    pub(crate) async fn update(
        &mut self,
        pool_id: u64,
        balance_0: Amount,
        balance_1: Amount,
        block_timestamp: Timestamp,
    ) -> Result<(), PoolError> {
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

        Ok(())
    }

    pub(crate) async fn mint_fee(&mut self, pool_id: u64) -> Result<(), PoolError> {
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
                }
            }
        }

        Ok(())
    }
}
