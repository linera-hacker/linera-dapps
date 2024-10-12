#![cfg_attr(target_arch = "wasm32", no_main)]

mod state;

use linera_sdk::{
    base::{Account, AccountOwner, Amount, ChannelName, Destination, WithContractAbi},
    views::{RootView, View},
    Contract, ContractRuntime,
};

use self::state::Application;
use erc20::ERC20Error;
use spec::{
    account::ChainAccountOwner,
    base::{BaseMessage, BaseOperation, CREATOR_CHAIN_CHANNEL},
    erc20::{ERC20Message, ERC20Operation, ERC20Response, InstantiationArgument},
    swap::{RouterApplicationAbi, RouterOperation, RouterResponse},
};

pub struct ApplicationContract {
    state: Application,
    runtime: ContractRuntime<Self>,
}

linera_sdk::contract!(ApplicationContract);

impl WithContractAbi for ApplicationContract {
    type Abi = erc20::ApplicationAbi;
}

impl Contract for ApplicationContract {
    type Message = ERC20Message;
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

    async fn execute_operation(&mut self, operation: ERC20Operation) -> ERC20Response {
        match operation {
            ERC20Operation::BaseOperation(base_operation) => self
                .execute_base_operation(base_operation)
                .expect("Failed OP: base operation"),
            ERC20Operation::Transfer { to, amount } => self
                .on_op_transfer(to, amount)
                .expect("Failed OP: transfer"),
            ERC20Operation::TransferFrom { from, amount, to } => self
                .on_op_transfer_from(from, amount, to)
                .expect("Failed OP: transfer from"),
            ERC20Operation::Approve { spender, value } => self
                .on_op_approve(spender, value)
                .expect("Failed OP: approve"),
            ERC20Operation::BalanceOf { owner } => self
                .on_op_balance_of(owner)
                .await
                .expect("Failed OP: balance of"),
            ERC20Operation::Mint { amount } => {
                self.on_op_mint(amount).await.expect("Failed OP: mint")
            }
        }
    }

