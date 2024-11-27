#![cfg_attr(target_arch = "wasm32", no_main)]

mod state;

use self::state::Application;
use linera_sdk::{
    base::{ApplicationId, ChannelName, Destination, WithContractAbi},
    views::{RootView, View},
    Contract, ContractRuntime,
};
use spec::{
    account::ChainAccountOwner,
    ams::{AMSApplicationAbi, AMSOperation, Metadata},
    base::{BaseMessage, BaseOperation, CREATOR_CHAIN_CHANNEL},
    swap::{
        abi::{SwapMessage, SwapOperation, SwapParameters, SwapResponse},
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
    type Parameters = SwapParameters;
    type InstantiationArgument = ();

    async fn load(mut runtime: ContractRuntime<Self>) -> Self {
        let mut state = Application::load(runtime.root_view_storage_context())
            .await
            .expect("Failed to load state");
        state
            .wlinera_application_id
            .set(runtime.application_parameters().wlinera_application_id);
        ApplicationContract {
            state,
            runtime,
            router: Router::new().await,
            pool_manager: PoolManager::new().await,
        }
    }

    async fn instantiate(&mut self, _argument: Self::InstantiationArgument) {
        let parameters = self.runtime.application_parameters();

        match parameters.ams_application_id {
            Some(application_id) => {
                self.register_ams(application_id, parameters);
            }
            _ => {}
        }
    }

    async fn execute_operation(&mut self, operation: Self::Operation) -> Self::Response {
        match operation {
            SwapOperation::BaseOperation(base_operation) => self
                .execute_base_operation(base_operation)
                .expect("Failed OP: base operation"),
            SwapOperation::PoolOperation(pool_operation) => self
                .execute_pool_operation(pool_operation)
                .await
                .expect("Failed OP: pool operation"),
            SwapOperation::RouterOperation(router_operation) => self
                .execute_router_operation(router_operation)
                .await
                .expect("Failed OP: router operation"),
        }
    }

    async fn execute_message(&mut self, message: Self::Message) {
        match message {
            SwapMessage::BaseMessage(base_message) => self
                .execute_base_message(base_message)
                .await
                .expect("Failed MSG: base message"),
            SwapMessage::PoolMessage(pool_message) => self
                .execute_pool_message(pool_message)
                .await
                .expect("Fail MSG: pool message"),
            SwapMessage::RouterMessage(router_message) => self
                .execute_router_message(router_message)
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
    fn register_ams(&mut self, ams_application_id: ApplicationId, parameters: SwapParameters) {
        let call = AMSOperation::Register {
            metadata: Metadata {
                creator: Some(runtime_owner(&mut self.runtime)),
                application_name: parameters.application_name,
                application_id: self.runtime.application_id().forget_abi(),
                application_type: "SWAP".to_string(),
                key_words: [
                    "ResPeer".to_string(),
                    "SWAP".to_string(),
                    "CheCko".to_string(),
                    "Linera".to_string(),
                ]
                .to_vec(),
                logo: parameters.logo,
                spec: None,
                description: parameters.description,
                discord: None,
                twitter: None,
                telegram: None,
                github: None,
                website: None,
                created_at: None,
            },
        };
        self.runtime.call_application(
            true,
            ams_application_id.with_abi::<AMSApplicationAbi>(),
            &call,
        );
    }

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

    async fn execute_pool_operation(
        &mut self,
        operation: PoolOperation,
    ) -> Result<SwapResponse, SwapError> {
        let (response, msg) = self
            .pool_manager
            .execute_operation(&mut self.runtime, &mut self.state, operation)
            .await?;
        match msg {
            Some((_msg, true)) => {
                self.runtime
                    .prepare_message(SwapMessage::PoolMessage(_msg))
                    .with_authentication()
                    .send_to(self.runtime.application_creator_chain_id());
            }
            _ => {}
        }
        match operation {
            PoolOperation::CreatePool {
                token_0,
                token_1,
                amount_0_initial: _,
                amount_1_initial: _,
                amount_0_virtual: _,
                amount_1_virtual: _,
            } => {
                let origin = runtime_owner(&mut self.runtime);
                self.runtime
                    .prepare_message(SwapMessage::RouterMessage(
                        RouterMessage::SubscribeNewERC20Token {
                            origin,
                            token: token_0,
                        },
                    ))
                    .with_authentication()
                    .send_to(self.runtime.application_creator_chain_id());
                if let Some(token_1) = token_1 {
                    self.runtime
                        .prepare_message(SwapMessage::RouterMessage(
                            RouterMessage::SubscribeNewERC20Token {
                                origin,
                                token: token_1,
                            },
                        ))
                        .with_authentication()
                        .send_to(self.runtime.application_creator_chain_id());
                }
            }
            _ => {}
        }
        Ok(SwapResponse::PoolResponse(response))
    }

    async fn execute_pool_message(&mut self, message: PoolMessage) -> Result<(), SwapError> {
        match self
            .pool_manager
            .execute_message(&mut self.runtime, &mut self.state, message)
            .await?
        {
            Some((msg, true)) => {
                self.publish_message(SwapMessage::PoolMessage(msg));
            }
            _ => {}
        }
        Ok(())
    }

    async fn execute_router_operation(
        &mut self,
        operation: RouterOperation,
    ) -> Result<SwapResponse, SwapError> {
        let (response, msg) = self
            .router
            .execute_operation(&mut self.runtime, &mut self.state, operation)
            .await?;
        match msg {
            Some((_msg, true)) => {
                self.runtime
                    .prepare_message(SwapMessage::RouterMessage(_msg))
                    .with_authentication()
                    .send_to(self.runtime.application_creator_chain_id());
            }
            _ => {}
        }
        Ok(SwapResponse::RouterResponse(response))
    }

    async fn execute_router_message(&mut self, message: RouterMessage) -> Result<(), SwapError> {
        match self
            .router
            .execute_message(&mut self.runtime, &mut self.state, message)
            .await?
        {
            Some((msg, true)) => {
                self.publish_message(SwapMessage::RouterMessage(msg));
            }
            _ => {}
        }
        Ok(())
    }
}
