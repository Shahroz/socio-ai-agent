# Socio AI Agent Makefile

.PHONY: help install install-frontend backend frontend dev build build-frontend test clean lint format
.PHONY: db-build db-start db-stop db-connect db-reset db-logs db-clean db-url db-migrate sqlx-prepare clean-docker
.PHONY: test-frontend test-backend test-fast generate-api-client generate-swift-sdk
.PHONY: app-run app-run-o11y dump-db-schema kill-8080 kill-8081 kill-8082 kill-8083
.PHONY: docker-build docker-run docker-clean dev-o11y

# Environment & Configuration Variables
ENV_FILE = .env
-include $(ENV_FILE)

POSTGRES_USER ?= localuser
POSTGRES_PASSWORD ?= localpassword
POSTGRES_DB ?= socio_v2
POSTGRES_PORT ?= 5447
DATABASE_URL ?= postgresql://$(POSTGRES_USER):$(POSTGRES_PASSWORD)@localhost:$(POSTGRES_PORT)/$(POSTGRES_DB)

IMAGE_NAME = socio-postgres-db
CONTAINER_NAME = socio-postgres-container
PGDATA_DIR = ./crates/socio/pgdata

# Flag for optional emulator usage in dev/app commands
EMULATOR ?= false

$(shell mkdir -p $(PGDATA_DIR))

# Default target
help:
	@echo "Socio AI Agent - Available commands:"
	@echo ""
	@echo "  Database:"
	@echo "    db-build           - Build custom PostgreSQL Docker image"
	@echo "    db-start           - Start PostgreSQL database"
	@echo "    db-stop            - Stop PostgreSQL database"
	@echo "    db-connect         - Connect to PostgreSQL database"
	@echo "    db-reset           - Reset database (stop, clean, start)"
	@echo "    db-logs            - Show PostgreSQL container logs"
	@echo "    db-clean           - Remove local PostgreSQL data directory"
	@echo "    db-url             - Display database connection URL"
	@echo "    db-migrate         - Run database migrations"
	@echo "    sqlx-prepare       - Prepare SQLx query metadata for offline builds"
	@echo "    clean-docker       - Stop and remove PostgreSQL container and image"
	@echo ""
	@echo "  Development:"
	@echo "    install            - Install all dependencies (Rust + Node.js)"
	@echo "    install-frontend   - Install frontend dependencies only"
	@echo "    backend            - Run the Rust backend server"
	@echo "    frontend           - Run the React frontend development server"
	@echo "    dev                - Run both backend and frontend in development mode"
	@echo "    dev-o11y           - Start development with observability features"
	@echo "    app-run            - Run backend application"
	@echo "    app-run-o11y       - Run backend with observability features"
	@echo "    setup              - Initial project setup"
	@echo ""
	@echo "  Building:"
	@echo "    build              - Build both backend and frontend for production"
	@echo "    build-frontend     - Build the React frontend for production"
	@echo ""
	@echo "  Testing:"
	@echo "    test               - Run all tests (frontend + backend)"
	@echo "    test-fast          - Run fast tests (frontend + backend)"
	@echo "    test-frontend      - Run only frontend tests"
	@echo "    test-backend       - Run only backend tests"
	@echo ""
	@echo "  Code Quality:"
	@echo "    lint               - Run linting for both backend and frontend"
	@echo "    format             - Format code for both backend and frontend"
	@echo ""
	@echo "  API Generation:"
	@echo "    generate-api-client - Generate TypeScript API client from OpenAPI"
	@echo "    generate-swift-sdk  - Generate Swift SDK from OpenAPI"
	@echo "    dump-db-schema     - Dump database schema to backend/db_schema.sql"
	@echo ""
	@echo "  Docker:"
	@echo "    docker-build       - Build Docker image for the main app"
	@echo "    docker-run         - Run Docker container"
	@echo "    docker-clean       - Remove stopped containers from the app image"
	@echo ""
	@echo "  Utilities:"
	@echo "    clean              - Clean build artifacts"
	@echo "    kill-8080          - Kill process using port 8080"
	@echo "    kill-8081          - Kill process using port 8081"
	@echo "    kill-8082          - Kill process using port 8082"
	@echo "    kill-8083          - Kill process using port 8083"
	@echo ""
	@echo "  Configuration Options:"
	@echo "    EMULATOR=true      - Use emulator (for dev/app-run commands)"
	@echo ""
	@echo "  Examples:"
	@echo "    make dev EMULATOR=true"
	@echo "    make test-fast"
	@echo "    make generate-api-client"
	@echo ""

