use linera_sdk::base::Amount;

pub trait ERC20 {
    fn total_supply(&self) -> Amount;
}
