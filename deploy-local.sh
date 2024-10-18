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

NETWORK_ID=1

case $NETWORK_ID in
  1)
    WALLET_10_PUBLIC_IPORT='210.209.69.38:23099'
    WALLET_11_PUBLIC_IPORT='210.209.69.38:23101'
    WALLET_12_PUBLIC_IPORT='210.209.69.38:23103'
    WALLET_13_PUBLIC_IPORT='210.209.69.38:23105'
    LOCAL_IP='172.21.132.203'
    ;;
  2)
    WALLET_10_PUBLIC_IPORT='172.16.31.73:40110'
    WALLET_11_PUBLIC_IPORT='172.16.31.73:40111'
    WALLET_12_PUBLIC_IPORT='172.16.31.73:40112'
    WALLET_13_PUBLIC_IPORT='172.16.31.73:40113'
    LOCAL_IP='localhost'
    ;;
  3)
    WALLET_10_PUBLIC_IPORT='localhost:30090'
    WALLET_11_PUBLIC_IPORT='localhost:30091'
    WALLET_12_PUBLIC_IPORT='localhost:30092'
    WALLET_13_PUBLIC_IPORT='localhost:30093'
    LOCAL_IP='localhost'
    ;;
esac

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

wallet_13_default_chain=`linera --with-wallet 13 wallet show | grep "Public Key" | awk '{print $2}'`
wallet_13_owner=`linera --with-wallet 13 wallet show | grep "Owner" | awk '{print $4}'`

wallet_10_default_chain=`linera --with-wallet 10 wallet show | grep "Public Key" | awk '{print $2}'`
wallet_10_owner=`linera --with-wallet 10 wallet show | grep "Owner" | awk '{print $4}'`

####
## Mint ERC20 token and WLINERA token initial liquidity to wallet 14 default chain directly
####

print $'\U01F4AB' $YELLOW " Deploying ERC20 application ..."
erc20_1_bid=`linera --with-wallet 10 publish-bytecode ./target/wasm32-unknown-unknown/release/erc20_{contract,service}.wasm`
erc20_1_appid=`linera --with-wallet 10 create-application $erc20_1_bid \
    --json-argument '{"initial_supply":"21000000","name":"Test Linera ERC20 Token","symbol":"TLA","decimals":18,"initial_currency":"0.00001","fixed_currency":false,"fee_percent":"0"}' \
    --json-parameters '{"initial_balances":{"{\"chain_id\":\"'$wallet_13_default_chain'\",\"owner\":\"User:'$wallet_13_owner'\"}":"5000000."}}' \
    `
print $'\U01f499' $LIGHTGREEN " ERC20 application deployed"
echo -e "    Bytecode ID:    $BLUE$erc20_1_bid$NC"
echo -e "    Application ID: $BLUE$erc20_1_appid$NC"

print $'\U01F4AB' $YELLOW " Deploying WTLINERA application ..."
erc20_2_bid=`linera --with-wallet 11 publish-bytecode ./target/wasm32-unknown-unknown/release/erc20_{contract,service}.wasm`
erc20_2_appid=`linera --with-wallet 11 create-application $erc20_2_bid \
    --json-argument '{"initial_supply":"21000000","name":"Wrapper Testnet LINERA Token","symbol":"WTLINERA","decimals":18,"initial_currency":"1","fixed_currency":true,"fee_percent":"0"}' \
    --json-parameters '{"initial_balances":{"{\"chain_id\":\"'$wallet_13_default_chain'\",\"owner\":\"User:'$wallet_13_owner'\"}":"5000000.","{\"chain_id\":\"'$wallet_10_default_chain'\",\"owner\":\"User:'$wallet_10_owner'\"}":"5000000."}}' \
    `
print $'\U01f499' $LIGHTGREEN " WLINERA application deployed"
echo -e "    Bytecode ID:    $BLUE$erc20_2_bid$NC"
echo -e "    Application ID: $BLUE$erc20_2_appid$NC"

print $'\U01F4AB' $YELLOW " Deploying Swap application ..."
swap_bid=`linera --with-wallet 12 publish-bytecode ./target/wasm32-unknown-unknown/release/swap_{contract,service}.wasm`
swap_appid=`linera --with-wallet 12 create-application $swap_bid`
print $'\U01f499' $LIGHTGREEN " Swap application deployed"
echo -e "    Bytecode ID:    $BLUE$swap_bid$NC"
echo -e "    Application ID: $BLUE$swap_appid$NC"

