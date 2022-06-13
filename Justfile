set export

DB_USER := env_var_or_default("DB_USER", "root")
DB_PASSWORD := env_var_or_default("DB_PASSWORD", "password")
DB_HOST := env_var_or_default("DB_HOST", "localhost")
DB_PORT := env_var_or_default("DB_PORT", "27017")
FUNCTIONS_CUSTOMHANDLER_PORT := env_var_or_default("FUNCTION_PORT", "14142")

PRODUCT_FILE := "./products.json"

_default:
  @just --list --unsorted

start-db:
    docker-compose -f infra/local-db/docker-compose.yaml up -d

stop-db:
    docker-compose -f infra/local-db/docker-compose.yaml down

check-db:
    cargo run --manifest-path ./api/Cargo.toml --bin check-db

reset-db:
    sudo rm -rf infra/local-db/database-data

init-db: reset-db stop-db start-db check-db
    cargo run --manifest-path api/Cargo.toml --bin init-db

build-api:
    cargo build --manifest-path ./api/Cargo.toml

run-api: build-api
    cargo run --manifest-path ./api/Cargo.toml --bin api

watch-api:
    cargo watch -C ./api/ -x 'run --bin api'

build-function:
    cargo build --manifest-path api/Cargo.toml --bin api
    cp ./api/target/x86_64-unknown-linux-musl/release/api ./products-api/api

build-function-prod:
    cargo build --manifest-path api/Cargo.toml --bin api --release --target=x86_64-unknown-linux-musl
    cp ./api/target/x86_64-unknown-linux-musl/release/api ./products-api/api

start-function: build-function
    func start --port {{ FUNCTIONS_CUSTOMHANDLER_PORT }} --custom