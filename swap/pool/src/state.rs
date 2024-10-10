use linera_sdk::{
    base::{Amount, ApplicationId},
    views::{linera_views, MapView, RegisterView, RootView, ViewStorageContext},
};
use num_bigint::BigUint;
use num_traits::cast::ToPrimitive;
use num_traits::FromPrimitive;
use spec::{account::ChainAccountOwner, erc20::ERC20, swap::Pool};
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
}

#[allow(dead_code)]
impl Application {
    async fn insert_pool(&mut self, pool: Pool) -> Result<(), PoolError> {
        match pool.token_1 {
            Some(_token_1) => {
                let mut token_pools = self
                    .erc20_erc20_pools
                    .get(&pool.token_0)
                    .await?
                    .unwrap_or(HashMap::new());
                if token_pools.get(&_token_1).is_some() {
                    return Err(PoolError::AlreadyExists);
                }
                token_pools.insert(_token_1, pool.clone());
                self.erc20_erc20_pools.insert(&pool.token_0, token_pools)?;
                self.pool_erc20_erc20s
                    .insert(&pool.id, [pool.token_0, _token_1].to_vec())?;
            }
            None => {
                if self.erc20_native_pools.get(&pool.token_0).await?.is_some() {
                    return Err(PoolError::AlreadyExists);
                }
                self.erc20_native_pools
                    .insert(&pool.token_0, pool.clone())?;
                self.pool_erc20_natives.insert(&pool.id, pool.token_0)?;
            }
        }
        Ok(())
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
    ) -> Result<(), PoolError> {
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
        self.insert_pool(pool).await
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
    ) -> Result<(), PoolError> {
        let mut pool = self.get_pool(pool_id).await?.expect("Invalid pool");
        pool.reserve_0 = balance_0;
        pool.reserve_1 = balance_1;
        // TODO: update price
        Ok(())
    }
}
