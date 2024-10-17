use spec::swap::abi::SwapApplicationAbi;
use thiserror::Error;

pub type ApplicationAbi = SwapApplicationAbi;

#[derive(Debug, Error)]
#[allow(dead_code)]
pub enum SwapError {}
