#![cfg_attr(target_arch = "wasm32", no_main)]

mod state;

use self::state::Application;
use linera_sdk::{
    base::{
        Account, AccountOwner, Amount, ApplicationId, ChannelName, Destination, Timestamp,
        WithContractAbi,
    },
    views::{RootView, View},
    Contract, ContractRuntime,
};
use spec::{
    account::ChainAccountOwner,
    base::{BaseMessage, BaseOperation, CREATOR_CHAIN_CHANNEL},
    erc20::{ERC20ApplicationAbi, ERC20Operation, ERC20Response},
    swap::{
        Pool, PoolApplicationAbi, PoolOperation, PoolResponse, RouterMessage, RouterOperation,
        RouterParameters, RouterResponse, RouterSubscriberSyncState,
    },
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
            RouterOperation::PoolOperation(pool_operation) => self
                .execute_pool_operation(base_operation)
                .await
                .expect("Fail OP: pool operation"),
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
            RouterOperation::Swap {
                token_0,
                token_1,
                amount_0_in,
                amount_1_in,
                amount_0_out_min,
                amount_1_out_min,
                to,
            } => self
                .on_op_swap(
                    token_0,
                    token_1,
                    amount_0_in,
                    amount_1_in,
                    amount_0_out_min,
                    amount_1_out_min,
                    to,
                )
                .expect("Failed OP: swap"),
        }
    }

    async fn execute_message(&mut self, message: Self::Message) {
        match message {
            RouterMessage::BaseMessage(base_message) => self
                .execute_base_message(base_message)
                .expect("Failed MSG: base message"),
            RouterMessage::PoolMessage(pool_message) => self
                .execute_pool_message(pool_message)
                .await
                .expect("Failed MSG: pool message"),
            RouterMessage::AddLiquidity {
                origin,
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
                    origin,
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
                origin,
                token_0,
                token_1,
                liquidity,
                amount_0_min,
                amount_1_min,
                to,
                deadline,
            } => self
                .on_msg_remove_liquidity(
                    origin,
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
            RouterMessage::Swap {
                origin,
                token_0,
                token_1,
                amount_0_in,
                amount_1_in,
                amount_0_out_min,
                amount_1_out_min,
                to,
            } => self
                .on_msg_swap(
                    origin,
                    token_0,
                    token_1,
                    amount_0_in,
                    amount_1_in,
                    amount_0_out_min,
                    amount_1_out_min,
                    to,
                )
                .await
                .expect("Failed MSG: swap"),
            RouterMessage::SubscriberSync { origin, state } => self
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

    fn execute_base_operation(
        &mut self,
        operation: BaseOperation,
    ) -> Result<RouterResponse, RouterError> {
        match operation {
            BaseOperation::SubscribeCreatorChain => self.on_op_subscribe_creator_chain(),
        }
    }

    fn on_op_subscribe_creator_chain(&mut self) -> Result<RouterResponse, RouterError> {
        let origin = self.runtime_owner();
        self.runtime
            .prepare_message(RouterMessage::BaseMessage(
                BaseMessage::SubscribeCreatorChain { origin },
            ))
            .with_authentication()
            .send_to(self.runtime.application_creator_chain_id());
        Ok(RouterResponse::Ok)
    }

    fn pool_application_id(&mut self) -> ApplicationId<PoolApplicationAbi> {
        self.runtime.application_parameters().pool_application_id
    }

    fn get_pool(&mut self, token_0: ApplicationId, token_1: Option<ApplicationId>) -> Option<Pool> {
        let call = PoolOperation::GetPoolWithTokenPair { token_0, token_1 };
        let pool_application_id = self.pool_application_id();
        let PoolResponse::Pool(pool) =
            self.runtime
                .call_application(true, pool_application_id, &call)
        else {
            todo!();
        };
        pool
    }

    fn get_pool_exchangable(
        &mut self,
        token_0: ApplicationId,
        token_1: Option<ApplicationId>,
    ) -> Option<(Pool, bool)> {
        if let Some(pool) = self.get_pool(token_0, token_1) {
            return Some((pool, false));
        }
        if token_1.is_none() {
            return None;
        }
        if let Some(pool) = self.get_pool(token_1.unwrap(), Some(token_0)) {
            return Some((pool, true));
        }
        None
    }

    fn get_or_create_pool(
        &mut self,
        token_0: ApplicationId,
        token_1: Option<ApplicationId>,
        amount_0_desired: Amount,
        amount_1_desired: Amount,
    ) -> Result<(Pool, bool, bool), RouterError> {
        if let Some((pool, exchanged)) = self.get_pool_exchangable(token_0, token_1) {
            return Ok((pool, false, exchanged));
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
            return Ok((pool, true, false));
        }
        Err(RouterError::CreatePoolError)
    }

    fn mint(
        &mut self,
        token_0: ApplicationId,
        token_1: Option<ApplicationId>,
        amount_0: Amount,
        amount_1: Amount,
        to: ChainAccountOwner,
    ) {
        let call = PoolOperation::MintWithTokenPair {
            token_0,
            token_1,
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

    fn calculate_swap_amount_1(&self, pool: Pool, amount_0: Amount) -> Result<Amount, RouterError> {
        if pool.reserve_0 <= Amount::ZERO || pool.reserve_1 <= Amount::ZERO {
            return Err(RouterError::InvalidAmount);
        }
        Ok(Amount::from_attos(
            amount_0
                .saturating_div(pool.reserve_0.into())
                .saturating_mul(pool.reserve_1.into()),
        ))
    }

    fn calculate_swap_amount_0(&self, pool: Pool, amount_1: Amount) -> Result<Amount, RouterError> {
        if pool.reserve_0 <= Amount::ZERO || pool.reserve_1 <= Amount::ZERO {
            return Err(RouterError::InvalidAmount);
        }
        Ok(Amount::from_attos(
            amount_1
                .saturating_div(pool.reserve_1.into())
                .saturating_mul(pool.reserve_0.into()),
        ))
    }

    fn calculate_amount_pair(
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
        let amount_1_optimal = self.calculate_swap_amount_1(pool.clone(), amount_0_desired)?;
        if amount_1_optimal <= amount_1_desired {
            if amount_1_optimal < amount_1_min {
                return Err(RouterError::InvalidAmount);
            }
            return Ok((amount_0_desired, amount_1_optimal));
        }
        let amount_0_optimal = self.calculate_swap_amount_0(pool, amount_1_desired)?;
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
        to: Option<ChainAccountOwner>,
        deadline: Timestamp,
    ) -> Result<RouterResponse, RouterError> {
        let (pool, created, exchanged) = self
            .get_or_create_pool(token_0, token_1, amount_0_desired, amount_1_desired)
            .expect("Invalid pool");

        log::info!(
            "Op add liquidity 111 {}, {}",
            self.runtime.chain_id(),
            created
        );

        let token_0 = if exchanged { token_1.unwrap() } else { token_0 };
        let token_1 = if exchanged { Some(token_0) } else { token_1 };
        let amount_0_desired = if exchanged {
            amount_1_desired
        } else {
            amount_0_desired
        };
        let amount_1_desired = if exchanged {
            amount_0_desired
        } else {
            amount_1_desired
        };
        let amount_0_min = if exchanged {
            amount_1_min
        } else {
            amount_0_min
        };
        let amount_1_min = if exchanged {
            amount_0_min
        } else {
            amount_1_min
        };

        let (amount_0, amount_1) = if created {
            (amount_0_desired, amount_1_desired)
        } else {
            self.calculate_amount_pair(
                pool.clone(),
                amount_0_desired,
                amount_1_desired,
                amount_0_min,
                amount_1_min,
            )?
        };

        // TODO: check balance

        let to = match to {
            Some(_to) => _to,
            None => ChainAccountOwner {
                chain_id: self.runtime.chain_id(),
                owner: Some(AccountOwner::User(
                    self.runtime.authenticated_signer().expect("Invalid owner"),
                )),
            },
        };

        log::info!("Op add liquidity {}", created);

        // Fake add reserves to calculate liquidity, it'll be persisted later
        let mut fake_pool = pool.clone();
        fake_pool.reserve_0.saturating_add_assign(amount_0);
        fake_pool.reserve_0.saturating_add_assign(amount_1);
        let liquidity = fake_pool.calculate_liquidity(amount_0, amount_1);

        let origin = self.runtime_owner();
        self.runtime
            .prepare_message(RouterMessage::AddLiquidity {
                origin,
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
            todo!();
        };
        balance
    }

    fn on_op_remove_liquidity(
        &mut self,
        token_0: ApplicationId,
        token_1: Option<ApplicationId>,
        liquidity: Amount,
        amount_0_min: Amount,
        amount_1_min: Amount,
        to: Option<ChainAccountOwner>,
        deadline: Timestamp,
    ) -> Result<RouterResponse, RouterError> {
        let (pool, exchanged) = self
            .get_pool_exchangable(token_0, token_1)
            .expect("Invalid pool");

        let token_0 = if exchanged { token_1.unwrap() } else { token_0 };
        let token_1 = if exchanged { Some(token_0) } else { token_1 };
        let amount_0_min = if exchanged {
            amount_1_min
        } else {
            amount_0_min
        };
        let amount_1_min = if exchanged {
            amount_0_min
        } else {
            amount_1_min
        };

        let balance_0 = self.balance_of_erc20(pool.token_0);
        let balance_1 = match pool.token_1 {
            Some(token_1) => self.balance_of_erc20(token_1),
            // TODO: here we should get balance of this application instance
            _ => todo!(),
        };
        let (amount_0, amount_1) = pool.calculate_amount_pair(liquidity, balance_0, balance_1);
        if amount_0 < amount_0_min || amount_1 < amount_1_min {
            return Err(RouterError::InvalidAmount);
        }

        let to = match to {
            Some(_to) => _to,
            None => ChainAccountOwner {
                chain_id: self.runtime.chain_id(),
                owner: Some(AccountOwner::User(
                    self.runtime.authenticated_signer().expect("Invalid owner"),
                )),
            },
        };

        let origin = self.runtime_owner();
        self.runtime
            .prepare_message(RouterMessage::RemoveLiquidity {
                origin,
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
        Ok(RouterResponse::AmountPair((amount_0, amount_1)))
    }

    fn on_op_calculate_swap_amount(
        &mut self,
        token_0: ApplicationId,
        token_1: Option<ApplicationId>,
        amount_1: Amount,
    ) -> Result<RouterResponse, RouterError> {
        let (pool, exchanged) = self
            .get_pool_exchangable(token_0, token_1)
            .expect("Invalid pool");

        let amount = if exchanged {
            self.calculate_swap_amount_1(pool, amount_1)?
        } else {
            self.calculate_swap_amount_0(pool, amount_1)?
        };

        Ok(RouterResponse::Amount(amount))
    }

    fn on_op_swap(
        &mut self,
        token_0: ApplicationId,
        token_1: Option<ApplicationId>,
        amount_0_in: Option<Amount>,
        amount_1_in: Option<Amount>,
        amount_0_out_min: Option<Amount>,
        amount_1_out_min: Option<Amount>,
        to: Option<ChainAccountOwner>,
    ) -> Result<RouterResponse, RouterError> {
        let (pool, exchanged) = self
            .get_pool_exchangable(token_0, token_1)
            .expect("Invalid pool");

        let token_0 = if exchanged { token_1.unwrap() } else { token_0 };
        let token_1 = if exchanged { Some(token_0) } else { token_1 };
        let amount_0_in = if exchanged { amount_1_in } else { amount_0_in };
        let amount_1_in = if exchanged { amount_0_in } else { amount_1_in };
        let amount_0_out_min = if exchanged {
            amount_1_out_min
        } else {
            amount_0_out_min
        };
        let amount_1_out_min = if exchanged {
            amount_0_out_min
        } else {
            amount_1_out_min
        };

        if let Some(_amount_0_out_min) = amount_0_out_min {
            if let Some(_amount_1_in) = amount_1_in {
                if self.calculate_swap_amount_0(pool.clone(), _amount_1_in)? < _amount_0_out_min {
                    return Err(RouterError::InvalidAmount);
                }
            }
        }
        if let Some(_amount_1_out_min) = amount_1_out_min {
            if let Some(_amount_0_in) = amount_0_in {
                if self.calculate_swap_amount_1(pool, _amount_0_in)? < _amount_1_out_min {
                    return Err(RouterError::InvalidAmount);
                }
            }
        }

        let to = match to {
            Some(_to) => _to,
            None => ChainAccountOwner {
                chain_id: self.runtime.chain_id(),
                owner: Some(AccountOwner::User(
                    self.runtime.authenticated_signer().expect("Invalid owner"),
                )),
            },
        };

        let origin = self.runtime_owner();
        self.runtime
            .prepare_message(RouterMessage::Swap {
                origin,
                token_0,
                token_1,
                amount_0_in,
                amount_1_in,
                amount_0_out_min,
                amount_1_out_min,
                to,
            })
            .with_authentication()
            .send_to(self.runtime.application_creator_chain_id());
        Ok(RouterResponse::Ok)
    }

    fn execute_base_message(&mut self, message: BaseMessage) -> Result<(), RouterError> {
        match message {
            BaseMessage::SubscribeCreatorChain { origin: _ } => {
                self.on_msg_subscribe_creator_chain()
            }
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

    fn receive_erc20_from(
        &mut self,
        token: ApplicationId,
        amount: Amount,
        from: ChainAccountOwner,
    ) {
        if self.runtime.chain_id() != self.runtime.application_creator_chain_id() {
            return;
        }

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

    async fn on_msg_add_liquidity(
        &mut self,
        origin: ChainAccountOwner,
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
        let (amount_0, amount_1) = if created_pool {
            (amount_0_desired, amount_1_desired)
        } else {
            let Some(pool) = self.get_pool(token_0, token_1) else {
                return Err(RouterError::InvalidPool);
            };
            self.calculate_amount_pair(
                pool.clone(),
                amount_0_desired,
                amount_1_desired,
                amount_0_min,
                amount_1_min,
            )?
        };

        // Create pool will mint shares and transfer tokens directly
        if !created_pool {
            if amount_0 > Amount::ZERO {
                self.receive_erc20_from(token_0, amount_0, origin);
            }
            if amount_1 > Amount::ZERO {
                match token_1 {
                    Some(_token_1) => {
                        self.receive_erc20_from(_token_1, amount_1, origin);
                    }
                    None => {
                        self.receive_native_from(amount_1);
                    }
                }
            }
            self.mint(token_0, token_1, amount_0, amount_1, to);
        }

        self.publish_message(RouterMessage::AddLiquidity {
            origin,
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

    fn burn(&mut self, pool: Pool, liquidity: Amount, to: ChainAccountOwner) {
        let call = PoolOperation::Burn {
            pool_id: pool.id,
            liquidity,
            to,
        };
        let pool_application_id = self.pool_application_id();
        let PoolResponse::AmountPair(_) =
            self.runtime
                .call_application(true, pool_application_id, &call)
        else {
            todo!();
        };
    }

    async fn on_msg_remove_liquidity(
        &mut self,
        origin: ChainAccountOwner,
        token_0: ApplicationId,
        token_1: Option<ApplicationId>,
        liquidity: Amount,
        amount_0_min: Amount,
        amount_1_min: Amount,
        to: ChainAccountOwner,
        deadline: Timestamp,
    ) -> Result<(), RouterError> {
        let pool = self.get_pool(token_0, token_1).expect("Invalid pool");

        let balance_0 = self.balance_of_erc20(pool.token_0);
        let balance_1 = match pool.token_1 {
            Some(token_1) => self.balance_of_erc20(token_1),
            // TODO: here we should get balance of this application instance
            _ => todo!(),
        };
        let (amount_0, amount_1) = pool.calculate_amount_pair(liquidity, balance_0, balance_1);
        if amount_0 < amount_0_min || amount_1 < amount_1_min {
            return Err(RouterError::InvalidAmount);
        }

        self.burn(pool, liquidity, to);

        self.publish_message(RouterMessage::RemoveLiquidity {
            origin,
            token_0,
            token_1,
            liquidity,
            amount_0_min,
            amount_1_min,
            to,
            deadline,
        });
        Ok(())
    }

    fn swap(
        &mut self,
        pool: Pool,
        amount_0_out: Amount,
        amount_1_out: Amount,
        to: ChainAccountOwner,
    ) {
        let call = PoolOperation::Swap {
            pool_id: pool.id,
            amount_0_out,
            amount_1_out,
            to,
        };
        let pool_application_id = self.pool_application_id();
        let PoolResponse::AmountPair(_) =
            self.runtime
                .call_application(true, pool_application_id, &call)
        else {
            todo!();
        };
    }

    async fn on_msg_swap(
        &mut self,
        origin: ChainAccountOwner,
        token_0: ApplicationId,
        token_1: Option<ApplicationId>,
        amount_0_in: Option<Amount>,
        amount_1_in: Option<Amount>,
        amount_0_out_min: Option<Amount>,
        amount_1_out_min: Option<Amount>,
        to: ChainAccountOwner,
    ) -> Result<(), RouterError> {
        let pool = self.get_pool(token_0, token_1).expect("Invalid pool");

        let mut amount_0_out = Amount::ZERO;
        let mut amount_1_out = Amount::ZERO;

        if let Some(_amount_0_out_min) = amount_0_out_min {
            if let Some(_amount_1_in) = amount_1_in {
                amount_0_out = self.calculate_swap_amount_0(pool.clone(), _amount_1_in)?;
                if amount_0_out < _amount_0_out_min {
                    return Err(RouterError::InvalidAmount);
                }
                if amount_0_out == Amount::ZERO {
                    return Err(RouterError::InvalidAmount);
                }
            }
        }
        if let Some(_amount_1_out_min) = amount_1_out_min {
            if let Some(_amount_0_in) = amount_0_in {
                amount_1_out = self.calculate_swap_amount_1(pool.clone(), _amount_0_in)?;
                if amount_1_out < _amount_1_out_min {
                    return Err(RouterError::InvalidAmount);
                }
                if amount_1_out == Amount::ZERO {
                    return Err(RouterError::InvalidAmount);
                }
            }
        }

        if amount_0_out > Amount::ZERO {
            self.receive_erc20_from(token_0, amount_0_in.unwrap(), origin);
        }
        if amount_1_out > Amount::ZERO {
            match token_1 {
                Some(_token_1) => self.receive_erc20_from(_token_1, amount_0_in.unwrap(), origin),
                None => self.receive_native_from(amount_0_in.unwrap()),
            }
        }

        self.swap(pool, amount_0_out, amount_1_out, to);

        self.publish_message(RouterMessage::Swap {
            origin,
            token_0,
            token_1,
            amount_0_in,
            amount_1_in,
            amount_0_out_min,
            amount_1_out_min,
            to,
        });

        Ok(())
    }

    async fn on_msg_subscriber_sync(
        &mut self,
        _origin: ChainAccountOwner,
        _state: RouterSubscriberSyncState,
    ) -> Result<(), RouterError> {
        Ok(())
    }

    fn execute_pool_operation(
        &mut self,
        operation: PoolOperation,
    ) -> Result<RouterResponse, RouterError> {
        match operation {
            PoolOperation::CreatePool {
                token_0,
                token_1,
                amount_0_initial,
                amount_1_initial,
                amount_0_virtual,
                amount_1_virtual,
            } => {
                self.on_op_create_pool(
                    token_0,
                    token_1,
                    amount_0_initial,
                    amount_1_initial,
                    amount_0_virtual,
                    amount_1_virtual,
                )
                .await
            }
            PoolOperation::SetFeeTo { pool_id, account } => self.on_op_set_fee_to(pool_id, account),
            PoolOperation::SetFeeToSetter { pool_id, account } => {
                self.on_op_set_fee_to_setter(pool_id, account)
            }
            PoolOperation::Mint {
                pool_id,
                amount_0,
                amount_1,
                to,
            } => self.on_op_mint(pool_id, amount_0, amount_1, to).await,
            PoolOperation::Burn {
                pool_id,
                liquidity,
                to,
            } => self.on_op_burn(pool_id, liquidity, to).await,
            PoolOperation::SwapWithPool {
                pool_id,
                amount_0_out,
                amount_1_out,
                to,
            } => self.on_op_swap_with_pool(pool_id, amount_0_out, amount_1_out, to),
            PoolOperation::GetPoolWithTokenPair { token_0, token_1 } => {
                self.on_op_get_pool_with_token_pair(token_0, token_1).await
            }
        }
    }

    fn execute_pool_message(&mut self, message: PoolMessage) -> Result<(), RouterError> {
        match message {
            PoolMessage::CreatePool {
                origin,
                token_0,
                token_1,
                amount_0_initial,
                amount_1_initial,
                amount_0_virtual,
                amount_1_virtual,
            } => {
                self.on_msg_create_pool(
                    origin,
                    token_0,
                    token_1,
                    amount_0_initial,
                    amount_1_initial,
                    amount_0_virtual,
                    amount_1_virtual,
                )
                .await
            }
            PoolMessage::SetFeeTo {
                origin,
                pool_id,
                account,
            } => self.on_msg_set_fee_to(origin, pool_id, account).await,
            PoolMessage::SetFeeToSetter {
                origin,
                pool_id,
                account,
            } => {
                self.on_msg_set_fee_to_setter(origin, pool_id, account)
                    .await
            }
            PoolMessage::Mint {
                origin,
                pool_id,
                amount_0,
                amount_1,
                to,
            } => {
                self.on_msg_mint(origin, pool_id, amount_0, amount_1, to)
                    .await
            }
            PoolMessage::Burn {
                origin,
                pool_id,
                liquidity,
                to,
            } => self.on_msg_burn(origin, pool_id, liquidity, to).await,
            PoolMessage::Swap {
                origin,
                pool_id,
                amount_0_out,
                amount_1_out,
                to,
            } => {
                self.on_msg_swap_with_pool(origin, pool_id, amount_0_out, amount_1_out, to)
                    .await
            }
        }
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

        if amount_0_initial > Amount::ZERO {
            self.receive_erc20_from(token_0, amount_0_initial);
        }
        if amount_1_initial > Amount::ZERO {
            match token_1 {
                Some(_token_1) => self.receive_erc20_from(_token_1, amount_1_initial),
                None => self.receive_native_from(amount_1_initial),
            }
        }

        let pool = self
            .state
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
        if !pool.virtual_initial_liquidity {
            self.mint(pool.id, amount_0_initial, amount_1_initial, creator)
                .await?;
        }

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

    fn on_op_swap_with_pool(
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
        if origin.chain_id != self.runtime.chain_id() {
            let pool = self
                .state
                .create_pool(
                    token_0,
                    token_1,
                    amount_0_initial,
                    amount_1_initial,
                    amount_0_virtual,
                    amount_1_virtual,
                    origin,
                    self.runtime.system_time(),
                )
                .await?;
            self.mint(pool.id, amount_0_initial, amount_1_initial, origin)
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

    async fn mint_with_pool(
        &mut self,
        pool_id: u64,
        amount_0: Amount,
        amount_1: Amount,
        to: ChainAccountOwner,
    ) -> Result<(), PoolError> {
        let pool = self.state.get_pool(pool_id).await?.expect("Invalid pool");

        let balance_0 = pool.reserve_0.saturating_add(amount_0);
        let balance_1 = pool.reserve_0.saturating_add(amount_1);

        self.state.mint_fee(pool_id).await?;
        let liquidity = pool.calculate_liquidity(amount_0, amount_1);
        self.state.mint(pool_id, liquidity, to.clone()).await?;
        Ok(self
            .state
            .update(pool_id, balance_0, balance_1, self.runtime.system_time())
            .await?)
    }

    async fn on_msg_mint(
        &mut self,
        origin: ChainAccountOwner,
        pool_id: u64,
        amount_0: Amount,
        amount_1: Amount,
        to: ChainAccountOwner,
    ) -> Result<(), PoolError> {
        self.mint(pool_id, amount_0, amount_1, to).await?;

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

    async fn on_msg_swap_with_pool(
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
}