    async fn execute_message(&mut self, message: ERC20Message) {
        match message {
            ERC20Message::BaseMessage(base_message) => self
                .execute_base_message(base_message)
                .expect("Failed MSG: base message"),
            ERC20Message::Transfer { to, amount } => self
                .on_msg_transfer(to, amount)
                .await
                .expect("Failed MSG: transfer"),
            ERC20Message::TransferFrom { from, amount, to } => self
                .on_msg_transfer_from(from, amount, to)
                .await
                .expect("Failed MSG: transfer from"),
            ERC20Message::Approve { spender, value } => self
                .on_msg_approve(spender, value)
                .await
                .expect("Failed MSG: approve"),
            ERC20Message::Mint { to, amount } => self
                .on_msg_mint(to, amount)
                .await
                .expect("Failed MSG: approve"),
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
    ) -> Result<ERC20Response, ERC20Error> {
        match operation {
            BaseOperation::SubscribeCreatorChain => self.on_op_subscribe_creator_chain(),
        }
    }

    fn on_op_subscribe_creator_chain(&mut self) -> Result<ERC20Response, ERC20Error> {
        self.runtime
            .prepare_message(ERC20Message::BaseMessage(
                BaseMessage::SubscribeCreatorChain,
            ))
            .with_authentication()
            .send_to(self.runtime.application_creator_chain_id());
        Ok(ERC20Response::Ok)
    }

    fn on_op_transfer(
        &mut self,
        to: ChainAccountOwner,
        amount: Amount,
    ) -> Result<ERC20Response, ERC20Error> {
        self.runtime
            .prepare_message(ERC20Message::Transfer { to, amount })
            .with_authentication()
            .send_to(self.runtime.application_creator_chain_id());
        Ok(ERC20Response::Ok)
    }

    fn on_op_transfer_from(
        &mut self,
        from: ChainAccountOwner,
        amount: Amount,
        to: ChainAccountOwner,
    ) -> Result<ERC20Response, ERC20Error> {
        self.runtime
            .prepare_message(ERC20Message::TransferFrom { from, amount, to })
            .with_authentication()
            .send_to(self.runtime.application_creator_chain_id());
        Ok(ERC20Response::Ok)
    }

    fn on_op_approve(
        &mut self,
        spender: ChainAccountOwner,
        value: Amount,
    ) -> Result<ERC20Response, ERC20Error> {
        self.runtime
            .prepare_message(ERC20Message::Approve { spender, value })
            .with_authentication()
            .send_to(self.runtime.application_creator_chain_id());
        Ok(ERC20Response::Ok)
    }

    async fn on_op_balance_of(
        &self,
        owner: ChainAccountOwner,
    ) -> Result<ERC20Response, ERC20Error> {
        Ok(ERC20Response::Balance(
            self.state
                .balances
                .get(&owner)
                .await?
                .unwrap_or(Amount::ZERO),
        ))
    }

    async fn on_op_mint(&mut self, amount: Amount) -> Result<ERC20Response, ERC20Error> {
        let to = self.message_owner();
        self.runtime
            .prepare_message(ERC20Message::Mint { to, amount })
            .with_authentication()
            .send_to(self.runtime.application_creator_chain_id());
        Ok(ERC20Response::Ok)
    }

    fn execute_base_message(&mut self, message: BaseMessage) -> Result<(), ERC20Error> {
        match message {
            BaseMessage::SubscribeCreatorChain => self.on_msg_subscribe_creator_chain(),
        }
    }

    fn on_msg_subscribe_creator_chain(&mut self) -> Result<(), ERC20Error> {
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

    fn publish_message(&mut self, message: ERC20Message) {
        if self.runtime.chain_id() != self.runtime.application_creator_chain_id() {
            return;
        }
        let dest = Destination::Subscribers(ChannelName::from(CREATOR_CHAIN_CHANNEL.to_vec()));
        self.runtime
            .prepare_message(message)
            .with_authentication()
            .send_to(dest);
    }

    async fn on_msg_transfer(
        &mut self,
        to: ChainAccountOwner,
        amount: Amount,
    ) -> Result<(), ERC20Error> {
        let sender = self.message_owner();

        self.state.transfer(sender, amount, to.clone()).await?;

        self.publish_message(ERC20Message::Transfer { to, amount });
        Ok(())
    }

    async fn on_msg_transfer_from(
        &mut self,
        from: ChainAccountOwner,
        amount: Amount,
        to: ChainAccountOwner,
    ) -> Result<(), ERC20Error> {
        let caller = self.message_owner();

        self.state
            .transfer_from(from.clone(), amount, to.clone(), caller)
            .await?;

        self.publish_message(ERC20Message::TransferFrom { from, amount, to });
        Ok(())
    }

    async fn on_msg_approve(
        &mut self,
        spender: ChainAccountOwner,
        value: Amount,
    ) -> Result<(), ERC20Error> {
        let owner = self.message_owner();

        self.state.approve(spender.clone(), value, owner).await?;

        self.publish_message(ERC20Message::Approve { spender, value });
        Ok(())
    }

    async fn on_msg_mint(
        &mut self,
        to: ChainAccountOwner,
        amount: Amount,
    ) -> Result<(), ERC20Error> {
        let token_0 = self.runtime.application_id().forget_abi();
        let token_1 = None;
        let call = RouterOperation::CalculateSwapAmount {
            token_0,
            token_1,
            amount_1: amount,
        };
        let RouterResponse::Amount(currency) =
            self.runtime
                .call_application(true, token_0.with_abi::<RouterApplicationAbi>(), &call)
        else {
            todo!()
        };
        let created_owner = Account {
            chain_id: self.runtime.application_creator_chain_id(),
            owner: None,
        };

        let to_owner = self.runtime.authenticated_signer();
        self.runtime.transfer(to_owner, created_owner, amount);

        self.state
            .deposit_native_and_exchange(to.clone(), amount, currency)
            .await;

        self.publish_message(ERC20Message::Mint { to, amount });
        Ok(())
    }

    fn message_owner(&mut self) -> ChainAccountOwner {
        let message_id = self.runtime.message_id().expect("Invalid message id");
        ChainAccountOwner {
            chain_id: message_id.chain_id,
            owner: Some(AccountOwner::User(
                self.runtime.authenticated_signer().expect("Invalid owner"),
            )),
        }
    }
}
