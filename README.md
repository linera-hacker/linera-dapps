<img src="https://raw.githubusercontent.com/linera-hacker/linera-dapps/refs/heads/master/assets/HackerLogoDark.svg">

## dApps
- [x] Linera ERC20 fungible token
- [ ] Linera ERC20 fungible token web app
- [x] Linera dex
- [ ] Linera dex web app

## How to run
```
git clone https://github.com/respeer-ai/linera-protocol.git
cd linera-protocol
git checkout respeer-maas-v0.12.1
cargo install --path linera-service --features storage-service
cargo install --path linera-storage-service --features storage-service
cd -

git clone https://github.com/respeer-ai/res-peer.git
cd res-peer
./deploy-local.sh -N devnet

git clone https://github.com/linera-hacker/linera-dapps.git
cd linera-dapps
./deploy-local.sh -N devnet
./deploy-erc20-real-liquidity.sh -N devnet
./deploy-erc20-virtual-liquidity.sh -N devnet
./deploy-new-user.sh -N devnet
```
