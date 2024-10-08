use linera_sdk::base::{Account, Amount, ApplicationId, Timestamp};

pub trait PoolQueryRoot {
    fn get_pool(token_0: ApplicationId, token_1: ApplicationId) -> Option<u64>;

    fn get_fee_to() -> Option<Account>;
}

pub trait PoolMutationRoot {
    // Just put all liquidity pool in one application
    fn create_pool(token_0: ApplicationId, token_1: ApplicationId) -> Vec<u8>;

    fn set_fee_to(account: Account) -> Vec<u8>;

    fn set_fee_to_setter(account: Account) -> Vec<u8>;

    // Return minted liquidity
    fn mint(to: Account) -> Vec<u8>;

    // Return pair token amount
    fn burn(to: Account) -> Vec<u8>;

    fn swap(amount_0_out: Amount, amount_1_out: Amount, to: Account) -> Vec<u8>;
}

pub trait RouterQueryRoot {}

pub trait RouterMutationRoot {
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
    ) -> Vec<u8>;

    // Return pair token amount
    fn remove_liquidity(
        token_0: ApplicationId,
        token_1: ApplicationId,
        liquidity: Amount,
        amount_0_min: Amount,
        amount_1_min: Amount,
        to: Account,
        deadline: Timestamp,
    ) -> Vec<u8>;
}
