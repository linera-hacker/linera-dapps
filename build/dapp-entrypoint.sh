#!/bin/bash

PROJECT_DIR=/opt/linera-project
cd $PROJECT_DIR

[ ! -d linera-dapps ] && git clone https://github.com/linera-hacker/linera-dapps.git
cd linera-dapps
git pull
sed -i "/read -p*/c sleep 100000000" ./deploy-local.sh
sed -i "/read -p*/c sleep 100000000" ./run-local.sh

sed -i "s/blob_gateway_app_id=.*/blob_gateway_app_id=\"$ENV_BLOB_APP_ID\"/g" ./deploy-local.sh
sed -i "s/blob_gateway_creation_chain_id=.*/blob_gateway_creation_chain_id=\"$ENV_BLOB_CHAIN_ID\"/g" ./deploy-local.sh

sed -i "s/WALLET_10_PUBLIC_IPORT=.*/WALLET_10_PUBLIC_IPORT='linera-dapps-service:30090'/g" ./run-local.sh
sed -i "s/WALLET_11_PUBLIC_IPORT=.*/WALLET_11_PUBLIC_IPORT='linera-dapps-service:30091'/g" ./run-local.sh
sed -i "s/WALLET_12_PUBLIC_IPORT=.*/WALLET_12_PUBLIC_IPORT='linera-dapps-service:30092'/g" ./run-local.sh
sed -i "s/WALLET_13_PUBLIC_IPORT=.*/WALLET_13_PUBLIC_IPORT='linera-dapps-service:30093'/g" ./run-local.sh
sed -i "s/WALLET_14_PUBLIC_IPORT=.*/WALLET_14_PUBLIC_IPORT='linera-dapps-service:30094'/g" ./run-local.sh
sed -i "s/BLOB_GATEWAY_PUBLIC_IPORT=.*/BLOB_GATEWAY_PUBLIC_IPORT='linera-dapps-service:9081'/g" ./run-local.sh
sed -i "s/LOCAL_IP=.*/LOCAL_IP='linera-dapps-service'/g" ./run-local.sh

sed -i "s/WALLET_10_PUBLIC_IPORT=.*/WALLET_10_PUBLIC_IPORT='linera-dapps-service:30090'/g" ./deploy-local.sh
sed -i "s/WALLET_11_PUBLIC_IPORT=.*/WALLET_11_PUBLIC_IPORT='linera-dapps-service:30091'/g" ./deploy-local.sh
sed -i "s/WALLET_12_PUBLIC_IPORT=.*/WALLET_12_PUBLIC_IPORT='linera-dapps-service:30092'/g" ./deploy-local.sh
sed -i "s/WALLET_13_PUBLIC_IPORT=.*/WALLET_13_PUBLIC_IPORT='linera-dapps-service:30093'/g" ./deploy-local.sh
sed -i "s/WALLET_14_PUBLIC_IPORT=.*/WALLET_14_PUBLIC_IPORT='linera-dapps-service:30094'/g" ./deploy-local.sh
sed -i "s/BLOB_GATEWAY_PUBLIC_IPORT=.*/BLOB_GATEWAY_PUBLIC_IPORT='linera-dapps-service:9081'/g" ./deploy-local.sh
sed -i "s/LOCAL_IP=.*/LOCAL_IP='linera-dapps-service'/g" ./deploy-local.sh

source ~/.cargo/env
export PATH=/root/.cargo/bin/:$PATH
if [ -d /root/linera-project/linera/dapps ]; then
  ./run-local.sh -N testnet-archimedes -n 5
else
  ./deploy-local.sh -N testnet-archimedes -n 5 -k 1
fi
