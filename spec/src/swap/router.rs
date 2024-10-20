use crate::{account::ChainAccountOwner, swap::pool::PoolResponse};
use async_graphql::scalar;
use linera_sdk::{
    base::{Amount, ApplicationId, Timestamp},
    graphql::GraphQLMutationRoot,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub enum RouterMessage {
    AddLiquidity {
        origin: ChainAccountOwner,
        token_0: ApplicationId,
        token_1: Option<ApplicationId>,
        amount_0_desired: Amount,
        amount_1_desired: Amount,
        amount_0_min: Amount,
        amount_1_min: Amount,
        to: ChainAccountOwner,
        deadline: Timestamp,
        block_timestamp: Timestamp,
    },
    RemoveLiquidity {
        origin: ChainAccountOwner,
        token_0: ApplicationId,
        token_1: Option<ApplicationId>,
        liquidity: Amount,
        amount_0_min: Amount,
        amount_1_min: Amount,
        to: ChainAccountOwner,
        deadline: Timestamp,
        block_timestamp: Timestamp,
    },
    Swap {
        origin: ChainAccountOwner,
        token_0: ApplicationId,
        token_1: Option<ApplicationId>,
        amount_0_in: Option<Amount>,
        amount_1_in: Option<Amount>,
        amount_0_out_min: Option<Amount>,
        amount_1_out_min: Option<Amount>,
        to: ChainAccountOwner,
        block_timestamp: Timestamp,
    },
    // Process one token in each message to let requested tokens to be subscribed
    SubscribeNewERC20Token {
        origin: ChainAccountOwner,
        token: ApplicationId,
    },
}

#[derive(Debug, Deserialize, Serialize, GraphQLMutationRoot)]
pub enum RouterOperation {
    CalculateSwapAmount {
        token_0: ApplicationId,
        token_1: Option<ApplicationId>,
        amount_1: Amount,
    },
    AddLiquidity {
        token_0: ApplicationId,
        token_1: Option<ApplicationId>,
        amount_0_desired: Amount,
        amount_1_desired: Amount,
        amount_0_min: Amount,
        amount_1_min: Amount,
        to: Option<ChainAccountOwner>,
        deadline: Timestamp,
    },
    RemoveLiquidity {
        token_0: ApplicationId,
        token_1: Option<ApplicationId>,
        liquidity: Amount,
        amount_0_min: Amount,
        amount_1_min: Amount,
        to: Option<ChainAccountOwner>,
        deadline: Timestamp,
    },
    Swap {
        token_0: ApplicationId,
        token_1: Option<ApplicationId>,
        amount_0_in: Option<Amount>,
        amount_1_in: Option<Amount>,
        amount_0_out_min: Option<Amount>,
        amount_1_out_min: Option<Amount>,
        to: Option<ChainAccountOwner>,
    },
}

scalar!(RouterOperation);

#[derive(Debug, Deserialize, Serialize, Default)]
pub enum RouterResponse {
    #[default]
    Ok,
    Liquidity((Amount, Amount, Amount)),
    Amount(Amount),
    AmountPair((Amount, Amount)),
    PoolResponse(PoolResponse),
}
