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
    fn total_supply(
        &self,
        ctx: &Context<'_>,
    ) -> impl std::future::Future<Output = Result<Amount, Error>> + Send;
}

pub trait ERC20MutationRoot {
    fn transfer(
        &self,
        ctx: &Context<'_>,
        to: ChainAccountOwner,
        amount: Amount,
    ) -> impl std::future::Future<Output = Result<Vec<u8>, Error>> + Send;
}
