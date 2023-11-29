set dotenv-load := false

# print options
default:
    @just --list --unsorted

# install cargo tools
init:
    cargo upgrade --incompatible
    cargo update
    cargo install cargo-rdme

# generate README
readme:
    cargo rdme --force

# format code
fmt:
    cargo fmt
    npx prettier --write .
    just --fmt --unstable

# check code
check:
    cargo check
    cargo clippy --all-targets --all-features

# run server locally for development
dev:
    RUST_LOG=debug MITBLOB_GIT_REPO=https://github.com/olidacombe/mitblob cargo watch -x run

# build project
build:
    cargo build --all-targets

# execute tests
test:
    cargo test
