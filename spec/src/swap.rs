use crate::{
    account::ChainAccountOwner,
    base::{BaseMessage, BaseOperation},
    erc20::ERC20,
};
use async_graphql::{Context, Error, Request, Response, SimpleObject};
use linera_sdk::{
    abi::{ContractAbi, ServiceAbi},
    base::{Amount, ApplicationId, Timestamp},
    graphql::GraphQLMutationRoot,
};
use serde::{Deserialize, Serialize};

pub struct RouterApplicationAbi;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct PoolParameters {
    pub router_application_id: ApplicationId<RouterApplicationAbi>,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum PoolMessage {
    BaseMessage(BaseMessage),
    CreatePool {
        token_0: ApplicationId,
        // None means add pair to native token
        token_1: Option<ApplicationId>,
        amount_0_initial: Amount,
        amount_1_initial: Amount,
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
    },
    Swap {
        pool_id: u64,
        amount_0_out: Amount,
        amount_1_out: Amount,
        to: ChainAccountOwner,
    },
}

#[derive(Debug, Deserialize, Serialize, GraphQLMutationRoot)]
pub enum PoolOperation {
    BaseOperation(BaseOperation),
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
    },
    Swap {
        pool_id: u64,
        amount_0_out: Amount,
        amount_1_out: Amount,
        to: ChainAccountOwner,
    },
}

#[derive(Debug, Clone, Deserialize, Serialize, SimpleObject)]
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

#[derive(Debug, Deserialize, Serialize, Default)]
pub enum PoolResponse {
    #[default]
    Ok,
    Liquidity(Amount),
    Amounts((Amount, Amount)),
}

pub trait PoolQueryRoot {
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
}

pub trait PoolMutationRoot {
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

    fn swap(
        &self,
        ctx: &Context<'_>,
        pool_id: u64,
        amount_0_out: Amount,
        amount_1_out: Amount,
        to: ChainAccountOwner,
    ) -> impl std::future::Future<Output = Result<Vec<u8>, Error>> + Send;
}

#[derive(Debug, Deserialize, Serialize)]
pub enum RouterMessage {
    BaseMessage(BaseMessage),
    AddLiquidity {
        token_0: ApplicationId,
        token_1: Option<ApplicationId>,
        amount_0_desired: Amount,
        amount_1_desired: Amount,
        amount_0_min: Amount,
        amount_1_min: Amount,
        to: ChainAccountOwner,
        deadline: Timestamp,
    },
    RemoveLiquidity {
        token_0: ApplicationId,
        token_1: Option<ApplicationId>,
        liquidity: Amount,
        amount_0_min: Amount,
        amount_1_min: Amount,
        to: ChainAccountOwner,
        deadline: Timestamp,
    },
}

#[derive(Debug, Deserialize, Serialize, GraphQLMutationRoot)]
pub enum RouterOperation {
    BaseOperation(BaseOperation),
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
        to: ChainAccountOwner,
        deadline: Timestamp,
    },
    RemoveLiquidity {
        token_0: ApplicationId,
        token_1: Option<ApplicationId>,
        liquidity: Amount,
        amount_0_min: Amount,
        amount_1_min: Amount,
        to: ChainAccountOwner,
        deadline: Timestamp,
    },
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub enum RouterResponse {
    #[default]
    Ok,
    Liquidity((Amount, Amount, Amount)),
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
        to: ChainAccountOwner,
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
        to: ChainAccountOwner,
        deadline: Timestamp,
    ) -> impl std::future::Future<Output = Result<Vec<u8>, Error>> + Send;
}
