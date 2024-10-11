#![cfg_attr(target_arch = "wasm32", no_main)]

mod state;

use self::state::Application;
use linera_sdk::{
    base::{Amount, ApplicationId, ChannelName, Destination, Timestamp, WithContractAbi, Account, AccountOwner},
    views::{RootView, View, ViewStorageContext},
    Contract, ContractRuntime,
};
use spec::{
    account::ChainAccountOwner,
    base::{BaseMessage, BaseOperation, CREATOR_CHAIN_CHANNEL},
    swap::{Pool, PoolApplicationAbi, RouterMessage, RouterOperation, RouterResponse, PoolOperation, PoolResponse, RouterParameters},
    erc20::{ERC20Operation, ERC20ApplicationAbi},
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
    type Parameters = RouterParameters;
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
                created_pool,
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
                    created_pool,
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

    fn pool_application_id(&mut self) -> ApplicationId<PoolApplicationAbi> {
        self.runtime.application_parameters().pool_application_id
    }

    fn get_pool(&mut self, token_0: ApplicationId, token_1: Option<ApplicationId>) -> Option<Pool> {
        let call = PoolOperation::GetPool { token_0, token_1 };
        let pool_application_id = self.pool_application_id();
        let PoolResponse::Pool(pool) =
            self.runtime
                .call_application(true, pool_application_id, &call)
        else {
            todo!();
        };
        pool
    }

    fn get_or_create_pool(
        &mut self,
        token_0: ApplicationId,
        token_1: Option<ApplicationId>,
        amount_0_desired: Amount,
        amount_1_desired: Amount,
    ) -> Result<(Pool, bool), RouterError> {
        if let Some(pool) = self.get_pool(token_0, token_1) {
            return Ok((pool, false));
        }

        let call = PoolOperation::CreatePool {
            token_0,
            token_1,
            amount_0_initial: amount_0_desired,
            amount_1_initial: amount_1_desired,
            amount_0_virtual: amount_0_desired,
            amount_1_virtual: amount_1_desired,
        };
        let pool_application_id = self.pool_application_id();
        self.runtime
            .call_application(true, pool_application_id, &call);

        if let Some(pool) = self.get_pool(token_0, token_1) {
            return Ok((pool, false));
        }
        Err(RouterError::CreatePoolError)
    }

    fn mint(&mut self, pool: Pool, amount_0: Amount, amount_1: Amount, to: ChainAccountOwner) {
        let call = PoolOperation::Mint {
            pool_id: pool.id,
            amount_0,
            amount_1,
            to,
        };
        let pool_application_id = self.pool_application_id();
        let PoolResponse::Liquidity(_) =
            self.runtime
                .call_application(true, pool_application_id, &call)
        else {
            todo!();
        };
    }

    fn calculate_swap_amount(&self, pool: Pool, amount_0: Amount) -> Result<Amount, RouterError> {
        if pool.reserve_0 <= Amount::ZERO || pool.reserve_1 <= Amount::ZERO {
            return Err(RouterError::InvalidAmount);
        }
        Ok(Amount::from_attos(amount_0
            .saturating_mul(pool.reserve_1.into())
            .saturating_div(pool.reserve_0.into())))
    }

    fn calculate_amounts(
        &self,
        pool: Pool,
        amount_0_desired: Amount,
        amount_1_desired: Amount,
        amount_0_min: Amount,
        amount_1_min: Amount,
    ) -> Result<(Amount, Amount), RouterError> {
        if pool.reserve_0 == Amount::ZERO && pool.reserve_1 == Amount::ZERO {
            return Ok((amount_0_desired, amount_1_desired));
        }
        let amount_1_optimal = self.calculate_swap_amount(pool.clone(), amount_0_desired)?;
        if amount_1_optimal <= amount_1_desired {
            if amount_1_optimal < amount_1_min {
                return Err(RouterError::InvalidAmount);
            }
            return Ok((amount_0_desired, amount_1_optimal));
        }
        let amount_0_optimal = self.calculate_swap_amount(pool, amount_1_desired)?;
        if amount_0_optimal > amount_0_desired {
            return Err(RouterError::InvalidAmount);
        }
        if amount_0_optimal < amount_0_min {
            return Err(RouterError::InvalidAmount);
        }
        Ok((amount_0_optimal, amount_1_desired))
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
        let (mut pool, created) = self
            .get_or_create_pool(token_0, token_1, amount_0_desired, amount_1_desired)
            .expect("Invalid pool");
        let (amount_0, amount_1) = if created {
            (amount_0_desired, amount_1_desired)
        } else {
            self.calculate_amounts(
                pool.clone(),
                amount_0_desired,
                amount_1_desired,
                amount_0_min,
                amount_1_min,
            )?
        };

        // Fake add reserves to calculate liquidity, it'll be persisted later
        let mut fake_pool = pool.clone();
        fake_pool.reserve_0.saturating_add_assign(amount_0);
        fake_pool.reserve_0.saturating_add_assign(amount_1);
        let liquidity = fake_pool.calculate_liquidity(amount_0, amount_1);

        self.runtime
            .prepare_message(RouterMessage::AddLiquidity {
                token_0,
                token_1,
                amount_0_desired,
                amount_1_desired,
                amount_0_min,
                amount_1_min,
                created_pool: created,
                to,
                deadline,
            })
            .with_authentication()
            .send_to(self.runtime.application_creator_chain_id());
        Ok(RouterResponse::Liquidity((amount_0, amount_1, liquidity)))
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

    fn transfer_erc20_from(&mut self, token: ApplicationId, amount: Amount) {
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

    fn transfer_native(&mut self, amount: Amount) {
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

    async fn on_msg_add_liquidity(
        &mut self,
        token_0: ApplicationId,
        token_1: Option<ApplicationId>,
        amount_0_desired: Amount,
        amount_1_desired: Amount,
        amount_0_min: Amount,
        amount_1_min: Amount,
        created_pool: bool,
        to: ChainAccountOwner,
        deadline: Timestamp,
    ) -> Result<(), RouterError> {
        let pool = self.get_pool(token_0, token_1).expect("Invalid pool");
        let (amount_0, amount_1) = if created_pool {
            (amount_0_desired, amount_1_desired)
        } else {
            self.calculate_amounts(
                pool.clone(),
                amount_0_desired,
                amount_1_desired,
                amount_0_min,
                amount_1_min,
            )?
        };

        if amount_0 > Amount::ZERO {
            self.transfer_erc20_from(token_0, amount_0);
        }
        if amount_1 > Amount::ZERO {
            match token_1 {
                Some(_token_1) => self.transfer_erc20_from(_token_1, amount_1),
                None => self.transfer_native(amount_1),
            }
        }

        self.mint(pool, amount_0, amount_1, to);

        self.publish_message(RouterMessage::AddLiquidity {
            token_0,
            token_1,
            amount_0_desired,
            amount_1_desired,
            amount_0_min,
            amount_1_min,
            created_pool,
            to,
            deadline,
        });
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
