use crate::account::ChainAccountOwner;
use crate::base::{BaseMessage, BaseOperation};
use async_graphql::{scalar, Context, Error, Request, Response};
use linera_sdk::{
    abi::{ContractAbi, ServiceAbi},
    base::Amount,
    graphql::GraphQLMutationRoot,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ERC20Parameters {
    pub initial_balances: HashMap<String, Amount>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct InstantiationArgument {
    pub initial_supply: Amount,
    pub name: String,
    pub symbol: String,
    pub decimals: u8,
    pub initial_currency: Option<Amount>,
    pub fixed_currency: Option<bool>,
    pub fee_percent: Option<Amount>,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum ERC20Message {
    BaseMessage(BaseMessage),
    Transfer {
        origin: ChainAccountOwner,
        to: ChainAccountOwner,
        amount: Amount,
    },
    TransferFrom {
        origin: ChainAccountOwner,
        from: ChainAccountOwner,
        amount: Amount,
        to: ChainAccountOwner,
        allowance_owner: ChainAccountOwner,
    },
    Approve {
        origin: ChainAccountOwner,
        spender: ChainAccountOwner,
        value: Amount,
    },
    Mint {
        origin: ChainAccountOwner,
        to: ChainAccountOwner,
        amount: Amount,
    },
    ChangeCreatedOwner {
        origin: ChainAccountOwner,
        new_owner: ChainAccountOwner,
    },
}

#[derive(Debug, Deserialize, Serialize, GraphQLMutationRoot)]
pub enum ERC20Operation {
    BaseOperation(BaseOperation),
    Transfer {
        to: ChainAccountOwner,
        amount: Amount,
    },
    TransferFrom {
        from: ChainAccountOwner,
        amount: Amount,
        to: ChainAccountOwner,
    },
    Approve {
        spender: ChainAccountOwner,
        value: Amount,
    },
    BalanceOf {
        owner: ChainAccountOwner,
    },
    Mint {
        amount: Amount,
    },
    ChangeCreatedOwner {
        new_owner: ChainAccountOwner,
    },
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub enum ERC20Response {
    #[default]
    Ok,
    Balance(Amount),
}

pub struct ERC20ApplicationAbi;

impl ContractAbi for ERC20ApplicationAbi {
    type Operation = ERC20Operation;
    type Response = ERC20Response;
}

impl ServiceAbi for ERC20ApplicationAbi {
    type Query = Request;
    type QueryResponse = Response;
}

pub trait ERC20QueryRoot {
    fn total_supply(
        &self,
        ctx: &Context<'_>,
    ) -> impl std::future::Future<Output = Result<Amount, Error>> + Send;

    fn name(
        &self,
        ctx: &Context<'_>,
    ) -> impl std::future::Future<Output = Result<String, Error>> + Send;

    fn symbol(
        &self,
        ctx: &Context<'_>,
    ) -> impl std::future::Future<Output = Result<String, Error>> + Send;

    fn decimals(
        &self,
        ctx: &Context<'_>,
    ) -> impl std::future::Future<Output = Result<u8, Error>> + Send;

    fn balance_of(
        &self,
        ctx: &Context<'_>,
        owner: ChainAccountOwner,
    ) -> impl std::future::Future<Output = Result<Amount, Error>> + Send;

    fn allowance(
        &self,
        ctx: &Context<'_>,
        owner: ChainAccountOwner,
        spender: ChainAccountOwner,
    ) -> impl std::future::Future<Output = Result<Amount, Error>> + Send;
}

pub trait ERC20MutationRoot {
    fn transfer(
        &self,
        ctx: &Context<'_>,
        to: ChainAccountOwner,
        amount: Amount,
    ) -> impl std::future::Future<Output = Result<Vec<u8>, Error>> + Send;

    fn transfer_from(
        &self,
        ctx: &Context<'_>,
        from: ChainAccountOwner,
        to: ChainAccountOwner,
        amount: Amount,
    ) -> impl std::future::Future<Output = Result<Vec<u8>, Error>> + Send;

    fn approve(
        &self,
        ctx: &Context<'_>,
        spender: ChainAccountOwner,
        value: Amount,
    ) -> impl std::future::Future<Output = Result<Vec<u8>, Error>> + Send;

    fn subscribe_creator_chain(
        &self,
        ctx: &Context<'_>,
    ) -> impl std::future::Future<Output = Result<Vec<u8>, Error>> + Send;
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct ERC20 {
    pub total_supply: Amount,
    pub balances: HashMap<ChainAccountOwner, Amount>,
}

scalar!(ERC20);

impl ERC20 {
    pub fn _mint(&mut self, to: ChainAccountOwner, amount: Amount) {
        self.total_supply.saturating_add_assign(amount);
        self.balances.insert(
            to.clone(),
            self.balances
                .get(&to)
                .unwrap_or(&Amount::ZERO)
                .saturating_add(amount),
        );
    }

    // Liquidity to be burn should be returned to application already
    pub fn _burn(&mut self, from: ChainAccountOwner, liquidity: Amount) {
        self.total_supply = self.total_supply.saturating_sub(liquidity);
        self.balances.insert(
            from.clone(),
            self.balances
                .get(&from)
                .unwrap_or(&Amount::ZERO)
                .saturating_sub(liquidity),
        );
    }
}