# Install dependencies
install: check-deps
	@echo "Installing Rust dependencies..."
	cargo build
	@echo "Installing Node.js dependencies..."
	cd crates/socio/frontend && npm install

# Install frontend dependencies only
install-frontend:
	@echo "Installing Node.js dependencies..."
	cd crates/socio/frontend && npm install

########################################
# Database Commands
########################################

db-build:
	@echo "Building custom PostgreSQL image '$(IMAGE_NAME)' from DockerfileDB..."
	@docker build -t $(IMAGE_NAME) -f crates/socio/DockerfileDB crates/socio/

db-start:
	@echo "Starting custom PostgreSQL container '$(CONTAINER_NAME)'..."
	@if docker ps --filter "name=$(CONTAINER_NAME)" --filter "status=running" | grep -q $(CONTAINER_NAME); then \
		echo "✅ PostgreSQL container '$(CONTAINER_NAME)' is already running"; \
	else \
		docker run --name $(CONTAINER_NAME) \
			-e POSTGRES_PASSWORD=$(POSTGRES_PASSWORD) \
			-e POSTGRES_USER=$(POSTGRES_USER) \
			-e POSTGRES_DB=$(POSTGRES_DB) \
			-p $(POSTGRES_PORT):5432 \
			-v $(abspath $(PGDATA_DIR)):/var/lib/postgresql/data \
			-d $(IMAGE_NAME) 2>/dev/null || \
		(docker start $(CONTAINER_NAME) 2>/dev/null && echo "✅ Started existing PostgreSQL container '$(CONTAINER_NAME)'") || \
		echo "❌ Failed to start PostgreSQL container"; \
	fi

db-stop:
	@echo "Stopping and removing custom PostgreSQL container '$(CONTAINER_NAME)'..."
	@docker stop $(CONTAINER_NAME) || true
	@docker rm $(CONTAINER_NAME) || true
	@echo "Container stopped and removed."

db-connect:
	@echo "Connecting to database $(POSTGRES_DB) in container $(CONTAINER_NAME)..."
	@docker exec -it $(CONTAINER_NAME) psql -U $(POSTGRES_USER) -d $(POSTGRES_DB)

db-reset: db-stop db-clean db-start
	@echo "Database reset complete (container stopped, data removed, container restarted)."

db-logs:
	@echo "Showing logs for container $(CONTAINER_NAME)..."
	@docker logs -f $(CONTAINER_NAME)

db-clean:
	@echo "Removing local data directory $(PGDATA_DIR)..."
	@rm -rf $(PGDATA_DIR)
	@mkdir -p $(PGDATA_DIR)
	@echo "Local data directory cleaned and recreated."

db-url:
	@echo "Database connection URL:"
	@echo "$(DATABASE_URL)"

db-migrate:
	@echo "Running migrations"
	@cd crates/socio/backend && DATABASE_URL=$(DATABASE_URL) sqlx migrate run

sqlx-prepare:
	@echo "Preparing SQLx query metadata for offline builds..."
	@DATABASE_URL=$(DATABASE_URL) cargo sqlx prepare --workspace
	@echo "✅ SQLx query metadata prepared in workspace .sqlx directory"

clean-docker:
	@echo "Stopping and removing container $(CONTAINER_NAME)..."
	@docker stop $(CONTAINER_NAME) || true
	@docker rm $(CONTAINER_NAME) || true
	@echo "Removing image $(IMAGE_NAME)..."
	@docker rmi $(IMAGE_NAME) || true
	@echo "Docker cleanup finished."

########################################
# Development Commands
########################################

# Run backend
backend:
	@echo "Starting Actix Web backend server..."
	cargo run --bin socio

# Run frontend
frontend:
	@echo "Starting React frontend development server..."
	cd crates/socio/frontend && npm run dev

# Set emulator environment if EMULATOR=true flag is provided
EMULATOR_ENV =
BASE_ENV = DATABASE_URL=$(DATABASE_URL) APP_ENV=dev
ifeq ($(EMULATOR),true)
	EMULATOR_ENV = $(BASE_ENV)
	EMULATOR_DEPS = 
else
	EMULATOR_ENV = $(BASE_ENV)
	EMULATOR_DEPS = 
