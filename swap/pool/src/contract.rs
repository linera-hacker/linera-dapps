#![cfg_attr(target_arch = "wasm32", no_main)]

mod state;

use linera_sdk::{
    base::{Amount, ApplicationId, WithContractAbi},
    views::{RootView, View, ViewStorageContext},
    Contract, ContractRuntime,
};

use self::state::Application;
use spec::{
    account::ChainAccountOwner,
    base::{BaseMessage, BaseOperation},
    swap::{PoolMessage, PoolOperation, PoolResponse},
};
use swap_pool::PoolError;

pub struct ApplicationContract {
    state: Application,
    runtime: ContractRuntime<Self>,
}

linera_sdk::contract!(ApplicationContract);

impl WithContractAbi for ApplicationContract {
    type Abi = swap_pool::ApplicationAbi;
}

impl Contract for ApplicationContract {
    type Message = PoolMessage;
    type Parameters = ();
    type InstantiationArgument = ();

    async fn load(runtime: ContractRuntime<Self>) -> Self {
        let state = Application::load(runtime.root_view_storage_context())
            .await
            .expect("Failed to load state");
        ApplicationContract { state, runtime }
    }

    async fn instantiate(&mut self, _argument: Self::InstantiationArgument) {}

    async fn execute_operation(&mut self, operation: PoolOperation) -> PoolResponse {
        match operation {
            PoolOperation::BaseOperation(base_operation) => self
                .execute_base_operation(base_operation)
                .expect("Failed OP: base operation"),
            PoolOperation::CreatePool {
                token_0,
                token_1,
                amount_0_initial,
                amount_1_initial,
                amount_0_virtual,
                amount_1_virtual,
            } => self
                .on_op_create_pool(
                    token_0,
                    token_1,
                    amount_0_initial,
                    amount_1_initial,
                    amount_0_virtual,
                    amount_1_virtual,
                )
                .expect("Failed OP: create pool"),
            PoolOperation::SetFeeTo { account } => self
                .on_op_set_fee_to(account)
                .expect("Failed OP: set fee to"),
            PoolOperation::SetFeeToSetter { account } => self
                .on_op_set_fee_to_setter(account)
                .expect("Failed OP: set fee to setter"),
            PoolOperation::Mint { to } => self.on_op_mint(to).expect("Failed OP: mint"),
            PoolOperation::Burn { to } => self.on_op_burn(to).expect("Failed OP: burn"),
            PoolOperation::Swap {
                amount_0_out,
                amount_1_out,
                to,
            } => self
                .on_op_swap(amount_0_out, amount_1_out, to)
                .expect("Failed OP: swap"),
        }
    }

    async fn execute_message(&mut self, _message: Self::Message) {}

    async fn store(mut self) {
        self.state.save().await.expect("Failed to save state");
    }
}

impl ApplicationContract {
    fn execute_base_operation(&mut self, operation: BaseOperation) -> Result<PoolResponse, PoolError> {
        match operation {
            BaseOperation::SubscribeCreatorChain => self.on_op_subscribe_creator_chain(),
        }
    }

    fn on_op_subscribe_creator_chain(&mut self) -> Result<PoolResponse, PoolError> {
        self.runtime
            .prepare_message(PoolMessage::BaseMessage(BaseMessage::SubscribeCreatorChain))
            .with_authentication()
            .send_to(self.runtime.application_creator_chain_id());
        Ok(PoolResponse::Ok)
    }

    fn on_op_create_pool(
        &self,
        token_0: ApplicationId,
        token_1: ApplicationId,
        amount_0_initial: Amount,
        amount_1_initial: Amount,
        amount_0_virtual: Amount,
        amount_1_virtual: Amount,
    ) -> Result<PoolResponse, PoolError> {
        Ok(PoolResponse::Ok)
    }

    fn on_op_set_fee_to(&self, account: ChainAccountOwner) -> Result<PoolResponse, PoolError> {
        Ok(PoolResponse::Ok)
    }

    fn on_op_set_fee_to_setter(
        &self,
        account: ChainAccountOwner,
    ) -> Result<PoolResponse, PoolError> {
        Ok(PoolResponse::Ok)
    }

    fn on_op_mint(&self, to: ChainAccountOwner) -> Result<PoolResponse, PoolError> {
        Ok(PoolResponse::Ok)
    }

    fn on_op_burn(&self, to: ChainAccountOwner) -> Result<PoolResponse, PoolError> {
        Ok(PoolResponse::Ok)
    }

    fn on_op_swap(
        &self,
        amount_0_out: Amount,
        amount_1_out: Amount,
        to: ChainAccountOwner,
    ) -> Result<PoolResponse, PoolError> {
        Ok(PoolResponse::Ok)
    }
}
