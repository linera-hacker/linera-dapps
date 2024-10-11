use erc20::ERC20Error;
use linera_sdk::base::Amount;
use linera_sdk::views::{linera_views, MapView, RegisterView, RootView, ViewStorageContext};
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
    pub initial_currency_fixed: RegisterView<bool>,
    pub basis_point_rate: RegisterView<Amount>,
}

#[allow(dead_code)]
impl Application {
    pub(crate) async fn instantiate(&mut self, argument: InstantiationArgument) {
        self.total_supply.set(argument.initial_supply);
        let _ = self
            .balances
            .insert(&argument.owner, argument.initial_supply);
        self.name.set(argument.name);
        self.symbol.set(argument.symbol);
        self.decimals.set(argument.decimals);
        self.initial_currency_fixed
            .set(argument.initial_currency_fixed.unwrap_or(false));
        self.initial_currency
            .set(argument.initial_currency.unwrap_or(Amount::ONE));
        self.basis_point_rate.set(argument.basis_point_rate.unwrap_or(Amount::ZERO));
    }

    pub(crate) async fn transfer(
        &mut self,
        sender: ChainAccountOwner,
        amount: Amount,
        to: ChainAccountOwner,
        created_owner: ChainAccountOwner,
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
        let fee_rate = *self.basis_point_rate.get();
        let fee = amount.saturating_mul(fee_rate.into());
        let send_amount = amount.try_sub(fee).expect("Invalid sub send amount");
        let new_receiver_balance = receiver_balance + send_amount;

        let _ = self.balances.insert(&sender, new_sender_balance);
        let _ = self.balances.insert(&to, new_receiver_balance);
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
        created_owner: ChainAccountOwner,
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
            Ok(Some(balance)) => balance,
            Ok(None) => amount,
            Err(_) => amount,
        };
        let fee_rate = *self.basis_point_rate.get();
        let fee = amount.saturating_mul(fee_rate.into());
        let send_amount = amount.try_sub(fee).expect("Invalid sub send amount");
        let new_to_balance = to_balance + send_amount;

        let _ = self.balances.insert(&from, new_from_balance);
        let _ = self.balances.insert(&to, new_to_balance);
        let _ = self.allowances.insert(&allowance_key, new_allowance);
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

    async fn checked_mul(a: Amount, b: Amount) -> Amount {
        let a_value: u128 = a.into();
        let b_value: u128 = b.into();
        let result = a_value * b_value;

        Amount::from(result)
    }

    pub(crate) async fn deposit_native_and_exchange(
        &mut self,
        caller: ChainAccountOwner,
        exchange_amount: Amount,
        currency: Amount,
    ) {
        let mut exchange_currency = self.initial_currency.get();
        if !self.initial_currency_fixed.get() {
            exchange_currency = &currency
        }
        let erc20_amount = Self::checked_mul(*exchange_currency, exchange_amount).await;

        let user_balance = match self.balances.get(&caller).await {
            Ok(Some(balance)) => balance,
            Ok(None) => Amount::ZERO,
            Err(_) => Amount::ZERO,
        };

        let _ = self.balances.insert(&caller, user_balance + erc20_amount);
    }
}