endif

app-run: $(EMULATOR_DEPS)
	@echo "Running app..."
	@$(MAKE) kill-8080
	cd crates/socio/backend && env $(EMULATOR_ENV) cargo run

app-run-o11y: $(EMULATOR_DEPS)
	@echo "Running app with observability..."
	@$(MAKE) kill-8080
	cd crates/socio/backend && env $(EMULATOR_ENV) cargo run --features newrelic

# Run both in development mode
dev:
	@echo "Starting development environment..."
	@echo "Backend will run on http://localhost:3000"
	@echo "Frontend will run on http://localhost:5173"
	@echo ""
	@echo "Starting backend in background..."
	cargo run --bin socio &
	@echo "Starting frontend..."
	cd crates/socio/frontend && npm run dev

dev-o11y:
	@echo "Starting development environment with observability..."
	@echo "Backend will run on http://localhost:3000"
	@echo "Frontend will run on http://localhost:5173"
	@echo ""
	@echo "Starting backend in background..."
	cargo run --bin socio --features newrelic &
	@echo "Starting frontend..."
	cd crates/socio/frontend && npm run dev

# Build for frontend production
build-frontend:
	@echo "Building frontend..."
	cd crates/socio/frontend && npm run build

# Build for production
build:
	@echo "Building backend..."
	cargo build --release
	@echo "Building frontend..."
	cd crates/socio/frontend && npm run build

########################################
# Testing Commands
########################################

# Run all tests: frontend and backend
test: test-frontend test-backend

# Fast test that skips slow tests
test-fast: test-frontend test-backend

test-frontend:
	@echo "Running frontend tests..."
	cd crates/socio/frontend && npm install --legacy-peer-deps && npm run test:run -- --exclude="tests/**/*" && npm run build

test-backend:
	@echo "Running backend clippy check..."
	cd crates/socio/backend && env DATABASE_URL=$(DATABASE_URL) APP_ENV=test cargo clippy -- -D clippy::disallowed-methods
	@echo "Running backend tests..."
	cd crates/socio/backend && env DATABASE_URL=$(DATABASE_URL) APP_ENV=test cargo test

# Lint code
lint:
	@echo "Linting Rust code..."
	cargo clippy
	@echo "Linting frontend code..."
	cd crates/socio/frontend && npm run lint

# Format code
format:
	@echo "Formatting Rust code..."
	cargo fmt
	@echo "Formatting frontend code..."
	cd crates/socio/frontend && npm run format || echo "No format script found"

# Clean build artifacts
clean:
	@echo "Cleaning Rust build artifacts..."
	cargo clean
	@echo "Cleaning frontend build artifacts..."
	cd crates/socio/frontend && rm -rf node_modules dist

# Initial setup
setup: install
	@echo "Setting up environment..."
	@if [ ! -f .env ]; then \
		cp env.example .env; \
		echo "Created .env file from env.example"; \
		echo "Please edit .env and add your Gemini API key"; \
	else \
		echo ".env file already exists"; \
	fi
	@echo ""
	@echo "Setup complete! Next steps:"
	@echo "1. Edit .env file and add your Gemini API key"
	@echo "2. Run 'make dev' to start development servers"
	@echo "3. Open http://localhost:5173 in your browser"

# Generate API types
generate-api:
	@echo "Generating API types from OpenAPI spec..."
	cd crates/socio/frontend && npm run generate-api

########################################
# API Generation Commands
########################################

dump-db-schema:
	@echo "Dumping database schema to crates/socio/backend/db_schema.sql..."
	@mkdir -p crates/socio/backend
	@if docker ps --filter "name=$(CONTAINER_NAME)" --filter "status=running" | grep -q $(CONTAINER_NAME); then \
		docker exec -e PGPASSWORD=$(POSTGRES_PASSWORD) $(CONTAINER_NAME) pg_dump --schema-only -U $(POSTGRES_USER) -d $(POSTGRES_DB) > crates/socio/backend/db_schema.sql && \
		echo "✅ Database schema dump complete."; \
	else \
		echo "❌ Database container '$(CONTAINER_NAME)' is not running. Please run 'make db-start' first."; \
		exit 1; \
	fi

