use crate::{
    account::ChainAccountOwner,
    base::{self, BaseMessage, BaseOperation},
    erc20::ERC20,
};
use async_graphql::{scalar, Context, Error, Request, Response, SimpleObject};
use linera_sdk::{
    abi::{ContractAbi, ServiceAbi},
    base::{Amount, ApplicationId, Timestamp},
    graphql::GraphQLMutationRoot,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub struct RouterApplicationAbi;

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

#[derive(Debug, Deserialize, Serialize)]
pub enum PoolResponse {
    Liquidity(Amount),
    AmountPair((Amount, Amount)),
    Pool(Option<Pool>),
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct RouterSubscriberSyncState {
    pub erc20_erc20_pools: HashMap<ApplicationId, HashMap<ApplicationId, Pool>>,
    pub erc20_native_pools: HashMap<ApplicationId, Pool>,
    pub pool_id: u64,
    pub pool_erc20_erc20s: HashMap<u64, Vec<ApplicationId>>,
    pub pool_erc20_natives: HashMap<u64, ApplicationId>,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum RouterMessage {
    BaseMessage(BaseMessage),
    PoolMessage(PoolMessage),
    AddLiquidity {
        origin: ChainAccountOwner,
        token_0: ApplicationId,
        token_1: Option<ApplicationId>,
        amount_0_desired: Amount,
        amount_1_desired: Amount,
        amount_0_min: Amount,
        amount_1_min: Amount,
        created_pool: bool,
        to: ChainAccountOwner,
        deadline: Timestamp,
    },
    RemoveLiquidity {
        origin: ChainAccountOwner,
        token_0: ApplicationId,
        token_1: Option<ApplicationId>,
        liquidity: Amount,
        amount_0_min: Amount,
        amount_1_min: Amount,
        to: ChainAccountOwner,
        deadline: Timestamp,
    },
    Swap {
        origin: ChainAccountOwner,
        token_0: ApplicationId,
        token_1: Option<ApplicationId>,
        amount_0_in: Option<Amount>,
        amount_1_in: Option<Amount>,
        amount_0_out_min: Option<Amount>,
        amount_1_out_min: Option<Amount>,
        to: ChainAccountOwner,
    },
    SubscriberSync {
        origin: ChainAccountOwner,
        state: RouterSubscriberSyncState,
    },
}

#[derive(Debug, Deserialize, Serialize, GraphQLMutationRoot)]
pub enum RouterOperation {
    BaseOperation(BaseOperation),
    PoolOperation(PoolOperation),
    CalculateSwapAmount {
        token_0: ApplicationId,
        token_1: Option<ApplicationId>,
        amount_1: Amount,
    },
    AddLiquidity {
        token_0: ApplicationId,
        token_1: Option<ApplicationId>,
        amount_0_desired: Amount,
        amount_1_desired: Amount,
        amount_0_min: Amount,
        amount_1_min: Amount,
        to: Option<ChainAccountOwner>,
        deadline: Timestamp,
    },
    RemoveLiquidity {
        token_0: ApplicationId,
        token_1: Option<ApplicationId>,
        liquidity: Amount,
        amount_0_min: Amount,
        amount_1_min: Amount,
        to: Option<ChainAccountOwner>,
        deadline: Timestamp,
    },
    Swap {
        token_0: ApplicationId,
        token_1: Option<ApplicationId>,
        amount_0_in: Option<Amount>,
        amount_1_in: Option<Amount>,
        amount_0_out_min: Option<Amount>,
        amount_1_out_min: Option<Amount>,
        to: Option<ChainAccountOwner>,
    },
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub enum RouterResponse {
    #[default]
    Ok,
    Liquidity((Amount, Amount, Amount)),
    Amount(Amount),
    AmountPair((Amount, Amount)),
    PoolResponse(PoolResponse),
}

impl ContractAbi for RouterApplicationAbi {
    type Operation = RouterOperation;
    type Response = RouterResponse;
}

impl ServiceAbi for RouterApplicationAbi {
    type Query = Request;
    type QueryResponse = Response;
}

pub trait RouterQueryRoot {
    // Return swap amount
    fn calculate_swap_amount(
        &self,
        ctx: &Context<'_>,
        token_0: ApplicationId,
        token_1: Option<ApplicationId>,
        amount_1: Amount,
    ) -> impl std::future::Future<Output = Result<Amount, Error>> + Send;

    fn get_pool(
        &self,
        ctx: &Context<'_>,
        pool_id: u64,
    ) -> impl std::future::Future<Output = Result<Option<Pool>, Error>> + Send;

    fn get_fee_to(
        &self,
        ctx: &Context<'_>,
        pool_id: u64,
    ) -> impl std::future::Future<Output = Result<Option<ChainAccountOwner>, Error>> + Send;

    fn get_pools(
        &self,
        ctx: &Context<'_>,
    ) -> impl std::future::Future<Output = Result<Vec<Pool>, Error>> + Send;
}

pub trait RouterMutationRoot {
    // Return pair token amount and liquidity
    fn add_liquidity(
        &self,
        ctx: &Context<'_>,
        token_0: ApplicationId,
        token_1: Option<ApplicationId>,
        amount_0_desired: Amount,
        amount_1_desired: Amount,
        amount_0_min: Amount,
        amount_1_min: Amount,
        to: Option<ChainAccountOwner>,
        deadline: Timestamp,
    ) -> impl std::future::Future<Output = Result<Vec<u8>, Error>> + Send;

    // Return pair token amount
    fn remove_liquidity(
        &self,
        ctx: &Context<'_>,
        token_0: ApplicationId,
        token_1: Option<ApplicationId>,
        liquidity: Amount,
        amount_0_min: Amount,
        amount_1_min: Amount,
        to: Option<ChainAccountOwner>,
        deadline: Timestamp,
    ) -> impl std::future::Future<Output = Result<Vec<u8>, Error>> + Send;

    fn swap(
        &self,
        ctx: &Context<'_>,
        token_0: ApplicationId,
        token_1: Option<ApplicationId>,
        amount_0_in: Option<Amount>,
        amount_1_in: Option<Amount>,
        amount_0_out_min: Option<Amount>,
        amount_1_out_min: Option<Amount>,
        to: Option<ChainAccountOwner>,
    ) -> impl std::future::Future<Output = Result<Vec<u8>, Error>> + Send;

    // Just put all liquidity pool in one application
    fn create_pool(
        &self,
        ctx: &Context<'_>,
        token_0: ApplicationId,
        token_1: Option<ApplicationId>,
        amount_0_initial: Amount,
        amount_1_initial: Amount,
        amount_0_virtual: Amount,
        amount_1_virtual: Amount,
    ) -> impl std::future::Future<Output = Result<Vec<u8>, Error>> + Send;

    fn get_pool_with_token_pair(
        &self,
        ctx: &Context<'_>,
        token_0: ApplicationId,
        token_1: Option<ApplicationId>,
    ) -> impl std::future::Future<Output = Result<Vec<u8>, Error>> + Send;

    fn set_fee_to(
        &self,
        ctx: &Context<'_>,
        pool_id: u64,
        account: ChainAccountOwner,
    ) -> impl std::future::Future<Output = Result<Vec<u8>, Error>> + Send;

    fn set_fee_to_setter(
        &self,
        ctx: &Context<'_>,
        pool_id: u64,
        account: ChainAccountOwner,
    ) -> impl std::future::Future<Output = Result<Vec<u8>, Error>> + Send;

    // Return minted liquidity
    fn mint(
        &self,
        ctx: &Context<'_>,
        pool_id: u64,
        amount_0: Amount,
        amount_1: Amount,
        to: ChainAccountOwner,
    ) -> impl std::future::Future<Output = Result<Vec<u8>, Error>> + Send;

    // Return pair token amount
    fn burn(
        &self,
        ctx: &Context<'_>,
        pool_id: u64,
        liquidity: Amount,
    ) -> impl std::future::Future<Output = Result<Vec<u8>, Error>> + Send;

    fn swap_with_pool(
        &self,
        ctx: &Context<'_>,
        pool_id: u64,
        amount_0_out: Amount,
        amount_1_out: Amount,
        to: ChainAccountOwner,
    ) -> impl std::future::Future<Output = Result<Vec<u8>, Error>> + Send;

    // TODO: how to inherit trait from base
    fn subscribe_creator_chain(
        &self,
        ctx: &Context<'_>,
    ) -> impl std::future::Future<Output = Result<Vec<u8>, Error>> + Send;
}
