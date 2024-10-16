#!/bin/bash

function cleanup() {
  kill -15 `ps | grep linera-proxy | awk '{print $1}'` > /dev/null 2>&1
  kill -15 `ps | grep linera-server | awk '{print $1}'` > /dev/null 2>&1
  kill -15 `ps | grep linera | awk '{print $1}'` > /dev/null 2>&1
  kill -15 `ps | grep socat | awk '{print $1}'` > /dev/null 2>&1
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

unset all_proxy
unset ALL_PROXY

WALLET_10_PUBLIC_IPORT='210.209.69.38:23099'
WALLET_11_PUBLIC_IPORT='210.209.69.38:23101'
WALLET_12_PUBLIC_IPORT='210.209.69.38:23103'
WALLET_13_PUBLIC_IPORT='210.209.69.38:23105'
WALLET_14_PUBLIC_IPORT='210.209.69.38:23109'
LOCAL_IP='172.21.132.203'

# WALLET_10_PUBLIC_IPORT='172.16.31.73:40110'
# WALLET_11_PUBLIC_IPORT='172.16.31.73:40111'
# WALLET_12_PUBLIC_IPORT='172.16.31.73:40112'
# WALLET_13_PUBLIC_IPORT='172.16.31.73:40113'
# WALLET_14_PUBLIC_IPORT='172.16.31.73:40114'
# LOCAL_IP='localhost'

# WALLET_10_PUBLIC_IPORT='localhost:30090'
# WALLET_11_PUBLIC_IPORT='localhost:30091'
# WALLET_12_PUBLIC_IPORT='localhost:30092'
# WALLET_13_PUBLIC_IPORT='localhost:30093'
# WALLET_13_PUBLIC_IPORT='localhost:30094'
# LOCAL_IP='localhost'

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
create_wallet 14

wallet_14_default_chain=`linera --with-wallet 14 wallet show | grep "Public Key" | awk '{print $2}'`
wallet_14_owner=`linera --with-wallet 14 wallet show | grep "Owner" | awk '{print $4}'`

####
## Mint ERC20 token and WLINERA token initial liquidity to wallet 14 default chain directly
####

print $'\U01F4AB' $YELLOW " Deploying ERC20 application ..."
erc20_1_bid=`linera --with-wallet 10 publish-bytecode ./target/wasm32-unknown-unknown/release/erc20_{contract,service}.wasm`
erc20_1_appid=`linera --with-wallet 10 create-application $erc20_1_bid \
    --json-argument '{"initial_supply":"21000000","name":"Test Linera ERC20 Token","symbol":"TLA","decimals":18,"initial_currency":"0.00001","fixed_currency":false,"fee_percent":"0"}' \
    --json-parameters '{"initial_balances":{"{\"chain_id\":\"'$wallet_14_default_chain'\",\"owner\":\"User:'$wallet_14_owner'\"}":"5000000."}}' \
    `
print $'\U01f499' $LIGHTGREEN " ERC20 application deployed"
echo -e "    Bytecode ID:    $BLUE$erc20_1_bid$NC"
echo -e "    Application ID: $BLUE$erc20_1_appid$NC"

print $'\U01F4AB' $YELLOW " Deploying WTLINERA application ..."
erc20_2_bid=`linera --with-wallet 11 publish-bytecode ./target/wasm32-unknown-unknown/release/erc20_{contract,service}.wasm`
erc20_2_appid=`linera --with-wallet 11 create-application $erc20_2_bid \
    --json-argument '{"initial_supply":"21000000","name":"Wrapper Testnet LINERA Token","symbol":"WTLINERA","decimals":18,"initial_currency":"1","fixed_currency":true,"fee_percent":"0"}' \
    --json-parameters '{"initial_balances":{"{\"chain_id\":\"'$wallet_14_default_chain'\",\"owner\":\"User:'$wallet_14_owner'\"}":"5000000."}}' \
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

linera --with-wallet 14 request-application $erc20_1_appid
linera --with-wallet 14 request-application $erc20_2_appid
linera --with-wallet 14 request-application $swap_pool_appid
linera --with-wallet 14 request-application $swap_router_appid

function print_apps() {
  print $'\U01F4AB' $YELLOW " $1"
  echo -e "  Default Chain:  $LIGHTGREEN$3$NC"
  echo -e "  Owner:          $LIGHTGREEN$4$NC"
  echo -e "    ERC20:        $BLUE$2/chains/$3/applications/$erc20_1_appid$NC"
  echo -e "    WLINERA:      $BLUE$2/chains/$3/applications/$erc20_2_appid$NC"
  echo -e "    Swap Pool:    $BLUE$2/chains/$3/applications/$swap_pool_appid$NC"
  echo -e "    Swap Router:  $BLUE$2/chains/$3/applications/$swap_router_appid$NC"
}

HTTP_HOST="http://$WALLET_10_PUBLIC_IPORT"
chain=`linera --with-wallet 10 wallet show | grep "Public Key" | awk '{print $2}'`
owner=`linera --with-wallet 10 wallet show | grep "Owner" | awk '{print $4}'`
print_apps "Wallet 10" $HTTP_HOST $chain $owner

wallet_10_erc20_1_service="http://$LOCAL_IP:30090/chains/$chain/applications/$erc20_1_appid"
wallet_10_erc20_2_service="http://$LOCAL_IP:30090/chains/$chain/applications/$erc20_2_appid"
wallet_10_swap_pool_service="http://$LOCAL_IP:30090/chains/$chain/applications/$swap_pool_appid"
wallet_10_swap_router_service="http://$LOCAL_IP:30090/chains/$chain/applications/$swap_router_appid"
wallet_10_default_chain=$chain
wallet_10_owner=$owner

wallet_10_public_erc20_1_service="$HTTP_HOST/chains/$chain/applications/$erc20_1_appid"
wallet_10_public_swap_router_service="$HTTP_HOST/chains/$chain/applications/$swap_router_appid"

HTTP_HOST="http://$WALLET_11_PUBLIC_IPORT"
chain=`linera --with-wallet 11 wallet show | grep "Public Key" | awk '{print $2}'`
owner=`linera --with-wallet 11 wallet show | grep "Owner" | awk '{print $4}'`
print_apps "Wallet 11" $HTTP_HOST $chain $owner

wallet_11_erc20_1_service="http://$LOCAL_IP:30091/chains/$chain/applications/$erc20_1_appid"
wallet_11_erc20_2_service="http://$LOCAL_IP:30091/chains/$chain/applications/$erc20_2_appid"
wallet_11_swap_pool_service="http://$LOCAL_IP:30091/chains/$chain/applications/$swap_pool_appid"
wallet_11_swap_router_service="http://$LOCAL_IP:30091/chains/$chain/applications/$swap_router_appid"
wallet_11_default_chain=$chain
wallet_11_owner=$owner

wallet_11_public_erc20_1_service="$HTTP_HOST/chains/$chain/applications/$erc20_1_appid"
wallet_11_public_erc20_2_service="$HTTP_HOST/chains/$chain/applications/$erc20_2_appid"

HTTP_HOST="http://$WALLET_12_PUBLIC_IPORT"
chain=`linera --with-wallet 12 wallet show | grep "Public Key" | awk '{print $2}'`
owner=`linera --with-wallet 12 wallet show | grep "Owner" | awk '{print $4}'`
print_apps "Wallet 12" $HTTP_HOST $chain $owner

wallet_12_erc20_1_service="http://$LOCAL_IP:30092/chains/$chain/applications/$erc20_1_appid"
wallet_12_erc20_2_service="http://$LOCAL_IP:30092/chains/$chain/applications/$erc20_2_appid"
wallet_12_swap_pool_service="http://$LOCAL_IP:30092/chains/$chain/applications/$swap_pool_appid"
wallet_12_swap_router_service="http://$LOCAL_IP:30092/chains/$chain/applications/$swap_router_appid"
wallet_12_default_chain=$chain
wallet_12_owner=$owner

wallet_12_public_erc20_1_service="$HTTP_HOST/chains/$chain/applications/$erc20_1_appid"
wallet_12_public_erc20_2_service="$HTTP_HOST/chains/$chain/applications/$erc20_2_appid"
wallet_12_public_swap_pool_service="$HTTP_HOST/chains/$chain/applications/$swap_pool_appid"
wallet_12_public_swap_router_service="$HTTP_HOST/chains/$chain/applications/$swap_router_appid"

HTTP_HOST="http://$WALLET_13_PUBLIC_IPORT"
chain=`linera --with-wallet 13 wallet show | grep "Public Key" | awk '{print $2}'`
owner=`linera --with-wallet 13 wallet show | grep "Owner" | awk '{print $4}'`
print_apps "Wallet 13" $HTTP_HOST $chain $owner

wallet_13_erc20_1_service="http://$LOCAL_IP:30093/chains/$chain/applications/$erc20_1_appid"
wallet_13_erc20_2_service="http://$LOCAL_IP:30093/chains/$chain/applications/$erc20_2_appid"
wallet_13_swap_pool_service="http://$LOCAL_IP:30093/chains/$chain/applications/$swap_pool_appid"
wallet_13_swap_router_service="http://$LOCAL_IP:30093/chains/$chain/applications/$swap_router_appid"
wallet_13_default_chain=$chain
wallet_13_owner=$owner

wallet_13_public_swap_router_service="$HTTP_HOST/chains/$chain/applications/$swap_router_appid"

HTTP_HOST="http://$WALLET_14_PUBLIC_IPORT"
chain=`linera --with-wallet 14 wallet show | grep "Public Key" | awk '{print $2}'`
owner=`linera --with-wallet 14 wallet show | grep "Owner" | awk '{print $4}'`
print_apps "Wallet 14" $HTTP_HOST $chain $owner

wallet_14_erc20_1_service="http://$LOCAL_IP:30094/chains/$chain/applications/$erc20_1_appid"
wallet_14_erc20_2_service="http://$LOCAL_IP:30094/chains/$chain/applications/$erc20_2_appid"
wallet_14_swap_pool_service="http://$LOCAL_IP:30094/chains/$chain/applications/$swap_pool_appid"
wallet_14_swap_router_service="http://$LOCAL_IP:30094/chains/$chain/applications/$swap_router_appid"

wallet_14_public_erc20_1_service="$HTTP_HOST/chains/$chain/applications/$erc20_1_appid"
wallet_14_public_erc20_2_service="$HTTP_HOST/chains/$chain/applications/$erc20_2_appid"
wallet_14_public_swap_pool_service="$HTTP_HOST/chains/$chain/applications/$swap_pool_appid"
wallet_14_public_swap_router_service="$HTTP_HOST/chains/$chain/applications/$swap_router_appid"

####
## We should
##   1 subscribe to pool creator chain
##   2 set router application id to pool
##   3 authorize balance from wallet 14 default chain to swap pool
####

run_service 10 &
run_service 11 &
run_service 12 &
run_service 13 &
run_service 14 &

sleep 5

print $'\U01F4AB' $YELLOW " Subscribe ERC20 creator chain..."
curl -H 'Content-Type: application/json' -X POST -d '{ "query": "mutation { subscribeCreatorChain }"}' $wallet_12_erc20_1_service
curl -H 'Content-Type: application/json' -X POST -d '{ "query": "mutation { subscribeCreatorChain }"}' $wallet_13_erc20_1_service
curl -H 'Content-Type: application/json' -X POST -d '{ "query": "mutation { subscribeCreatorChain }"}' $wallet_14_erc20_1_service
echo
print $'\U01F4AB' $YELLOW " Subscribe WLINERA creator chain..."
curl -H 'Content-Type: application/json' -X POST -d '{ "query": "mutation { subscribeCreatorChain }"}' $wallet_12_erc20_2_service
curl -H 'Content-Type: application/json' -X POST -d '{ "query": "mutation { subscribeCreatorChain }"}' $wallet_13_erc20_2_service
curl -H 'Content-Type: application/json' -X POST -d '{ "query": "mutation { subscribeCreatorChain }"}' $wallet_14_erc20_2_service
echo
print $'\U01F4AB' $YELLOW " Subscribe pool creator chain..."
curl -H 'Content-Type: application/json' -X POST -d '{ "query": "mutation { subscribeCreatorChain }"}' $wallet_13_swap_pool_service
curl -H 'Content-Type: application/json' -X POST -d '{ "query": "mutation { subscribeCreatorChain }"}' $wallet_14_swap_pool_service
echo
print $'\U01F4AB' $YELLOW " Subscribe router creator chain..."
curl -H 'Content-Type: application/json' -X POST -d '{ "query": "mutation { subscribeCreatorChain }"}' $wallet_14_swap_router_service
echo
print $'\U01F4AB' $YELLOW " Set router application id to pool..."
curl -H 'Content-Type: application/json' -X POST -d "{ \"query\": \"mutation { setRouterApplicationId(applicationId:\\\"$swap_router_appid\\\")}\"}" $wallet_12_swap_pool_service
echo
print $'\U01F4AB' $YELLOW " Authorize ERC20 to pool application..."
curl -H 'Content-Type: application/json' -X POST -d "{ \"query\": \"mutation { approve(spender: {chain_id: \\\"$wallet_12_default_chain\\\", owner:\\\"Application:$swap_pool_appid\\\"},value:\\\"4000000.\\\")}\"}" $wallet_14_erc20_1_service
echo
print $'\U01F4AB' $YELLOW " Authorize WLINERA to pool application..."
curl -H 'Content-Type: application/json' -X POST -d "{ \"query\": \"mutation { approve(spender: {chain_id: \\\"$wallet_12_default_chain\\\", owner:\\\"Application:$swap_pool_appid\\\"},value:\\\"4000000.\\\")}\"}" $wallet_14_erc20_2_service
echo
print $'\U01F4AB' $YELLOW " Authorize ERC20 to router application..."
curl -H 'Content-Type: application/json' -X POST -d "{ \"query\": \"mutation { approve(spender: {chain_id: \\\"$wallet_13_default_chain\\\", owner:\\\"Application:$swap_router_appid\\\"},value:\\\"500000.\\\")}\"}" $wallet_14_erc20_1_service
echo
print $'\U01F4AB' $YELLOW " Authorize WLINERA to router application..."
curl -H 'Content-Type: application/json' -X POST -d "{ \"query\": \"mutation { approve(spender: {chain_id: \\\"$wallet_13_default_chain\\\", owner:\\\"Application:$swap_router_appid\\\"},value:\\\"500000.\\\")}\"}" $wallet_14_erc20_2_service
echo

print $'\U01F4AB' $YELLOW " Query ERC20 allowance with..."
print $'\U01F4AB' $LIGHTGREEN " $wallet_10_public_erc20_1_service"
print $'\U01F4AB' $LIGHTGREEN " $wallet_12_public_erc20_1_service"
echo -e "query {\n\
  allowance(\n\
    owner: {\n\
      chain_id:\"$wallet_10_default_chain\",\n\
      owner:\"User:$wallet_10_owner\"\n\
    },\n\
    spender: {\n\
      chain_id:\"$wallet_12_default_chain\",\n\
      owner:\"Application:$swap_pool_appid\"\n\
    }\n\
  )\n\
  balanceOf(owner: {\n\
    chain_id:\"$wallet_10_default_chain\",\n\
    owner:\"User:$wallet_10_owner\"\n\
  })\n\
  totalSupply\n\
  name\n\
  symbol\n\
  decimals\n\
}"

print $'\U01F4AB' $YELLOW " Query WLINERA allowance with..."
print $'\U01F4AB' $LIGHTGREEN " $wallet_11_public_erc20_2_service"
print $'\U01F4AB' $LIGHTGREEN " $wallet_12_public_erc20_2_service"
echo -e "query {\n\
  allowance(\n\
    owner: {\n\
      chain_id:\"$wallet_11_default_chain\",\n\
      owner:\"User:$wallet_11_owner\"\n\
    },\n\
    spender: {\n\
      chain_id:\"$wallet_12_default_chain\",\n\
      owner:\"Application:$swap_pool_appid\"\n\
    }\n\
  )\n\
  balanceOf(owner: {\n\
    chain_id:\"$wallet_11_default_chain\",\n\
    owner:\"User:$wallet_11_owner\"\n\
  })\n\
  totalSupply\n\
  name\n\
  symbol\n\
  decimals\n\
}"

print $'\U01F4AB' $YELLOW " Query ERC20 balance with..."
print $'\U01F4AB' $LIGHTGREEN " $wallet_10_public_erc20_1_service"
print $'\U01F4AB' $LIGHTGREEN " $wallet_14_public_erc20_1_service"
echo -e "query {\n\
  allowance(\n\
    owner: {\n\
      chain_id:\"$wallet_14_default_chain\",\n\
      owner:\"User:$wallet_14_owner\"\n\
    },\n\
    spender: {\n\
      chain_id:\"$wallet_12_default_chain\",\n\
      owner:\"Application:$swap_pool_appid\"\n\
    }\n\
  )\n\
  balanceOf(owner: {\n\
    chain_id:\"$wallet_14_default_chain\",\n\
    owner:\"User:$wallet_14_owner\"\n\
  })\n\
  totalSupply\n\
  name\n\
  symbol\n\
  decimals\n\
}"

print $'\U01F4AB' $YELLOW " Query WLINERA balance with..."
print $'\U01F4AB' $LIGHTGREEN " $wallet_11_public_erc20_2_service"
print $'\U01F4AB' $LIGHTGREEN " $wallet_14_public_erc20_2_service"
echo -e "query {\n\
  allowance(\n\
    owner: {\n\
      chain_id:\"$wallet_14_default_chain\",\n\
      owner:\"User:$wallet_14_owner\"\n\
    },\n\
    spender: {\n\
      chain_id:\"$wallet_12_default_chain\",\n\
      owner:\"Application:$swap_pool_appid\"\n\
    }\n\
  )\n\
  balanceOf(owner: {\n\
    chain_id:\"$wallet_14_default_chain\",\n\
    owner:\"User:$wallet_14_owner\"\n\
  })\n\
  totalSupply\n\
  name\n\
  symbol\n\
  decimals\n\
}"

print $'\U01F4AB' $YELLOW " Add liquidity with..."
print $'\U01F4AB' $LIGHTGREEN " $wallet_14_public_swap_router_service"
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

print $'\U01F4AB' $YELLOW " Query pools with..."
print $'\U01F4AB' $LIGHTGREEN " $wallet_12_public_swap_pool_service"
print $'\U01F4AB' $LIGHTGREEN " $wallet_14_public_swap_pool_service"
echo -e "query {\n\
  getPools {\n\
    id\n\
    token0\n\
    token1\n\
    poolFeeRate\n\
    price0Cumulative\n\
    price1Cumulative\n\
    amount0Initial\n\
    amount1Initial\n\
    kLast\n\
    blockTimestamp\n\
    protocolFeeRate\n\
    virtualInitialLiquidity\n\
    reserve0\n\
    reserve1\n\
    erc20\n\
    feeTo\n\
    feeToSetter\n\
  }\n\
}"

trap cleanup INT
read -p "  Press any key to exit"
print $'\U01f499' $LIGHTGREEN " Exit ..."

cleanup

