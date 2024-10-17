#![cfg_attr(target_arch = "wasm32", no_main)]

mod state;

use self::state::Application;
use linera_sdk::{
    base::{
        ChannelName, Destination,
        WithContractAbi,
    },
    views::{View, RootView},
    Contract, ContractRuntime,
};
use spec::{
    account::ChainAccountOwner,
    base::{BaseMessage, BaseOperation, CREATOR_CHAIN_CHANNEL},
    swap::{
        abi::{SwapMessage, SwapOperation, SwapResponse},
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
            SwapOperation::PoolOperation(pool_operation) => SwapResponse::PoolResponse(
                self.pool_manager
                    .execute_operation(&mut self.runtime, &mut self.state, pool_operation)
                    .await
                    .expect("Failed OP: pool operation"),
            ),
            SwapOperation::RouterOperation(router_operation) => SwapResponse::RouterResponse(
                self.router
                    .execute_operation(&mut self.runtime, &mut self.state, router_operation)
                    .await
                    .expect("Failed OP: router operation"),
            ),
        }
    }

    async fn execute_message(&mut self, message: Self::Message) {
        match message {
            SwapMessage::BaseMessage(base_message) => self
                .execute_base_message(base_message)
                .await
                .expect("Failed MSG: base message"),
            SwapMessage::PoolMessage(pool_message) => self
                .pool_manager
                .execute_message(&mut self.runtime, &mut self.state, pool_message)
                .await
                .expect("Fail MSG: pool message"),
            SwapMessage::RouterMessage(router_message) => self
                .router
                .execute_message(&mut self.runtime, &mut self.state, router_message)
                .await
                .expect("Fail MSG: router message"),
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
