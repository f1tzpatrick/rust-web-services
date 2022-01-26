
.PHONY: backend-dependencies run-backend

backend-dependencies:
	@set -e; \
		cd backend/warp-webservice; \
		cargo build

run-backend: backend-dependencies
	@set -e; \
		PRODUCT_FILE=./products.json \
		./backend/warp-webservice/target/debug/warp-webservice
