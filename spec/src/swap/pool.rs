use crate::{account::ChainAccountOwner, base, erc20::ERC20};
use async_graphql::{scalar, SimpleObject};
use linera_sdk::{
    base::{Amount, ApplicationId, Timestamp},
    graphql::GraphQLMutationRoot,
};
use serde::{Deserialize, Serialize};

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
    pub pool_fee_rate: Amount,
    pub protocol_fee_rate: Amount,
    pub erc20: ERC20,
    pub fee_to: ChainAccountOwner,
    pub fee_to_setter: ChainAccountOwner,
    pub price_0_cumulative: Amount,
    pub price_1_cumulative: Amount,
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
    Mint {
        origin: ChainAccountOwner,
        pool_id: u64,
        amount_0: Amount,
        amount_1: Amount,
        to: ChainAccountOwner,
    },
    Burn {
        origin: ChainAccountOwner,
        pool_id: u64,
        liquidity: Amount,
        to: ChainAccountOwner,
    },
    SwapWithPool {
        origin: ChainAccountOwner,
        pool_id: u64,
        amount_0_out: Amount,
        amount_1_out: Amount,
        to: ChainAccountOwner,
    },
}

#[derive(Debug, Deserialize, Serialize, GraphQLMutationRoot)]
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
    Mint {
        pool_id: u64,
        amount_0: Amount,
        amount_1: Amount,
        to: ChainAccountOwner,
    },
    Burn {
        pool_id: u64,
        liquidity: Amount,
        to: ChainAccountOwner,
    },
    SwapWithPool {
        pool_id: u64,
        amount_0_out: Amount,
        amount_1_out: Amount,
        to: ChainAccountOwner,
    },

    // Helper operation
    GetPoolWithTokenPair {
        token_0: ApplicationId,
        token_1: Option<ApplicationId>,
    },
}

scalar!(PoolOperation);

impl Pool {
    pub fn calculate_liquidity(&self, amount_0: Amount, amount_1: Amount) -> Amount {
        let total_supply = self.erc20.total_supply;

        if total_supply == Amount::ZERO {
            base::sqrt(amount_0.saturating_mul(amount_1.into()))
        } else {
            Amount::from_attos(
                amount_0
                    .saturating_mul(total_supply.into())
                    .saturating_div(self.reserve_0.into())
                    .min(
                        amount_1
                            .saturating_mul(total_supply.into())
                            .saturating_div(self.reserve_1.into()),
                    ),
            )
        }
    }

    pub fn calculate_amount_pair(
        &self,
        liquidity: Amount,
        balance_0: Amount,
        balance_1: Amount,
    ) -> (Amount, Amount) {
        let amount_0: Amount = Amount::from_attos(
            liquidity
                .saturating_mul(balance_0.into())
                .saturating_div(self.erc20.total_supply),
        );
        let amount_1: Amount = Amount::from_attos(
            liquidity
                .saturating_mul(balance_1.into())
                .saturating_div(self.erc20.total_supply),
        );
        if amount_0 == Amount::ZERO || amount_1 == Amount::ZERO {
            panic!("Invalid liquidity");
        }
        (amount_0, amount_1)
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
