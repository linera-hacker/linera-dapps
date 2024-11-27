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
    WALLET_10_PUBLIC_IPORT='210.209.69.38:23099'
    WALLET_11_PUBLIC_IPORT='210.209.69.38:23101'
    WALLET_12_PUBLIC_IPORT='210.209.69.38:23103'
    WALLET_13_PUBLIC_IPORT='210.209.69.38:23105'
    WALLET_14_PUBLIC_IPORT='210.209.69.38:23106'
    LOCAL_IP='172.21.132.203'
    ;;
  2)
    WALLET_10_PUBLIC_IPORT='172.16.31.73:40110'
    WALLET_11_PUBLIC_IPORT='172.16.31.73:40111'
    WALLET_12_PUBLIC_IPORT='172.16.31.73:40112'
    WALLET_13_PUBLIC_IPORT='172.16.31.73:40113'
    WALLET_14_PUBLIC_IPORT='172.16.31.73:40114'
    LOCAL_IP='172.16.31.73'
    ;;
  3)
    WALLET_10_PUBLIC_IPORT='localhost:30090'
    WALLET_11_PUBLIC_IPORT='localhost:30091'
    WALLET_12_PUBLIC_IPORT='localhost:30092'
    WALLET_13_PUBLIC_IPORT='localhost:30093'
    WALLET_14_PUBLIC_IPORT='localhost:30094'
    LOCAL_IP='localhost'
    ;;
  4)
    WALLET_10_PUBLIC_IPORT='172.16.31.73:30090'
    WALLET_11_PUBLIC_IPORT='172.16.31.73:30091'
    WALLET_12_PUBLIC_IPORT='172.16.31.73:30092'
    WALLET_13_PUBLIC_IPORT='172.16.31.73:30093'
    WALLET_14_PUBLIC_IPORT='172.16.31.73:30094'
    LOCAL_IP='172.16.31.73'
    ;;
  5)
    WALLET_10_PUBLIC_IPORT='172.16.31.42:30090'
    WALLET_11_PUBLIC_IPORT='172.16.31.42:30091'
    WALLET_12_PUBLIC_IPORT='172.16.31.42:30092'
    WALLET_13_PUBLIC_IPORT='172.16.31.42:30093'
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

  rm -rf $WALLET_BASE/wallet_$1.json $WALLET_BASE/client_$1.db

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
  local_port=`expr 30080 + $1`
  pub_port=`expr 40100 + $1`

  __run_service $1 $local_port > $PROJECT_ROOT/service_$local_port.log 2>&1 &

  sleep 3
  socat TCP4-LISTEN:$pub_port TCP4:localhost:$local_port
}

rm $WALLET_BASE -rf
mkdir $WALLET_BASE -p

create_wallet 10
create_wallet 11
create_wallet 12
create_wallet 13
create_wallet 14

wallet_13_default_chain=`linera --with-wallet 13 wallet show | grep "Public Key" | awk '{print $2}'`
wallet_13_owner=`linera --with-wallet 13 wallet show | grep "Owner" | awk '{print $4}'`

wallet_10_default_chain=`linera --with-wallet 10 wallet show | grep "Public Key" | awk '{print $2}'`
wallet_10_owner=`linera --with-wallet 10 wallet show | grep "Owner" | awk '{print $4}'`

print $'\U01F4AB' $YELLOW " Deploying AMS application ..."
ams_bid=`linera --with-wallet 14 publish-bytecode ./target/wasm32-unknown-unknown/release/ams_{contract,service}.wasm`
ams_appid=`linera --with-wallet 14 create-application $ams_bid \
    --json-argument '{"application_types": ["SWAP", "ERC20", "AMS"]}' \
    `
print $'\U01f499' $LIGHTGREEN " AMS application deployed"
echo -e "    Bytecode ID:    $BLUE$ams_bid$NC"
echo -e "    Application ID: $BLUE$ams_appid$NC"

linera --with-wallet 10 request-application $ams_appid
linera --with-wallet 11 request-application $ams_appid
linera --with-wallet 12 request-application $ams_appid
linera --with-wallet 13 request-application $ams_appid

sleep 3

function run_service_timeout() {
  timeout 3s ./run_service.sh $1
}

run_service_timeout 10 &
run_service_timeout 11 &
run_service_timeout 12 &
run_service_timeout 13 &
run_service_timeout 14 &

