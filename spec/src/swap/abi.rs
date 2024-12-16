use crate::{
    account::ChainAccountOwner,
    base::{BaseMessage, BaseOperation},
    swap::{
        pool::{Pool, PoolMessage, PoolOperation, PoolResponse},
        router::{RouterMessage, RouterOperation, RouterResponse},
        state::{SubscriberSyncState, Transaction},
    },
};
use async_graphql::{Context, Error, Request, Response};
use linera_sdk::{
    abi::{ContractAbi, ServiceAbi},
    base::{Amount, ApplicationId, Timestamp},
    graphql::GraphQLMutationRoot,
};
use serde::{Deserialize, Serialize};

pub struct SwapApplicationAbi;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SwapParameters {
    pub wlinera_application_id: Option<ApplicationId>,
    pub ams_application_id: Option<ApplicationId>,
    pub application_name: String,
    pub logo: String,
    pub description: String,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub enum SwapResponse {
    #[default]
    Ok,
    PoolResponse(PoolResponse),
    RouterResponse(RouterResponse),
}

#[derive(Debug, Deserialize, Serialize, GraphQLMutationRoot)]
pub enum SwapOperation {
    BaseOperation(BaseOperation),
    PoolOperation(PoolOperation),
    RouterOperation(RouterOperation),
}

#[derive(Debug, Deserialize, Serialize)]
pub enum SwapMessage {
    BaseMessage(BaseMessage),
    PoolMessage(PoolMessage),
    RouterMessage(RouterMessage),
    SubscriberSync {
        origin: ChainAccountOwner,
        state: SubscriberSyncState,
    },
}

impl ContractAbi for SwapApplicationAbi {
    type Operation = SwapOperation;
    type Response = SwapResponse;
}

impl ServiceAbi for SwapApplicationAbi {
    type Query = Request;
    type QueryResponse = Response;
}

pub trait SwapQueryRoot {
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

    fn get_transactions(
        &self,
        ctx: &Context<'_>,
        pool_id: Option<u64>,
        start_id: Option<u64>,
        start_timestamp: Option<Timestamp>,
    ) -> impl std::future::Future<Output = Result<Vec<Transaction>, Error>> + Send;
    
    fn get_owner_liquidity(
        &self,
        ctx: &Context<'_>,
        pool_id: u64,
        owner: ChainAccountOwner,
    ) -> impl std::future::Future<Output = Result<Amount, Error>> + Send;

    fn subscribed_creator_chain(
        &self,
        ctx: &Context<'_>,
    ) -> impl std::future::Future<Output = Result<bool, Error>> + Send;
}

pub trait SwapMutationRoot {
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

    // Let token creator create virtual initial liquidity
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
