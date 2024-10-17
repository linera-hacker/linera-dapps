#![cfg_attr(target_arch = "wasm32", no_main)]

mod state;

use self::state::Application;
use linera_sdk::{
    base::{
        Account, AccountOwner, Amount, ApplicationId, ChannelName, Destination, Timestamp,
        WithContractAbi,
    },
    views::{RootView, View},
    Contract, ContractRuntime,
};
use spec::{
    account::ChainAccountOwner,
    base::{BaseMessage, BaseOperation, CREATOR_CHAIN_CHANNEL},
    erc20::{ERC20ApplicationAbi, ERC20Operation, ERC20Response},
    swap::abi::{
        SubscriberSyncState, SwapOperation, SwapMessage, SwapResponse,
    },
};

pub struct ApplicationContract {
    state: Application,
    runtime: ContractRuntime<Self>,
    router: Router,
    pool_manager: PoolManager,
}

linera_sdk::contract!(ApplicationContract);

impl WithContractAbi for ApplicationContract {
    type Abi = swap::ApplicationAbi;
}

impl Contract for ApplicationContract {
    type Message = SwapMessage;
    type Parameters = ();
    type InstantiationArgument = ();

    async fn load(runtime: ContractRuntime<Self>) -> Self {
        let state = Application::load(runtime.root_view_storage_context())
            .await
            .expect("Failed to load state");
        ApplicationContract { state, runtime }
    }

    async fn instantiate(&mut self, _argument: Self::InstantiationArgument) {
        self.runtime.application_parameters();
    }

    async fn execute_operation(&mut self, operation: Self::Operation) -> Self::Response {
        SwapResponse::Ok
    }

    async fn execute_message(&mut self, message: Self::Message) {
    }

    async fn store(mut self) {
        self.state.save().await.expect("Failed to save state");
    }
}

