# Socio AI Agent Makefile

.PHONY: help install backend frontend dev build build-frontend test clean lint format

# Default target
help:
	@echo "Socio AI Agent - Available commands:"
	@echo ""
	@echo "  install     - Install all dependencies (Rust + Node.js)"
	@echo "  backend     - Run the Rust backend server"
	@echo "  frontend    - Run the React frontend development server"
	@echo "  dev         - Run both backend and frontend in development mode"
	@echo "  build       - Build both backend and frontend for production"
	@echo "  build-frontend - Build the React frontend for production"
	@echo "  test        - Run all tests"
	@echo "  lint        - Run linting for both backend and frontend"
	@echo "  format      - Format code for both backend and frontend"
	@echo "  clean       - Clean build artifacts"
	@echo "  setup       - Initial project setup"
	@echo ""

# Install dependencies
install:
	@echo "Installing Rust dependencies..."
	cargo build
	@echo "Installing Node.js dependencies..."
	cd crates/socio/frontend && npm install

# Run backend
backend:
	@echo "Starting Actix Web backend server..."
	cargo run --bin socio

# Run frontend
frontend:
	@echo "Starting React frontend development server..."
	cd crates/socio/frontend && npm run dev

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

# Run tests
test:
	@echo "Running Rust tests..."
	cargo test
	@echo "Running frontend tests..."
	cd crates/socio/frontend && npm test

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

# Docker commands (for future use)
docker-build:
	@echo "Building Docker image..."
	docker build -t socio-ai-agent .

docker-run:
	@echo "Running Docker container..."
	docker run -p 3000:3000 -p 5173:5173 socio-ai-agent

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
	@echo "All dependencies found!"
