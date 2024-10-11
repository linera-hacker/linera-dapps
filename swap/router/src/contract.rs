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
        RouterParameters, RouterResponse,
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
            RouterMessage::Swap {
                token_0,
                token_1,
                amount_0_in,
                amount_1_in,
                amount_0_out_min,
                amount_1_out_min,
                to,
            } => self
                .on_msg_swap(
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
            return Ok((pool, false, false));
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

    fn calculate_swap_amount_1(&self, pool: Pool, amount_0: Amount) -> Result<Amount, RouterError> {
        if pool.reserve_0 <= Amount::ZERO || pool.reserve_1 <= Amount::ZERO {
            return Err(RouterError::InvalidAmount);
        }
        Ok(Amount::from_attos(
            amount_0
                .saturating_mul(pool.reserve_1.into())
                .saturating_div(pool.reserve_0.into()),
        ))
    }

    fn calculate_swap_amount_0(&self, pool: Pool, amount_1: Amount) -> Result<Amount, RouterError> {
        if pool.reserve_0 <= Amount::ZERO || pool.reserve_1 <= Amount::ZERO {
            return Err(RouterError::InvalidAmount);
        }
        Ok(Amount::from_attos(
            amount_1
                .saturating_mul(pool.reserve_0.into())
                .saturating_div(pool.reserve_1.into()),
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
        let amount_0_optimal = self.calculate_swap_amount_1(pool, amount_1_desired)?;
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
        let (pool, created, exchanged) = self
            .get_or_create_pool(token_0, token_1, amount_0_desired, amount_1_desired)
            .expect("Invalid pool");

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
        to: ChainAccountOwner,
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
        to: ChainAccountOwner,
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

        self.runtime
            .prepare_message(RouterMessage::Swap {
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

    fn transfer_native_from(&mut self, amount: Amount) {
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
            self.calculate_amount_pair(
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
                None => self.transfer_native_from(amount_1),
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
            self.transfer_erc20_from(token_0, amount_0_in.unwrap());
        }
        if amount_1_out > Amount::ZERO {
            match token_1 {
                Some(_token_1) => self.transfer_erc20_from(_token_1, amount_0_in.unwrap()),
                None => self.transfer_native_from(amount_0_in.unwrap()),
            }
        }

        self.swap(pool, amount_0_out, amount_1_out, to);

        self.publish_message(RouterMessage::Swap {
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
}
