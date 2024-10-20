use crate::{
    account::ChainAccountOwner,
    base::{self, BigAmount},
    erc20::ERC20,
};
use async_graphql::{scalar, SimpleObject};
use linera_sdk::{
    base::{Amount, ApplicationId, Timestamp},
    graphql::GraphQLMutationRoot,
};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Clone, Deserialize, Eq, PartialEq, Serialize, SimpleObject)]
pub struct Pool {
    pub id: u64,
    pub token_0: ApplicationId,
    // None means add pair to native token
    pub token_1: Option<ApplicationId>,
    pub virtual_initial_liquidity: bool,
    pub amount_0_initial: Amount,
    pub amount_1_initial: Amount,
    pub reserve_0: Amount,
    pub reserve_1: Amount,
    pub pool_fee_percent: u16,
    pub protocol_fee_percent: u16,
    pub erc20: ERC20,
    pub fee_to: ChainAccountOwner,
    pub fee_to_setter: ChainAccountOwner,
    pub price_0_cumulative: BigAmount,
    pub price_1_cumulative: BigAmount,
    pub k_last: Amount,
    pub block_timestamp: Timestamp,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum PoolMessage {
    // Only for application creator to create pool with virtual initial liquidity
    CreatePool {
        origin: ChainAccountOwner,
        token_0: ApplicationId,
        // None means add pair to native token
        token_1: Option<ApplicationId>,
        amount_0_initial: Amount,
        amount_1_initial: Amount,
        amount_0_virtual: Amount,
        amount_1_virtual: Amount,
        block_timestamp: Timestamp,
    },
    SetFeeTo {
        origin: ChainAccountOwner,
        pool_id: u64,
        account: ChainAccountOwner,
    },
    SetFeeToSetter {
        origin: ChainAccountOwner,
        pool_id: u64,
        account: ChainAccountOwner,
    },
}

#[derive(Debug, Copy, Clone, Deserialize, Serialize, GraphQLMutationRoot)]
pub enum PoolOperation {
    // Only for application creator to create pool with virtual initial liquidity
    CreatePool {
        token_0: ApplicationId,
        // None means add pair to native token
        token_1: Option<ApplicationId>,
        // Actual deposited initial liquidity
        // New listed token must not be 0
        amount_0_initial: Amount,
        amount_1_initial: Amount,
        // Virtual initial liquidity
        // Both must not be 0, new listed token virtual liquidity must be equal to initial
        // liquidity. If both initial amounts are not 0, then both virtual must be equal to initial
        amount_0_virtual: Amount,
        amount_1_virtual: Amount,
    },
    SetFeeTo {
        pool_id: u64,
        account: ChainAccountOwner,
    },
    SetFeeToSetter {
        pool_id: u64,
        account: ChainAccountOwner,
    },
}

scalar!(PoolOperation);

#[derive(Debug, Error)]
#[allow(dead_code)]
pub enum PoolError {
    #[error("Invalid liquidity")]
    InvalidLiquidity,

    #[error("Invalid amount")]
    InvalidAmount,

    #[error("Invalid liquidity")]
    InsufficientLiquidity,

    #[error("Broken K")]
    BrokenK,
}

impl Pool {
    pub fn calculate_initial_liquidity(amount_0: Amount, amount_1: Amount) -> Amount {
        base::mul_then_sqrt(amount_0, amount_1)
    }

    pub fn calculate_liquidity(&self, amount_0: Amount, amount_1: Amount) -> Amount {
        let total_supply = self.erc20.total_supply;

        if total_supply == Amount::ZERO {
            base::mul_then_sqrt(amount_0, amount_1)
        } else {
            base::mul_then_div(amount_0, total_supply, self.reserve_0).min(base::mul_then_div(
                amount_1,
                total_supply,
                self.reserve_1,
            ))
        }
    }

    pub fn calculate_liquidity_amount_pair(
        &self,
        liquidity: Amount,
        balance_0: Amount,
        balance_1: Amount,
    ) -> Result<(Amount, Amount), PoolError> {
        let amount_0 = base::mul_then_div(liquidity, balance_0, self.erc20.total_supply);
        let amount_1 = base::mul_then_div(liquidity, balance_1, self.erc20.total_supply);
        if amount_0 == Amount::ZERO || amount_1 == Amount::ZERO {
            return Err(PoolError::InvalidAmount);
        }
        Ok((amount_0, amount_1))
    }

