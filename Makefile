ifneq (,$(wildcard ./.env))
    include .env
    export
endif

COMPOSE := docker compose

## up: Start all containers.
.PHONY: up
up:
	@$(COMPOSE) up --build

## down: Stop all containers.
.PHONY: down
down:
	@$(COMPOSE) down

## help: Display all targets.
.PHONY: help
help:
	@echo ''
	@echo 'Usage:'
	@echo '  make [target]'
	@echo ''
	@echo 'Targets:'
	@sed -n 's/^##//p' ${MAKEFILE_LIST} | column -t -s ':' |  sed -e 's/^/ /'

.DEFAULT_GOAL := help
