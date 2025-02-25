use crate::{
    account::ChainAccountOwner,
    base::{BaseMessage, BaseOperation},
    blob_gateway::StoreType,
};
use async_graphql::{scalar, Context, Error, Request, Response, SimpleObject};
use linera_sdk::{
    abi::{ContractAbi, ServiceAbi},
    base::{Amount, ApplicationId},
    graphql::GraphQLMutationRoot,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ERC20Parameters {
    pub initial_balances: HashMap<String, Amount>,
    pub swap_application_id: Option<ApplicationId>,
    pub token_metadata: Option<TokenMetadata>,
}
scalar!(ERC20Parameters);

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct InstantiationArgument {
    pub initial_supply: Amount,
    pub name: String,
    pub symbol: String,
    pub decimals: u8,
    pub initial_currency: Option<Amount>,
    pub fixed_currency: Option<bool>,
    pub fee_percent: Option<Amount>,
    pub ams_application_id: Option<ApplicationId>,
    pub blob_gateway_application_id: Option<ApplicationId>,
}

scalar!(InstantiationArgument);

#[derive(Default, Debug, Clone, Deserialize, Serialize, Eq, PartialEq)]
pub struct TokenMetadata {
    pub logo_store_type: StoreType,
    pub logo: String,
    pub description: String,
    pub twitter: Option<String>,
    pub telegram: Option<String>,
    pub discord: Option<String>,
    pub website: Option<String>,
    pub github: Option<String>,
    pub mintable: bool,
}

scalar!(TokenMetadata);

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize, SimpleObject)]
pub struct ChainAccountOwnerBalance {
    pub owner: ChainAccountOwner,
    pub balance: Amount,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct SubscriberSyncState {
    pub total_supply: Amount,
    pub balances: HashMap<ChainAccountOwner, Amount>,
    pub allowances: HashMap<ChainAccountOwner, HashMap<ChainAccountOwner, Amount>>,
    pub locked_allowances: HashMap<ChainAccountOwner, Amount>,
    pub name: String,
    pub symbol: String,
    pub decimals: u8,
    pub initial_currency: Amount,
    pub fixed_currency: bool,
    pub fee_percent: Amount,
    pub owner: Option<ChainAccountOwner>,
    pub owner_balance: Amount,
    pub token_metadata: Option<TokenMetadata>,
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
        cur_amount: Amount,
    },
    TransferOwnership {
        origin: ChainAccountOwner,
        new_owner: ChainAccountOwner,
    },
    SubscriberSync {
        origin: ChainAccountOwner,
        state: SubscriberSyncState,
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
        to: Option<ChainAccountOwner>,
        amount: Amount,
    },
    TransferOwnership {
        new_owner: ChainAccountOwner,
    },
    OwnerOf,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub enum ERC20Response {
    #[default]
    Ok,
    Balance(Amount),
    Owner(ChainAccountOwner),
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

    fn token_metadata(
        &self,
        ctx: &Context<'_>,
    ) -> impl std::future::Future<Output = Result<Option<TokenMetadata>, Error>> + Send;

    fn balance_top_list(
        &self,
        ctx: &Context<'_>,
        limit: usize,
    ) -> impl std::future::Future<Output = Result<Vec<ChainAccountOwnerBalance>, Error>> + Send;

    fn subscribed_creator_chain(
        &self,
        ctx: &Context<'_>,
    ) -> impl std::future::Future<Output = Result<bool, Error>> + Send;
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

    fn mint(
        &self,
        ctx: &Context<'_>,
        to: Option<ChainAccountOwner>,
        amount: Amount,
    ) -> impl std::future::Future<Output = Result<Vec<u8>, Error>> + Send;
}

#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq, Default, SimpleObject)]
pub struct ERC20 {
    pub total_supply: Amount,
    pub balances: HashMap<ChainAccountOwner, Amount>,
}

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
