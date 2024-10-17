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
    swap::{
        abi::{SwapMessage, SwapOperation, SwapResponse},
        pool::{PoolMessage, PoolOperation},
        router::{RouterMessage, RouterOperation},
        state::SubscriberSyncState,
    },
};
use swap::SwapError;
use swap_router::{
    pool_manager::PoolManager,
    router::Router,
    runtime::{message_owner, runtime_owner},
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
        ApplicationContract {
            state,
            runtime,
            router: Router::new().await,
            pool_manager: PoolManager::new().await,
        }
    }

    async fn instantiate(&mut self, _argument: Self::InstantiationArgument) {
        self.runtime.application_parameters();
    }

    async fn execute_operation(&mut self, operation: Self::Operation) -> Self::Response {
        match operation {
            SwapOperation::BaseOperation(base_operation) => self
                .execute_base_operation(base_operation)
                .expect("Failed OP: base operation"),
            SwapOperation::PoolOperation(_) | SwapOperation::RouterOperation(_) => todo!(),
        }
    }

    async fn execute_message(&mut self, message: Self::Message) {
        match message {
            SwapMessage::BaseMessage(base_message) => self
                .execute_base_message(base_message)
                .await
                .expect("Failed MSG: base message"),
            SwapMessage::PoolMessage(_) | SwapMessage::RouterMessage(_) => todo!(),
            SwapMessage::SubscriberSync { origin, state } => self
                .on_msg_subscriber_sync(origin, state)
                .await
                .expect("Failed MSG: subscriber sync"),
        }
    }

    async fn store(mut self) {
        self.state.save().await.expect("Failed to save state");
    }
}

impl ApplicationContract {
    fn execute_base_operation(
        &mut self,
        operation: BaseOperation,
    ) -> Result<SwapResponse, SwapError> {
        match operation {
            BaseOperation::SubscribeCreatorChain => self.on_op_subscribe_creator_chain(),
        }
    }

    fn on_op_subscribe_creator_chain(&mut self) -> Result<SwapResponse, SwapError> {
        let origin = runtime_owner(&mut self.runtime);
        self.runtime
            .prepare_message(SwapMessage::BaseMessage(
                BaseMessage::SubscribeCreatorChain { origin },
            ))
            .with_authentication()
            .send_to(self.runtime.application_creator_chain_id());
        Ok(SwapResponse::Ok)
    }

    async fn execute_base_message(&mut self, message: BaseMessage) -> Result<(), SwapError> {
        match message {
            BaseMessage::SubscribeCreatorChain { origin: _ } => {
                self.on_msg_subscribe_creator_chain().await
            }
        }
    }

    async fn on_msg_subscribe_creator_chain(&mut self) -> Result<(), SwapError> {
        let message_id = self.runtime.message_id().expect("Invalid message id");
        if message_id.chain_id == self.runtime.application_creator_chain_id() {
            return Ok(());
        }

        self.runtime.subscribe(
            message_id.chain_id,
            ChannelName::from(CREATOR_CHAIN_CHANNEL.to_vec()),
        );

        let origin = message_owner(&mut self.runtime);
        let state = self.state.to_subscriber_sync_state().await?;
        self.runtime
            .prepare_message(SwapMessage::SubscriberSync { origin, state })
            .with_authentication()
            .send_to(message_id.chain_id);

        Ok(())
    }

    fn publish_message(&mut self, message: SwapMessage) {
        if self.runtime.chain_id() != self.runtime.application_creator_chain_id() {
            return;
        }
        let dest = Destination::Subscribers(ChannelName::from(CREATOR_CHAIN_CHANNEL.to_vec()));
        self.runtime
            .prepare_message(message)
            .with_authentication()
            .send_to(dest);
    }

    async fn on_msg_subscriber_sync(
        &mut self,
        origin: ChainAccountOwner,
        state: SubscriberSyncState,
    ) -> Result<(), SwapError> {
        self.state.from_subscriber_sync_state(state.clone()).await?;
        self.publish_message(SwapMessage::SubscriberSync { origin, state });
        Ok(())
    }
}
