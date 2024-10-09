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
    pub pools: MapView<ApplicationId, HashMap<ApplicationId, Pool>>,
    pub pool_id: RegisterView<u64>,
}

impl Application {
    pub(crate) async fn create_pool(
        &mut self,
        token_0: ApplicationId,
        token_1: ApplicationId,
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
            virtual_initial_liquidity: amount_0_initial == amount_0_virtual
                && amount_1_initial == amount_1_virtual,
            amount_0_initial,
            amount_1_initial,
            pool_fee_rate: Amount::from_str("0.3")?,
            protocol_fee_rate: Amount::from_str("0.05")?,
            erc20: ERC20::default(),
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
        let mut token_pools = self.pools.get(&token_0).await?.unwrap_or(HashMap::new());
        match token_pools.get(&token_1) {
            Some(_) => return Err(PoolError::AlreadyExists),
            _ => {}
        }
        token_pools.insert(token_1, pool);
        self.pools.insert(&token_0, token_pools)?;

        Ok(())
    }
}