linera --with-wallet 12 request-application $erc20_1_appid
linera --with-wallet 12 request-application $erc20_2_appid

linera --with-wallet 13 request-application $erc20_1_appid
linera --with-wallet 13 request-application $erc20_2_appid
linera --with-wallet 13 request-application $swap_appid

linera --with-wallet 10 request-application $swap_appid
linera --with-wallet 10 request-application $erc20_2_appid

function print_apps() {
  print $'\U01F4AB' $YELLOW " $1"
  echo -e "  Default Chain:  $LIGHTGREEN$3$NC"
  echo -e "  Owner:          $LIGHTGREEN$4$NC"
  echo -e "    ERC20:        $BLUE$2/chains/$3/applications/$erc20_1_appid$NC"
  echo -e "    WLINERA:      $BLUE$2/chains/$3/applications/$erc20_2_appid$NC"
  echo -e "    Swap:         $BLUE$2/chains/$3/applications/$swap_appid$NC"
}

HTTP_HOST="http://$WALLET_10_PUBLIC_IPORT"
chain=`linera --with-wallet 10 wallet show | grep "Public Key" | awk '{print $2}'`
owner=`linera --with-wallet 10 wallet show | grep "Owner" | awk '{print $4}'`
print_apps "Wallet 10" $HTTP_HOST $chain $owner

wallet_10_erc20_1_service="http://$LOCAL_IP:30090/chains/$chain/applications/$erc20_1_appid"
wallet_10_erc20_2_service="http://$LOCAL_IP:30090/chains/$chain/applications/$erc20_2_appid"
wallet_10_swap_service="http://$LOCAL_IP:30090/chains/$chain/applications/$swap_appid"
wallet_10_default_chain=$chain
wallet_10_owner=$owner

wallet_10_public_erc20_1_service="$HTTP_HOST/chains/$chain/applications/$erc20_1_appid"
wallet_10_public_swap_service="$HTTP_HOST/chains/$chain/applications/$swap_appid"

HTTP_HOST="http://$WALLET_11_PUBLIC_IPORT"
chain=`linera --with-wallet 11 wallet show | grep "Public Key" | awk '{print $2}'`
owner=`linera --with-wallet 11 wallet show | grep "Owner" | awk '{print $4}'`
print_apps "Wallet 11" $HTTP_HOST $chain $owner

wallet_11_erc20_1_service="http://$LOCAL_IP:30091/chains/$chain/applications/$erc20_1_appid"
wallet_11_erc20_2_service="http://$LOCAL_IP:30091/chains/$chain/applications/$erc20_2_appid"
wallet_11_swap_service="http://$LOCAL_IP:30091/chains/$chain/applications/$swap_appid"
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
wallet_12_swap_service="http://$LOCAL_IP:30092/chains/$chain/applications/$swap_appid"
wallet_12_default_chain=$chain
wallet_12_owner=$owner

wallet_12_public_erc20_1_service="$HTTP_HOST/chains/$chain/applications/$erc20_1_appid"
wallet_12_public_erc20_2_service="$HTTP_HOST/chains/$chain/applications/$erc20_2_appid"
wallet_12_public_swap_service="$HTTP_HOST/chains/$chain/applications/$swap_appid"

HTTP_HOST="http://$WALLET_13_PUBLIC_IPORT"
chain=`linera --with-wallet 13 wallet show | grep "Public Key" | awk '{print $2}'`
owner=`linera --with-wallet 13 wallet show | grep "Owner" | awk '{print $4}'`
print_apps "Wallet 13" $HTTP_HOST $chain $owner

wallet_13_erc20_1_service="http://$LOCAL_IP:30093/chains/$chain/applications/$erc20_1_appid"
wallet_13_erc20_2_service="http://$LOCAL_IP:30093/chains/$chain/applications/$erc20_2_appid"
wallet_13_swap_service="http://$LOCAL_IP:30093/chains/$chain/applications/$swap_appid"
wallet_13_default_chain=$chain
wallet_13_owner=$owner

