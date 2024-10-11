#![cfg_attr(target_arch = "wasm32", no_main)]

mod state;

use self::state::Application;
use linera_sdk::{
    base::{Amount, ApplicationId, ChannelName, Destination, Timestamp, WithContractAbi},
    views::{RootView, View, ViewStorageContext},
    Contract, ContractRuntime,
};
use spec::{
    account::ChainAccountOwner,
    base::{BaseMessage, BaseOperation, CREATOR_CHAIN_CHANNEL},
    swap::{RouterMessage, RouterOperation, RouterResponse},
};
use swap_router::RouterError;

pub struct ApplicationContract {
    state: Application,
    runtime: ContractRuntime<Self>,
}

linera_sdk::contract!(ApplicationContract);

impl WithContractAbi for ApplicationContract {
    type Abi = swap_router::ApplicationAbi;
}

impl Contract for ApplicationContract {
    type Message = RouterMessage;
    type Parameters = ();
    type InstantiationArgument = ();

    async fn load(runtime: ContractRuntime<Self>) -> Self {
        let state = Application::load(runtime.root_view_storage_context())
            .await
            .expect("Failed to load state");
        ApplicationContract { state, runtime }
    }

    async fn instantiate(&mut self, _argument: Self::InstantiationArgument) {}

    async fn execute_operation(&mut self, operation: Self::Operation) -> Self::Response {
        match operation {
            RouterOperation::BaseOperation(base_operation) => self
                .execute_base_operation(base_operation)
                .expect("Failed OP: base operation"),
            RouterOperation::AddLiquidity {
                token_0,
                token_1,
                amount_0_desired,
                amount_1_desired,
                amount_0_min,
                amount_1_min,
                to,
                deadline,
            } => self
                .on_op_add_liquidity(
                    token_0,
                    token_1,
                    amount_0_desired,
                    amount_1_desired,
                    amount_0_min,
                    amount_1_min,
                    to,
                    deadline,
                )
                .expect("Failed OP: add liquidity"),
            RouterOperation::RemoveLiquidity {
                token_0,
                token_1,
                liquidity,
                amount_0_min,
                amount_1_min,
                to,
                deadline,
            } => self
                .on_op_remove_liquidity(
                    token_0,
                    token_1,
                    liquidity,
                    amount_0_min,
                    amount_1_min,
                    to,
                    deadline,
                )
                .expect("Failed OP: remove liquidity"),
            RouterOperation::CalculateSwapAmount {
                token_0,
                token_1,
                amount_1,
            } => self
                .on_op_calculate_swap_amount(token_0, token_1, amount_1)
                .expect("Failed OP: calculate swap amount"),
        }
    }

