FROM UBUNTU2004_DOCKER_REGISTRY_URL

user root

ARG ALL_PROXY
ARG GONAME
ARG GOTAR
ARG MYSQL_USERNAME
ARG MYSQL_PASSWORD

ARG BRANCH_NAME
ARG ENV_SWAP_CHAIN_ID
ARG ENV_SWAP_APP_ID
ARG ENV_SWAP_SERVER_ADDR

COPY $GOTAR /$GONAME
RUN tar -C /usr -xzf /$GONAME

RUN apt-get update -y
RUN apt-get install git wget make curl jq vim -y
COPY entrypoint.sh /usr/local/bin/entrypoint-inner.sh
RUN echo "#!/bin/sh" > /usr/local/bin/entrypoint.sh
RUN echo "all_proxy=$ALL_PROXY /usr/local/bin/entrypoint-inner.sh" >> /usr/local/bin/entrypoint.sh
RUN chmod +x /usr/local/bin/entrypoint-inner.sh
RUN chmod +x /usr/local/bin/entrypoint.sh

CMD ["/usr/local/bin/entrypoint.sh"]
