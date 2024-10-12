#!/bin/bash

kill -9 `ps | grep linera | awk '{print $1}'`

unset RUSTFLAGS
cargo build --release --target wasm32-unknown-unknown

WALLET_BASE=/tmp/linera/dapps
mkdir -p $WALLET_BASE
rm $WALLET_BASE/* -rf

BLUE='\033[1;34m'
YELLOW='\033[1;33m'
LIGHTGREEN='\033[1;32m'
NC='\033[0m'

function print() {
  echo -e $1$2$3$NC
}

function create_wallet() {
  export LINERA_WALLET_$1=$WALLET_BASE/wallet_$1.json
  export LINERA_STORAGE_$1=rocksdb:$WALLET_BASE/client_$1.db

  linera -w $1 wallet init --faucet http://localhost:40080 --with-new-chain
  linera -w $1 wallet show
}

function run_service () {
  local_port=`expr 30080 + $1`
  pub_port=`expr 40100 + $1`

  linera -w $1 service --port $local_port --external-signing false &
  if [ ! $? -eq 0 ]; then
    echo "Run with official release"
    linera -w $1 service --port $local_port &
  fi

  sleep 10
  socat TCP4-LISTEN:$pub_port TCP4:localhost:$local_port
}

create_wallet 10
create_wallet 11
create_wallet 12
create_wallet 13

print $'\U01F4AB' $YELLOW " Deploying ERC20 application ..."
erc20_1_bid=`linera --with-wallet 10 publish-bytecode ./target/wasm32-unknown-unknown/release/erc20_{contract,service}.wasm`
erc20_1_appid=`linera --with-wallet 10 create-application $erc20_1_bid \
    --json-argument '{"initial_supply":"20000000","name":"Test Linera ERC20 Token","symbol":"TLA","decimals":18,"initial_currency":"0.00001","fixed_currency":false,"fee_percent":"0.1"}' \
    --json-parameters '{"initial_balances": {}}' \
    `
print $'\U01f499' $LIGHTGREEN " ERC20 application deployed"
echo -e "    Bytecode ID:    $BLUE$erc20_1_bid$NC"
echo -e "    Application ID: $BLUE$erc20_1_appid$NC"

timeout 10s linera -w 10 service --port 30090 --external-signing false

print $'\U01F4AB' $YELLOW " Deploying WTLINERA application ..."
erc20_2_bid=`linera --with-wallet 11 publish-bytecode ./target/wasm32-unknown-unknown/release/erc20_{contract,service}.wasm`
erc20_2_appid=`linera --with-wallet 11 create-application $erc20_2_bid \
    --json-argument '{"initial_supply":"20000000","name":"Wrapper Testnet LINERA Token","symbol":"WTLINERA","decimals":18,"initial_currency":"1","fixed_currency":true,"fee_percent":"0.1"}' \
    --json-parameters '{"initial_balances": {}}' \
    `
print $'\U01f499' $LIGHTGREEN " WLINERA application deployed"
echo -e "    Bytecode ID:    $BLUE$erc20_2_bid$NC"
echo -e "    Application ID: $BLUE$erc20_2_appid$NC"

linera --with-wallet 12 request-application $erc20_1_appid
linera --with-wallet 12 request-application $erc20_2_appid

timeout 10s linera -w 10 service --port 30090 --external-signing false
timeout 10s linera -w 11 service --port 30091 --external-signing false
timeout 10s linera -w 12 service --port 30092 --external-signing false

print $'\U01F4AB' $YELLOW " Deploying Swap Pool application ..."
swap_pool_bid=`linera --with-wallet 12 publish-bytecode ./target/wasm32-unknown-unknown/release/swap_pool_{contract,service}.wasm`
swap_pool_appid=`linera --with-wallet 12 create-application $swap_pool_bid \
    `
print $'\U01f499' $LIGHTGREEN " Swap Pool application deployed"
echo -e "    Bytecode ID:    $BLUE$swap_pool_bid$NC"
echo -e "    Application ID: $BLUE$swap_pool_appid$NC"

linera --with-wallet 13 request-application $erc20_1_appid
linera --with-wallet 13 request-application $erc20_2_appid
linera --with-wallet 13 request-application $swap_pool_appid

timeout 10s linera -w 10 service --port 30090 --external-signing false
timeout 10s linera -w 11 service --port 30091 --external-signing false
timeout 10s linera -w 12 service --port 30092 --external-signing false
timeout 10s linera -w 13 service --port 30093 --external-signing false

print $'\U01F4AB' $YELLOW " Deploying Swap Router application ..."
swap_router_bid=`linera --with-wallet 13 publish-bytecode ./target/wasm32-unknown-unknown/release/swap_router_{contract,service}.wasm`
swap_router_appid=`linera --with-wallet 13 create-application $swap_router_bid \
    --json-parameters "{\"pool_application_id\":\"$swap_pool_appid\"}" \
    `
print $'\U01f499' $LIGHTGREEN " Swap Router application deployed"
echo -e "    Bytecode ID:    $BLUE$swap_router_bid$NC"
echo -e "    Application ID: $BLUE$swap_router_appid$NC"

run_service 10 &
run_service 11 &
run_service 12 &
run_service 13 &

sleep 1000000
