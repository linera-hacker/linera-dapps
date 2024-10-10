use async_graphql::scalar;
use linera_sdk::{base::Amount, graphql::GraphQLMutationRoot};
use serde::{Deserialize, Serialize};
use num_bigint::BigUint;
use num_traits::{cast::ToPrimitive, FromPrimitive};

#[derive(Debug, Deserialize, Serialize)]
pub enum BaseMessage {
    SubscribeCreatorChain,
}

#[derive(Debug, Deserialize, Serialize, GraphQLMutationRoot)]
pub enum BaseOperation {
    SubscribeCreatorChain,
}

scalar!(BaseOperation);

pub const CREATOR_CHAIN_CHANNEL: &[u8] = b"creator_chain_subscriptions";

pub fn sqrt(amount: Amount) -> Amount {
    Amount::from_attos(
        BigUint::sqrt(
            &BigUint::from_u128(u128::from(amount)).expect("Couldn't convert amount"),
        ).to_u128().expect("Couldn't convert BigUint")
    )
}
