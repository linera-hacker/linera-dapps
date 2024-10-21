<img src="https://raw.githubusercontent.com/linera-hacker/linera-dapps/refs/heads/master/assets/HackerLogoDark.svg">

## dApps
- [x] Linera ERC20 fungible token
- [ ] Linera ERC20 fungible token web app
- [x] Linera dex
- [ ] Linera dex web app

## How to run
### Note

Official Devnet is compatible with v0.12.1 in which the order of executed message is not correct.
ResPeer fork cherry-pick the fix commit from latest branch then it could work with current Devnet.

### Compile toolchain
```
git clone https://github.com/respeer-ai/linera-protocol.git
cd linera-protocol
git checkout respeer-maas-v0.12.1
cargo install --path linera-service --features storage-service
cargo install --path linera-storage-service --features storage-service
cd -
```
### Deploy to official DevNet
```
git clone https://github.com/respeer-ai/res-peer.git
cd res-peer
./deploy-devnet.sh -N devnet

git clone https://github.com/linera-hacker/linera-dapps.git
cd linera-dapps
./deploy-local.sh -N devnet
./deploy-erc20-real-liquidity.sh -N devnet
./deploy-erc20-virtual-liquidity.sh -N devnet
./deploy-new-user.sh -N devnet
```
### Deploy to local testnet
```
git clone https://github.com/respeer-ai/res-peer.git
cd res-peer
./deploy-local.sh -N localnet

git clone https://github.com/linera-hacker/linera-dapps.git
cd linera-dapps
./deploy-local.sh -N localnet
./deploy-erc20-real-liquidity.sh -N localnet
./deploy-erc20-virtual-liquidity.sh -N localnet
./deploy-new-user.sh -N localnet
```
