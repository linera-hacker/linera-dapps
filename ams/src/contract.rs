#![cfg_attr(target_arch = "wasm32", no_main)]

mod state;

use linera_sdk::{
    base::{AccountOwner, ApplicationId, ChannelName, Destination, Signature, WithContractAbi},
    views::{RootView, View},
    Contract, ContractRuntime,
};

use self::state::Application;
use ams::AMSError;
use spec::{
    account::ChainAccountOwner,
    ams::{
        AMSMessage, AMSOperation, AMSResponse, InstantiationArgument, Metadata, SubscriberSyncState,
    },
    base::{BaseMessage, BaseOperation, CREATOR_CHAIN_CHANNEL},
};

pub struct ApplicationContract {
    state: Application,
    runtime: ContractRuntime<Self>,
}

linera_sdk::contract!(ApplicationContract);

impl WithContractAbi for ApplicationContract {
    type Abi = ams::ApplicationAbi;
}

impl Contract for ApplicationContract {
    type Message = AMSMessage;
    type Parameters = ();
    type InstantiationArgument = InstantiationArgument;

    async fn load(runtime: ContractRuntime<Self>) -> Self {
        let state = Application::load(runtime.root_view_storage_context())
            .await
            .expect("Failed to load state");
        ApplicationContract { state, runtime }
    }

    async fn instantiate(&mut self, argument: InstantiationArgument) {
        let owner = ChainAccountOwner {
            chain_id: self.runtime.chain_id(),
            owner: Some(AccountOwner::User(
                self.runtime.authenticated_signer().expect("Invalid owner"),
            )),
        };
        self.state.instantiate(argument, owner).await;
    }

    async fn execute_operation(&mut self, operation: AMSOperation) -> AMSResponse {
        match operation {
            AMSOperation::BaseOperation(base_operation) => self
                .execute_base_operation(base_operation)
                .expect("Failed OP: base operation"),
            AMSOperation::Register { metadata } => self
                .on_op_register(metadata)
                .await
                .expect("Failed OP: register"),
            AMSOperation::Claim {
                application_id,
                signature,
            } => self
                .on_op_claim(application_id, signature)
                .await
                .expect("Failed OP: claim"),
            AMSOperation::AddApplicationType { application_type } => self
                .on_op_add_application_type(application_type)
                .await
                .expect("Failed OP: add application type"),
            AMSOperation::Update {
                application_id,
                metadata,
            } => self
                .on_op_update(application_id, metadata)
                .await
                .expect("Failed OP: update"),
        }
    }

    async fn execute_message(&mut self, message: AMSMessage) {
        match message {
            AMSMessage::BaseMessage(base_message) => self
                .execute_base_message(base_message)
                .await
                .expect("Failed MSG: base message"),
            AMSMessage::Register { origin, metadata } => self
                .on_msg_register(origin, metadata)
                .await
                .expect("Failed MSG: register"),
            AMSMessage::Claim {
                origin: _,
                application_id,
                signature,
            } => self
                .on_msg_claim(application_id, signature)
                .await
                .expect("Failed MSG: claim"),
            AMSMessage::AddApplicationType {
                origin,
                application_type,
            } => self
                .on_msg_add_application_type(origin, application_type)
                .await
                .expect("Failed MSG: add application type"),
            AMSMessage::Update {
                origin: _,
                application_id,
                metadata,
            } => self
                .on_msg_update(application_id, metadata)
                .await
                .expect("Failed MSG: update"),
            AMSMessage::SubscriberSync { origin: _, state } => self
                .on_msg_subscriber_sync(state)
                .await
                .expect("Failed MSG: subscriber sync state"),
        }
    }

    async fn store(mut self) {
        self.state.save().await.expect("Failed to save state");
    }
}

impl ApplicationContract {
    fn runtime_owner(&mut self) -> ChainAccountOwner {
        match self.runtime.authenticated_caller_id() {
            Some(application_id) => ChainAccountOwner {
                chain_id: self.runtime.chain_id(),
                owner: Some(AccountOwner::Application(application_id)),
            },
            _ => ChainAccountOwner {
                chain_id: self.runtime.chain_id(),
                owner: Some(AccountOwner::User(
                    self.runtime.authenticated_signer().expect("Invalid owner"),
                )),
            },
        }
    }

    fn execute_base_operation(
        &mut self,
        operation: BaseOperation,
    ) -> Result<AMSResponse, AMSError> {
        match operation {
            BaseOperation::SubscribeCreatorChain => self.on_op_subscribe_creator_chain(),
        }
    }

