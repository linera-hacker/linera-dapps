use linera_sdk::{
    base::{Account, AccountOwner, Amount, ApplicationId},
    Contract, ContractRuntime,
};
use spec::{
    account::ChainAccountOwner,
    erc20::{ERC20ApplicationAbi, ERC20Operation, ERC20Response},
};

pub fn message_owner<T>(mut runtime: ContractRuntime<T>) -> ChainAccountOwner
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

pub fn runtime_owner<T>(mut runtime: ContractRuntime<T>) -> ChainAccountOwner
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
    mut runtime: ContractRuntime<T>,
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
    mut runtime: ContractRuntime<T>,
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

pub fn transfer_native<T>(mut runtime: ContractRuntime<T>, amount: Amount, to: ChainAccountOwner)
where
    T: Contract,
{
    let owner = runtime.authenticated_signer();
    let to = Account {
        chain_id: to.chain_id,
        owner: match to.owner {
            Some(AccountOwner::User(owner)) => Some(owner),
            _ => None,
        },
    };
    runtime.transfer(owner, to, amount);
}
