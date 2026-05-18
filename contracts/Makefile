# UpCurrent Smart Contracts Makefile

.PHONY: help build test clean deploy-testnet bindings install-deps

# Default target
help:
	@echo "UpCurrent Smart Contracts - Available Commands:"
	@echo ""
	@echo "  make build          - Build all contracts for release"
	@echo "  make test           - Run all contract tests"
	@echo "  make clean          - Clean build artifacts"
	@echo "  make deploy-testnet - Deploy contracts to Stellar testnet"
	@echo "  make bindings       - Generate TypeScript bindings"
	@echo "  make install-deps   - Install required dependencies"
	@echo "  make check          - Run cargo check"
	@echo "  make fmt            - Format code with rustfmt"
	@echo "  make clippy         - Run clippy linter"

# Install dependencies
install-deps:
	@echo "Installing Rust toolchain and dependencies..."
	rustup target add wasm32-unknown-unknown
	cargo install --locked soroban-cli --features opt

# Build all contracts
build:
	@echo "Building contracts..."
	cargo build --target wasm32-unknown-unknown --release
	@echo "Build complete! WASM files are in target/wasm32-unknown-unknown/release/"

# Run tests
test:
	@echo "Running tests..."
	cargo test

# Clean build artifacts
clean:
	@echo "Cleaning build artifacts..."
	cargo clean

# Check code without building
check:
	@echo "Running cargo check..."
	cargo check --target wasm32-unknown-unknown

# Format code
fmt:
	@echo "Formatting code..."
	cargo fmt --all

# Run clippy
clippy:
	@echo "Running clippy..."
	cargo clippy --target wasm32-unknown-unknown -- -D warnings

# Deploy to testnet (requires STELLAR_ACCOUNT env var)
deploy-testnet:
	@echo "Deploying to Stellar testnet..."
	@if [ -z "$(STELLAR_ACCOUNT)" ]; then \
		echo "Error: STELLAR_ACCOUNT environment variable not set"; \
		echo "Usage: STELLAR_ACCOUNT=test-account make deploy-testnet"; \
		exit 1; \
	fi
	soroban contract deploy \
		--wasm target/wasm32-unknown-unknown/release/upcurrent_escrow.wasm \
		--source $(STELLAR_ACCOUNT) \
		--network testnet

# Generate TypeScript bindings
bindings:
	@echo "Generating TypeScript bindings..."
	@if [ ! -f target/wasm32-unknown-unknown/release/upcurrent_escrow.wasm ]; then \
		echo "Error: Contract not built. Run 'make build' first."; \
		exit 1; \
	fi
	stellar contract bindings typescript \
		--wasm ./target/wasm32-unknown-unknown/release/upcurrent_escrow.wasm \
		--output-dir ../src/contracts/upcurrent
	@echo "Bindings generated in ../src/contracts/upcurrent/"

# Build and test in one command
all: build test
	@echo "Build and test complete!"
