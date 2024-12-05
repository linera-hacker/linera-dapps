use async_graphql::{ Enum, Request, Response};
use linera_sdk::{
  abi::{ContractAbi, ServiceAbi},
  base::CryptoHash,
  graphql::GraphQLMutationRoot,
};
use serde::{Deserialize, Serialize};


#[derive(Debug, Deserialize, Serialize, Clone, Eq, PartialEq, Enum, Copy)]
pub enum BlobDataType {
    Image,
    Text,
}

#[derive(Debug, Deserialize, Serialize, GraphQLMutationRoot)]
pub enum BlobOperation {
    Register {
        data_type: BlobDataType,
        blob_hash: CryptoHash,
    },
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub enum BlobGatewayResponse {
    #[default]
    Ok,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum BlobMessage {
    Register {
        data_type: BlobDataType,
        blob_hash: CryptoHash,
    },
}

pub struct BlobGatewayApplicationAbi;

impl ContractAbi for BlobGatewayApplicationAbi {
  type Operation = BlobOperation;
  type Response = BlobGatewayResponse;
}

impl ServiceAbi for BlobGatewayApplicationAbi {
  type Query = Request;
  type QueryResponse = Response;
}

pub trait BlobQueryRoot {

}
