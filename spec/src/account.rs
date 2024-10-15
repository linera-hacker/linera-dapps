use async_graphql::scalar;
use linera_sdk::base::{AccountOwner, ChainId};
use serde::{Deserialize, Serialize};
use std::hash::{Hash, Hasher};

#[derive(Copy, Debug, Clone, Deserialize, Serialize, Eq, PartialEq)]
pub struct ChainAccountOwner {
    pub chain_id: ChainId,
    pub owner: Option<AccountOwner>,
}

scalar!(ChainAccountOwner);

impl Hash for ChainAccountOwner {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.chain_id.hash(state);
        if let Some(owner) = self.owner {
            owner.to_string().hash(state);
        }
    }
}

impl std::fmt::Display for ChainAccountOwner {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "ChainAccountOwner:{}:{:?}", self.chain_id, self.owner)
    }
}
