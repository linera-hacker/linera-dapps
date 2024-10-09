use async_graphql::scalar;
use linera_sdk::graphql::GraphQLMutationRoot;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub enum BaseMessage {
    SubscribeCreatorChain,
}

#[derive(Debug, Deserialize, Serialize, GraphQLMutationRoot)]
pub enum BaseOperation {
    SubscribeCreatorChain,
}

scalar!(BaseOperation);