generate-api-client: $(EMULATOR_DEPS)
	@$(MAKE) kill-8081
	@echo "Attempting to generate API client..."
	@PORT_TO_TRY="" && \
	for port in 8081 8082 8083; do \
		echo "Checking port $$port..."; \
		if ! lsof -i:$$port > /dev/null; then \
			echo "Port $$port is available."; \
			PORT_TO_TRY=$$port; \
			break; \
		else \
			echo "Port $$port is in use."; \
		fi; \
	done; \
	if [ -z "$$PORT_TO_TRY" ]; then \
		echo "Error: Could not find an available port (tried 8081-8083)."; \
		exit 1; \
	fi; \
	echo "Starting backend server on port $$PORT_TO_TRY in background..."; \
	cd crates/socio/backend && env $(EMULATOR_ENV) PORT=$$PORT_TO_TRY cargo run & SERVER_PID=$$!; \
	echo "Backend server started (PID: $$SERVER_PID). Waiting for /health endpoint..."; \
	MAX_WAIT=60; \
	WAIT_COUNT=0; \
	HEALTH_URL="http://127.0.0.1:$$PORT_TO_TRY/health"; \
	while ! curl --output /dev/null --silent --fail $$HEALTH_URL; do \
		if [ $$WAIT_COUNT -ge $$MAX_WAIT ]; then \
			echo "Error: Timeout waiting for backend at $$HEALTH_URL."; \
			echo "Stopping backend server (PID: $$SERVER_PID)..."; \
			kill $$SERVER_PID || true; \
			exit 1; \
		fi; \
		printf "."; \
		sleep 1; \
		WAIT_COUNT=$$(($$WAIT_COUNT + 1)); \
	done; \
	echo "\nBackend is healthy. Generating frontend API client..."; \
	OPENAPI_URL="http://127.0.0.1:$$PORT_TO_TRY/api-docs/openapi.json"; \
	GENERATION_SUCCESS=false; \
	if cd crates/socio/frontend && \
	   echo "Cleaning existing API client files..." && \
	   rm -rf src/api/* && \
	   echo "Generating full TypeScript client from $$OPENAPI_URL..." && \
	   npx openapi-typescript-codegen --input $$OPENAPI_URL --output src/api --client fetch --useOptions; then \
		echo "API client generation successful."; \
		GENERATION_SUCCESS=true; \
	else \
		echo "API client generation failed."; \
	fi; \
	echo "Stopping backend server (PID: $$SERVER_PID)..."; \
	kill $$SERVER_PID || true; \
	echo "Backend server stopped."; \
	if [ "$$GENERATION_SUCCESS" = false ]; then \
		exit 1; \
	fi; \
	echo "Done."

generate-swift-sdk: $(EMULATOR_DEPS)
	@$(MAKE) kill-8081
	@echo "Attempting to generate Swift SDK..."
	@PORT_TO_TRY="" && \
	for port in 8081 8082 8083; do \
		echo "Checking port $$port..."; \
		if ! lsof -i:$$port > /dev/null; then \
			echo "Port $$port is available."; \
			PORT_TO_TRY=$$port; \
			break; \
		else \
			echo "Port $$port is in use."; \
		fi; \
	done; \
	if [ -z "$$PORT_TO_TRY" ]; then \
		echo "Error: Could not find an available port (tried 8081-8083)."; \
		exit 1; \
	fi; \
	echo "Starting backend server on port $$PORT_TO_TRY in background..."; \
	cd crates/socio/backend && env $(EMULATOR_ENV) PORT=$$PORT_TO_TRY cargo run & SERVER_PID=$$!; \
	echo "Backend server started (PID: $$SERVER_PID). Waiting for /health endpoint..."; \
	MAX_WAIT=60; \
	WAIT_COUNT=0; \
	HEALTH_URL="http://127.0.0.1:$$PORT_TO_TRY/health"; \
	while ! curl --output /dev/null --silent --fail $$HEALTH_URL; do \
		if [ $$WAIT_COUNT -ge $$MAX_WAIT ]; then \
			echo "Error: Timeout waiting for backend at $$HEALTH_URL."; \
			echo "Stopping backend server (PID: $$SERVER_PID)..."; \
			kill $$SERVER_PID || true; \
			exit 1; \
		fi; \
		printf "."; \
		sleep 1; \
		WAIT_COUNT=$$(($$WAIT_COUNT + 1)); \
	done; \
	echo "\nBackend is healthy. Generating Swift SDK..."; \
	OPENAPI_URL="http://127.0.0.1:$$PORT_TO_TRY/api-docs/openapi.json"; \
	GENERATION_SUCCESS=false; \
	echo "Creating mobile/ios/SDK directory if it doesn't exist..."; \
	mkdir -p mobile/ios/SDK; \
	if echo "Downloading OpenAPI spec..." && \
	   curl -s $$OPENAPI_URL -o mobile/ios/openapi.json && \
	   echo "Cleaning existing Swift SDK files..." && \
	   rm -rf mobile/ios/SDK/* && \
	   echo "Generating Swift SDK from downloaded spec..." && \
	   docker run --rm -v "$(PWD):/local" openapitools/openapi-generator-cli generate \
	     -i /local/mobile/ios/openapi.json \
	     -g swift5 \
	     -o /local/mobile/ios/SDK \
	     --additional-properties=projectName=SocioSDK,podVersion=1.0.0,podSummary="Socio AI Agent API SDK for iOS",swiftUseApiNamespace=true,useSPMFileStructure=true,generateEnums=true,enumNameSuffix=,identifiable=false && \
	   rm mobile/ios/openapi.json && \
	   echo "Applying patches to generated SDK..." && \
	   cd mobile/ios/SDK && \
	   for patch in ../patches/*.patch; do \
	     if [ -f "$$patch" ]; then \
	       echo "Applying patch: $$patch"; \
	       git apply "$$patch" || echo "Warning: Failed to apply patch $$patch"; \
	     fi; \
	   done && \
	   cd ../../..; then \
		echo "Swift SDK generation and patching successful."; \
		GENERATION_SUCCESS=true; \
	else \
		echo "Swift SDK generation or patching failed."; \
	fi; \
	echo "Stopping backend server (PID: $$SERVER_PID)..."; \
	kill $$SERVER_PID || true; \
	echo "Backend server stopped."; \
	if [ "$$GENERATION_SUCCESS" = false ]; then \
		exit 1; \
	fi; \
	echo "Done."

########################################
# Utility Commands
########################################

kill-8080:
	@echo "Killing process on port 8080..."
	@PID=$$(lsof -ti tcp:8080); \
	if [ ! -z "$$PID" ]; then \
		kill -9 $$PID; \
		echo "Killed process on port 8080."; \
	else \
		echo "No process is using port 8080."; \
	fi

kill-8081:
	@echo "Killing process on port 8081..."
	@PID=$$(lsof -ti tcp:8081); \
	if [ ! -z "$$PID" ]; then \
		kill -9 $$PID; \
		echo "Killed process on port 8081."; \
	else \
		echo "No process is using port 8081."; \
	fi

kill-8082:
	@echo "Killing process on port 8082..."
	@PID=$$(lsof -ti tcp:8082); \
	if [ ! -z "$$PID" ]; then \
		kill -9 $$PID; \
		echo "Killed process on port 8082."; \
	else \
		echo "No process is using port 8082."; \
	fi

kill-8083:
	@echo "Killing process on port 8083..."
	@PID=$$(lsof -ti tcp:8083); \
	if [ ! -z "$$PID" ]; then \
		kill -9 $$PID; \
		echo "Killed process on port 8083."; \
	else \
		echo "No process is using port 8083."; \
	fi

########################################
# Docker Commands
########################################

# Build the Docker image for the main app
docker-build:
	docker build -t socio-ai-agent .

# Run the Docker container mapping port 3000
docker-run:
	docker run --env-file .env -p 3000:3000 socio-ai-agent

# Optionally remove stopped containers from this image
docker-clean:
	-docker rm $$(docker ps -a -q --filter "ancestor=socio-ai-agent") || true

# Development helpers
watch-backend:
	@echo "Watching backend for changes..."
	cargo watch -x run

watch-frontend:
	@echo "Watching frontend for changes..."
	cd crates/socio/frontend && npm run dev

# Check if required tools are installed
check-deps:
	@echo "Checking dependencies..."
	@command -v cargo >/dev/null 2>&1 || { echo "Rust/Cargo not found. Please install Rust."; exit 1; }
	@command -v node >/dev/null 2>&1 || { echo "Node.js not found. Please install Node.js."; exit 1; }
	@command -v npm >/dev/null 2>&1 || { echo "npm not found. Please install npm."; exit 1; }
	@command -v docker >/dev/null 2>&1 || { echo "Docker not found. Please install Docker."; exit 1; }
	@echo "All dependencies found!"
