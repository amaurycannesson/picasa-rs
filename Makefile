.PHONY: build-db-image run-db

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
