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
  testnet-archimedes)
    faucet_url=https://faucet.testnet-archimedes.linera.net
    ;;
  devnet|*)
    faucet_url=https://faucet.devnet-2024-09-04.linera.net
    ;;
esac

case $NETWORK_ID in
  1)
    WALLET_60_PUBLIC_IPORT='210.209.69.38:23303'
    LOCAL_IP='172.21.132.203'
    ;;
  2)
    WALLET_60_PUBLIC_IPORT='172.16.31.73:41160'
    LOCAL_IP='172.16.31.73'
    ;;
  3)
    WALLET_60_PUBLIC_IPORT='localhost:31140'
    LOCAL_IP='localhost'
    ;;
  4)
    WALLET_60_PUBLIC_IPORT='172.16.31.73:31140'
    LOCAL_IP='172.16.31.73'
    ;;
  5)
    WALLET_50_PUBLIC_IPORT='172.16.31.42:31130'
    LOCAL_IP='172.16.31.42'
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
  linera -w $1 service --port $2
}

function run_service () {
  local_port=`expr 31080 + $1`
  pub_port=`expr 41100 + $1`

  __run_service $1 $local_port > $PROJECT_ROOT/service_$local_port.log 2>&1 &

  sleep 3
  socat TCP4-LISTEN:$pub_port TCP4:localhost:$local_port
}

create_wallet 60

wallet_60_default_chain=`linera --with-wallet 60 wallet show | grep "Public Key" | awk '{print $2}'`
wallet_60_owner=`linera --with-wallet 60 wallet show | grep "Owner" | awk '{print $4}'`

####
## Use WLINERA and SWAP application created by deploy-local.sh
####

swap_creation_chain=`grep "SWAP_CREATION_CHAIN" ${PROJECT_ROOT}/.local-defi-materials | awk -F '=' '{print $2}'`
swap_creation_owner=`grep "SWAP_CREATION_OWNER" ${PROJECT_ROOT}/.local-defi-materials | awk -F '=' '{print $2}'`
swap_appid=`grep "SWAP_APPID" ${PROJECT_ROOT}/.local-defi-materials | awk -F '=' '{print $2}'`
swap_workaround_creation_chain_rpc_endpoint=`grep "SWAP_WORKAROUND_CREATION_CHAIN_RPC_ENDPOINT" ${PROJECT_ROOT}/.local-defi-materials | awk -F '=' '{print $2}'`
wlinera_appid=`grep "WLINERA_APPID" ${PROJECT_ROOT}/.local-defi-materials | awk -F '=' '{print $2}'`

print $'\U01f499' $LIGHTGREEN " WLINERA application"
echo -e "    Bytecode ID:    $BLUE$wlinera_bid$NC"
echo -e "    Application ID: $BLUE$wlinera_appid$NC"

print $'\U01f499' $LIGHTGREEN " Swap application"
echo -e "    Bytecode ID:    $BLUE$swap_bid$NC"
echo -e "    Application ID: $BLUE$swap_appid$NC"
echo -e "    Creation chain: $BLUE$swap_creation_chain$NC"
echo -e "    Creation owner: $BLUE$swap_creation_owner$NC"

print $'\U01F4AB' $YELLOW " Deploying my ERC20 application ..."
erc20_1_bid=`linera --with-wallet 60 publish-bytecode ./target/wasm32-unknown-unknown/release/erc20_{contract,service}.wasm`
erc20_1_appid=`linera --with-wallet 60 create-application $erc20_1_bid \
    --json-argument '{"initial_supply":"21000000","name":"Test Linera ERC20 Token","symbol":"TLMYV","decimals":18,"initial_currency":"0.00001","fixed_currency":false,"fee_percent":"0"}' \
    --json-parameters '{"initial_balances":{"{\"chain_id\":\"'$swap_creation_chain'\",\"owner\":\"User:'$swap_creation_owner'\"}":"5000000."},"swap_application_id":"'$swap_appid'", "token_metadata":{"logo":"data:image/png;base64,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","twitter":"https://x.com/mysite-vl","telegram":"https://t.me/mysite-vl","discord":"https://discord.com/invite/mysite-vl","website":"https://mysite-vl.com","github":"https://github.com/mysite-vl","description":"mysite-vl description","mintable":true}}' \
    `
