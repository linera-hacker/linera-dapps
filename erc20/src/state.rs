use erc20::ERC20Error;
use linera_sdk::base::Amount;
use linera_sdk::views::{
    linera_views, MapView, RegisterView, RootView, ViewStorageContext,
};
use serde::{Deserialize, Serialize};
use spec::{account::ChainAccountOwner, erc20::InstantiationArgument};

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
    pub name: RegisterView<String>,
    pub symbol: RegisterView<String>,
    pub decimals: RegisterView<u8>,
    pub initial_currency: RegisterView<Amount>,
    pub fixed_currency: RegisterView<bool>,
    pub fee_percent: RegisterView<Amount>,
    pub created_owner: RegisterView<Option<ChainAccountOwner>>,
}

#[allow(dead_code)]
impl Application {
    pub(crate) async fn instantiate(
        &mut self,
        argument: InstantiationArgument,
        owner: ChainAccountOwner,
    ) {
        self.created_owner.set(Some(owner));
        self.total_supply.set(argument.initial_supply);
        let _ = self.balances.insert(&owner, argument.initial_supply);
        self.name.set(argument.name);
        self.symbol.set(argument.symbol);
        self.decimals.set(argument.decimals);
        self.fixed_currency
            .set(argument.fixed_currency.unwrap_or(false));
        self.initial_currency
            .set(argument.initial_currency.unwrap_or(Amount::ONE));
        self.fee_percent.set(argument.fee_percent.unwrap_or(Amount::ZERO));
    }

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

        let new_sender_balance = sender_balance.saturating_sub(amount);
        let receiver_balance = match self.balances.get(&to).await {
            Ok(Some(balance)) => balance,
            Ok(None) => Amount::ZERO,
            Err(_) => Amount::ZERO,
        };
        let fee_percent = *self.fee_percent.get();
        let fee = amount.saturating_mul(fee_percent.into());
        let send_amount = amount.try_sub(fee).expect("Invalid sub send amount");
        let new_receiver_balance = receiver_balance.saturating_add(send_amount);

        let _ = self.balances.insert(&sender, new_sender_balance);
        let _ = self.balances.insert(&to, new_receiver_balance);
        let created_owner = self.created_owner.get().expect("Invalid created owner");
        if fee > Amount::ZERO {
            let _ = self.balances.insert(&created_owner, fee);
        }
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

        let new_from_balance = from_balance.saturating_sub(amount);
        let new_allowance = allowance.saturating_sub(amount);

        let to_balance = match self.balances.get(&to).await {
            Ok(Some(balance)) => balance,
            Ok(None) => amount,
            Err(_) => amount,
        };
        let fee_percent = *self.fee_percent.get();
        let fee = amount.saturating_mul(fee_percent.into());
        let send_amount = amount.try_sub(fee).expect("Invalid sub send amount");
        let new_to_balance = to_balance.saturating_add(send_amount);

        let _ = self.balances.insert(&from, new_from_balance);
        let _ = self.balances.insert(&to, new_to_balance);
        let _ = self.allowances.insert(&allowance_key, new_allowance);
        let created_owner = self.created_owner.get().expect("Invalid created owner");
        if fee > Amount::ZERO {
            let _ = self.balances.insert(&created_owner, fee);
        }

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

    pub(crate) async fn deposit_native_and_exchange(
        &mut self,
        caller: ChainAccountOwner,
        exchange_amount: Amount,
        currency: Amount,
    ) {
        let mut exchange_currency = self.initial_currency.get();
        if !self.fixed_currency.get() {
            exchange_currency = &currency
        }
        let erc20_amount = exchange_currency.saturating_mul(exchange_amount.into());

        let user_balance = match self.balances.get(&caller).await {
            Ok(Some(balance)) => balance,
            Ok(None) => Amount::ZERO,
            Err(_) => Amount::ZERO,
        };

        let new_user_balance = user_balance.saturating_add(erc20_amount);
        let _ = self.balances.insert(&caller, new_user_balance);
    }

    pub(crate) async fn airdrop(
        &mut self,
        airdrop_amount: Amount,
        airdrop_owners: Vec<ChainAccountOwner>,
    ) -> Result<(), ERC20Error> {
        let created_owner = self.created_owner.get().expect("Invalid created owner");
        for owner in airdrop_owners {
            let allowance_key = AllowanceKey::new(created_owner.clone(), owner);
            let allowance = match self.allowances.get(&allowance_key).await {
                Ok(Some(balance)) => balance,
                Ok(None) => Amount::ZERO,
                Err(_) => Amount::ZERO,
            };

            if allowance < airdrop_amount {
                return Err(ERC20Error::InvalidInitialAmount);
            }

            let from_balance = match self.balances.get(&created_owner).await {
                Ok(Some(balance)) => balance,
                Ok(None) => Amount::ZERO,
                Err(_) => Amount::ZERO,
            };
    
            if from_balance < airdrop_amount {
                return Err(ERC20Error::InvalidInitialAmount);
            }
    
            let to_balance = match self.balances.get(&owner).await {
                Ok(Some(balance)) => balance,
                Ok(None) => airdrop_amount,
                Err(_) => airdrop_amount,
            };

            let new_from_balance = from_balance.saturating_sub(airdrop_amount);
            let new_to_balance = to_balance.saturating_add(airdrop_amount);
            let new_allowance = allowance.saturating_sub(airdrop_amount);
            
            let _ = self.balances.insert(&created_owner, new_from_balance);
            let _ = self.balances.insert(&owner, new_to_balance);
            let _ = self.allowances.insert(&allowance_key, new_allowance);
        }
        Ok(())
    }
}