    pub fn calculate_swap_amount_1(&self, amount_0: Amount) -> Result<Amount, PoolError> {
        if self.reserve_0 <= Amount::ZERO || self.reserve_1 <= Amount::ZERO {
            return Err(PoolError::InvalidAmount);
        }
        Ok(base::mul_then_div(amount_0, self.reserve_1, self.reserve_0))
    }

    pub fn calculate_swap_amount_0(&self, amount_1: Amount) -> Result<Amount, PoolError> {
        if self.reserve_0 <= Amount::ZERO || self.reserve_1 <= Amount::ZERO {
            return Err(PoolError::InvalidAmount);
        }
        Ok(base::mul_then_div(amount_1, self.reserve_0, self.reserve_1))
    }

    pub fn calculate_swap_amount_pair(
        &self,
        amount_0_desired: Amount,
        amount_1_desired: Amount,
        amount_0_min: Amount,
        amount_1_min: Amount,
    ) -> Result<(Amount, Amount), PoolError> {
        if self.reserve_0 == Amount::ZERO && self.reserve_1 == Amount::ZERO {
            return Ok((amount_0_desired, amount_1_desired));
        }
        let amount_1_optimal = self.calculate_swap_amount_1(amount_0_desired)?;
        if amount_1_optimal <= amount_1_desired {
            if amount_1_optimal < amount_1_min {
                return Err(PoolError::InvalidAmount);
            }
            return Ok((amount_0_desired, amount_1_optimal));
        }
        let amount_0_optimal = self.calculate_swap_amount_0(amount_1_desired)?;
        if amount_0_optimal > amount_0_desired {
            return Err(PoolError::InvalidAmount);
        }
        if amount_0_optimal < amount_0_min {
            return Err(PoolError::InvalidAmount);
        }
        Ok((amount_0_optimal, amount_1_desired))
    }

    pub fn calculate_adjust_amount_pair(
        &self,
        amount_0_out: Amount,
        amount_1_out: Amount,
    ) -> Result<(Amount, Amount), PoolError> {
        let amount_0_in = if self.reserve_0 > amount_0_out {
            amount_0_out
        } else {
            Amount::ZERO
        };
        let amount_1_in = if self.reserve_1 > amount_1_out {
            amount_1_out
        } else {
            Amount::ZERO
        };
        if amount_0_in <= Amount::ZERO && amount_1_in <= Amount::ZERO {
            return Err(PoolError::InsufficientLiquidity);
        }
        let balance_0_adjusted = self
            .reserve_0
            .saturating_sub(base::mul_percent_10000(amount_0_in, self.pool_fee_percent));
        let balance_1_adjusted = self
            .reserve_1
            .saturating_sub(base::mul_percent_10000(amount_1_in, self.pool_fee_percent));
        if base::mul(balance_0_adjusted, balance_1_adjusted)
            >= base::mul(self.reserve_0, self.reserve_1)
        {
            return Err(PoolError::BrokenK);
        }
        Ok((amount_0_in, amount_1_in))
    }

    pub fn calculate_price_cumulative_pair(&self, time_elapsed: u128) -> (BigAmount, BigAmount) {
        let mut price_0_cumulative = self.price_0_cumulative.clone();
        let mut price_1_cumulative = self.price_1_cumulative.clone();

        if time_elapsed > 0 && self.reserve_0 > Amount::ZERO && self.reserve_1 > Amount::ZERO {
            price_0_cumulative =
                self.price_0_cumulative
                    .clone()
                    .add(base::div_then_mul_to_big_amount(
                        self.reserve_1,
                        self.reserve_0,
                        Amount::from_attos(time_elapsed),
                    ));
            price_1_cumulative =
                self.price_1_cumulative
                    .clone()
                    .add(base::div_then_mul_to_big_amount(
                        self.reserve_0,
                        self.reserve_1,
                        Amount::from_attos(time_elapsed),
                    ));
        }

        (price_0_cumulative, price_1_cumulative)
    }
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub enum PoolResponse {
    #[default]
    Ok,
    Liquidity(Amount),
    AmountPair((Amount, Amount)),
    Pool(Option<Pool>),
}
