use linera_sdk::{Contract, ContractRuntime};
use spec::swap::abi::SwapApplicationState;
use thiserror::Error;

#[derive(Debug, Error)]
#[allow(dead_code)]
pub enum RouterError {}

pub struct Router<T: Contract> {
    state: SwapApplicationState,
    runtime: ContractRuntime<T>,
}