wallet_13_public_erc20_1_service="$HTTP_HOST/chains/$chain/applications/$erc20_1_appid"
wallet_13_public_erc20_2_service="$HTTP_HOST/chains/$chain/applications/$erc20_2_appid"
wallet_13_public_swap_service="$HTTP_HOST/chains/$chain/applications/$swap_appid"

####
## We should
##   1 subscribe to pool creator chain
##   2 set swap application id to pool
##   3 authorize balance from wallet 13 default chain to swap pool
####

run_service 10 &
run_service 11 &
run_service 12 &
run_service 13 &

sleep 5

print $'\U01F4AB' $YELLOW " Subscribe ERC20 creator chain..."
curl -H 'Content-Type: application/json' -X POST -d '{ "query": "mutation { subscribeCreatorChain }"}' $wallet_12_erc20_1_service
curl -H 'Content-Type: application/json' -X POST -d '{ "query": "mutation { subscribeCreatorChain }"}' $wallet_13_erc20_1_service
echo
print $'\U01F4AB' $YELLOW " Subscribe WLINERA creator chain..."
curl -H 'Content-Type: application/json' -X POST -d '{ "query": "mutation { subscribeCreatorChain }"}' $wallet_10_erc20_2_service
curl -H 'Content-Type: application/json' -X POST -d '{ "query": "mutation { subscribeCreatorChain }"}' $wallet_12_erc20_2_service
curl -H 'Content-Type: application/json' -X POST -d '{ "query": "mutation { subscribeCreatorChain }"}' $wallet_13_erc20_2_service
echo
print $'\U01F4AB' $YELLOW " Subscribe swap creator chain..."
curl -H 'Content-Type: application/json' -X POST -d '{ "query": "mutation { subscribeCreatorChain }"}' $wallet_13_swap_service
echo
print $'\U01F4AB' $YELLOW " Authorize ERC20 to swap application..."
curl -H 'Content-Type: application/json' -X POST -d "{ \"query\": \"mutation { approve(spender: {chain_id: \\\"$wallet_12_default_chain\\\", owner:\\\"Application:$swap_appid\\\"},value:\\\"4500000.\\\")}\"}" $wallet_13_erc20_1_service
curl -H 'Content-Type: application/json' -X POST -d "{ \"query\": \"mutation { approve(spender: {chain_id: \\\"$wallet_12_default_chain\\\", owner:\\\"Application:$swap_appid\\\"},value:\\\"4500000.\\\")}\"}" $wallet_10_erc20_1_service
echo
print $'\U01F4AB' $YELLOW " Authorize WLINERA to swap application..."
curl -H 'Content-Type: application/json' -X POST -d "{ \"query\": \"mutation { approve(spender: {chain_id: \\\"$wallet_12_default_chain\\\", owner:\\\"Application:$swap_appid\\\"},value:\\\"4500000.\\\")}\"}" $wallet_13_erc20_2_service
## Wallet 10 will create pool with initial liquidity
curl -H 'Content-Type: application/json' -X POST -d "{ \"query\": \"mutation { approve(spender: {chain_id: \\\"$wallet_12_default_chain\\\", owner:\\\"Application:$swap_appid\\\"},value:\\\"4500000.\\\")}\"}" $wallet_10_erc20_2_service
echo
print $'\U01F4AB' $YELLOW " Create liquidity pool by ERC20 1 creator..."
curl -H 'Content-Type: application/json' -X POST -d "{ \"query\": \"mutation { createPool(token0: \\\"$erc20_1_appid\\\", token1: \\\"$erc20_2_appid\\\", amount0Initial:\\\"20\\\", amount1Initial:\\\"20\\\", amount0Virtual:\\\"20\\\", amount1Virtual:\\\"20\\\")}\"}" $wallet_10_swap_service
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
      owner:\"Application:$swap_appid\"\n\
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
      owner:\"Application:$swap_appid\"\n\
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
print $'\U01F4AB' $LIGHTGREEN " $wallet_12_public_erc20_1_service"
echo -e "query {\n\
  allowance(\n\
    owner: {\n\
      chain_id:\"$wallet_13_default_chain\",\n\
      owner:\"User:$wallet_13_owner\"\n\
    },\n\
    spender: {\n\
      chain_id:\"$wallet_12_default_chain\",\n\
      owner:\"Application:$swap_appid\"\n\
    }\n\
  )\n\
  balanceOf(owner: {\n\
    chain_id:\"$wallet_13_default_chain\",\n\
    owner:\"User:$wallet_13_owner\"\n\
  })\n\
  totalSupply\n\
  name\n\
  symbol\n\
  decimals\n\
}"

