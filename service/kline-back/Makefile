REPO_ROOT := "./"

.DEFAULT_GOAL:=help
SHELL:=/usr/bin/env bash

COLOR:=\\033[36m
NOCOLOR:=\\033[0m
GITREPO=$(shell git remote -v | grep fetch | awk '{print $$2}' | sed 's/\.git//g' | sed 's/https:\/\///g')

PROJECTS=  zeus
GO_PROJECTS= zeus

##@ init project
init:
	cp -f .githooks/* .git/hooks

go.mod:
	go mod init ${GITREPO}
	go mod tidy -compat=1.19

deps: 
	all_proxy=${all_proxy} bash -x ${REPO_ROOT}/hack/deps.sh

##@ Verify

.PHONY: add-verify-hook verify verify-build verify-golangci-lint verify-go-mod verify-shellcheck verify-spelling all

add-verify-hook: ## Adds verify scripts to git pre-commit hooks.
# Note: The pre-commit hooks can be bypassed by using the flag --no-verify when
# performing a git commit.
	git config --local core.hooksPath "${REPO_ROOT}/.githooks"

# TODO(lint): Uncomment verify-shellcheck once we finish shellchecking the repo.
verify:  go.mod verify-golangci-lint verify-go-mod #verify-shellcheck ## Runs verification scripts to ensure correct execution
	all_proxy=${all_proxy} bash -x ${REPO_ROOT}/hack/verify.sh

verify-shellcheck: ## Runs shellcheck
	${REPO_ROOT}/hack/verify-shellcheck.sh

gen-ent:
	go install entgo.io/ent/cmd/ent@v0.11.2
	go get entgo.io/ent/cmd/ent@v0.11.2
	go run -mod=mod entgo.io/ent/cmd/ent generate --feature entql,sql/lock,sql/execquery,sql/upsert,privacy,schema/snapshot,sql/modifier ./zeus/pkg/db/ent/schema

ifdef AIMPROJECT
PROJECTS= $(AIMPROJECT)
endif

ifndef DEVELOPMENT
DEVELOPMENT= dev
endif

ifndef DOCKER_REGISTRY
DOCKER_REGISTRY= uhub.service.ucloud.cn
endif

ifndef TAG
TAG= latest
endif

.PHONY: build build-docker release-docker deploy-to-k8s-cluster

build: 
	@for x in $(PROJECTS); do \
		${REPO_ROOT}/$${x}/script/build.sh $(TAG);\
	done

build-docker:
	@for x in $(PROJECTS); do \
		${REPO_ROOT}/$${x}/script/build-docker-image.sh $(TAG) $(DOCKER_REGISTRY);\
	done

release-docker:
	@for x in $(PROJECTS); do \
		${REPO_ROOT}/$${x}/script/release-docker-image.sh $(TAG) $(DOCKER_REGISTRY);\
	done
	
deploy-to-k8s-cluster:
	@for x in $(PROJECTS); do \
		${REPO_ROOT}/$${x}/script/deploy-to-k8s-cluster.sh $(TAG) $(DOCKER_REGISTRY);\
	done

prepare-golang-env:
	all_proxy=${all_proxy} bash -x ${REPO_ROOT}/hack/set-golang-env.sh

prepare-node-env:
	all_proxy=${all_proxy} ${REPO_ROOT}/hack/set-node-env.sh

##@ Tests

.PHONY: go-unit-test go-ut
# TODO:build unit test system
go-unit-test: verify-build 
# go-unit-test: verify-build before-test test-go after-test

# before-test:
# 	@for x in $(GO_PROJECTS); do \
# 		${REPO_ROOT}/$${x}/script/before-test.sh;\
# 	done

# test-go:
# 	@for x in $(GO_PROJECTS); do \
# 		${REPO_ROOT}/$${x}/script/test-go.sh;\
# 	done

# after-test:
# 	@for x in $(GO_PROJECTS); do \
# 		${REPO_ROOT}/$${x}/script/after-test.sh;\
# 	done

test-verbose:
	VERBOSE=1 make test

##@ Helpers

.PHONY: help

help:  ## Display this help
	@awk \
		-v "col=${COLOR}" -v "nocol=${NOCOLOR}" \
		' \
			BEGIN { \
				FS = ":.*##" ; \
				printf "\nUsage:\n  make %s<target>%s\n", col, nocol \
			} \
			/^[a-zA-Z_-]+:.*?##/ { \
				printf "  %s%-15s%s %s\n", col, $$1, nocol, $$2 \
			} \
			/^##@/ { \
				printf "\n%s%s%s\n", col, substr($$0, 5), nocol \
			} \
		' $(MAKEFILE_LIST)
