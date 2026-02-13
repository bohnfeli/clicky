.PHONY: help build test lint fmt check clean docs install audit release

help:
	@echo "Available commands:"
	@echo "  make build       - Build the project"
	@echo "  make test        - Run tests"
	@echo "  make lint        - Run clippy linter"
	@echo "  make fmt         - Format code"
	@echo "  make check       - Run all quality gates"
	@echo "  make clean       - Clean build artifacts"
	@echo "  make docs        - Build documentation"
	@echo "  make install     - Install required tools"
	@echo "  make audit       - Run security audit"
	@echo "  make release     - Build release version"

build:
	cargo build --all-features

test:
	cargo test --all-features

lint:
	cargo clippy --all-features -- -D warnings

fmt:
	cargo fmt -- --check

fmt-fix:
	cargo fmt

check: build test lint fmt audit docs
	@echo "All quality gates passed!"

clean:
	cargo clean

docs:
	cargo doc --no-deps --all-features

install:
	cargo install cargo-audit

audit:
	cargo audit

release:
	cargo build --release --all-features
