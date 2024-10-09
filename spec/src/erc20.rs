use crate::account::ChainAccountOwner;
use async_graphql::{Context, Error};
use linera_sdk::{
    base::{Account, AccountOwner, Amount, ChainId},
    graphql::GraphQLMutationRoot,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, GraphQLMutationRoot)]
pub enum Operation {
    Transfer {
        from: Option<AccountOwner>,
        amount: Amount,
        to: ChainAccountOwner,
    },
}

pub trait ERC20QueryRoot {
    async fn total_supply(&self, ctx: &Context<'_>) -> Result<Amount, Error>;
}

pub trait ERC20MutationRoot {
    async fn transfer(
        &self,
        ctx: &Context<'_>,
        to: Account,
        amount: Amount,
    ) -> Result<Vec<u8>, Error>;
}
