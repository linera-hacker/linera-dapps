use async_graphql::{Context, Error};
use linera_sdk::{
    base::{Amount, ApplicationId, Timestamp},
    graphql::GraphQLMutationRoot,
};
use serde::{Deserialize, Serialize};
use crate::account::ChainAccountOwner;

#[derive(Debug, Deserialize, Serialize, GraphQLMutationRoot)]
pub enum PoolOperation {
    CreatePool {
        token_0: ApplicationId,
        token_1: ApplicationId,
    },
}

pub trait PoolQueryRoot {
    async fn get_pool(
        &self,
        ctx: &Context<'_>,
        token_0: ApplicationId,
        token_1: ApplicationId,
    ) -> Result<Option<u64>, Error>;

    async fn get_fee_to(&self, ctx: &Context<'_>) -> Result<Option<ChainAccountOwner>, Error>;
}

pub trait PoolMutationRoot {
    // Just put all liquidity pool in one application
    async fn create_pool(
        &self,
        ctx: &Context<'_>,
        token_0: ApplicationId,
        token_1: ApplicationId,
    ) -> Result<Vec<u8>, Error>;

    async fn set_fee_to(&self, ctx: &Context<'_>, account: ChainAccountOwner) -> Result<Vec<u8>, Error>;

    async fn set_fee_to_setter(
        &self,
        ctx: &Context<'_>,
        account: ChainAccountOwner,
    ) -> Result<Vec<u8>, Error>;

    // Return minted liquidity
    async fn mint(&self, ctx: &Context<'_>, to: ChainAccountOwner) -> Result<Vec<u8>, Error>;

    // Return pair token amount
    async fn burn(&self, ctx: &Context<'_>, to: ChainAccountOwner) -> Result<Vec<u8>, Error>;

    async fn swap(
        &self,
        ctx: &Context<'_>,
        amount_0_out: Amount,
        amount_1_out: Amount,
        to: ChainAccountOwner,
    ) -> Result<Vec<u8>, Error>;
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
    }
}

pub trait RouterQueryRoot {
    async fn example_func(&self, ctx: &Context<'_>) -> Result<u64, Error>;
}

pub trait RouterMutationRoot {
    // Return pair token amount and liquidity
    async fn add_liquidity(
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
    ) -> Result<Vec<u8>, Error>;

    // Return pair token amount
    async fn remove_liquidity(
        &self,
        ctx: &Context<'_>,
        token_0: ApplicationId,
        token_1: ApplicationId,
        liquidity: Amount,
        amount_0_min: Amount,
        amount_1_min: Amount,
        to: ChainAccountOwner,
        deadline: Timestamp,
    ) -> Result<Vec<u8>, Error>;
}
