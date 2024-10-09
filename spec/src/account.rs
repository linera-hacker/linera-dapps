use async_graphql::InputObject;
use linera_sdk::base::{AccountOwner, ChainId};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, InputObject)]
pub struct ChainAccountOwner {
    pub chain_id: ChainId,
    pub owner: Option<AccountOwner>,
}
