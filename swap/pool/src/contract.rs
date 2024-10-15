#![cfg_attr(target_arch = "wasm32", no_main)]

mod state;

use linera_sdk::{
    base::{
        Account, AccountOwner, Amount, ApplicationId, ChannelName, Destination, WithContractAbi,
    },
    views::{RootView, View},
    Contract, ContractRuntime,
};

use self::state::Application;
use spec::{
    account::ChainAccountOwner,
    base::{BaseMessage, BaseOperation, CREATOR_CHAIN_CHANNEL},
    erc20::{ERC20ApplicationAbi, ERC20Operation, ERC20Response},
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

    async fn instantiate(&mut self, _argument: Self::InstantiationArgument) {
        self.runtime.application_parameters();
    }

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
                .await
                .expect("Failed OP: create pool"),
            PoolOperation::SetFeeTo { pool_id, account } => self
                .on_op_set_fee_to(pool_id, account)
                .expect("Failed OP: set fee to"),
            PoolOperation::SetFeeToSetter { pool_id, account } => self
                .on_op_set_fee_to_setter(pool_id, account)
                .expect("Failed OP: set fee to setter"),
            PoolOperation::Mint {
                pool_id,
                amount_0,
                amount_1,
                to,
            } => self
                .on_op_mint(pool_id, amount_0, amount_1, to)
                .await
                .expect("Failed OP: mint"),
            PoolOperation::Burn {
                pool_id,
                liquidity,
                to,
            } => self
                .on_op_burn(pool_id, liquidity, to)
                .await
                .expect("Failed OP: burn"),
            PoolOperation::Swap {
                pool_id,
                amount_0_out,
                amount_1_out,
                to,
            } => self
                .on_op_swap(pool_id, amount_0_out, amount_1_out, to)
                .expect("Failed OP: swap"),
            PoolOperation::GetPoolWithTokenPair { token_0, token_1 } => self
                .on_op_get_pool_with_token_pair(token_0, token_1)
                .await
                .expect("Failed OP: get pool"),
            PoolOperation::SetRouterApplicationId { application_id } => self
                .on_op_set_router_application_id(application_id)
                .expect("Failed OP: set router application id"),
        }
    }

    async fn execute_message(&mut self, message: PoolMessage) {
        match message {
            PoolMessage::BaseMessage(base_message) => self
                .execute_base_message(base_message)
                .expect("Failed MSG: base message"),
            PoolMessage::CreatePool {
                origin,
                token_0,
                token_1,
                amount_0_initial,
                amount_1_initial,
                amount_0_virtual,
                amount_1_virtual,
            } => self
                .on_msg_create_pool(
                    origin,
                    token_0,
                    token_1,
                    amount_0_initial,
                    amount_1_initial,
                    amount_0_virtual,
                    amount_1_virtual,
                )
                .await
                .expect("Failed MSG: create pool"),
            PoolMessage::SetFeeTo {
                origin,
                pool_id,
                account,
            } => self
                .on_msg_set_fee_to(origin, pool_id, account)
                .await
                .expect("Failed MSG: set fee to"),
            PoolMessage::SetFeeToSetter {
                origin,
                pool_id,
                account,
            } => self
                .on_msg_set_fee_to_setter(origin, pool_id, account)
                .await
                .expect("Failed MSG: set fee to setter"),
            PoolMessage::Mint {
                origin,
                pool_id,
                amount_0,
                amount_1,
                to,
            } => self
                .on_msg_mint(origin, pool_id, amount_0, amount_1, to)
                .await
                .expect("Failed MSG: mint"),
            PoolMessage::Burn {
                origin,
                pool_id,
                liquidity,
                to,
            } => self
                .on_msg_burn(origin, pool_id, liquidity, to)
                .await
                .expect("Failed MSG: burn"),
            PoolMessage::Swap {
                origin,
                pool_id,
                amount_0_out,
                amount_1_out,
                to,
            } => self
                .on_msg_swap(origin, pool_id, amount_0_out, amount_1_out, to)
                .await
                .expect("Failed MSG: swap"),
            PoolMessage::SetRouterApplicationId {
                origin,
                application_id,
            } => self
                .on_msg_set_router_application_id(origin, application_id)
                .await
                .expect("Failed MSG: set router application id"),
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
        let origin = self.runtime_owner();
        self.runtime
            .prepare_message(PoolMessage::BaseMessage(
                BaseMessage::SubscribeCreatorChain { origin },
            ))
            .with_authentication()
            .send_to(self.runtime.application_creator_chain_id());
        Ok(PoolResponse::Ok)
    }

    async fn on_op_create_pool(
        &mut self,
        token_0: ApplicationId,
        token_1: Option<ApplicationId>,
        amount_0_initial: Amount,
        amount_1_initial: Amount,
        amount_0_virtual: Amount,
        amount_1_virtual: Amount,
    ) -> Result<PoolResponse, PoolError> {
        let creator = self.runtime_owner();

        self.state
            .create_pool(
                token_0,
                token_1,
                amount_0_initial,
                amount_1_initial,
                amount_0_virtual,
                amount_1_virtual,
                creator,
                self.runtime.system_time(),
            )
            .await?;

        self.runtime
            .prepare_message(PoolMessage::CreatePool {
                origin: creator,
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
        let origin = self.runtime_owner();
        self.runtime
            .prepare_message(PoolMessage::SetFeeTo {
                origin,
                pool_id,
                account,
            })
            .with_authentication()
            .send_to(self.runtime.application_creator_chain_id());
        Ok(PoolResponse::Ok)
    }

    fn on_op_set_fee_to_setter(
        &mut self,
        pool_id: u64,
        account: ChainAccountOwner,
    ) -> Result<PoolResponse, PoolError> {
        let origin = self.runtime_owner();
        self.runtime
            .prepare_message(PoolMessage::SetFeeToSetter {
                origin,
                pool_id,
                account,
            })
            .with_authentication()
            .send_to(self.runtime.application_creator_chain_id());
        Ok(PoolResponse::Ok)
    }

    fn balance_of_erc20(&mut self, token: ApplicationId) -> Amount {
        let owner = ChainAccountOwner {
            chain_id: self.runtime.application_creator_chain_id(),
            owner: Some(AccountOwner::Application(
                self.runtime.application_id().forget_abi(),
            )),
        };

        let call = ERC20Operation::BalanceOf { owner };
        let ERC20Response::Balance(balance) =
            self.runtime
                .call_application(true, token.with_abi::<ERC20ApplicationAbi>(), &call)
        else {
            todo!()
        };
        balance
    }

    async fn on_op_mint(
        &mut self,
        pool_id: u64,
        amount_0: Amount,
        amount_1: Amount,
        to: ChainAccountOwner,
    ) -> Result<PoolResponse, PoolError> {
        // Only router application on its creator chain can call
        let caller = self
            .runtime
            .authenticated_caller_id()
            .expect("Invalid caller");
        let Some(router_application_id) = self.state.get_router_application_id() else {
            return Err(PoolError::PermissionDenied);
        };
        if router_application_id != caller {
            return Err(PoolError::PermissionDenied);
        }

        // To here, router should already transfer tokens
        let pool = self.state.get_pool(pool_id).await?.expect("Invalid pool");
        if pool.token_1.is_none() {
            return Err(PoolError::NotSupported);
        }
        // Liquidity calculated here may be not accurate, it may changed when process message
        let liquidity = pool.calculate_liquidity(amount_0, amount_1);

        let origin = self.runtime_owner();
        self.runtime
            .prepare_message(PoolMessage::Mint {
                origin,
                pool_id,
                amount_0,
                amount_1,
                to,
            })
            .with_authentication()
            .send_to(self.runtime.application_creator_chain_id());
        Ok(PoolResponse::Liquidity(liquidity))
    }

    async fn calculate_amount_pair(
        &mut self,
        pool_id: u64,
        liquidity: Amount,
    ) -> Result<(Amount, Amount), PoolError> {
        let pool = self.state.get_pool(pool_id).await?.expect("Invalid pool");
        let balance_0 = self.balance_of_erc20(pool.token_0);
        let balance_1 = match pool.token_1 {
            Some(token_1) => self.balance_of_erc20(token_1),
            // TODO: here we should get balance of this application instance
            _ => todo!(),
        };
        Ok(pool.calculate_amount_pair(liquidity, balance_0, balance_1))
    }

    async fn on_op_burn(
        &mut self,
        pool_id: u64,
        liquidity: Amount,
        to: ChainAccountOwner,
    ) -> Result<PoolResponse, PoolError> {
        // To here, shares should already be returned
        // Only router application on its creator chain can call
        let caller = self
            .runtime
            .authenticated_caller_id()
            .expect("Invalid caller");
        let Some(router_application_id) = self.state.get_router_application_id() else {
            return Err(PoolError::PermissionDenied);
        };
        if router_application_id != caller {
            return Err(PoolError::PermissionDenied);
        }

        let amounts = self.calculate_amount_pair(pool_id, liquidity).await?;

        let origin = self.runtime_owner();
        self.runtime
            .prepare_message(PoolMessage::Burn {
                origin,
                pool_id,
                liquidity,
                to,
            })
            .with_authentication()
            .send_to(self.runtime.application_creator_chain_id());

        Ok(PoolResponse::AmountPair(amounts))
    }

    fn on_op_swap(
        &mut self,
        pool_id: u64,
        amount_0_out: Amount,
        amount_1_out: Amount,
        to: ChainAccountOwner,
    ) -> Result<PoolResponse, PoolError> {
        if amount_0_out <= Amount::ZERO || amount_1_out <= Amount::ZERO {
            return Err(PoolError::InvalidAmount);
        }

        let origin = self.runtime_owner();
        self.runtime
            .prepare_message(PoolMessage::Swap {
                origin,
                pool_id,
                amount_0_out,
                amount_1_out,
                to,
            })
            .with_authentication()
            .send_to(self.runtime.application_creator_chain_id());
        Ok(PoolResponse::Ok)
    }

    async fn on_op_get_pool_with_token_pair(
        &self,
        token_0: ApplicationId,
        token_1: Option<ApplicationId>,
    ) -> Result<PoolResponse, PoolError> {
        let pool = self
            .state
            .get_pool_with_token_pair(token_0, token_1)
            .await?;
        Ok(PoolResponse::Pool(pool))
    }

    fn on_op_set_router_application_id(
        &mut self,
        application_id: ApplicationId,
    ) -> Result<PoolResponse, PoolError> {
        if self.runtime.chain_id() != self.runtime.application_creator_chain_id() {
            return Err(PoolError::PermissionDenied);
        }
        let origin = self.runtime_owner();
        self.runtime
            .prepare_message(PoolMessage::SetRouterApplicationId {
                origin,
                application_id,
            })
            .with_authentication()
            .send_to(self.runtime.application_creator_chain_id());
        Ok(PoolResponse::Ok)
    }

    fn execute_base_message(&mut self, message: BaseMessage) -> Result<(), PoolError> {
        match message {
            BaseMessage::SubscribeCreatorChain { origin: _ } => {
                self.on_msg_subscribe_creator_chain()
            }
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

    fn receive_erc20_from(&mut self, token: ApplicationId, amount: Amount) {
        if self.runtime.chain_id() != self.runtime.application_creator_chain_id() {
            return;
        }

        let message_id = self.runtime.message_id().expect("Invalid message id");

        let from = ChainAccountOwner {
            chain_id: message_id.chain_id,
            owner: Some(AccountOwner::User(
                self.runtime.authenticated_signer().expect("Invalid owner"),
            )),
        };
        let to = ChainAccountOwner {
            chain_id: self.runtime.application_creator_chain_id(),
            owner: Some(AccountOwner::Application(
                self.runtime.application_id().forget_abi(),
            )),
        };

        let call = ERC20Operation::TransferFrom { from, amount, to };
        self.runtime
            .call_application(true, token.with_abi::<ERC20ApplicationAbi>(), &call);
    }

    fn transfer_erc20_to(&mut self, token: ApplicationId, amount: Amount, to: ChainAccountOwner) {
        if self.runtime.chain_id() != self.runtime.application_creator_chain_id() {
            return;
        }

        let call = ERC20Operation::Transfer { amount, to };
        self.runtime
            .call_application(true, token.with_abi::<ERC20ApplicationAbi>(), &call);
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
        ChainAccountOwner {
            chain_id: self.runtime.chain_id(),
            owner: Some(AccountOwner::User(
                self.runtime.authenticated_signer().expect("Invalid owner"),
            )),
        }
    }

    fn receive_native_from(&mut self, amount: Amount) {
        if self.runtime.chain_id() != self.runtime.application_creator_chain_id() {
            return;
        }

        let to = Account {
            chain_id: self.runtime.application_creator_chain_id(),
            owner: None,
            // TODO: we should transfer to application for liquidity
            /*
            owner: Some(AccountOwner::Application(
                self.runtime.application_id().forget_abi(),
            )),
            */
        };

        let owner = self.runtime.authenticated_signer();
        self.runtime.transfer(owner, to, amount);
    }

    fn transfer_native_to(&mut self, _amount: Amount, _to: ChainAccountOwner) {
        if self.runtime.chain_id() != self.runtime.application_creator_chain_id() {
            return;
        }
        panic!("Not supported");
    }

    async fn on_msg_create_pool(
        &mut self,
        origin: ChainAccountOwner,
        token_0: ApplicationId,
        token_1: Option<ApplicationId>,
        amount_0_initial: Amount,
        amount_1_initial: Amount,
        amount_0_virtual: Amount,
        amount_1_virtual: Amount,
    ) -> Result<(), PoolError> {
        let creator = self.message_owner();

        if amount_0_initial > Amount::ZERO {
            self.receive_erc20_from(token_0, amount_0_initial);
        }
        if amount_1_initial > Amount::ZERO {
            match token_1 {
                Some(_token_1) => self.receive_erc20_from(_token_1, amount_1_initial),
                None => self.receive_native_from(amount_1_initial),
            }
        }

        if origin.chain_id != self.runtime.chain_id() {
            self.state
                .create_pool(
                    token_0,
                    token_1,
                    amount_0_initial,
                    amount_1_initial,
                    amount_0_virtual,
                    amount_1_virtual,
                    creator,
                    self.runtime.system_time(),
                )
                .await?;
        }

        self.publish_message(PoolMessage::CreatePool {
            origin,
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
        origin: ChainAccountOwner,
        pool_id: u64,
        account: ChainAccountOwner,
    ) -> Result<(), PoolError> {
        let setter = self.message_owner();
        self.state
            .set_fee_to(pool_id, account.clone(), setter)
            .await?;
        self.publish_message(PoolMessage::SetFeeTo {
            origin,
            pool_id,
            account,
        });
        Ok(())
    }

    async fn on_msg_set_fee_to_setter(
        &mut self,
        origin: ChainAccountOwner,
        pool_id: u64,
        account: ChainAccountOwner,
    ) -> Result<(), PoolError> {
        let setter = self.message_owner();
        self.state
            .set_fee_to_setter(pool_id, account.clone(), setter)
            .await?;
        self.publish_message(PoolMessage::SetFeeToSetter {
            origin,
            pool_id,
            account,
        });
        Ok(())
    }

    async fn on_msg_mint(
        &mut self,
        origin: ChainAccountOwner,
        pool_id: u64,
        amount_0: Amount,
        amount_1: Amount,
        to: ChainAccountOwner,
    ) -> Result<(), PoolError> {
        let pool = self.state.get_pool(pool_id).await?.expect("Invalid pool");

        let balance_0 = self.balance_of_erc20(pool.token_0);
        let balance_1 = match pool.token_1 {
            Some(token_1) => self.balance_of_erc20(token_1),
            // TODO: here we should get balance of this application instance
            _ => return Err(PoolError::NotSupported),
            /*
                self
                .runtime
                .chain_balance(self.runtime.application_creator_chain_id()),
            */
        };

        if balance_0.saturating_sub(pool.reserve_0) < amount_0 {
            return Err(PoolError::InsufficientFunds);
        }
        if balance_1.saturating_sub(pool.reserve_1) < amount_1 {
            return Err(PoolError::InsufficientFunds);
        }

        self.state.mint_fee(pool_id).await?;
        let liquidity = pool.calculate_liquidity(amount_0, amount_1);
        self.state.mint(pool_id, liquidity, to.clone()).await?;
        self.state
            .update(pool_id, balance_0, balance_1, self.runtime.system_time())
            .await?;

        self.publish_message(PoolMessage::Mint {
            origin,
            pool_id,
            amount_0,
            amount_1,
            to,
        });
        Ok(())
    }

    async fn on_msg_burn(
        &mut self,
        origin: ChainAccountOwner,
        pool_id: u64,
        liquidity: Amount,
        to: ChainAccountOwner,
    ) -> Result<(), PoolError> {
        let myself = ChainAccountOwner {
            chain_id: self.runtime.application_creator_chain_id(),
            owner: Some(AccountOwner::Application(
                self.runtime.application_id().forget_abi(),
            )),
        };

        self.state.burn(pool_id, liquidity, myself).await?;

        self.state.mint_fee(pool_id).await?;
        let (amount_0, amount_1) = self.calculate_amount_pair(pool_id, liquidity).await?;
        let pool = self.state.get_pool(pool_id).await?.expect("Invalid pool");
        self.transfer_erc20_to(pool.token_0, amount_0, to);
        match pool.token_1 {
            Some(token_1) => self.transfer_erc20_to(token_1, amount_1, to),
            // TODO: transfer native token
            _ => todo!(),
        }

        self.publish_message(PoolMessage::Burn {
            origin,
            pool_id,
            liquidity,
            to,
        });
        Ok(())
    }

    async fn on_msg_swap(
        &mut self,
        origin: ChainAccountOwner,
        pool_id: u64,
        amount_0_out: Amount,
        amount_1_out: Amount,
        to: ChainAccountOwner,
    ) -> Result<(), PoolError> {
        let pool = self.state.get_pool(pool_id).await?.expect("Invalid pool");
        if amount_0_out > Amount::ZERO {
            self.transfer_erc20_to(pool.token_0, amount_0_out, to);
        }
        if amount_1_out > Amount::ZERO {
            match pool.token_1 {
                Some(token_1) => self.transfer_erc20_to(token_1, amount_1_out, to),
                // TODO: transfer native token
                _ => todo!(),
            }
        }

        let balance_0 = self.balance_of_erc20(pool.token_0);
        let balance_1 = match pool.token_1 {
            Some(token_1) => self.balance_of_erc20(token_1),
            // TODO: transfer native token
            _ => todo!(),
        };

        let amount_0_in =
            balance_0.saturating_sub(pool.reserve_0.saturating_mul(amount_0_out.into()));
        let amount_1_in =
            balance_1.saturating_sub(pool.reserve_1.saturating_mul(amount_1_out.into()));
        if amount_0_in <= Amount::ZERO && amount_1_in <= Amount::ZERO {
            return Err(PoolError::InsufficientLiquidity);
        }

        // TODO: rate should be percent
        let balance_0_adjusted =
            balance_0.saturating_sub(amount_0_in.saturating_mul(pool.pool_fee_rate.into()));
        let balance_1_adjusted =
            balance_1.saturating_sub(amount_1_in.saturating_mul(pool.pool_fee_rate.into()));
        if balance_0_adjusted.saturating_mul(balance_1_adjusted.into())
            >= pool.reserve_0.saturating_mul(pool.reserve_1.into())
        {
            return Err(PoolError::BrokenK);
        }
        self.state
            .update(pool_id, balance_0, balance_1, self.runtime.system_time())
            .await?;

        self.publish_message(PoolMessage::Swap {
            origin,
            pool_id,
            amount_0_out,
            amount_1_out,
            to,
        });
        Ok(())
    }

    async fn on_msg_set_router_application_id(
        &mut self,
        origin: ChainAccountOwner,
        application_id: ApplicationId,
    ) -> Result<(), PoolError> {
        self.state.set_router_application_id(application_id).await;
        self.publish_message(PoolMessage::SetRouterApplicationId {
            origin,
            application_id,
        });
        Ok(())
    }
}
