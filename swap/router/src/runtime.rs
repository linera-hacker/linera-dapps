#![cfg_attr(target_arch = "wasm32", no_main)]

use crate::state::Application;
use linera_sdk::{
    base::{
        Account, AccountOwner, Amount, ApplicationId, ChannelName, Destination, Timestamp,
        WithContractAbi,
    },
    views::{RootView, View},
    Contract, ContractRuntime,
};
use spec::{
    account::ChainAccountOwner,
    base::{BaseMessage, BaseOperation, CREATOR_CHAIN_CHANNEL},
    erc20::{ERC20ApplicationAbi, ERC20Operation, ERC20Response},
    swap::{
        Pool, PoolMessage, PoolOperation, PoolResponse, RouterMessage, RouterOperation,
        RouterResponse, RouterSubscriberSyncState,
    },
};
use swap_router::{PoolError, RouterError};

pub fn message_owner(runtime: ContractRuntime<T>) -> ChainAccountOwner {
    let message_id = runtime.message_id().expect("Invalid message id");
    ChainAccountOwner {
        chain_id: message_id.chain_id,
        owner: Some(AccountOwner::User(
                runtime.authenticated_signer().expect("Invalid owner"),
                )),
    }
}

pub fn runtime_owner(runtime: ContractRuntime<T>) -> ChainAccountOwner {
    ChainAccountOwner {
        chain_id: runtime.chain_id(),
        owner: Some(AccountOwner::User(
                runtime.authenticated_signer().expect("Invalid owner"),
                )),
    }
}

pub fn balance_of_erc20(runtime: ContractRuntime<T>, token: ApplicationId, owner: ChainAccountOwner) -> Amount {
    let call = ERC20Operation::BalanceOf { owner };
    let ERC20Response::Balance(balance) =
        runtime
        .call_application(true, token.with_abi::<ERC20ApplicationAbi>(), &call)
        else {
            todo!();
        };
        balance
}

pub fn transfer_erc20(
    runtime: ContractRuntime<T>,
    token: ApplicationId,
    amount: Amount,
    owner: ChainAccountOwner,
    to: ChainAccountOwner,
    ) {
    let call = ERC20Operation::TransferFrom { from, amount, to };
    self.runtime
        .call_application(true, token.with_abi::<ERC20ApplicationAbi>(), &call);
}

fn transfer_native(runtime: ContractRuntime<T>, amount: Amount, owner: ChainAccountOwner, to: ChainAccountOwner) {
    let owner = .runtime.authenticated_signer();
    runtime.transfer(owner, to, amount);
}
