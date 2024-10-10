use erc20::ERC20Error;
use linera_sdk::base::Amount;
use linera_sdk::views::{linera_views, MapView, RegisterView, RootView, ViewStorageContext};
use serde::{Deserialize, Serialize};
use spec::account::ChainAccountOwner;

#[derive(
    Serialize, Deserialize, Debug, Clone, async_graphql::SimpleObject, async_graphql::InputObject,
)]
pub struct AllowanceKey {
    pub owner: ChainAccountOwner,
    pub spender: ChainAccountOwner,
}

impl AllowanceKey {
    pub fn new(owner: ChainAccountOwner, spender: ChainAccountOwner) -> Self {
        Self { owner, spender }
    }
}

#[derive(RootView, async_graphql::SimpleObject)]
#[view(context = "ViewStorageContext")]
pub struct Application {
    pub value: RegisterView<u64>,
    // Add fields here.
    pub total_supply: RegisterView<Amount>,
    pub balances: MapView<ChainAccountOwner, Amount>,
    pub allowances: MapView<AllowanceKey, Amount>,
}

#[allow(dead_code)]
impl Application {
    pub(crate) async fn transfer(
        &mut self,
        sender: ChainAccountOwner,
        amount: Amount,
        to: ChainAccountOwner,
    ) -> Result<(), ERC20Error> {
        let sender_balance = match self.balances.get(&sender).await {
            Ok(Some(balance)) => balance,
            Ok(None) => Amount::ZERO,
            Err(_) => Amount::ZERO,
        };

        if sender_balance < amount {
            return Err(ERC20Error::InvalidInitialAmount);
        }

        let new_sender_balance = sender_balance - amount;
        let receiver_balance = match self.balances.get(&to).await {
            Ok(Some(balance)) => balance,
            Ok(None) => Amount::ZERO,
            Err(_) => Amount::ZERO,
        };

        let _ = self.balances.insert(&sender, new_sender_balance);
        let _ = self.balances.insert(&to, receiver_balance);
        Ok(())
    }

    pub(crate) async fn transfer_from(
        &mut self,
        from: ChainAccountOwner,
        amount: Amount,
        to: ChainAccountOwner,
        caller: ChainAccountOwner,
    ) -> Result<(), ERC20Error> {
        let allowance_key = AllowanceKey::new(from.clone(), caller);
        let allowance = match self.allowances.get(&allowance_key).await {
            Ok(Some(balance)) => balance,
            Ok(None) => Amount::ZERO,
            Err(_) => Amount::ZERO,
        };

        if allowance < amount {
            return Err(ERC20Error::InvalidInitialAmount);
        }

        let from_balance = match self.balances.get(&from).await {
            Ok(Some(balance)) => balance,
            Ok(None) => Amount::ZERO,
            Err(_) => Amount::ZERO,
        };

        if from_balance < amount {
            return Err(ERC20Error::InvalidInitialAmount);
        }

        let new_from_balance = from_balance - amount;
        let new_allowance = allowance - amount;
        let to_balance = match self.balances.get(&to).await {
            Ok(Some(balance)) => balance + amount,
            Ok(None) => amount,
            Err(_) => amount,
        };

        let _ = self.balances.insert(&from, new_from_balance);
        let _ = self.balances.insert(&to, to_balance);
        let _ = self.allowances.insert(&allowance_key, new_allowance);

        Ok(())
    }

    pub(crate) async fn approve(
        &mut self,
        spender: ChainAccountOwner,
        value: Amount,
        owner: ChainAccountOwner,
    ) -> Result<(), ERC20Error> {
        let allowance_key = AllowanceKey::new(owner, spender.clone());
        let _ = self.allowances.insert(&allowance_key, value);

        Ok(())
    }
}
