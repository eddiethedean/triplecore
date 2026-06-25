fmt:
    cargo fmt

lint:
    cargo clippy --all-targets --all-features -- -D warnings

test:
    cargo test --all-features

test-rust:
    cargo test --all-features

build:
    cargo build --all-features

build-release:
    cargo build --release --all-features

check:
    cargo fmt --check
    cargo clippy --all-targets --all-features -- -D warnings
    cargo test --all-features

docs:
    cargo doc --no-deps --all-features --open

release-check: check
