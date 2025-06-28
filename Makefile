.PHONY: build-db-image run-db reset-db run-migrations

ENV_FILE := .env

ifeq ($(TEST),true)
  ENV_FILE := .env.test
endif

include $(ENV_FILE)
export

build-db-image:
	docker build \
		--platform linux/amd64 \
		--build-arg POSTGRES_USER=$(PICASA_POSTGRES_USER) \
		--file ./postgres.Dockerfile \
		--tag picasa-db:latest \
		.

run-db:
	docker run \
		--platform linux/amd64 \
		--detach \
		--name $(PICASA_POSTGRES_DB) \
		--publish $(PICASA_POSTGRES_PORT):5432 \
		--env POSTGRES_DB=$(PICASA_POSTGRES_DB) \
		--env POSTGRES_USER=$(PICASA_POSTGRES_USER) \
		--env POSTGRES_PASSWORD=$(PICASA_POSTGRES_PASSWORD) \
		picasa-db:latest

reset-db:
	docker stop $(PICASA_POSTGRES_DB)
	docker rm $(PICASA_POSTGRES_DB)
	make run-db	

run-migrations:
	diesel migration run \
		--database-url postgres://$(PICASA_POSTGRES_USER):$(PICASA_POSTGRES_PASSWORD)@localhost:$(PICASA_POSTGRES_PORT)/$(PICASA_POSTGRES_DB)
	