print $'\U01f499' $LIGHTGREEN " ERC20 application TLMYV deployed"
echo -e "    Bytecode ID:    $BLUE$erc20_1_bid$NC"
echo -e "    Application ID: $BLUE$erc20_1_appid$NC"

linera --with-wallet 60 request-application $wlinera_appid
linera --with-wallet 60 request-application $swap_appid

print $'\U01F4AB' $YELLOW " Wait for requestApplication execution..."
sleep 3

function print_apps() {
  print $'\U01F4AB' $YELLOW " $1"
  echo -e "  Default Chain:  $LIGHTGREEN$3$NC"
  echo -e "  Owner:          $LIGHTGREEN$4$NC"
  echo -e "    WLINERA:      $BLUE$2/chains/$3/applications/$wlinera_appid$NC"
  echo -e "    Swap:         $BLUE$2/chains/$3/applications/$swap_appid$NC"
}

HTTP_HOST="http://$WALLET_60_PUBLIC_IPORT"
chain=`linera --with-wallet 60 wallet show | grep "Public Key" | awk '{print $2}'`
owner=`linera --with-wallet 60 wallet show | grep "Owner" | awk '{print $4}'`
print_apps "Wallet 60" $HTTP_HOST $chain $owner

wallet_60_erc20_1_service="http://$LOCAL_IP:31140/chains/$chain/applications/$erc20_1_appid"
wallet_60_wlinera_service="http://$LOCAL_IP:31140/chains/$chain/applications/$wlinera_appid"
wallet_60_swap_service="http://$LOCAL_IP:31140/chains/$chain/applications/$swap_appid"
wallet_60_default_chain=$chain
wallet_60_owner=$owner

wallet_60_public_erc20_1_service="$HTTP_HOST/chains/$chain/applications/$erc20_1_appid"
wallet_60_public_wlinera_service="$HTTP_HOST/chains/$chain/applications/$wlinera_appid"
wallet_60_public_swap_service="$HTTP_HOST/chains/$chain/applications/$swap_appid"

####
## We should
##   1 subscribe to pool creator chain
##   2 authorize balance from wallet 13 default chain to swap pool
## Swap will subscribe to chain directly when it's pool is created
####

run_service 60 &

sleep 5

####
## We should request our application on swap chain firstly and this may be not needed in future
####
print $'\U01F4AB' $YELLOW " Request TLMYV on SWAP creator chain..."
curl -H 'Content-Type: application/json' -X POST \
    -d '{ "query": "mutation { requestApplication(chainId: \"'$swap_creation_chain'\", applicationId: \"'$erc20_1_appid'\", targetChainId: \"'$wallet_60_default_chain'\") }" }' \
    $swap_workaround_creation_chain_rpc_endpoint
echo

print $'\U01F4AB' $YELLOW " Wait for requestApplication execution..."
sleep 3

####
## If we create TLMYV/WLINERA pool in swap later, we don't need to subscribe here
####

print $'\U01F4AB' $YELLOW " Subscribe WLINERA creator chain..."
curl -H 'Content-Type: application/json' -X POST -d '{ "query": "mutation { subscribeCreatorChain }"}' $wallet_60_wlinera_service
echo
print $'\U01F4AB' $YELLOW " Subscribe swap creator chain..."
curl -H 'Content-Type: application/json' -X POST -d '{ "query": "mutation { subscribeCreatorChain }"}' $wallet_60_swap_service
echo

print $'\U01F4AB' $YELLOW " Wait for subscription execution..."
sleep 3

print $'\U01F4AB' $YELLOW " Authorize ERC20 to swap application..."
curl -H 'Content-Type: application/json' -X POST -d "{ \"query\": \"mutation { approve(spender: {chain_id: \\\"$swap_creation_chain\\\", owner:\\\"Application:$swap_appid\\\"},value:\\\"4500000.\\\")}\"}" $wallet_60_erc20_1_service
echo

