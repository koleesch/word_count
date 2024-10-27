SHELL := /bin/bash
.PHONY: help

help:
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-15s\033[0m %s\n", $$1, $$2}'

clean: ## Clean the project using cargo
	cargo clean

build: fmt ## Build the project using cargo	
	cargo build

lint: ## Lint the project using cargo
	@rustup component add clippy 2> /dev/null
	cargo clippy

fmt: ## Format the project using cargo
	@rustup component add rustfmt 2> /dev/null
	cargo fmt

rust-version: ## shows the setup
	@echo "Rust command-line utility versions:"
	rustc --version 			#rust compiler
	cargo --version 			#rust package manager
	rustfmt --version			#rust code formatter
	rustup --version			#rust toolchain manager
	clippy-driver --version		#rust linter

test: ## Test the project using cargo
	cargo test

run: ## Run the project using cargo
	cargo run

release: ## Build a release binary using cargo
	cargo build --release

all: fmt lint test build run ## make all 
