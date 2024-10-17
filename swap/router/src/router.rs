use linera_sdk::{Contract, ContractRuntime};
use spec::swap::state::SwapApplicationState;
use std::sync::Arc;
use thiserror::Error;

#[derive(Debug, Error)]
#[allow(dead_code)]
pub enum RouterError {}

pub struct Router {}

impl Router {
    pub async fn new() -> Self {
        Router {}
    }
}
