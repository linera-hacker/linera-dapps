use linera_sdk::{
    base::{Account, AccountOwner, Amount, ApplicationId},
    Contract, ContractRuntime,
};
use spec::{
    account::ChainAccountOwner,
    erc20::{ERC20ApplicationAbi, ERC20Operation, ERC20Response},
};

pub fn message_owner<T>(runtime: &mut ContractRuntime<T>) -> ChainAccountOwner
where
    T: Contract,
{
    let message_id = runtime.message_id().expect("Invalid message id");
    ChainAccountOwner {
        chain_id: message_id.chain_id,
        owner: Some(AccountOwner::User(
            runtime.authenticated_signer().expect("Invalid owner"),
        )),
    }
}

pub fn runtime_owner<T>(runtime: &mut ContractRuntime<T>) -> ChainAccountOwner
where
    T: Contract,
{
    ChainAccountOwner {
        chain_id: runtime.chain_id(),
        owner: Some(AccountOwner::User(
            runtime.authenticated_signer().expect("Invalid owner"),
        )),
    }
}

pub fn balance_of_erc20<T>(
    runtime: &mut ContractRuntime<T>,
    token: ApplicationId,
    owner: ChainAccountOwner,
) -> Amount
where
    T: Contract,
{
    let call = ERC20Operation::BalanceOf { owner };
    let ERC20Response::Balance(balance) =
        runtime.call_application(true, token.with_abi::<ERC20ApplicationAbi>(), &call)
    else {
        todo!();
    };
    balance
}

pub fn transfer_erc20<T>(
    runtime: &mut ContractRuntime<T>,
    token: ApplicationId,
    amount: Amount,
    owner: ChainAccountOwner,
    to: ChainAccountOwner,
) where
    T: Contract,
{
    let call = ERC20Operation::TransferFrom {
        from: owner,
        amount,
        to,
    };
    runtime.call_application(true, token.with_abi::<ERC20ApplicationAbi>(), &call);
}

pub fn receive_erc20_from_runtime_owner<T>(
    runtime: &mut ContractRuntime<T>,
    token: ApplicationId,
    amount: Amount,
    to: ChainAccountOwner,
) where
    T: Contract,
{
    let owner = runtime_owner(runtime);
    transfer_erc20(runtime, token, amount, owner, to)
}

pub fn receive_erc20_from_runtime_owner_to_application_creation<T>(
    runtime: &mut ContractRuntime<T>,
    token: ApplicationId,
    amount: Amount,
) where
    T: Contract,
{
    let to = ChainAccountOwner {
        chain_id: runtime.application_creator_chain_id(),
        owner: Some(AccountOwner::Application(
            runtime.application_id().forget_abi(),
        )),
    };
    receive_erc20_from_runtime_owner(runtime, token, amount, to)
}

pub fn transfer_native<T>(
    runtime: &mut ContractRuntime<T>,
    amount: Amount,
    from: ChainAccountOwner,
    to: ChainAccountOwner,
) where
    T: Contract,
{
    let from = match from.owner {
        Some(AccountOwner::User(owner)) => Some(owner),
        _ => None,
    };
    let to = Account {
        chain_id: to.chain_id,
        owner: match to.owner {
            Some(AccountOwner::User(owner)) => Some(owner),
            _ => None,
        },
    };
    runtime.transfer(from, to, amount);
}

pub fn receive_native_from_runtime_owner<T>(
    runtime: &mut ContractRuntime<T>,
    amount: Amount,
    to: ChainAccountOwner,
) where
    T: Contract,
{
    let owner = runtime_owner(runtime);
    transfer_native(runtime, amount, owner, to)
}

pub fn transfer_token<T>(
    runtime: &mut ContractRuntime<T>,
    token: Option<ApplicationId>,
    amount: Amount,
    from: ChainAccountOwner,
    to: ChainAccountOwner,
) where
    T: Contract,
{
    match token {
        Some(_token) => transfer_erc20(runtime, _token, amount, from, to),
        _ => transfer_native(runtime, amount, from, to),
    }
}

pub fn receive_token_from_runtime_owner<T>(
    runtime: &mut ContractRuntime<T>,
    token: Option<ApplicationId>,
    amount: Amount,
    to: ChainAccountOwner,
) where
    T: Contract,
{
    let owner = runtime_owner(runtime);
    transfer_token(runtime, token, amount, owner, to)
}

pub fn receive_token_from_runtime_owner_to_application_creation<T>(
    runtime: &mut ContractRuntime<T>,
    token: Option<ApplicationId>,
    amount: Amount,
) where
    T: Contract,
{
    let to = ChainAccountOwner {
        chain_id: runtime.application_creator_chain_id(),
        owner: Some(AccountOwner::Application(
            runtime.application_id().forget_abi(),
        )),
    };
    receive_token_from_runtime_owner(runtime, token, amount, to)
}