    async fn execute_message(&mut self, message: Self::Message) {
        match message {
            RouterMessage::BaseMessage(base_message) => self
                .execute_base_message(base_message)
                .expect("Failed MSG: base message"),
            RouterMessage::AddLiquidity {
                token_0,
                token_1,
                amount_0_desired,
                amount_1_desired,
                amount_0_min,
                amount_1_min,
                to,
                deadline,
            } => self
                .on_msg_add_liquidity(
                    token_0,
                    token_1,
                    amount_0_desired,
                    amount_1_desired,
                    amount_0_min,
                    amount_1_min,
                    to,
                    deadline,
                )
                .await
                .expect("Failed MSG: add liquidity"),
            RouterMessage::RemoveLiquidity {
                token_0,
                token_1,
                liquidity,
                amount_0_min,
                amount_1_min,
                to,
                deadline,
            } => self
                .on_msg_remove_liquidity(
                    token_0,
                    token_1,
                    liquidity,
                    amount_0_min,
                    amount_1_min,
                    to,
                    deadline,
                )
                .await
                .expect("Failed MSG: remove liquidity"),
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
    ) -> Result<RouterResponse, RouterError> {
        match operation {
            BaseOperation::SubscribeCreatorChain => self.on_op_subscribe_creator_chain(),
        }
    }

    fn on_op_subscribe_creator_chain(&mut self) -> Result<RouterResponse, RouterError> {
        self.runtime
            .prepare_message(RouterMessage::BaseMessage(
                BaseMessage::SubscribeCreatorChain,
            ))
            .with_authentication()
            .send_to(self.runtime.application_creator_chain_id());
        Ok(RouterResponse::Ok)
    }

    fn on_op_add_liquidity(
        &mut self,
        token_0: ApplicationId,
        token_1: Option<ApplicationId>,
        amount_0_desired: Amount,
        amount_1_desired: Amount,
        amount_0_min: Amount,
        amount_1_min: Amount,
        to: ChainAccountOwner,
        deadline: Timestamp,
    ) -> Result<RouterResponse, RouterError> {
        self.runtime
            .prepare_message(RouterMessage::AddLiquidity {
                token_0,
                token_1,
                amount_0_desired,
                amount_1_desired,
                amount_0_min,
                amount_1_min,
                to,
                deadline,
            })
            .with_authentication()
            .send_to(self.runtime.application_creator_chain_id());
        Ok(RouterResponse::Ok)
    }

    fn on_op_remove_liquidity(
        &mut self,
        token_0: ApplicationId,
        token_1: Option<ApplicationId>,
        liquidity: Amount,
        amount_0_min: Amount,
        amount_1_min: Amount,
        to: ChainAccountOwner,
        deadline: Timestamp,
    ) -> Result<RouterResponse, RouterError> {
        self.runtime
            .prepare_message(RouterMessage::RemoveLiquidity {
                token_0,
                token_1,
                liquidity,
                amount_0_min,
                amount_1_min,
                to,
                deadline,
            })
            .with_authentication()
            .send_to(self.runtime.application_creator_chain_id());
        Ok(RouterResponse::Ok)
    }

    fn on_op_calculate_swap_amount(
        &self,
        token_0: ApplicationId,
        token_1: Option<ApplicationId>,
        amount_1: Amount,
    ) -> Result<RouterResponse, RouterError> {
        Ok(RouterResponse::Ok)
    }

    fn execute_base_message(&mut self, message: BaseMessage) -> Result<(), RouterError> {
        match message {
            BaseMessage::SubscribeCreatorChain => self.on_msg_subscribe_creator_chain(),
        }
    }

    fn on_msg_subscribe_creator_chain(&mut self) -> Result<(), RouterError> {
        let message_id = self.runtime.message_id().expect("Invalid message id");
        if message_id.chain_id == self.runtime.application_creator_chain_id() {
            return Ok(());
        }

        self.runtime.subscribe(
            message_id.chain_id,
            ChannelName::from(CREATOR_CHAIN_CHANNEL.to_vec()),
        );
        Ok(())
    }

    fn publish_message(&mut self, message: RouterMessage) {
        if self.runtime.chain_id() != self.runtime.application_creator_chain_id() {
            return;
        }
        let dest = Destination::Subscribers(ChannelName::from(CREATOR_CHAIN_CHANNEL.to_vec()));
        self.runtime
            .prepare_message(message)
            .with_authentication()
            .send_to(dest);
    }

    async fn on_msg_add_liquidity(
        &mut self,
        token_0: ApplicationId,
        token_1: Option<ApplicationId>,
        amount_0_desired: Amount,
        amount_1_desired: Amount,
        amount_0_min: Amount,
        amount_1_min: Amount,
        to: ChainAccountOwner,
        deadline: Timestamp,
    ) -> Result<(), RouterError> {
        Ok(())
    }

    async fn on_msg_remove_liquidity(
        &mut self,
        token_0: ApplicationId,
        token_1: Option<ApplicationId>,
        liquidity: Amount,
        amount_0_min: Amount,
        amount_1_min: Amount,
        to: ChainAccountOwner,
        deadline: Timestamp,
    ) -> Result<(), RouterError> {
        Ok(())
    }
}
