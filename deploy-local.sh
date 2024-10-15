#!/bin/bash

function cleanup() {
  kill -15 `ps | grep linera-proxy | awk '{print $1}'` > /dev/null 2>&1
  kill -15 `ps | grep linera-server | awk '{print $1}'` > /dev/null 2>&1
  kill -15 `ps | grep linera | awk '{print $1}'` > /dev/null 2>&1
  kill -9 `ps | grep socat | awk '{print $1}'` > /dev/null 2>&1
}

cleanup

unset RUSTFLAGS
unset TMPDIR
unset ALL_PROXY
unset all_proxy

cargo build --release --target wasm32-unknown-unknown

WALLET_BASE=/tmp/linera/dapps
mkdir -p $WALLET_BASE
rm $WALLET_BASE/* -rf

WALLET_10_PUBLIC_IPORT='210.209.69.38:23103'
WALLET_14_PUBLIC_IPORT='210.209.69.38:23105'
LOCAL_IP='172.21.132.203'

BLUE='\033[1;34m'
YELLOW='\033[1;33m'
LIGHTGREEN='\033[1;32m'
NC='\033[0m'

PROJECT_ROOT=$HOME/linera-project
mkdir -p $PROJECT_ROOT

function print() {
  echo -e $1$2$3$NC
}

function create_wallet() {
  export LINERA_WALLET_$1=$WALLET_BASE/wallet_$1.json
  export LINERA_STORAGE_$1=rocksdb:$WALLET_BASE/client_$1.db

  linera -w $1 wallet init --faucet http://localhost:40080 --with-new-chain
  linera -w $1 wallet show
}

function __run_service() {
  linera -w $1 service --port $2 --external-signing false
  if [ ! $? -eq 0 ]; then
    echo "Run with official release"
    linera -w $1 service --port $2
  fi
}

function run_service () {
  local_port=`expr 30080 + $1`
  pub_port=`expr 40100 + $1`

  __run_service $1 $local_port > $PROJECT_ROOT/service_$local_port.log 2>&1 &

  sleep 3
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

print $'\U01F4AB' $YELLOW " Deploying Swap Router application ..."
swap_router_bid=`linera --with-wallet 13 publish-bytecode ./target/wasm32-unknown-unknown/release/swap_router_{contract,service}.wasm`
swap_router_appid=`linera --with-wallet 13 create-application $swap_router_bid \
    --json-parameters "{\"pool_application_id\":\"$swap_pool_appid\"}" \
    `
print $'\U01f499' $LIGHTGREEN " Swap Router application deployed"
echo -e "    Bytecode ID:    $BLUE$swap_router_bid$NC"
echo -e "    Application ID: $BLUE$swap_router_appid$NC"

function print_apps() {
  print $'\U01F4AB' $YELLOW " $1"
  echo -e "    ERC20:        $BLUE$2/chains/$3/applications/$erc20_1_appid$NC"
  echo -e "    WLINERA:      $BLUE$2/chains/$3/applications/$erc20_2_appid$NC"
  echo -e "    Swap Pool:    $BLUE$2/chains/$3/applications/$swap_pool_appid$NC"
  echo -e "    Swap Router:  $BLUE$2/chains/$3/applications/$swap_router_appid$NC"
}

HTTP_HOST="http://$WALLET_10_PUBLIC_IPORT"
chain=`linera --with-wallet 10 wallet show | grep "Public Key" | awk '{print $2}'`
print_apps "Wallet 10" $HTTP_HOST $chain

wallet_10_erc20_1_service="http://$LOCAL_IP:30090/chains/$chain/applications/$erc20_1_appid"

chain=`linera --with-wallet 11 wallet show | grep "Public Key" | awk '{print $2}'`
print_apps "Wallet 11" $HTTP_HOST $chain

wallet_11_erc20_2_service="http://$LOCAL_IP:30091/chains/$chain/applications/$erc20_2_appid"

chain=`linera --with-wallet 12 wallet show | grep "Public Key" | awk '{print $2}'`
print_apps "Wallet 12" $HTTP_HOST $chain

wallet_12_swap_pool_service="http://$LOCAL_IP:30092/chains/$chain/applications/$swap_pool_appid"

HTTP_HOST="http://$WALLET_14_PUBLIC_IPORT"
chain=`linera --with-wallet 13 wallet show | grep "Public Key" | awk '{print $2}'`
print_apps "Wallet 13" $HTTP_HOST $chain

####
## We should
##   1 subscribe to pool creator chain
##   2 set router application id to pool
####
wallet_13_swap_pool_service="http://$LOCAL_IP:30093/chains/$chain/applications/$swap_pool_appid"
wallet_13_swap_router_service="$HTTP_HOST/chains/$chain/applications/$swap_router_appid"

run_service 10 &
run_service 11 &
run_service 12 &
run_service 13 &

sleep 5

print $'\U01F4AB' $YELLOW " Subscribe erc20 1 creator chain..."
curl -H 'Content-Type: application/json' -X POST -d '{ "query": "mutation { subscribeCreatorChain }"}' $wallet_10_erc20_1_service
echo
print $'\U01F4AB' $YELLOW " Subscribe erc20 2 creator chain..."
curl -H 'Content-Type: application/json' -X POST -d '{ "query": "mutation { subscribeCreatorChain }"}' $wallet_11_erc20_2_service
echo
print $'\U01F4AB' $YELLOW " Subscribe pool creator chain..."
curl -H 'Content-Type: application/json' -X POST -d '{ "query": "mutation { subscribeCreatorChain }"}' $wallet_13_swap_pool_service
echo
print $'\U01F4AB' $YELLOW " Set router application id to pool..."
curl -H 'Content-Type: application/json' -X POST -d "{ \"query\": \"mutation { setRouterApplicationId(applicationId:\\\"$swap_router_appid\\\")}\"}" $wallet_12_swap_pool_service
echo

print $'\U01F4AB' $YELLOW " Add liquidity with..."
print $'\U01F4AB' $LIGHTGREEN " $wallet_13_swap_router_service"
echo -e "mutation {\n\
  addLiquidity (\n\
    token0: \"$erc20_1_appid\",\n\
    token1: \"$erc20_2_appid\",\n\
    amount0Desired: \"2000\",\n\
    amount1Desired: \"2000\",\n\
    amount0Min: \"2000\",\n\
    amount1Min: \"2000\",\n\
    deadline: 0,\n\
  )\n\
}"

trap cleanup INT
read -p "  Press any key to exit"
print $'\U01f499' $LIGHTGREEN " Exit ..."

cleanup

