use crate::account::ChainAccountOwner;
use async_graphql::{Context, Error};
use linera_sdk::{
    base::{Account, AccountOwner, Amount, ChainId},
    graphql::GraphQLMutationRoot,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, GraphQLMutationRoot)]
pub enum ERC20Operation {
    Transfer {
        from: Option<AccountOwner>,
        amount: Amount,
        to: ChainAccountOwner,
    },
}

pub trait ERC20QueryRoot {
    async fn total_supply(&self, ctx: &Context<'_>) -> Result<Amount, Error>;
    async fn name(&self, ctx: &Context<'_>) -> Result<String, Error>;
    async fn symbol(&self, ctx: &Context<'_>) -> Result<String, Error>;
    async fn decimals(&self, ctx: &Context<'_>) -> Result<u8, Error>;
    async fn balance_of(
        &self, 
        ctx: &Context<'_>,
        owner: ChainAccountOwner,
    ) -> Result<Amount, Error>;
}

pub trait ERC20MutationRoot {
    async fn transfer(
        &self,
        ctx: &Context<'_>,
        to: ChainAccountOwner,
        amount: Amount,
    ) -> Result<Vec<u8>, Error>;
    async fn transfer_from(
        &self,
        ctx: &Context<'_>,
        from: ChainAccountOwner,
        to: ChainAccountOwner,
        amount: Amount,
    ) -> Result<Vec<u8>, Error>;
    async fn approve(
        &self,
        ctx: &Context<'_>,
        spender: ChainAccountOwner,
        value: Amount,
    ) -> Result<Vec<u8>, Error>;
    async fn allowance(
        &self,
        ctx: &Context<'_>,
        owner: ChainAccountOwner,
        spender: ChainAccountOwner,
    ) -> Result<Vec<u8>, Error>;
}
