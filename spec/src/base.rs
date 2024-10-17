use crate::account::ChainAccountOwner;
use async_graphql::{scalar, Context, Error};
use linera_sdk::{base::Amount, graphql::GraphQLMutationRoot};
use num_bigint::BigUint;
use num_traits::{cast::ToPrimitive, FromPrimitive};
use serde::{Deserialize, Serialize};
use std::ops::{Div, Mul};

#[derive(Debug, Deserialize, Serialize)]
pub enum BaseMessage {
    SubscribeCreatorChain { origin: ChainAccountOwner },
}

#[derive(Debug, Deserialize, Serialize, GraphQLMutationRoot)]
pub enum BaseOperation {
    SubscribeCreatorChain,
}

scalar!(BaseOperation);

pub trait BaseMutationRoot {
    fn subscribe_creator_chain(
        &self,
        ctx: &Context<'_>,
    ) -> impl std::future::Future<Output = Result<Vec<u8>, Error>> + Send;
}

pub const CREATOR_CHAIN_CHANNEL: &[u8] = b"creator_chain_subscriptions";

pub fn sqrt(amount: Amount) -> Amount {
    Amount::from_attos(
        BigUint::sqrt(&BigUint::from_u128(u128::from(amount)).expect("Couldn't convert amount"))
            .to_u128()
            .expect("Couldn't convert BigUint"),
    )
}

pub fn mul(amount_1: Amount, amount_2: Amount) -> Amount {
    let _amount_1 = &BigUint::from_u128(u128::from(amount_1)).expect("Couldn't convert amount");
    let _amount_2 = &BigUint::from_u128(u128::from(amount_2)).expect("Couldn't convert amount");
    Amount::from_attos(
        _amount_1
            .mul(_amount_2)
            .to_u128()
            .expect("Couldn't convert BigUint"),
    )
}

pub fn mul_then_div(amount_1: Amount, amount_2: Amount, amount_3: Amount) -> Amount {
    let _amount_1 = &BigUint::from_u128(u128::from(amount_1)).expect("Couldn't convert amount");
    let _amount_2 = &BigUint::from_u128(u128::from(amount_2)).expect("Couldn't convert amount");
    let _amount_3 = &BigUint::from_u128(u128::from(amount_3)).expect("Couldn't convert amount");
    Amount::from_attos(
        _amount_1
            .mul(_amount_2)
            .div(_amount_3)
            .to_u128()
            .expect("Couldn't convert BigUint"),
    )
}

pub fn mul_then_sqrt(amount_1: Amount, amount_2: Amount) -> Amount {
    let _amount_1 = &BigUint::from_u128(u128::from(amount_1)).expect("Couldn't convert amount");
    let _amount_2 = &BigUint::from_u128(u128::from(amount_2)).expect("Couldn't convert amount");
    Amount::from_attos(
        BigUint::sqrt(&_amount_1.mul(_amount_2))
            .to_u128()
            .expect("Couldn't convert BigUint"),
    )
}
