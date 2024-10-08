use linera_sdk::base::{Account, Amount, ApplicationId, Timestamp};

pub trait Pool {
    type Error: std::fmt::Debug;

    // Just put all liquidity pool in one application
    fn create_pool(token_0: ApplicationId, token_1: ApplicationId) -> Result<(), Self::Error>;

    fn set_fee_to(account: Account) -> Result<(), Self::Error>;

    fn set_fee_to_setter(account: Account) -> Result<(), Self::Error>;

    // Return minted liquidity
    fn mint(to: Account) -> Result<Amount, Self::Error>;

    // Return pair token amount
    fn burn(to: Account) -> Result<(Amount, Amount), Self::Error>;

    fn swap(amount_0_out: Amount, amount_1_out: Amount, to: Account) -> Result<(), Self::Error>;

    fn get_pool(token_0: ApplicationId, token_1: ApplicationId) -> Option<u64>;

    fn get_fee_to() -> Option<Account>;
}

pub trait Router {
    type Error: std::fmt::Debug;

    // Return pair token amount and liquidity
    fn add_liquidity(
        token_0: ApplicationId,
        token_1: ApplicationId,
        amount_0_desired: Amount,
        amount_1_desired: Amount,
        amount_0_min: Amount,
        amount_1_min: Amount,
        to: Account,
        deadline: Timestamp,
    ) -> Result<(Amount, Amount, Amount), Self::Error>;

    // Return pair token amount
    fn remove_liquidity(
        token_0: ApplicationId,
        token_1: ApplicationId,
        liquidity: Amount,
        amount_0_min: Amount,
        amount_1_min: Amount,
        to: Account,
        deadline: Timestamp,
    ) -> Result<(Amount, Amount), Self::Error>;
}