sleep 5

####
## Mint ERC20 token and WLINERA token initial liquidity to wallet 14 default chain directly
####

print $'\U01F4AB' $YELLOW " Deploying WTLINERA application ..."
erc20_2_bid=`linera --with-wallet 11 publish-bytecode ./target/wasm32-unknown-unknown/release/erc20_{contract,service}.wasm`
erc20_2_appid=`linera --with-wallet 11 create-application $erc20_2_bid \
    --json-argument '{"initial_supply":"21000000","name":"Wrapper Testnet LINERA Token","symbol":"WTLINERA","decimals":18,"initial_currency":"1","fixed_currency":true,"fee_percent":"0","ams_application_id":"'$ams_appid'"}' \
    --json-parameters '{"initial_balances":{"{\"chain_id\":\"'$wallet_13_default_chain'\",\"owner\":\"User:'$wallet_13_owner'\"}":"5000000.","{\"chain_id\":\"'$wallet_10_default_chain'\",\"owner\":\"User:'$wallet_10_owner'\"}":"5000000."}, "token_metadata":{"logo":"data:image/png;base64,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","twitter":"https://x.com/home2","telegram":"https://t.me/mysite2","discord":"https://discord.com/invite/mysite2","website":"https://mysite2.com","github":"https://github.com/mysite2","description":"mysite2 description","mintable":true}}' \
    `
print $'\U01f499' $LIGHTGREEN " WLINERA application deployed"
echo -e "    Bytecode ID:    $BLUE$erc20_2_bid$NC"
echo -e "    Application ID: $BLUE$erc20_2_appid$NC"

print $'\U01F4AB' $YELLOW " Deploying Swap application ..."
swap_bid=`linera --with-wallet 12 publish-bytecode ./target/wasm32-unknown-unknown/release/swap_{contract,service}.wasm`
swap_appid=`linera --with-wallet 12 create-application $swap_bid \
    --json-parameters '{"wlinera_application_id": "'$erc20_2_appid'","ams_application_id":"'$ams_appid'","logo":"","description":"","application_name":"Linera Swap (CheCko)"}' \
    `
print $'\U01f499' $LIGHTGREEN " Swap application deployed"
echo -e "    Bytecode ID:    $BLUE$swap_bid$NC"
echo -e "    Application ID: $BLUE$swap_appid$NC"

print $'\U01F4AB' $YELLOW " Deploying ERC20 application ..."
erc20_1_bid=`linera --with-wallet 10 publish-bytecode ./target/wasm32-unknown-unknown/release/erc20_{contract,service}.wasm`
erc20_1_appid=`linera --with-wallet 10 create-application $erc20_1_bid \
    --json-argument '{"initial_supply":"21000000","name":"Test Linera ERC20 Token","symbol":"TLA","decimals":18,"initial_currency":"0.00001","fixed_currency":false,"fee_percent":"0","ams_application_id":"'$ams_appid'"}' \
    --json-parameters '{"initial_balances":{"{\"chain_id\":\"'$wallet_13_default_chain'\",\"owner\":\"User:'$wallet_13_owner'\"}":"5000000."},"swap_application_id":"'$swap_appid'", "token_metadata":{"logo":"data:image/png;base64,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","twitter":"https://x.com/mysite","telegram":"https://t.me/mysite","discord":"https://discord.com/invite/mysite","website":"https://mysite.com","github":"https://github.com/mysite","description":"mysite description","mintable":true}}' \
    `
print $'\U01f499' $LIGHTGREEN " ERC20 application deployed"
echo -e "    Bytecode ID:    $BLUE$erc20_1_bid$NC"
echo -e "    Application ID: $BLUE$erc20_1_appid$NC"

linera --with-wallet 12 request-application $erc20_1_appid
linera --with-wallet 12 request-application $erc20_2_appid

linera --with-wallet 10 request-application $swap_appid
linera --with-wallet 10 request-application $erc20_2_appid
linera --with-wallet 10 request-application $swap_appid

linera --with-wallet 13 request-application $erc20_1_appid
linera --with-wallet 13 request-application $erc20_2_appid
linera --with-wallet 13 request-application $swap_appid

print $'\U01F4AB' $YELLOW " Wait for requestApplication execution..."
sleep 3

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

erc20_1_creation_chain=$chain

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

wlinera_creation_chain=$chain

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

