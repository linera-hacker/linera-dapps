use async_graphql::scalar;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq)]
pub struct TokenMetadata {
    pub logo: String,
    pub description: String,
    pub twitter: String,
    pub telegram: String,
    pub discord: String,
    pub website: String,
    pub github: String,
    pub mintable: bool,
}

scalar!(TokenMetadata);

impl std::fmt::Display for TokenMetadata {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "TokenMetadata:{}:{:?}:{:?}:{:?}:{:?}:{:?}:{:?}:{:?}", self.logo, self.description, self.twitter, self.telegram, self.discord, self.website, self.github, self.mintable)
    }
}
