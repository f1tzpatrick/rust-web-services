
.PHONY: backend-dependencies run-backend

backend-dependencies:
	@set -e; \
		cd backend/webservice; \
		cargo build

run-backend: backend-dependencies
	@set -e; \
		ROCKET_CONFIG=./backend/webservice/Rocket.toml \
		ROCKET_PRODUCTS_FILE=./products.json \
		./backend/webservice/target/debug/webservice
