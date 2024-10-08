use linera_sdk::base::{Account, Amount};
use async_graphql::{Context, Error};

pub trait ERC20QueryRoot {
    async fn total_supply(&self, ctx: &Context<'_>) -> Result<Amount, Error>;
}

pub trait ERC20MutationRoot {
    async fn transfer(&self, ctx: &Context<'_>, to: Account, amount: Amount) -> Result<Vec<u8>, Error>;
}
