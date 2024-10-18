use crate::account::ChainAccountOwner;
use async_graphql::{scalar, Context, Error};
use linera_sdk::{base::Amount, graphql::GraphQLMutationRoot};
use num_bigint::BigUint;
use num_traits::{cast::ToPrimitive, FromPrimitive};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::{
    ops::{Add, Div, Mul},
    str::FromStr,
};

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

#[derive(Debug, Clone, Eq, PartialEq, Default)]
pub struct BigAmount(BigUint);

impl BigAmount {
    pub fn add(self, amount: BigAmount) -> BigAmount {
        BigAmount(self.0.add(amount.0))
    }
}

impl Serialize for BigAmount {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.0.to_string().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for BigAmount {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(BigAmount(
            BigUint::from_str(&String::deserialize(deserializer)?)
                .expect("Couldn't convert BigUint"),
        ))
    }
}

scalar!(BigAmount);

pub fn mul(amount_1: Amount, amount_2: Amount) -> BigUint {
    let _amount_1 = BigUint::from_u128(u128::from(amount_1)).expect("Couldn't convert amount");
    let _amount_2 = BigUint::from_u128(u128::from(amount_2)).expect("Couldn't convert amount");
    _amount_1.mul(_amount_2)
}

pub fn mul_percent_10000(amount_1: Amount, percent: u16) -> Amount {
    let _amount_1 = BigUint::from_u128(u128::from(amount_1)).expect("Couldn't convert amount");
    let _percent = BigUint::from_u16(percent).expect("Couldn't convert amount");
    let _hundred = BigUint::from_u16(10000).expect("Couldn't convert amount");
    Amount::from_attos(
        _amount_1
            .mul(_percent)
            .div(_hundred)
            .to_u128()
            .expect("Couldn't convert BigUint"),
    )
}

pub fn mul_then_div(amount_1: Amount, amount_2: Amount, amount_3: Amount) -> Amount {
    let _amount_1 = BigUint::from_u128(u128::from(amount_1)).expect("Couldn't convert amount");
    let _amount_2 = BigUint::from_u128(u128::from(amount_2)).expect("Couldn't convert amount");
    let _amount_3 = BigUint::from_u128(u128::from(amount_3)).expect("Couldn't convert amount");
    Amount::from_attos(
        _amount_1
            .mul(_amount_2)
            .div(_amount_3)
            .to_u128()
            .expect("Couldn't convert BigUint"),
    )
}

pub fn mul_then_sqrt(amount_1: Amount, amount_2: Amount) -> Amount {
    let _amount_1 = BigUint::from_u128(u128::from(amount_1)).expect("Couldn't convert amount");
    let _amount_2 = BigUint::from_u128(u128::from(amount_2)).expect("Couldn't convert amount");
    Amount::from_attos(
        BigUint::sqrt(&_amount_1.mul(_amount_2))
            .to_u128()
            .expect("Couldn't convert BigUint"),
    )
}

pub fn mul_then_add(amount_1: Amount, amount_2: Amount, amount_3: Amount) -> Amount {
    let _amount_1 = BigUint::from_u128(u128::from(amount_1)).expect("Couldn't convert amount");
    let _amount_2 = BigUint::from_u128(u128::from(amount_2)).expect("Couldn't convert amount");
    let _amount_3 = BigUint::from_u128(u128::from(amount_3)).expect("Couldn't convert amount");
    Amount::from_attos(
        _amount_1
            .mul(_amount_2)
            .add(_amount_3)
            .to_u128()
            .expect("Couldn't convert BigUint"),
    )
}

pub fn div_then_mul_to_big_amount(
    divisor: Amount,
    dividend: Amount,
    extra: Amount,
) -> BigAmount {
    let _divisor = BigUint::from_u128(u128::from(divisor)).expect("Couldn't convert amount");
    let _dividend = BigUint::from_u128(u128::from(dividend)).expect("Couldn't convert amount");
    let _extra = BigUint::from_u128(u128::from(extra)).expect("Couldn't convert amount");
    BigAmount(
        _divisor
            .mul(_extra)
            .div(_dividend),
    )
}
