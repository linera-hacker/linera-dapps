use crate::account::ChainAccountOwner;
use async_graphql::{Context, Error};
use linera_sdk::{
    base::{Amount, ApplicationId, Timestamp},
    graphql::GraphQLMutationRoot,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, GraphQLMutationRoot)]
pub enum PoolOperation {
    CreatePool {
        token_0: ApplicationId,
        token_1: ApplicationId,
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
        account: ChainAccountOwner,
    },
    SetFeeToSetter {
        account: ChainAccountOwner,
    },
    Mint {
        to: ChainAccountOwner,
    },
    Burn {
        to: ChainAccountOwner,
    },
    Swap {
        amount_0_out: Amount,
        amount_1_out: Amount,
        to: ChainAccountOwner,
    },
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub enum PoolResponse {
    #[default]
    Ok,
    PoolId(u64),
}

pub trait PoolQueryRoot {
    fn get_pool(
        &self,
        ctx: &Context<'_>,
        token_0: ApplicationId,
        token_1: ApplicationId,
    ) -> impl std::future::Future<Output = Result<Option<u64>, Error>> + Send;

    fn get_fee_to(
        &self,
        ctx: &Context<'_>,
    ) -> impl std::future::Future<Output = Result<Option<ChainAccountOwner>, Error>> + Send;
}

pub trait PoolMutationRoot {
    // Just put all liquidity pool in one application
    fn create_pool(
        &self,
        ctx: &Context<'_>,
        token_0: ApplicationId,
        token_1: ApplicationId,
        amount_0_initial: Amount,
        amount_1_initial: Amount,
        amount_0_virtual: Amount,
        amount_1_virtual: Amount,
    ) -> impl std::future::Future<Output = Result<Vec<u8>, Error>> + Send;

    fn set_fee_to(
        &self,
        ctx: &Context<'_>,
        account: ChainAccountOwner,
    ) -> impl std::future::Future<Output = Result<Vec<u8>, Error>> + Send;

    fn set_fee_to_setter(
        &self,
        ctx: &Context<'_>,
        account: ChainAccountOwner,
    ) -> impl std::future::Future<Output = Result<Vec<u8>, Error>> + Send;

    // Return minted liquidity
    fn mint(
        &self,
        ctx: &Context<'_>,
        to: ChainAccountOwner,
    ) -> impl std::future::Future<Output = Result<Vec<u8>, Error>> + Send;

    // Return pair token amount
    fn burn(
        &self,
        ctx: &Context<'_>,
        to: ChainAccountOwner,
    ) -> impl std::future::Future<Output = Result<Vec<u8>, Error>> + Send;

    fn swap(
        &self,
        ctx: &Context<'_>,
        amount_0_out: Amount,
        amount_1_out: Amount,
        to: ChainAccountOwner,
    ) -> impl std::future::Future<Output = Result<Vec<u8>, Error>> + Send;
}

#[derive(Debug, Deserialize, Serialize, GraphQLMutationRoot)]
pub enum RouterOperation {
    AddLiquidity {
        token_0: ApplicationId,
        token_1: ApplicationId,
        amount_0_desired: Amount,
        amount_1_desired: Amount,
        amount_0_min: Amount,
        amount_1_min: Amount,
        to: ChainAccountOwner,
        deadline: Timestamp,
    },
    RemoveLiquidity {
        token_0: ApplicationId,
        token_1: ApplicationId,
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

pub trait RouterQueryRoot {
    fn example_func(
        &self,
        ctx: &Context<'_>,
    ) -> impl std::future::Future<Output = Result<u64, Error>> + Send;
}

pub trait RouterMutationRoot {
    // Return pair token amount and liquidity
    fn add_liquidity(
        &self,
        ctx: &Context<'_>,
        token_0: ApplicationId,
        token_1: ApplicationId,
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
        token_1: ApplicationId,
        liquidity: Amount,
        amount_0_min: Amount,
        amount_1_min: Amount,
        to: ChainAccountOwner,
        deadline: Timestamp,
    ) -> impl std::future::Future<Output = Result<Vec<u8>, Error>> + Send;
}
