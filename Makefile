
.PHONY: backend-dependencies run-backend

backend-dependencies:
	@set -e; \
		cd backend/webservice; \
		cargo build

run-backend:
	@set -e; \
		cd backend/webservice; \
		cargo run
