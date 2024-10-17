use linera_sdk::{Contract, ContractRuntime};
use spec::swap::state::SwapApplicationState;
use thiserror::Error;

#[derive(Debug, Error)]
#[allow(dead_code)]
pub enum RouterError {}

pub struct Router<T: Contract> {
    state: SwapApplicationState,
    runtime: ContractRuntime<T>,
}
