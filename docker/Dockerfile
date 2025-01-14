FROM jrei/systemd-ubuntu:20.04 as builder

user root
ARG ALL_PROXY

RUN chmod 777 /tmp
RUN apt-get update -y
RUN apt-get install git wget make curl jq vim gcc unzip libclang-dev g++ socat -y

RUN mkdir /opt/linera-project
RUN mkdir /opt/linera-project/protoc

RUN cd /opt/linera-project/protoc \
    && wget https://github.com/protocolbuffers/protobuf/releases/download/v29.1/protoc-29.1-linux-x86_64.zip \
    && unzip protoc-29.1-linux-x86_64.zip

RUN cd /opt/linera-project \
    && curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs > rustup.sh \
    && sh rustup.sh -y

RUN cd /opt/linera-project \
    && git clone https://github.com/respeer-ai/linera-protocol.git

RUN cd /opt/linera-project/linera-protocol \
    && export PATH=$HOME/.cargo/bin:$PATH \
    && export PATH=/opt/linera-project/protoc/bin:$PATH \
    && git checkout respeer-maas-testnet_archimedes-5fcbc190-2024_12_06 \
    && export all_proxy=$ALL_PROXY \
    && CARGO_NET_GIT_FETCH_WITH_CLI=true cargo install --path linera-service --features disable-native-rpc,enable-wallet-rpc,enable-request-application,storage-service \
    && CARGO_NET_GIT_FETCH_WITH_CLI=true cargo install --path linera-storage-service --features storage-service \
    && cargo clean

RUN cd /opt/linera-project \
    && git clone https://github.com/linera-hacker/linera-dapps.git

RUN cd /opt/linera-project/linera-dapps \
    && export PATH=$HOME/.cargo/bin:$PATH \
    && export PATH=/opt/linera-project/protoc/bin:$PATH \
    && export all_proxy=$ALL_PROXY \
    && CARGO_NET_GIT_FETCH_WITH_CLI=true cargo build --release --target wasm32-unknown-unknown

FROM jrei/systemd-ubuntu:20.04 as dapps-service

user root
ARG ALL_PROXY
RUN chmod 777 /tmp
RUN apt-get update -y
RUN apt-get install socat curl -y

RUN mkdir -p /opt/linera-project/linera-dapps
RUN mkdir -p /opt/linera-project/linera-dapps/assets
RUN mkdir -p /opt/linera-project/linera-dapps/target/wasm32-unknown-unknown/release
COPY --from=0 /root/.cargo/bin/linera /usr/local/bin/linera
COPY --from=0 /opt/linera-project/linera-dapps/run-local.sh /opt/linera-project/linera-dapps
COPY --from=0 /opt/linera-project/linera-dapps/deploy-local.sh /opt/linera-project/linera-dapps
COPY --from=0 /opt/linera-project/linera-dapps/assets/HackerLogoDark.png /opt/linera-project/linera-dapps/assets
COPY --from=0 /opt/linera-project/linera-dapps/target/wasm32-unknown-unknown/release/ams_contract.wasm /opt/linera-project/linera-dapps/target/wasm32-unknown-unknown/release
COPY --from=0 /opt/linera-project/linera-dapps/target/wasm32-unknown-unknown/release/ams_service.wasm /opt/linera-project/linera-dapps/target/wasm32-unknown-unknown/release
COPY --from=0 /opt/linera-project/linera-dapps/target/wasm32-unknown-unknown/release/erc20_contract.wasm /opt/linera-project/linera-dapps/target/wasm32-unknown-unknown/release
COPY --from=0 /opt/linera-project/linera-dapps/target/wasm32-unknown-unknown/release/erc20_service.wasm /opt/linera-project/linera-dapps/target/wasm32-unknown-unknown/release
COPY --from=0 /opt/linera-project/linera-dapps/target/wasm32-unknown-unknown/release/swap_contract.wasm /opt/linera-project/linera-dapps/target/wasm32-unknown-unknown/release
COPY --from=0 /opt/linera-project/linera-dapps/target/wasm32-unknown-unknown/release/swap_service.wasm /opt/linera-project/linera-dapps/target/wasm32-unknown-unknown/release

COPY entrypoint.sh /usr/local/bin/entrypoint-inner.sh
RUN echo "#!/bin/sh" > /usr/local/bin/entrypoint.sh
RUN echo "all_proxy=$ALL_PROXY /usr/local/bin/entrypoint-inner.sh" >> /usr/local/bin/entrypoint.sh
RUN chmod +x /usr/local/bin/entrypoint-inner.sh
RUN chmod +x /usr/local/bin/entrypoint.sh

CMD ["/usr/local/bin/entrypoint.sh"]
