use async_graphql::scalar;
use linera_sdk::base::{AccountOwner, ChainId};
use serde::{Deserialize, Serialize};

#[derive(Copy, Debug, Clone, Deserialize, Serialize, Eq, Hash, PartialEq)]
pub struct ChainAccountOwner {
    pub chain_id: ChainId,
    pub owner: Option<AccountOwner>,
}

scalar!(ChainAccountOwner);
