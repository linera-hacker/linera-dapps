#![cfg_attr(target_arch = "wasm32", no_main)]

mod state;

use linera_sdk::{
    base::{AccountOwner, Amount, ApplicationId, ChannelName, Destination, WithContractAbi},
    views::{RootView, View},
    Contract, ContractRuntime,
};

use self::state::Application;
use spec::{
    account::ChainAccountOwner,
    base::{BaseMessage, BaseOperation, CREATOR_CHAIN_CHANNEL},
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
            PoolOperation::SetFeeTo { pool_id, account } => self
                .on_op_set_fee_to(pool_id, account)
                .expect("Failed OP: set fee to"),
            PoolOperation::SetFeeToSetter { pool_id, account } => self
                .on_op_set_fee_to_setter(pool_id, account)
                .expect("Failed OP: set fee to setter"),
            PoolOperation::Mint { pool_id, to } => {
                self.on_op_mint(pool_id, to).expect("Failed OP: mint")
            }
            PoolOperation::Burn { pool_id, to } => {
                self.on_op_burn(pool_id, to).expect("Failed OP: burn")
            }
            PoolOperation::Swap {
                pool_id,
                amount_0_out,
                amount_1_out,
                to,
            } => self
                .on_op_swap(pool_id, amount_0_out, amount_1_out, to)
                .expect("Failed OP: swap"),
        }
    }

    async fn execute_message(&mut self, message: PoolMessage) {
        match message {
            PoolMessage::BaseMessage(base_message) => self
                .execute_base_message(base_message)
                .expect("Failed MSG: base message"),
            PoolMessage::CreatePool {
                token_0,
                token_1,
                amount_0_initial,
                amount_1_initial,
                amount_0_virtual,
                amount_1_virtual,
            } => self
                .on_msg_create_pool(
                    token_0,
                    token_1,
                    amount_0_initial,
                    amount_1_initial,
                    amount_0_virtual,
                    amount_1_virtual,
                )
                .await
                .expect("Failed MSG: create pool"),
            PoolMessage::SetFeeTo { pool_id, account } => self
                .on_msg_set_fee_to(pool_id, account)
                .await
                .expect("Failed MSG: set fee to"),
            PoolMessage::SetFeeToSetter { pool_id, account } => self
                .on_msg_set_fee_to_setter(pool_id, account)
                .await
                .expect("Failed MSG: set fee to setter"),
            PoolMessage::Mint { pool_id, to } => self
                .on_msg_mint(pool_id, to)
                .await
                .expect("Failed MSG: mint"),
            PoolMessage::Burn { pool_id, to } => self
                .on_msg_burn(pool_id, to)
                .await
                .expect("Failed MSG: burn"),
            PoolMessage::Swap {
                pool_id,
                amount_0_out,
                amount_1_out,
                to,
            } => self
                .on_msg_swap(pool_id, amount_0_out, amount_1_out, to)
                .await
                .expect("Failed MSG: swap"),
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
    ) -> Result<PoolResponse, PoolError> {
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
        &mut self,
        token_0: ApplicationId,
        token_1: ApplicationId,
        amount_0_initial: Amount,
        amount_1_initial: Amount,
        amount_0_virtual: Amount,
        amount_1_virtual: Amount,
    ) -> Result<PoolResponse, PoolError> {
        self.runtime
            .prepare_message(PoolMessage::CreatePool {
                token_0,
                token_1,
                amount_0_initial,
                amount_1_initial,
                amount_0_virtual,
                amount_1_virtual,
            })
            .with_authentication()
            .send_to(self.runtime.application_creator_chain_id());
        // We cannot get pool here so we just let creator to process it and return Ok
        Ok(PoolResponse::Ok)
    }

    fn on_op_set_fee_to(
        &mut self,
        pool_id: u64,
        account: ChainAccountOwner,
    ) -> Result<PoolResponse, PoolError> {
        self.runtime
            .prepare_message(PoolMessage::SetFeeTo { pool_id, account })
            .with_authentication()
            .send_to(self.runtime.application_creator_chain_id());
        Ok(PoolResponse::Ok)
    }

    fn on_op_set_fee_to_setter(
        &mut self,
        pool_id: u64,
        account: ChainAccountOwner,
    ) -> Result<PoolResponse, PoolError> {
        self.runtime
            .prepare_message(PoolMessage::SetFeeToSetter { pool_id, account })
            .with_authentication()
            .send_to(self.runtime.application_creator_chain_id());
        Ok(PoolResponse::Ok)
    }

    fn on_op_mint(
        &mut self,
        pool_id: u64,
        to: ChainAccountOwner,
    ) -> Result<PoolResponse, PoolError> {
        self.runtime
            .prepare_message(PoolMessage::Mint { pool_id, to })
            .with_authentication()
            .send_to(self.runtime.application_creator_chain_id());
        Ok(PoolResponse::Ok)
    }

    fn on_op_burn(
        &mut self,
        pool_id: u64,
        to: ChainAccountOwner,
    ) -> Result<PoolResponse, PoolError> {
        self.runtime
            .prepare_message(PoolMessage::Burn { pool_id, to })
            .with_authentication()
            .send_to(self.runtime.application_creator_chain_id());
        Ok(PoolResponse::Ok)
    }

    fn on_op_swap(
        &mut self,
        pool_id: u64,
        amount_0_out: Amount,
        amount_1_out: Amount,
        to: ChainAccountOwner,
    ) -> Result<PoolResponse, PoolError> {
        self.runtime
            .prepare_message(PoolMessage::Swap {
                pool_id,
                amount_0_out,
                amount_1_out,
                to,
            })
            .with_authentication()
            .send_to(self.runtime.application_creator_chain_id());
        Ok(PoolResponse::Ok)
    }

    fn execute_base_message(&mut self, message: BaseMessage) -> Result<(), PoolError> {
        match message {
            BaseMessage::SubscribeCreatorChain => self.on_msg_subscribe_creator_chain(),
        }
    }

    fn on_msg_subscribe_creator_chain(&mut self) -> Result<(), PoolError> {
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

    fn publish_message(&mut self, message: PoolMessage) {
        if self.runtime.chain_id() != self.runtime.application_creator_chain_id() {
            return;
        }
        let dest = Destination::Subscribers(ChannelName::from(CREATOR_CHAIN_CHANNEL.to_vec()));
        self.runtime
            .prepare_message(message)
            .with_authentication()
            .send_to(dest);
    }

    async fn on_msg_create_pool(
        &mut self,
        token_0: ApplicationId,
        token_1: ApplicationId,
        amount_0_initial: Amount,
        amount_1_initial: Amount,
        amount_0_virtual: Amount,
        amount_1_virtual: Amount,
    ) -> Result<(), PoolError> {
        let owner = self.runtime.authenticated_signer().expect("Invalid owner");
        let message_id = self.runtime.message_id().expect("Invalid message id");

        let creator = ChainAccountOwner {
            chain_id: message_id.chain_id,
            owner: Some(AccountOwner::User(owner)),
        };

        // TODO: transfer both token from creators' account

        self.state
            .create_pool(
                token_0,
                token_1,
                amount_0_initial,
                amount_1_initial,
                amount_0_virtual,
                amount_1_virtual,
                creator,
            )
            .await?;

        self.publish_message(PoolMessage::CreatePool {
            token_0,
            token_1,
            amount_0_initial,
            amount_1_initial,
            amount_0_virtual,
            amount_1_virtual,
        });

        Ok(())
    }

    async fn on_msg_set_fee_to(
        &mut self,
        pool_id: u64,
        account: ChainAccountOwner,
    ) -> Result<(), PoolError> {
        self.state.set_fee_to(pool_id, account.clone()).await?;
        self.publish_message(PoolMessage::SetFeeTo { pool_id, account });
        Ok(())
    }

    async fn on_msg_set_fee_to_setter(
        &mut self,
        pool_id: u64,
        account: ChainAccountOwner,
    ) -> Result<(), PoolError> {
        self.state
            .set_fee_to_setter(pool_id, account.clone())
            .await?;
        self.publish_message(PoolMessage::SetFeeToSetter { pool_id, account });
        Ok(())
    }

    async fn on_msg_mint(&mut self, pool_id: u64, to: ChainAccountOwner) -> Result<(), PoolError> {
        self.state.mint(pool_id, to.clone()).await?;
        self.publish_message(PoolMessage::Mint { pool_id, to });
        Ok(())
    }

    async fn on_msg_burn(&mut self, pool_id: u64, to: ChainAccountOwner) -> Result<(), PoolError> {
        self.state.burn(pool_id, to.clone()).await?;
        self.publish_message(PoolMessage::Burn { pool_id, to });
        Ok(())
    }

    async fn on_msg_swap(
        &mut self,
        pool_id: u64,
        amount_0_out: Amount,
        amount_1_out: Amount,
        to: ChainAccountOwner,
    ) -> Result<(), PoolError> {
        self.state
            .swap(pool_id, amount_0_out, amount_1_out, to.clone())
            .await?;
        self.publish_message(PoolMessage::Swap {
            pool_id,
            amount_0_out,
            amount_1_out,
            to,
        });
        Ok(())
    }
}
