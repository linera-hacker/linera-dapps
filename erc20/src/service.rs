#![cfg_attr(target_arch = "wasm32", no_main)]

mod state;

use self::state::Application;
use async_graphql::{EmptySubscription, Object, Schema};
use linera_sdk::{
    base::{Amount, WithServiceAbi},
    views::View,
    Service, ServiceRuntime,
};
use spec::{
    account::ChainAccountOwner,
    base::BaseOperation,
    erc20::{ERC20MutationRoot, ERC20Operation, ERC20QueryRoot},
};
use std::sync::Arc;

pub struct ApplicationService {
    state: Arc<Application>,
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
        }
    }

    async fn handle_query(&self, query: Self::Query) -> Self::QueryResponse {
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
        self.state.name.get().clone()
    }

    async fn symbol(&self) -> String {
        self.state.symbol.get().clone()
    }

    async fn decimals(&self) -> u8 {
        self.state.decimals.get().clone()
    }

    async fn balance_of(&self, owner: ChainAccountOwner) -> Amount {
        self.state.balance_of(owner).await.unwrap_or(Amount::ZERO)
    }

    async fn allowance(&self, owner: ChainAccountOwner, spender: ChainAccountOwner) -> Amount {
        self.state
            .owner_allowance(owner, spender)
            .await
            .unwrap_or(Amount::ZERO)
    }
}

struct MutationRoot {}

#[Object]
impl ERC20MutationRoot for MutationRoot {
    async fn transfer(&self, to: ChainAccountOwner, amount: Amount) -> Vec<u8> {
        bcs::to_bytes(&ERC20Operation::Transfer { to, amount }).unwrap()
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

    async fn subscribe_creator_chain(&self) -> Vec<u8> {
        bcs::to_bytes(&ERC20Operation::BaseOperation(
            BaseOperation::SubscribeCreatorChain,
        ))
        .unwrap()
    }

    async fn mint(&self, to: Option<ChainAccountOwner>, amount: Amount) -> Vec<u8> {
        bcs::to_bytes(&ERC20Operation::Mint { to, amount }).unwrap()
    }
}
