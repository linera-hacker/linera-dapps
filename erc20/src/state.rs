use erc20::ERC20Error;
use linera_sdk::base::Amount;
use linera_sdk::views::{linera_views, MapView, RegisterView, RootView, ViewStorageContext};
use serde::{Deserialize, Serialize};
use spec::{account::ChainAccountOwner, erc20::InstantiationArgument};

use std::collections::HashMap;

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
    pub minted_supply: RegisterView<Amount>,
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
        self.name.set(argument.name);
        self.symbol.set(argument.symbol);
        self.decimals.set(argument.decimals);
        self.fixed_currency
            .set(argument.fixed_currency.unwrap_or(false));
        self.initial_currency
            .set(argument.initial_currency.unwrap_or(Amount::ONE));
        self.fee_percent
            .set(argument.fee_percent.unwrap_or(Amount::ZERO));
        self.minted_supply.set(Amount::ZERO);
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

        log::info!("Sender {:?} balance {} amount {}", sender, sender_balance, amount);

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
        let fee = Amount::from_attos(
            amount
                .saturating_mul(fee_percent.into())
                .saturating_div(Amount::ONE),
        );
        let send_amount = amount.saturating_sub(fee);
        let new_receiver_balance = receiver_balance.saturating_add(send_amount);

        let _ = self.balances.insert(&sender, new_sender_balance);
        let _ = self.balances.insert(&to, new_receiver_balance);
        let created_owner = self.created_owner.get().expect("Invalid created owner");
        if fee > Amount::ZERO {
            let created_owner_balance = match self.balances.get(&created_owner).await {
                Ok(Some(balance)) => balance,
                Ok(None) => Amount::ZERO,
                Err(_) => Amount::ZERO,
            };
            let new_created_owner_balance = created_owner_balance.saturating_add(fee);
            let _ = self
                .balances
                .insert(&created_owner, new_created_owner_balance);
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
        let fee = Amount::from_attos(
            amount
                .saturating_mul(fee_percent.into())
                .saturating_div(Amount::ONE),
        );
        let send_amount = amount.saturating_sub(fee);
        let new_to_balance = to_balance.saturating_add(send_amount);

        let _ = self.balances.insert(&from, new_from_balance);
        let _ = self.balances.insert(&to, new_to_balance);
        let _ = self.allowances.insert(&allowance_key, new_allowance);
        let created_owner = self.created_owner.get().expect("Invalid created owner");
        if fee > Amount::ZERO {
            let created_owner_balance = match self.balances.get(&created_owner).await {
                Ok(Some(balance)) => balance,
                Ok(None) => Amount::ZERO,
                Err(_) => Amount::ZERO,
            };
            let new_created_owner_balance = created_owner_balance.saturating_add(fee);
            let _ = self
                .balances
                .insert(&created_owner, new_created_owner_balance);
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
        let erc20_amount = Amount::from_attos(
            exchange_currency
                .saturating_mul(exchange_amount.into())
                .saturating_div(Amount::ONE),
        );

        let user_balance = match self.balances.get(&caller).await {
            Ok(Some(balance)) => balance,
            Ok(None) => Amount::ZERO,
            Err(_) => Amount::ZERO,
        };

        let minted_supply = self.minted_supply.get();
        let new_minted_supply = minted_supply.saturating_add(erc20_amount);
        self.minted_supply.set(new_minted_supply);

        let new_user_balance = user_balance.saturating_add(erc20_amount);
        let _ = self.balances.insert(&caller, new_user_balance);
    }

    pub(crate) async fn airdrop(
        &mut self,
        initial_balances: HashMap<String, Amount>,
    ) -> Result<(), ERC20Error> {
        let total_supply = self.total_supply.get();
        let mut airdrop_amount = Amount::ZERO;
        for (owner_str, amount) in initial_balances.into_iter() {
            let owner: ChainAccountOwner = serde_json::from_str(&owner_str).unwrap();
            airdrop_amount = airdrop_amount.saturating_add(amount);
            if *total_supply < airdrop_amount {
                return Err(ERC20Error::InvalidInitialAmount);
            }
            let _ = self.balances.insert(&owner, amount);
        }
        let created_owner = self.created_owner.get().expect("Invalid created owner");
        let created_owner_balance = match self.balances.get(&created_owner).await {
            Ok(Some(balance)) => balance,
            Ok(None) => Amount::ZERO,
            Err(_) => Amount::ZERO,
        };
        let balance = total_supply.saturating_sub(airdrop_amount);
        let new_created_owner_balance = created_owner_balance.saturating_add(balance);
        let _ = self
            .balances
            .insert(&created_owner, new_created_owner_balance);
        Ok(())
    }

    pub(crate) async fn change_created_owner(
        &mut self,
        owner: ChainAccountOwner,
        new_owner: ChainAccountOwner,
    ) -> Result<(), ERC20Error> {
        let old_owner = self.created_owner.get().expect("Invalid created owner");
        if old_owner != owner {
            return Err(ERC20Error::PermissionDenied);
        }

        let from_balance = match self.balances.get(&owner).await {
            Ok(Some(balance)) => balance,
            Ok(None) => Amount::ZERO,
            Err(_) => Amount::ZERO,
        };

        let to_balance = match self.balances.get(&new_owner).await {
            Ok(Some(balance)) => balance,
            Ok(None) => Amount::ZERO,
            Err(_) => Amount::ZERO,
        };

        let new_to_balance = to_balance.saturating_add(from_balance);
        let _ = self.balances.insert(&owner, Amount::ZERO);
        let _ = self.balances.insert(&owner, new_to_balance);

        self.created_owner.set(Some(new_owner));

        Ok(())
    }
}
