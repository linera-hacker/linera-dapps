#!/bin/bash

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

  linera -w $1 service --port $local_port --external-signing true --listener-skip-process-inbox
  if [ ! $? -eq 0 ]; then
    echo "Run with official release"
    linera -w $1 service --port $local_port
  fi

  sleep 10
  socat TCP4-LISTEN:$pub_port TCP4:localhost:$local_port &
}

create_wallet 10

print $'\U01F4AB' $YELLOW " Deploying ERC20 application ..."
erc20_bid=`linera --with-wallet 10 publish-bytecode ./target/wasm32-unknown-unknown/release/erc20_{contract,service}.wasm`
print $'\U01f499' $LIGHTGREEN " ERC20 application deployed"
echo -e "    Bytecode ID:    $BLUE$erc20_bid$NC"
echo -e "    Application ID: $BLUE$erc20_appid$NC"

run_service 10 &

sleep 1000000
