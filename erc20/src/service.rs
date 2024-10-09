#![cfg_attr(target_arch = "wasm32", no_main)]

mod state;

use self::state::{AllowanceKey, Application};
use async_graphql::{Context, EmptySubscription, Object, Schema};
use linera_sdk::{
    base::{Account, Amount, WithServiceAbi},
    views::View,
    Service, ServiceRuntime,
};
use spec::{
    account::ChainAccountOwner,
    erc20::{ERC20MutationRoot, ERC20Operation, ERC20QueryRoot},
};
use std::{
    fmt::format,
    sync::{Arc, Mutex},
};

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
        let schema = Schema::build(
            QueryRoot {
                state: self.state.clone(),
            },
            MutationRoot {},
            EmptySubscription,
        )
        .finish();
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

struct MutationRoot {}

#[Object]
impl ERC20MutationRoot for MutationRoot {
    async fn transfer(&self, to: ChainAccountOwner, amount: Amount) -> Vec<u8> {
        bcs::to_bytes(&ERC20Operation::Transfer {
            from: None,
            to,
            amount,
        })
        .unwrap()
    }
    async fn transfer_from(
        &self,
        from: ChainAccountOwner,
        to: ChainAccountOwner,
        amount: Amount,
    ) -> Vec<u8> {
        bcs::to_bytes(&ERC20Operation::TransferFrom { from, to, amount }).unwrap()
    }
    async fn approve(&self, spender: ChainAccountOwner, value: Amount) -> Vec<u8> {
        bcs::to_bytes(&ERC20Operation::Approve { spender, value }).unwrap()
    }
    async fn allowance(&self, owner: ChainAccountOwner, spender: ChainAccountOwner) -> Vec<u8> {
        bcs::to_bytes(&ERC20Operation::Allowance { owner, spender }).unwrap()
    }
}
