use erc20::ERC20Error;
use linera_sdk::base::Amount;
use linera_sdk::views::{linera_views, MapView, RegisterView, RootView, ViewStorageContext};
use spec::{
    account::ChainAccountOwner,
    erc20::{InstantiationArgument, SubscriberSyncState},
};

use std::collections::HashMap;

#[derive(RootView, async_graphql::SimpleObject)]
#[view(context = "ViewStorageContext")]
pub struct Application {
    // Add fields here.
    pub total_supply: RegisterView<Amount>,
    pub balances: MapView<ChainAccountOwner, Amount>,
    pub allowances: MapView<ChainAccountOwner, HashMap<ChainAccountOwner, Amount>>,
    pub locked_allowances: MapView<ChainAccountOwner, Amount>,
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
        let sender_balance = self.balance_of(sender).await?;
        if sender_balance < amount {
            return Err(ERC20Error::InsufficientFunds);
        }

        let new_sender_balance = sender_balance.saturating_sub(amount);
        let receiver_balance = self.balance_of(to).await?;
        let fee_percent = *self.fee_percent.get();
        let fee = Amount::from_attos(
            amount
                .saturating_mul(fee_percent.into())
                .saturating_div(Amount::ONE),
        );
        let send_amount = amount.saturating_sub(fee);
        let new_receiver_balance = receiver_balance.saturating_add(send_amount);

        self.update_owner_balance(sender, new_sender_balance)
            .await?;
        self.update_owner_balance(to, new_receiver_balance).await?;

