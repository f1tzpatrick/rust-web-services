set export

DB_USER := env_var_or_default("DB_USER", "root")
DB_PASSWORD := env_var_or_default("DB_PASSWORD", "password")
DB_HOST := env_var_or_default("DB_HOST", "localhost")
DB_PORT := env_var_or_default("DB_PORT", "27017")

PRODUCT_FILE := "./products.json"

_default:
  @just --list --unsorted

start-db:
    docker-compose -f infra/local-db/docker-compose.yaml up -d

stop-db:
    docker-compose -f infra/local-db/docker-compose.yaml down

check-db:
    cargo run --manifest-path ./api/Cargo.toml --bin check-db

build-api:
    cargo build --manifest-path ./api/Cargo.toml

run-api: build-api
    cargo run --manifest-path ./api/Cargo.toml --bin api