echo "SWAP_CREATION_CHAIN=$chain" > $PROJECT_ROOT/.local-defi-materials
echo "SWAP_CREATION_OWNER=$owner" >> $PROJECT_ROOT/.local-defi-materials
echo "SWAP_APPID=$swap_appid" >> $PROJECT_ROOT/.local-defi-materials
echo "SWAP_WORKAROUND_CREATION_CHAIN_RPC_ENDPOINT=http://$LOCAL_IP:30092" >> $PROJECT_ROOT/.local-defi-materials
echo "WLINERA_APPID=$erc20_2_appid" >> $PROJECT_ROOT/.local-defi-materials
echo "AMS_APPID=$ams_appid" >> $PROJECT_ROOT/.local-defi-materials

swap_creation_chain=$chain

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


HTTP_HOST="http://$WALLET_14_PUBLIC_IPORT"
chain=`linera --with-wallet 14 wallet show | grep "Public Key" | awk '{print $2}'`
owner=`linera --with-wallet 14 wallet show | grep "Owner" | awk '{print $4}'`
print_apps "Wallet 14" $HTTP_HOST $chain $owner

wallet_14_public_ams_service="$HTTP_HOST/chains/$chain/applications/$ams_appid"

ams_creation_chain=$chain

####
## We should
##   1 subscribe to pool creator chain
##   2 authorize balance from wallet 13 default chain to swap pool
## Swap will subscribe to chain directly when it's pool is created
####

run_service 10 &
run_service 11 &
run_service 12 &
run_service 13 &
run_service 14 &

sleep 5

print $'\U01F4AB' $YELLOW " Subscribe ERC20 creator chain..."
# curl -H 'Content-Type: application/json' -X POST -d '{ "query": "mutation { subscribeCreatorChain }"}' $wallet_12_erc20_1_service
curl -H 'Content-Type: application/json' -X POST -d '{ "query": "mutation { subscribeCreatorChain }"}' $wallet_13_erc20_1_service
echo
print $'\U01F4AB' $YELLOW " Subscribe WLINERA creator chain..."
curl -H 'Content-Type: application/json' -X POST -d '{ "query": "mutation { subscribeCreatorChain }"}' $wallet_10_erc20_2_service
# curl -H 'Content-Type: application/json' -X POST -d '{ "query": "mutation { subscribeCreatorChain }"}' $wallet_12_erc20_2_service
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

print $'\U01F4AB' $YELLOW " Mint ERC20 1 with..."
print $'\U01F4AB' $LIGHTGREEN " $wallet_13_public_erc20_1_service"
echo -e "mutation {\n\
  mint(amount: \"1.\")\n\
}"

print $'\U01F4AB' $YELLOW " Query Applications with..."
print $'\U01F4AB' $LIGHTGREEN " $wallet_14_public_ams_service"
timestamp_us=$(date +%s%6N)
echo -e "query {\n\
  applications(\n\
    createdBefore: $timestamp_us\n\
    createdAfter: 0\n\
    applicationType: \"ERC20\"\n\
    limit: 5\n\
  )\n\
}"

print $'\U01F4AB' $YELLOW " Query Application with..."
print $'\U01F4AB' $LIGHTGREEN " $wallet_14_public_ams_service"
echo -e "query {\n\
  application(\n\
    applicationId: \"$swap_appid\"\n\
  )\n\
}"

print $'\U01F4AB' $LIGHTGREEN " AMS Application: $ams_appid"
print $'\U01F4AB' $LIGHTGREEN " AMS Creation Chain: $ams_creation_chain"
print $'\U01F4AB' $LIGHTGREEN " Swap Application: $swap_appid"
print $'\U01F4AB' $LIGHTGREEN " Swap Creation Chain: $swap_creation_chain"
print $'\U01F4AB' $LIGHTGREEN " WLINERA Application: $erc20_2_appid"
print $'\U01F4AB' $LIGHTGREEN " WLINERA Creation Chain: $wlinera_creation_chain"
print $'\U01F4AB' $LIGHTGREEN " TLA Application: $erc20_1_appid"
print $'\U01F4AB' $LIGHTGREEN " TLA Creation Chain: $erc20_1_creation_chain"

trap cleanup INT
read -p "  Press any key to exit"
print $'\U01f499' $LIGHTGREEN " Exit ..."

cleanup