        if let Some(owner) = self.owner.get() {
            if fee > Amount::ZERO {
                let owner_balance = self.owner_balance.get();
                let newbalance_of = owner_balance.saturating_add(fee);
                self.update_owner_balance(*owner, newbalance_of).await?;
            }
        }
        Ok(())
    }

    pub(crate) async fn balance_of(&self, owner: ChainAccountOwner) -> Result<Amount, ERC20Error> {
        if let Some(application_creator) = self.owner.get() {
            if *application_creator == owner {
                return Ok(*self.owner_balance.get());
            }
        }

        match self.balances.get(&owner).await? {
            Some(balance) => Ok(balance),
            None => Ok(Amount::ZERO),
        }
    }

    async fn update_owner_balance(
        &mut self,
        owner: ChainAccountOwner,
        amount: Amount,
    ) -> Result<(), ERC20Error> {
        if let Some(application_creator) = self.owner.get() {
            if *application_creator == owner {
                self.owner_balance.set(amount);
                return Ok(());
            }
        }
        Ok(self.balances.insert(&owner, amount)?)
    }

    pub async fn owner_allowance(
        &self,
        owner: ChainAccountOwner,
        spender: ChainAccountOwner,
    ) -> Result<Amount, ERC20Error> {
        match self.allowances.get(&owner).await? {
            Some(allowances) => match allowances.get(&spender) {
                Some(allowance) => Ok(*allowance),
                _ => Ok(Amount::ZERO),
            },
            _ => Ok(Amount::ZERO),
        }
    }

    async fn update_owner_allowance(
        &mut self,
        owner: ChainAccountOwner,
        spender: ChainAccountOwner,
        amount: Amount,
    ) -> Result<(), ERC20Error> {
        let mut allowances = match self.allowances.get(&owner).await? {
            Some(allowances) => allowances,
            None => HashMap::new(),
        };
        let allowance = match allowances.get(&spender) {
            Some(_allowance) => *_allowance,
            None => Amount::ZERO,
        };

        let allowance_decrease = allowance.saturating_sub(amount);
        let locked_allowance = match self.locked_allowances.get(&owner).await? {
            Some(allowance) => allowance,
            None => Amount::ZERO,
        };
        let locked_allowance = locked_allowance.saturating_sub(allowance_decrease);
        if locked_allowance < Amount::ZERO {
            return Err(ERC20Error::InsufficientFunds);
        }

        self.locked_allowances.insert(&owner, locked_allowance)?;
        allowances.insert(spender, amount);
        Ok(self.allowances.insert(&owner, allowances)?)
    }

    pub(crate) async fn transfer_from(
        &mut self,
        from: ChainAccountOwner,
        amount: Amount,
        to: ChainAccountOwner,
        caller: ChainAccountOwner,
    ) -> Result<(), ERC20Error> {
        let allowance = self.owner_allowance(from.clone(), caller).await?;
        if allowance < amount {
            return Err(ERC20Error::InsufficientFunds);
        }

        let from_balance = self.balance_of(from).await?;
        if from_balance < amount {
            return Err(ERC20Error::InsufficientFunds);
        }

        let new_from_balance = from_balance.saturating_sub(amount);
        let new_allowance = allowance.saturating_sub(amount);

        let to_balance = self.balance_of(to).await?;

        // TODO: if owner is not set, fee should be 0
        let fee_percent = *self.fee_percent.get();
        let fee = Amount::from_attos(
            amount
                .saturating_mul(fee_percent.into())
                .saturating_div(Amount::ONE),
        );
        let send_amount = amount.saturating_sub(fee);
        let new_to_balance = to_balance.saturating_add(send_amount);

        self.update_owner_balance(from, new_from_balance).await?;
        self.update_owner_balance(to, new_to_balance).await?;
        self.update_owner_allowance(from, caller, new_allowance)
            .await?;

        if let Some(owner) = self.owner.get() {
            if fee > Amount::ZERO {
                let owner_balance = self.owner_balance.get();
                let newbalance_of = owner_balance.saturating_add(fee);
                self.update_owner_balance(*owner, newbalance_of).await?;
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
        let balance = self.balance_of(owner).await?;
        let locked_allowance = match self.locked_allowances.get(&owner).await? {
            Some(allowance) => allowance,
            None => Amount::ZERO,
        };
        log::info!(
            "{}, {}, {}, {}",
            balance.saturating_sub(locked_allowance),
            balance,
            locked_allowance,
            value
        );
        if balance.saturating_sub(locked_allowance) < value {
            return Err(ERC20Error::InsufficientFunds);
        }
        self.update_owner_allowance(owner, spender, value).await
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

        let user_balance = self.balance_of(caller).await?;
        let owner_balance = *self.owner_balance.get();
        if owner_balance < erc20_amount {
            return Err(ERC20Error::InsufficientFunds);
        }

        let new_user_balance = user_balance.saturating_add(erc20_amount);
        let newbalance_of = owner_balance.saturating_sub(erc20_amount);

        self.owner_balance.set(newbalance_of);
        Ok(self.balances.insert(&caller, new_user_balance)?)
    }

    pub(crate) async fn airdrop(
        &mut self,
        initial_balances: HashMap<String, Amount>,
    ) -> Result<(), ERC20Error> {
        let owner_balance = self.owner_balance.get();
        let mut airdrop_amount = Amount::ZERO;
        for (owner_str, amount) in initial_balances.into_iter() {
            let owner: ChainAccountOwner = serde_json::from_str(&owner_str).unwrap();
            airdrop_amount = airdrop_amount.saturating_add(amount);
            if *owner_balance < airdrop_amount {
                return Err(ERC20Error::InsufficientFunds);
            }
            self.balances.insert(&owner, amount)?;
        }
        let balance = owner_balance.saturating_sub(airdrop_amount);
        self.owner_balance.set(balance);
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

    pub(crate) async fn to_subscriber_sync_state(&self) -> Result<SubscriberSyncState, ERC20Error> {
        let mut state = SubscriberSyncState {
            total_supply: *self.total_supply.get(),
            balances: HashMap::new(),
            allowances: HashMap::new(),
            locked_allowances: HashMap::new(),
            name: self.name.get().clone(),
            symbol: self.symbol.get().clone(),
            decimals: *self.decimals.get(),
            initial_currency: *self.initial_currency.get(),
            fixed_currency: *self.fixed_currency.get(),
            fee_percent: *self.fee_percent.get(),
            owner: self.owner.get().clone(),
            owner_balance: *self.owner_balance.get(),
        };
        self.balances
            .for_each_index_value(|index, value| {
                state.balances.insert(index, value);
                Ok(())
            })
            .await?;
        self.allowances
            .for_each_index_value(|index, allowances| {
                let mut _allowances = HashMap::new();
                for (key, value) in &allowances {
                    _allowances.insert(*key, *value);
                }
                state.allowances.insert(index, _allowances);
                Ok(())
            })
            .await?;
        self.locked_allowances
            .for_each_index_value(|index, value| {
                state.locked_allowances.insert(index, value);
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
        for (owner, allowances) in &state.allowances {
            let mut _allowances = HashMap::new();
            for (key, value) in allowances {
                _allowances.insert(*key, *value);
            }
            self.allowances.insert(owner, _allowances)?;
        }
        for (key, value) in &state.locked_allowances {
            self.locked_allowances.insert(key, *value)?;
        }
        self.name.set(state.name);
        self.symbol.set(state.symbol);
        self.decimals.set(state.decimals);
        self.initial_currency.set(state.initial_currency);
        self.fixed_currency.set(state.fixed_currency);
        self.fee_percent.set(state.fee_percent);
        self.owner.set(state.owner);
        self.owner_balance.set(state.owner_balance);
        Ok(())
    }
}
