use linera_sdk::{
    base::{BytecodeId, ChainId, Owner},
    views::{linera_views, MapView, RegisterView, RootView, ViewStorageContext},
};

use proxy::InstantiateArgument;

// State is only on creation chain

#[derive(RootView, async_graphql::SimpleObject)]
#[view(context = "ViewStorageContext")]
pub struct ProxyState {
    pub operator: RegisterView<Option<Owner>>,
    pub bytecode_id: RegisterView<Option<BytecodeId>>,
    pub validator_chains: MapView<Owner, Vec<ChainId>>,
}

#[allow(dead_code)]
impl ProxyState {
    pub(crate) async fn initialize(&mut self, argument: InstantiateArgument) {

    }

    pub(crate) async fn set_operator(&mut self, operator: Owner) {

    }

    pub(crate) async fn set_child_bytecode_id(&mut self, bytecode_id: BytecodeId) {

    }

    pub(crate) async fn register_validator(&mut self, owner: Owner) {

    }

    pub(crate) async fn deregister_validator(&mut self, owner: Owner) {

    }
}
