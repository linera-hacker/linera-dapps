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

PROJECT_ROOT=$HOME/linera-project
mkdir -p $PROJECT_ROOT

WALLET_BASE=$PROJECT_ROOT/linera/dapps
mkdir -p $WALLET_BASE
rm $WALLET_BASE/* -rf

options="N:n:"
NETWORK_ID=1
NETWORK_TYPE=devnet

while getopts $options opt; do
  case ${opt} in
    N) NETWORK_TYPE=${OPTARG} ;;
    n) NETWORK_ID=${OPTARG} ;;
  esac
done

case $NETWORK_TYPE in
  localnet)
    faucet_url=http://localhost:40080
    ;;
  devnet|*)
    faucet_url=https://faucet.devnet-2024-09-04.linera.net
    ;;
esac

case $NETWORK_ID in
  1)
    WALLET_80_PUBLIC_IPORT='210.209.69.38:23305'
    LOCAL_IP='172.21.132.203'
    ;;
  2)
    WALLET_80_PUBLIC_IPORT='172.16.31.73:41180'
    LOCAL_IP='172.16.31.73'
    ;;
  3)
    WALLET_80_PUBLIC_IPORT='localhost:31160'
    LOCAL_IP='localhost'
    ;;
  4)
    WALLET_80_PUBLIC_IPORT='172.16.31.73:31160'
    LOCAL_IP='172.16.31.73'
    ;;
esac

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

  linera -w $1 wallet init --faucet $faucet_url --with-new-chain
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
  local_port=`expr 31080 + $1`
  pub_port=`expr 41100 + $1`

  __run_service $1 $local_port > $PROJECT_ROOT/service_$local_port.log 2>&1 &

  sleep 3
  socat TCP4-LISTEN:$pub_port TCP4:localhost:$local_port
}

create_wallet 80

wallet_80_default_chain=`linera --with-wallet 80 wallet show | grep "Public Key" | awk '{print $2}'`
wallet_80_owner=`linera --with-wallet 80 wallet show | grep "Owner" | awk '{print $4}'`

####
## Use WLINERA and SWAP application created by deploy-local.sh
####

swap_creation_chain=`grep "SWAP_CREATION_CHAIN" ${PROJECT_ROOT}/.local-defi-materials | awk -F '=' '{print $2}'`
swap_creation_owner=`grep "SWAP_CREATION_OWNER" ${PROJECT_ROOT}/.local-defi-materials | awk -F '=' '{print $2}'`
swap_appid=`grep "SWAP_APPID" ${PROJECT_ROOT}/.local-defi-materials | awk -F '=' '{print $2}'`
wlinera_appid=`grep "WLINERA_APPID" ${PROJECT_ROOT}/.local-defi-materials | awk -F '=' '{print $2}'`
tlmy_appid=`grep "ERC20_TLMY_APPID" ${PROJECT_ROOT}/.local-defi-materials | awk -F '=' '{print $2}'`

print $'\U01f499' $LIGHTGREEN " WLINERA application"
echo -e "    Application ID: $BLUE$wlinera_appid$NC"

print $'\U01f499' $LIGHTGREEN " TLMY application"
echo -e "    Application ID: $BLUE$tlmy_appid$NC"

print $'\U01f499' $LIGHTGREEN " Swap application"
echo -e "    Application ID: $BLUE$swap_appid$NC"

linera --with-wallet 80 request-application $swap_appid
linera --with-wallet 80 request-application $wlinera_appid
linera --with-wallet 80 request-application $tlmy_appid

print $'\U01F4AB' $YELLOW " Wait for requestApplication execution..."
sleep 3

function print_apps() {
  print $'\U01F4AB' $YELLOW " $1"
  echo -e "  Default Chain:  $LIGHTGREEN$3$NC"
  echo -e "  Owner:          $LIGHTGREEN$4$NC"
  echo -e "    Swap:         $BLUE$2/chains/$3/applications/$swap_appid$NC"
  echo -e "    WLINERA:      $BLUE$2/chains/$3/applications/$wlinera_appid$NC"
  echo -e "    TLMY:         $BLUE$2/chains/$3/applications/$tlmy_appid$NC"
}

HTTP_HOST="http://$WALLET_80_PUBLIC_IPORT"
chain=`linera --with-wallet 80 wallet show | grep "Public Key" | awk '{print $2}'`
owner=`linera --with-wallet 80 wallet show | grep "Owner" | awk '{print $4}'`
print_apps "Wallet 80" $HTTP_HOST $chain $owner

wallet_80_tlmy_service="http://$LOCAL_IP:31160/chains/$chain/applications/$tlmy_appid"
wallet_80_wlinera_service="http://$LOCAL_IP:31160/chains/$chain/applications/$wlinera_appid"
wallet_80_swap_service="http://$LOCAL_IP:31160/chains/$chain/applications/$swap_appid"
wallet_80_default_chain=$chain
wallet_80_owner=$owner

wallet_80_public_tlmy_service="$HTTP_HOST/chains/$chain/applications/$tlmy_appid"
wallet_80_public_wlinera_service="$HTTP_HOST/chains/$chain/applications/$wlinera_appid"
wallet_80_public_swap_service="$HTTP_HOST/chains/$chain/applications/$swap_appid"

####
## We should
##   1 subscribe to pool creator chain
##   2 subscribe to wlinera creator chain
##   3 subscribe to tlmy creator chain
##   4 mint wlinera and tlmy
##   5 swap
####

run_service 80 &

sleep 5

####
## If we create TLMY/WLINERA pool in swap later, we don't need to subscribe here
####

print $'\U01F4AB' $YELLOW " Subscribe TLMY creator chain..."
curl -H 'Content-Type: application/json' -X POST -d '{ "query": "mutation { subscribeCreatorChain }"}' $wallet_80_tlmy_service
echo
print $'\U01F4AB' $YELLOW " Subscribe WLINERA creator chain..."
curl -H 'Content-Type: application/json' -X POST -d '{ "query": "mutation { subscribeCreatorChain }"}' $wallet_80_wlinera_service
echo
print $'\U01F4AB' $YELLOW " Subscribe swap creator chain..."
curl -H 'Content-Type: application/json' -X POST -d '{ "query": "mutation { subscribeCreatorChain }"}' $wallet_80_swap_service
echo
print $'\U01F4AB' $YELLOW " Wait for subscription execution..."
sleep 3
print $'\U01F4AB' $YELLOW " Mint WLINERA..."
curl -H 'Content-Type: application/json' -X POST -d '{ "query": "mutation { mint(amount: \"2.2318\") }"}' $wallet_80_wlinera_service
echo
print $'\U01F4AB' $YELLOW " Mint TLMY..."
curl -H 'Content-Type: application/json' -X POST -d '{ "query": "mutation { mint(amount: \"3.2789\") }"}' $wallet_80_tlmy_service
echo

print $'\U01F4AB' $YELLOW " Query ERC20 allowance and balance with..."
print $'\U01F4AB' $LIGHTGREEN " $wallet_80_public_tlmy_service"
print $'\U01F4AB' $LIGHTGREEN " $wallet_80_public_wlinera_service"
echo -e "query {\n\
  allowance(\n\
    owner: {\n\
      chain_id:\"$wallet_80_default_chain\",\n\
      owner:\"User:$wallet_80_owner\"\n\
    },\n\
    spender: {\n\
      chain_id:\"$swap_creation_chain\",\n\
      owner:\"Application:$swap_appid\"\n\
    }\n\
  )\n\
  balanceOf(owner: {\n\
    chain_id:\"$wallet_80_default_chain\",\n\
    owner:\"User:$wallet_80_owner\"\n\
  })\n\
  totalSupply\n\
  name\n\
  symbol\n\
  decimals\n\
}"

print $'\U01F4AB' $YELLOW " Created pool with..."
print $'\U01F4AB' $LIGHTGREEN " $wallet_80_public_swap_service"
echo -e "mutation {\n\
  createPool (\n\
    token0: \"$tlmy_appid\",\n\
    token1: \"$wlinera_appid\",\n\
    amount0Initial: \"5\",\n\
    amount1Initial: \"1\",\n\
    amount0Virtual: \"5\",\n\
    amount1Virtual: \"1\",\n\
  )\n\
}"

print $'\U01F4AB' $YELLOW " Add liquidity with..."
print $'\U01F4AB' $LIGHTGREEN " $wallet_80_public_swap_service"
echo -e "mutation {\n\
  addLiquidity (\n\
    token0: \"$tlmy_appid\",\n\
    token1: \"$wlinera_appid\",\n\
    amount0Desired: \"5\",\n\
    amount1Desired: \"1\",\n\
    amount0Min: \"5\",\n\
    amount1Min: \"1\",\n\
    deadline: 0,\n\
  )\n\
}"

print $'\U01F4AB' $YELLOW " Remove liquidity with..."
print $'\U01F4AB' $LIGHTGREEN " $wallet_80_public_swap_service"
echo -e "mutation {\n\
  removeLiquidity (\n\
    token0: \"$tlmy_appid\",\n\
    token1: \"$wlinera_appid\",\n\
    liquidity: \"2\",\n\
    amount0Min: \"0.2\",\n\
    amount1Min: \"0.2\",\n\
    deadline: 0,\n\
  )\n\
}"

print $'\U01F4AB' $YELLOW " Swap with..."
print $'\U01F4AB' $LIGHTGREEN " $wallet_80_public_swap_service"
echo -e "mutation {\n\
  swap (\n\
    token0: \"$tlmy_appid\",\n\
    token1: \"$wlinera_appid\",\n\
    amount0In: \"1.\",\n\
    amount1In: \"1.\",\n\
    amount0OutMin: \"0.01\",\n\
    amount1OutMin: \"0.01\",\n\
  )\n\
}"

print $'\U01F4AB' $YELLOW " Query pools with..."
print $'\U01F4AB' $LIGHTGREEN " $wallet_80_public_swap_service"
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

print $'\U01F4AB' $YELLOW " Mint WLINERA with..."
print $'\U01F4AB' $LIGHTGREEN " $wallet_80_public_wlinera_service"
echo -e "mutation {\n\
  mint(amount: \"1.\")\n\
}"

print $'\U01F4AB' $YELLOW " Mint TLMY with..."
print $'\U01F4AB' $LIGHTGREEN " $wallet_80_public_tlmy_service"
echo -e "mutation {\n\
  mint(amount: \"1.\")\n\
}"

print $'\U01F4AB' $YELLOW " Query TLMY/WLINERA price with..."
print $'\U01F4AB' $LIGHTGREEN " $wallet_80_public_swap_service"
echo -e "query {\n\
  calculateSwapAmount(\n\
    token0: \"$tlmy_appid\",\n\
    token1: \"$wlinera_appid\",\n\
    amount1: \"1\"\n\
  )\n\
}"

trap cleanup INT
read -p "  Press any key to exit"
print $'\U01f499' $LIGHTGREEN " Exit ..."

cleanup

