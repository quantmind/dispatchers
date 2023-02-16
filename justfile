# cargo build
build:
    cargo build --all-features

# cargo watch all fixtures
watch:
    cargo watch --features shared

# create documentations
doc:
    cargo doc --all-features --no-deps --open

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

# cargo test
test:
    cargo test --all-features

# cargo publish
publish:
    cargo publish --all-features
