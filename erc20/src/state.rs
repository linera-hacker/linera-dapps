use erc20::ERC20Error;
use linera_sdk::base::Amount;
use linera_sdk::views::{linera_views, MapView, RegisterView, RootView, ViewStorageContext};
use spec::{
    account::ChainAccountOwner,
    erc20::{AllowanceKey, InstantiationArgument, SubscriberSyncState},
};

use std::collections::HashMap;

#[derive(RootView, async_graphql::SimpleObject)]
#[view(context = "ViewStorageContext")]
pub struct Application {
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
    pub owner: RegisterView<Option<ChainAccountOwner>>,
    pub owner_balance: RegisterView<Amount>,
}

#[allow(dead_code)]
impl Application {
    pub(crate) async fn instantiate(
        &mut self,
        argument: InstantiationArgument,
        owner: ChainAccountOwner,
    ) {
        self.owner.set(Some(owner));
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
        self.owner_balance.set(argument.initial_supply);
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

        log::info!(
            "Sender {:?} balance {} amount {}",
            sender,
            sender_balance,
            amount
        );

        if sender_balance < amount {
            return Err(ERC20Error::InsufficientFunds);
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

        self.balances.insert(&sender, new_sender_balance)?;
        self.balances.insert(&to, new_receiver_balance)?;
        if let Some(_) = self.owner.get() {
            if fee > Amount::ZERO {
                let owner_balance = self.owner_balance.get();
                let new_owner_balance = owner_balance.saturating_add(fee);
                self.owner_balance.set(new_owner_balance);
            }
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
            return Err(ERC20Error::InsufficientFunds);
        }

        let from_balance = match self.balances.get(&from).await {
            Ok(Some(balance)) => balance,
            Ok(None) => Amount::ZERO,
            Err(_) => Amount::ZERO,
        };

        if from_balance < amount {
            return Err(ERC20Error::InsufficientFunds);
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

        self.balances.insert(&from, new_from_balance)?;
        self.balances.insert(&to, new_to_balance)?;
        self.allowances.insert(&allowance_key, new_allowance)?;
        if let Some(_) = self.owner.get() {
            if fee > Amount::ZERO {
                let owner_balance = self.owner_balance.get();
                let new_owner_balance = owner_balance.saturating_add(fee);
                self.owner_balance.set(new_owner_balance);
            }
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
        Ok(self.allowances.insert(&allowance_key, value)?)
    }

    pub(crate) async fn deposit_native_and_exchange(
        &mut self,
        caller: ChainAccountOwner,
        exchange_amount: Amount,
        currency: Amount,
    ) -> Result<(), ERC20Error> {
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
        let owner_balance = *self.owner_balance.get();
        if owner_balance < erc20_amount {
            return Err(ERC20Error::InsufficientFunds);
        }

        let new_user_balance = user_balance.saturating_add(erc20_amount);
        let new_owner_balance = owner_balance.saturating_sub(erc20_amount);

        self.owner_balance.set(new_owner_balance);
        Ok(self.balances.insert(&caller, new_user_balance)?)
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
                return Err(ERC20Error::InsufficientFunds);
            }
            self.balances.insert(&owner, amount)?;
        }
        let owner_balance = self.owner_balance.get();
        let balance = total_supply.saturating_sub(airdrop_amount);
        let new_owner_balance = owner_balance.saturating_add(balance);
        self.owner_balance.set(new_owner_balance);
        Ok(())
    }

    pub(crate) async fn transfer_ownership(
        &mut self,
        owner: ChainAccountOwner,
        new_owner: ChainAccountOwner,
    ) -> Result<(), ERC20Error> {
        if let Some(old_owner) = self.owner.get() {
            if *old_owner != owner {
                return Err(ERC20Error::PermissionDenied);
            }
        } else {
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
        self.balances.insert(&owner, Amount::ZERO)?;
        self.balances.insert(&owner, new_to_balance)?;

        self.owner.set(Some(new_owner));

        Ok(())
    }

    pub(crate) async fn to_sunscriber_sync_state(&self) -> Result<SubscriberSyncState, ERC20Error> {
        let mut state = SubscriberSyncState {
            total_supply: *self.total_supply.get(),
            balances: HashMap::new(),
            allowances: HashMap::new(),
            name: self.name.get().clone(),
            symbol: self.symbol.get().clone(),
            decimals: *self.decimals.get(),
            initial_currency: *self.initial_currency.get(),
            fixed_currency: *self.fixed_currency.get(),
            fee_percent: *self.fee_percent.get(),
            owner: self.owner.get().clone(),
        };
        self.balances
            .for_each_index_value(|index, value| {
                state.balances.insert(index, value);
                Ok(())
            })
            .await?;
        self.allowances
            .for_each_index_value(|index, value| {
                state.allowances.insert(index, value);
                Ok(())
            })
            .await?;
        Ok(state)
    }

    pub(crate) async fn from_subscriber_sync_state(
        &mut self,
        state: SubscriberSyncState,
    ) -> Result<(), ERC20Error> {
        self.total_supply.set(state.total_supply);
        for (key, value) in &state.balances {
            self.balances.insert(key, *value)?;
        }
        for (key, value) in &state.allowances {
            self.allowances.insert(key, *value)?;
        }
        self.name.set(state.name);
        self.symbol.set(state.symbol);
        self.decimals.set(state.decimals);
        self.initial_currency.set(state.initial_currency);
        self.fixed_currency.set(state.fixed_currency);
        self.fee_percent.set(state.fee_percent);
        self.owner.set(state.owner);
        Ok(())
    }
}
