use linera_sdk::base::{Account, Amount};

pub trait ERC20QueryRoot {
    fn total_supply(&self) -> Amount;
}

pub trait ERC20MutationRoot {
    type Error: std::fmt::Debug;

    fn transfer(to: Account, amount: Amount) -> Vec<u8>;
}
