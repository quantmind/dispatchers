# cargo build and test
build:
    cargo build --all-features --tests

# cargo watch all fixtures
watch:
    cargo watch --features async

# run broadcaster example
broadcaster:
    cargo run --all-features --example broadcaster

# run the mutable example
mutable:
    cargo run --example mutable

# run linters
lint:
    cargo clippy --all-features --tests -- -D warnings
    cargo fmt --all


# run linters ci
lint-ci:
    cargo clippy --all-features --tests -- -D warnings
    cargo fmt --all -- --check