print $'\U01F4AB' $YELLOW " Wait for authorization..."
sleep 3

print $'\U01F4AB' $YELLOW " Create liquidity pool by ERC20 1 creator..."
curl -H 'Content-Type: application/json' -X POST -d "{ \"query\": \"mutation { createPool(token0: \\\"$erc20_1_appid\\\", token1: \\\"$wlinera_appid\\\", amount0Initial:\\\"5\\\", amount1Initial:\\\"0\\\", amount0Virtual:\\\"5\\\", amount1Virtual:\\\"1\\\")}\"}" $wallet_60_swap_service
echo

print $'\U01F4AB' $YELLOW " Query ERC20 allowance and balance with..."
print $'\U01F4AB' $LIGHTGREEN " $wallet_60_public_erc20_1_service"
print $'\U01F4AB' $LIGHTGREEN " $wallet_60_public_wlinera_service"
echo -e "query {\n\
  allowance(\n\
    owner: {\n\
      chain_id:\"$wallet_60_default_chain\",\n\
      owner:\"User:$wallet_60_owner\"\n\
    },\n\
    spender: {\n\
      chain_id:\"$swap_creation_chain\",\n\
      owner:\"Application:$swap_appid\"\n\
    }\n\
  )\n\
  balanceOf(owner: {\n\
    chain_id:\"$wallet_60_default_chain\",\n\
    owner:\"User:$wallet_60_owner\"\n\
  })\n\
  totalSupply\n\
  name\n\
  symbol\n\
  decimals\n\
}"

print $'\U01F4AB' $YELLOW " Created pool with..."
print $'\U01F4AB' $LIGHTGREEN " $wallet_60_public_swap_service"
echo -e "mutation {\n\
  createPool (\n\
    token0: \"$erc20_1_appid\",\n\
    token1: \"$wlinera_appid\",\n\
    amount0Initial: \"5\",\n\
    amount1Initial: \"1\",\n\
    amount0Virtual: \"5\",\n\
    amount1Virtual: \"1\",\n\
  )\n\
}"

print $'\U01F4AB' $YELLOW " Add liquidity with..."
print $'\U01F4AB' $LIGHTGREEN " $wallet_60_public_swap_service"
echo -e "mutation {\n\
  addLiquidity (\n\
    token0: \"$erc20_1_appid\",\n\
    token1: \"$wlinera_appid\",\n\
    amount0Desired: \"5\",\n\
    amount1Desired: \"1\",\n\
    amount0Min: \"5\",\n\
    amount1Min: \"1\",\n\
    deadline: 0,\n\
  )\n\
}"

print $'\U01F4AB' $YELLOW " Remove liquidity with..."
print $'\U01F4AB' $LIGHTGREEN " $wallet_60_public_swap_service"
echo -e "mutation {\n\
  removeLiquidity (\n\
    token0: \"$erc20_1_appid\",\n\
    token1: \"$wlinera_appid\",\n\
    liquidity: \"2\",\n\
    amount0Min: \"0.2\",\n\
    amount1Min: \"0.2\",\n\
    deadline: 0,\n\
  )\n\
}"

print $'\U01F4AB' $YELLOW " Swap with..."
print $'\U01F4AB' $LIGHTGREEN " $wallet_60_public_swap_service"
echo -e "mutation {\n\
  swap (\n\
    token0: \"$erc20_1_appid\",\n\
    token1: \"$linera_appid\",\n\
    amount0In: \"1.\",\n\
    amount1In: \"1.\",\n\
    amount0OutMin: \"0.01\",\n\
    amount1OutMin: \"0.01\",\n\
  )\n\
}"

print $'\U01F4AB' $YELLOW " Query pools with..."
print $'\U01F4AB' $LIGHTGREEN " $wallet_60_public_swap_service"
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
print $'\U01F4AB' $LIGHTGREEN " $wallet_60_public_wlinera_service"
echo -e "mutation {\n\
  mint(amount: \"1.\")\n\
}"

trap cleanup INT
read -p "  Press any key to exit"
print $'\U01f499' $LIGHTGREEN " Exit ..."

cleanup

