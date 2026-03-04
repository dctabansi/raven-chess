# Build and run the backend
dev: build
    cd backend && uv run backend

# Build the engine and sync dependencies
build:
    cd backend && uv sync
    cd backend && uv run maturin develop

# Build with release optimizations
release:
    cd backend && uv sync
    cd backend && uv run maturin develop --release

# Build the engine CLI binary
cli:
    cd engine && cargo build

# Run the engine CLI binary
run-cli: cli
    cd engine && cargo run --bin engine-cli

# Run the standard Rust linter
lint:
    cd engine && cargo clippy --all-targets --all-features

# Run the Rust linter with extra-strict, highly opinionated rules
lint-pedantic:
    cd engine && cargo clippy --all-targets --all-features -- -W clippy::pedantic

# Run all tests (eventually add python tests)
test:
    cd engine && cargo test --workspace

# Clean all build artifacts
clean:
    cd engine && cargo clean
    -rm -rf backend/.venv

# Remove lock files
clean-locks: clean
    rm backend/uv.lock engine/Cargo.lock
