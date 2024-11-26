#![cfg_attr(target_arch = "wasm32", no_main)]

mod state;

use linera_sdk::{
    base::{
        Account, AccountOwner, Amount, ApplicationId, ChannelName, Destination, Owner,
        WithContractAbi,
    },
    views::{RootView, View},
    Contract, ContractRuntime,
};

use self::state::Application;
use ams::AMSError;
use spec::{
    account::ChainAccountOwner,
    ams::{
        AMSMessage, AMSOperation, AMSParameters, AMSResponse, InstantiationArgument,
        SubscriberSyncState,
    },
    base::{BaseMessage, BaseOperation, CREATOR_CHAIN_CHANNEL},
    swap::{
        abi::{SwapApplicationAbi, SwapOperation, SwapResponse},
        router::{RouterOperation, RouterResponse},
    },
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
    type Parameters = AMSParameters;
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
            AMSOperation::Register { metadata: Metadata } => self
                .on_op_register(metadata)
                .await
                .expect("Failed OP: register"),
            AMSOperation::Register { metadata: Metadata } => self
                .on_op_register(metadata)
                .await
                .expect("Failed OP: transfer"),
        }
    }

    async fn execute_message(&mut self, message: AMSMessage) {
        match message {
            AMSMessage::BaseMessage(base_message) => self
                .execute_base_message(base_message)
                .await
                .expect("Failed MSG: base message"),
            AMSMessage::Transfer { origin, to, amount } => self
                .on_msg_transfer(origin, to, amount)
                .await
                .expect("Failed MSG: transfer"),
            AMSMessage::TransferFrom {
                origin,
                from,
                amount,
                to,
                allowance_owner,
            } => self
                .on_msg_transfer_from(origin, from, amount, to, allowance_owner)
                .await
                .expect("Failed MSG: transfer from"),
            AMSMessage::Approve {
                origin,
                spender,
                value,
            } => self
                .on_msg_approve(origin, spender, value)
                .await
                .expect("Failed MSG: approve"),
            AMSMessage::Mint {
                origin,
                to,
                cur_amount,
            } => self
                .on_msg_mint(origin, to, cur_amount)
                .await
                .expect("Failed MSG: mint"),
            AMSMessage::TransferOwnership { origin, new_owner } => self
                .on_msg_transfer_ownership(origin, new_owner)
                .await
                .expect("Failed MSG: transfer ownership"),
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
    fn swap_application_id(&mut self) -> Option<ApplicationId> {
        self.runtime.application_parameters().swap_application_id
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

    async fn on_op_transfer(
        &mut self,
        to: ChainAccountOwner,
        amount: Amount,
    ) -> Result<AMSResponse, AMSError> {
        let origin = self.runtime_owner();
        self.state.transfer(origin, amount, to.clone()).await?;

        self.runtime
            .prepare_message(AMSMessage::Transfer { origin, to, amount })
            .with_authentication()
            .send_to(self.runtime.application_creator_chain_id());
        Ok(AMSResponse::Ok)
    }

    async fn on_op_transfer_from(
        &mut self,
        from: ChainAccountOwner,
        amount: Amount,
        to: ChainAccountOwner,
    ) -> Result<AMSResponse, AMSError> {
        // If it's called from application, caller will be application creation chain
        // If it's called from owner, we don't know which chain will be the caller, so currently we
        // don't support
        let Some(allowance_owner_application) = self.runtime.authenticated_caller_id() else {
            return Err(AMSError::NotSupported);
        };
        let allowance_owner = ChainAccountOwner {
            chain_id: allowance_owner_application.creation.chain_id,
            owner: Some(AccountOwner::Application(allowance_owner_application)),
        };
        self.state
            .transfer_from(from.clone(), amount, to.clone(), allowance_owner)
            .await?;

        let origin = self.runtime_owner();
        self.runtime
            .prepare_message(AMSMessage::TransferFrom {
                origin,
                from,
                amount,
                to,
                allowance_owner,
            })
            .with_authentication()
            .send_to(self.runtime.application_creator_chain_id());
        Ok(AMSResponse::Ok)
    }

    async fn on_op_approve(
        &mut self,
        spender: ChainAccountOwner,
        value: Amount,
    ) -> Result<AMSResponse, AMSError> {
        let origin = self.runtime_owner();
        self.state.approve(spender.clone(), value, origin).await?;
        self.runtime
            .prepare_message(AMSMessage::Approve {
                origin,
                spender,
                value,
            })
            .with_authentication()
            .send_to(self.runtime.application_creator_chain_id());
        Ok(AMSResponse::Ok)
    }

    async fn on_op_balance_of(&self, owner: ChainAccountOwner) -> Result<AMSResponse, AMSError> {
        Ok(AMSResponse::Balance(self.state.balance_of(owner).await?))
    }

    async fn on_op_transfer_ownership(
        &mut self,
        new_owner: ChainAccountOwner,
    ) -> Result<AMSResponse, AMSError> {
        let origin = self.runtime_owner();
        self.runtime
            .prepare_message(AMSMessage::TransferOwnership { origin, new_owner })
            .with_authentication()
            .send_to(self.runtime.application_creator_chain_id());
        Ok(AMSResponse::Ok)
    }

    async fn on_op_get_owner(&mut self) -> Result<AMSResponse, AMSError> {
        let owner = self.state.owner.get().expect("Invalid owner");
        Ok(AMSResponse::Owner(owner))
    }

    async fn on_op_mint(
        &mut self,
        to: Option<ChainAccountOwner>,
        amount: Amount,
    ) -> Result<AMSResponse, AMSError> {
        let origin = self.runtime_owner();
        let to = to.unwrap_or(origin);

        let swap_application_id = self.swap_application_id();
        let fixed_currency = *self.state.fixed_currency.get();
        let mut cur_amount = amount;

        if !fixed_currency && swap_application_id.is_some() {
            let token_0 = self.runtime.application_id().forget_abi();
            let token_1 = None;
            let call = SwapOperation::RouterOperation(RouterOperation::CalculateSwapAmount {
                token_0,
                token_1,
                amount_1: amount,
            });
            let SwapResponse::RouterResponse(RouterResponse::Amount(swap_amount)) =
                self.runtime.call_application(
                    true,
                    swap_application_id
                        .unwrap()
                        .with_abi::<SwapApplicationAbi>(),
                    &call,
                )
            else {
                return Err(AMSError::CalculateCurrencyError);
            };
            cur_amount = swap_amount;
        }

        let runtime_application_creation = ChainAccountOwner {
            chain_id: self.runtime.application_creator_chain_id(),
            owner: Some(AccountOwner::Application(
                self.runtime.application_id().forget_abi(),
            )),
        };
        let to_account = Account {
            chain_id: runtime_application_creation.chain_id,
            owner: match runtime_application_creation.owner {
                Some(AccountOwner::User(owner)) => Some(owner),
                _ => None,
            },
        };

        let chain_owner = self.state.owner.get().expect("Invalid owner");

        if to == chain_owner {
            return Err(AMSError::PermissionDenied);
        }
        let chain_balance = self.runtime.chain_balance();

        let mut from_owner: Option<Owner> = None;
        if chain_balance < cur_amount {
            from_owner = match chain_owner.owner {
                Some(AccountOwner::User(owner)) => Some(owner),
                _ => None,
            };
        }

        self.runtime.transfer(from_owner, to_account, amount);

        self.state
            .deposit_native_and_exchange(to.clone(), cur_amount)
            .await?;

        self.runtime
            .prepare_message(AMSMessage::Mint {
                origin,
                to,
                cur_amount,
            })
            .with_authentication()
            .send_to(self.runtime.application_creator_chain_id());
        Ok(AMSResponse::Ok)
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

    async fn on_msg_transfer(
        &mut self,
        origin: ChainAccountOwner,
        to: ChainAccountOwner,
        amount: Amount,
    ) -> Result<(), AMSError> {
        if origin.chain_id != self.runtime.chain_id() {
            self.state.transfer(origin, amount, to.clone()).await?;
        }
        self.publish_message(AMSMessage::Transfer { origin, to, amount });
        Ok(())
    }

    async fn on_msg_transfer_from(
        &mut self,
        origin: ChainAccountOwner,
        from: ChainAccountOwner,
        amount: Amount,
        to: ChainAccountOwner,
        allowance_owner: ChainAccountOwner,
    ) -> Result<(), AMSError> {
        if origin.chain_id != self.runtime.chain_id() {
            self.state
                .transfer_from(from.clone(), amount, to.clone(), allowance_owner)
                .await?;
        }

        self.publish_message(AMSMessage::TransferFrom {
            origin,
            from,
            amount,
            to,
            allowance_owner,
        });
        Ok(())
    }

    async fn on_msg_approve(
        &mut self,
        origin: ChainAccountOwner,
        spender: ChainAccountOwner,
        value: Amount,
    ) -> Result<(), AMSError> {
        if origin.chain_id != self.runtime.chain_id() {
            self.state.approve(spender.clone(), value, origin).await?;
        }

        self.publish_message(AMSMessage::Approve {
            origin,
            spender,
            value,
        });
        Ok(())
    }

    async fn on_msg_transfer_ownership(
        &mut self,
        origin: ChainAccountOwner,
        new_owner: ChainAccountOwner,
    ) -> Result<(), AMSError> {
        let owner = self.message_owner();

        self.state.transfer_ownership(owner, new_owner).await?;

        self.publish_message(AMSMessage::TransferOwnership { origin, new_owner });
        Ok(())
    }

    async fn on_msg_mint(
        &mut self,
        origin: ChainAccountOwner,
        to: ChainAccountOwner,
        cur_amount: Amount,
    ) -> Result<(), AMSError> {
        if origin.chain_id != self.runtime.chain_id() {
            self.state
                .deposit_native_and_exchange(to.clone(), cur_amount)
                .await?;
        }

        self.publish_message(AMSMessage::Mint {
            origin,
            to,
            cur_amount,
        });
        Ok(())
    }

    async fn on_msg_subscriber_sync(&mut self, state: SubscriberSyncState) -> Result<(), AMSError> {
        if *self.state.total_supply.get() > Amount::ZERO {
            log::warn!("Stale subscriber state on {}", self.runtime.chain_id());
            return Ok(());
        }
        self.state.from_subscriber_sync_state(state).await
    }
}