print $'\U01F4AB' $YELLOW " Query WLINERA balance with..."
print $'\U01F4AB' $LIGHTGREEN " $wallet_11_public_erc20_2_service"
print $'\U01F4AB' $LIGHTGREEN " $wallet_13_public_erc20_2_service"
echo -e "query {\n\
  allowance(\n\
    owner: {\n\
      chain_id:\"$wallet_13_default_chain\",\n\
      owner:\"User:$wallet_13_owner\"\n\
    },\n\
    spender: {\n\
      chain_id:\"$wallet_12_default_chain\",\n\
      owner:\"Application:$swap_appid\"\n\
    }\n\
  )\n\
  balanceOf(owner: {\n\
    chain_id:\"$wallet_13_default_chain\",\n\
    owner:\"User:$wallet_13_owner\"\n\
  })\n\
  totalSupply\n\
  name\n\
  symbol\n\
  decimals\n\
}"

print $'\U01F4AB' $YELLOW " Created pool with..."
print $'\U01F4AB' $LIGHTGREEN " $wallet_10_public_swap_service"
echo -e "mutation {\n\
  createPool (\n\
    token0: \"$erc20_1_appid\",\n\
    token1: \"$erc20_2_appid\",\n\
    amount0Initial: \"20\",\n\
    amount1Initial: \"20\",\n\
    amount0Virtual: \"20\",\n\
    amount1Virtual: \"20\",\n\
  )\n\
}"

print $'\U01F4AB' $YELLOW " Add liquidity with..."
print $'\U01F4AB' $LIGHTGREEN " $wallet_13_public_swap_service"
echo -e "mutation {\n\
  addLiquidity (\n\
    token0: \"$erc20_1_appid\",\n\
    token1: \"$erc20_2_appid\",\n\
    amount0Desired: \"20\",\n\
    amount1Desired: \"20\",\n\
    amount0Min: \"20\",\n\
    amount1Min: \"20\",\n\
    deadline: 0,\n\
  )\n\
}"

print $'\U01F4AB' $YELLOW " Remove liquidity with..."
print $'\U01F4AB' $LIGHTGREEN " $wallet_13_public_swap_service"
echo -e "mutation {\n\
  removeLiquidity (\n\
    token0: \"$erc20_1_appid\",\n\
    token1: \"$erc20_2_appid\",\n\
    liquidity: \"5\",\n\
    amount0Min: \"1.\",\n\
    amount1Min: \"1.\",\n\
    deadline: 0,\n\
  )\n\
}"

print $'\U01F4AB' $YELLOW " Swap with..."
print $'\U01F4AB' $LIGHTGREEN " $wallet_13_public_swap_service"
echo -e "mutation {\n\
  swap (\n\
    token0: \"$erc20_1_appid\",\n\
    token1: \"$erc20_2_appid\",\n\
    amount0In: \"10.\",\n\
    amount1In: \"10.\",\n\
    amount0OutMin: \"1.\",\n\
    amount1OutMin: \"1.\",\n\
  )\n\
}"

print $'\U01F4AB' $YELLOW " Query pools with..."
print $'\U01F4AB' $LIGHTGREEN " $wallet_12_public_swap_service"
print $'\U01F4AB' $LIGHTGREEN " $wallet_13_public_swap_service"
echo -e "query {\n\
  getPools {\n\
    id\n\
    token0\n\
    token1\n\
    poolFeePercent\n\
    price0Cumulative\n\
    price1Cumulative\n\
    amount0Initial\n\
    amount1Initial\n\
    kLast\n\
    blockTimestamp\n\
    protocolFeePercent\n\
    virtualInitialLiquidity\n\
    reserve0\n\
    reserve1\n\
    erc20 {
      balances\n\
      totalSupply\n\
    }\n\
    feeTo\n\
    feeToSetter\n\
  }\n\
}"

trap cleanup INT
read -p "  Press any key to exit"
print $'\U01f499' $LIGHTGREEN " Exit ..."

cleanup

