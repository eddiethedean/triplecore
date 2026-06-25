.PHONY: fmt lint test build check docs

fmt:
	cargo fmt

lint:
	cargo clippy --all-targets --all-features -- -D warnings

test:
	cargo test --all-features

build:
	cargo build --all-features

check: fmt lint test

docs:
	cargo doc --no-deps --all-features