    fn on_op_subscribe_creator_chain(&mut self) -> Result<AMSResponse, AMSError> {
        let origin = self.runtime_owner();
        self.runtime
            .prepare_message(AMSMessage::BaseMessage(
                BaseMessage::SubscribeCreatorChain { origin },
            ))
            .with_authentication()
            .send_to(self.runtime.application_creator_chain_id());
        Ok(AMSResponse::Ok)
    }

    async fn on_op_register(&mut self, mut metadata: Metadata) -> Result<AMSResponse, AMSError> {
        let origin = self.runtime_owner();

        metadata.creator = Some(origin);
        metadata.created_at = Some(self.runtime.system_time());

        self.runtime
            .prepare_message(AMSMessage::Register { origin, metadata })
            .with_authentication()
            .send_to(self.runtime.application_creator_chain_id());
        Ok(AMSResponse::Ok)
    }

    async fn on_op_claim(
        &mut self,
        _application_id: ApplicationId,
        _signature: Signature,
    ) -> Result<AMSResponse, AMSError> {
        Err(AMSError::NotImplemented)
    }

    async fn on_op_add_application_type(
        &mut self,
        application_type: String,
    ) -> Result<AMSResponse, AMSError> {
        let origin = self.runtime_owner();
        self.runtime
            .prepare_message(AMSMessage::AddApplicationType {
                origin,
                application_type,
            })
            .with_authentication()
            .send_to(self.runtime.application_creator_chain_id());
        Ok(AMSResponse::Ok)
    }

    async fn on_op_update(
        &self,
        _application_id: ApplicationId,
        _metadata: Metadata,
    ) -> Result<AMSResponse, AMSError> {
        Err(AMSError::NotImplemented)
    }

    async fn execute_base_message(&mut self, message: BaseMessage) -> Result<(), AMSError> {
        match message {
            BaseMessage::SubscribeCreatorChain { origin } => {
                self.on_msg_subscribe_creator_chain(origin).await
            }
        }
    }

    async fn on_msg_subscribe_creator_chain(
        &mut self,
        origin: ChainAccountOwner,
    ) -> Result<(), AMSError> {
        let message_id = self.runtime.message_id().expect("Invalid message id");
        if message_id.chain_id == self.runtime.application_creator_chain_id() {
            return Ok(());
        }

        self.runtime.subscribe(
            message_id.chain_id,
            ChannelName::from(CREATOR_CHAIN_CHANNEL.to_vec()),
        );

        let state = self.state.to_subscriber_sync_state().await?;
        self.runtime
            .prepare_message(AMSMessage::SubscriberSync { origin, state })
            .with_authentication()
            .send_to(message_id.chain_id);

        Ok(())
    }

    fn publish_message(&mut self, message: AMSMessage) {
        if self.runtime.chain_id() != self.runtime.application_creator_chain_id() {
            return;
        }
        let dest = Destination::Subscribers(ChannelName::from(CREATOR_CHAIN_CHANNEL.to_vec()));
        self.runtime
            .prepare_message(message)
            .with_authentication()
            .send_to(dest);
    }

    async fn on_msg_register(
        &mut self,
        origin: ChainAccountOwner,
        metadata: Metadata,
    ) -> Result<(), AMSError> {
        self.state.register_application(metadata.clone()).await?;
        self.publish_message(AMSMessage::Register { origin, metadata });
        Ok(())
    }

    async fn on_msg_claim(
        &mut self,
        _application_id: ApplicationId,
        _signature: Signature,
    ) -> Result<(), AMSError> {
        Err(AMSError::NotImplemented)
    }

    async fn on_msg_add_application_type(
        &mut self,
        origin: ChainAccountOwner,
        application_type: String,
    ) -> Result<(), AMSError> {
        self.state
            .add_application_type(origin, application_type.clone())
            .await?;
        self.publish_message(AMSMessage::AddApplicationType {
            origin,
            application_type,
        });
        Ok(())
    }

    async fn on_msg_update(
        &mut self,
        _application_id: ApplicationId,
        _metadata: Metadata,
    ) -> Result<(), AMSError> {
        Err(AMSError::NotImplemented)
    }

    async fn on_msg_subscriber_sync(&mut self, state: SubscriberSyncState) -> Result<(), AMSError> {
        self.state.from_subscriber_sync_state(state).await
    }
}
