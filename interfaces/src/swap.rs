use linera_sdk::base::{Account, Amount, ApplicationId, Timestamp};
use async_graphql::{Context, Error};

pub trait PoolQueryRoot {
    async fn get_pool(&self, ctx: &Context<'_>, token_0: ApplicationId, token_1: ApplicationId) -> Result<Option<u64>, Error>;

    async fn get_fee_to(&self, ctx: &Context<'_>) -> Result<Option<Account>, Error>;
}

pub trait PoolMutationRoot {
    // Just put all liquidity pool in one application
    async fn create_pool(&self, ctx: &Context<'_>, token_0: ApplicationId, token_1: ApplicationId) -> Result<Vec<u8>, Error>;

    async fn set_fee_to(&self, ctx: &Context<'_>, account: Account) -> Result<Vec<u8>, Error>;

    async fn set_fee_to_setter(&self, ctx: &Context<'_>, account: Account) -> Result<Vec<u8>, Error>;

    // Return minted liquidity
    async fn mint(&self, ctx: &Context<'_>, to: Account) -> Result<Vec<u8>, Error>;

    // Return pair token amount
    async fn burn(&self, ctx: &Context<'_>, to: Account) -> Result<Vec<u8>, Error>;

    async fn swap(&self, ctx: &Context<'_>, amount_0_out: Amount, amount_1_out: Amount, to: Account) -> Result<Vec<u8>, Error>;
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
        to: Account,
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
        to: Account,
        deadline: Timestamp,
    ) -> Result<Vec<u8>, Error>;
}
