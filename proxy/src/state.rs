use linera_sdk::{
    base::{BytecodeId, ChainId, Owner},
    views::{linera_views, MapView, RegisterView, RootView, ViewStorageContext},
};

#[derive(RootView, async_graphql::SimpleObject)]
#[view(context = "ViewStorageContext")]
pub struct Proxy {
    pub child_bytecode_id: RegisterView<Option<BytecodeId>>,
    pub validator_chain_ids: MapView<Owner, Vec<ChainId>>,
}
