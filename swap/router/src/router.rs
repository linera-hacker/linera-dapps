use linera_sdk::{Contract, ContractRuntime};
use spec::swap::abi::SwapApplicationState;
pub struct Router<T: Contract> {
    state: SwapApplicationState,
    runtime: ContractRuntime<T>,
}
