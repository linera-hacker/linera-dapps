#![cfg_attr(target_arch = "wasm32", no_main)]

mod state;

use self::state::{Application, AllowanceKey};
use async_graphql::{Context, EmptySubscription, Object, Schema};
use linera_sdk::{
    base::{Account, Amount, WithServiceAbi},
    views::View,
    Service, ServiceRuntime,
};
use spec::{
    account::ChainAccountOwner,
    erc20::{ERC20MutationRoot, ERC20QueryRoot},
};
use std::{fmt::format, sync::{Arc, Mutex}};

pub struct ApplicationService {
    state: Arc<Application>,
    runtime: Arc<Mutex<ServiceRuntime<Self>>>,
}

linera_sdk::service!(ApplicationService);

impl WithServiceAbi for ApplicationService {
    type Abi = erc20::ApplicationAbi;
}

impl Service for ApplicationService {
    type Parameters = ();

    async fn new(runtime: ServiceRuntime<Self>) -> Self {
        let state = Application::load(runtime.root_view_storage_context())
            .await
            .expect("Failed to load state");
        ApplicationService {
            state: Arc::new(state),
            runtime: Arc::new(Mutex::new(runtime)),
        }
    }

    async fn handle_query(&self, query: Self::Query) -> Self::QueryResponse {
        let runtime = self.runtime.clone();
        let schema = Schema::build(QueryRoot {state: self.state.clone()}, MutationRoot{state: self.state.clone()}, EmptySubscription).finish();
        schema.execute(query).await
    }
}

struct QueryRoot {
    state: Arc<Application>,
}

#[Object]
impl ERC20QueryRoot for QueryRoot {
    async fn total_supply(&self) -> Amount {
        *self.state.total_supply.get()
    }
    async fn name(&self) -> String {
        "Token".to_string()
    }
    async fn symbol(&self) -> String {
        "TK".to_string()
    }
    async fn decimals(&self) -> u8 {
        18
    }
    async fn balance_of(&self, owner: ChainAccountOwner) -> Amount {
        match self.state.balances.get(&owner).await {
            Ok(Some(balance)) => balance,
            Ok(None) => Amount::ZERO,
            Err(_) => Amount::ZERO,
        }
    }
}

struct MutationRoot {
    state: Arc<Application>,
}

#[Object]
impl ERC20MutationRoot for MutationRoot {
    async fn transfer(&self, to: ChainAccountOwner, amount: Amount) -> Vec<u8> {
        let sender = ChainAccountOwner::clone(&self.);
        let sender_balance = match self.state.balances.get(&sender).await {
            Ok(Some(balance)) => balance,
            Ok(None) => Amount::ZERO,
            Err(_) => Amount::ZERO,
        };

        if sender_balance < amount {
            return Vec::from("Insufficient balance");
        }

        let new_sender_balance = sender_balance - amount;
        let receiver_balance = match self.state.balances.get(&to.into()).await {
            Ok(Some(balance)) => balance,
            Ok(None) => Amount::ZERO,
            Err(_) => Amount::ZERO,
        };

        self.state.balances.insert(*sender, new_sender_balance);
        self.state.balances.insert(*to, receiver_balance);
        
        Vec::from("Transfer successful".as_bytes())
    }
    async fn transfer_from(&self, from: ChainAccountOwner, to: ChainAccountOwner, amount: Amount) -> Vec<u8> {
        let caller = Account::default();
        let allowance_key = AllowanceKey::new(from.into(), caller);
        let allowance = match self.state.allowances.get(&allowance_key).await {
            Ok(Some(balance)) => balance,
            Ok(None) => Amount::ZERO,
            Err(_) => Amount::ZERO,
        };

        if allowance < amount {
            return Vec::from("Allowance exceeded".as_bytes());
        }

        let from_balance = match self.state.balances.get(&from.into()).await {
            Ok(Some(balance)) => balance,
            Ok(None) => Amount::ZERO,
            Err(_) => Amount::ZERO,
        };

        if from_balance < amount {
            return Vec::from("Insufficient balance".as_bytes());
        }

        let new_from_balance = from_balance - amount;
        let new_allowance = allowance - amount;
        let to_balance = match self.state.balances.get(&to.into()).await {
            Ok(Some(balance)) => balance + amount,
            Ok(None) => amount,
            Err(_) => amount,
        };

        self.state.balances.insert(from.into(), new_from_balance);
        self.state.balances.insert(to.into(), to_balance);
        self.state.allowances.insert(*allowance_key, new_allowance);

        Vec::from("Transfer from successful")
    }
    async fn approve(&self, spender: ChainAccountOwner, value: Amount) -> Vec<u8> {
        let owner = Account::default();
        let allowance_key = AllowanceKey::new(owner, spender.into());
        self.state.allowances.insert(*allowance_key, value);
        Vec::from("Approval successful".as_bytes())
    }
    async fn allowance(&self, owner: ChainAccountOwner, spender: ChainAccountOwner) -> Vec<u8> {
        let allowance_key = AllowanceKey::new(owner.into(), spender.into());
        let allowance = match self.state.allowances.get(&allowance_key).await {
            Ok(Some(balance)) => balance,
            Ok(None) => Amount::ZERO,
            Err(_) => Amount::ZERO,
        };
        Vec::from(format!("Allowance: {:?}", allowance).as_bytes())
    }
}
