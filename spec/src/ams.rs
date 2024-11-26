use crate::{
    account::ChainAccountOwner,
    base::{BaseMessage, BaseOperation},
};
use async_graphql::scalar;
use async_graphql::{Context, Error, Request, Response, SimpleObject};
use linera_sdk::{
    abi::{ContractAbi, ServiceAbi},
    base::{ApplicationId, Signature, Timestamp},
    graphql::GraphQLMutationRoot,
    views::{MapView, QueueView, RootView},
    ViewStorageContext,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct InstantiationArgument {
    pub application_types: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq)]
pub struct Metadata {
    pub creator: ChainAccountOwner,
    pub application_id: ApplicationId,
    pub application_type: String,
    pub key_words: Vec<String>,
    pub logo: String,
    pub description: String,
    pub twitter: String,
    pub telegram: String,
    pub discord: String,
    pub website: String,
    pub github: String,
    /// JSON spec of registered application
    pub spec: String,
    pub created_at: Timestamp,
}

scalar!(Metadata);

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct SubscriberSyncState {
    pub application_types: Vec<String>,
    pub applications: HashMap<ApplicationId, Metadata>,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum AMSMessage {
    BaseMessage(BaseMessage),
    Register {
        origin: ChainAccountOwner,
        metadata: Metadata,
    },
    SubscriberSync {
        origin: ChainAccountOwner,
        state: SubscriberSyncState,
    },
}

#[derive(Debug, Deserialize, Serialize, GraphQLMutationRoot)]
pub enum AMSOperation {
    BaseOperation(BaseOperation),
    Register { metadata: Metadata },
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub enum AMSResponse {
    #[default]
    Ok,
}

pub struct AMSApplicationAbi;

impl ContractAbi for AMSApplicationAbi {
    type Operation = AMSOperation;
    type Response = AMSResponse;
}

impl ServiceAbi for AMSApplicationAbi {
    type Query = Request;
    type QueryResponse = Response;
}

pub trait AMSQueryRoot {
    fn applications(
        &self,
        ctx: &Context<'_>,
        created_before: Option<Timestamp>,
        created_after: Option<Timestamp>,
        application_type: Option<String>,
    ) -> impl std::future::Future<Output = Result<Vec<Metadata>, Error>> + Send;

    fn application(
        &self,
        ctx: &Context<'_>,
        application_id: ApplicationId,
    ) -> impl std::future::Future<Output = Result<Metadata, Error>> + Send;
}

pub trait AMSMutationRoot {
    fn claim(
        &self,
        ctx: &Context<'_>,
        application_id: ApplicationId,
        signature: Signature,
    ) -> impl std::future::Future<Output = Result<Vec<u8>, Error>> + Send;

    fn add_application_type(
        &self,
        ctx: &Context<'_>,
        application_type: Option<String>,
    ) -> impl std::future::Future<Output = Result<Vec<u8>, Error>> + Send;

    fn update(
        &self,
        ctx: &Context<'_>,
        application_id: ApplicationId,
        metadata: Metadata,
    ) -> impl std::future::Future<Output = Result<Vec<u8>, Error>> + Send;

    fn subscribe_creator_chain(
        &self,
        ctx: &Context<'_>,
    ) -> impl std::future::Future<Output = Result<Vec<u8>, Error>> + Send;
}

#[derive(RootView, SimpleObject)]
#[view(context = "ViewStorageContext")]
pub struct AMS {
    pub application_types: QueueView<String>,
    pub applications: MapView<ApplicationId, Metadata>,
}
