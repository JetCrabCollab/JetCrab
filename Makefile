.PHONY: help fmt clippy test build clean dev

help: ## Show this help message
	@echo "JetCrab Development Commands:"
	@echo ""
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-15s\033[0m %s\n", $$1, $$2}'

fmt: ## Format code with rustfmt
	cargo fmt --all

clippy: ## Run clippy linter
	cargo clippy --all-targets --all-features -- -D warnings

test: ## Run tests
	cargo test

build: ## Build the project
	cargo build

clean: ## Clean build artifacts
	cargo clean

dev: fmt clippy test build ## Run all development checks

check: ## Run fmt check and clippy
	cargo fmt --all -- --check
	cargo clippy --all-targets --all-features -- -D warnings

bench: ## Run benchmarks
	cargo bench

doc: ## Generate documentation
	cargo doc --open

run-examples: ## Run all examples
	cargo run --example basic_usage
	cargo run --example e2e_test
	cargo run --example ecmascript_2024_compliance 