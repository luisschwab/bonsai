alias b := build
alias c := check
alias d := delete
alias f := fmt
alias r := run
alias rr := run-release
alias h := hot

_default:
    @just --list --list-heading $'bonsai\n'

# Build `bonsai`
build:
    cargo build

# Check code: formatting, compilation, and linting
check:
    cargo +nightly fmt --all -- --check
    cargo check
    cargo clippy -- -D warnings

# Delete files: data-signet, settings, target, lockfile
delete item="data-signet":
    just _delete-{{ item }}

# Format code
fmt:
    cargo +nightly fmt

# Run the code
run:
    cargo run

# Run the code in release mode
run-release:
    cargo run --release

# Run with experimental hot-reloading (will crash if the node is active)
hot:
    cargo hot --features hot-reloading

# Delete signet chaindata
_delete-data-signet:
    rm -rf ~/.bonsai/signet

# Delete settings file
_delete-settings:
    rm ~/.bonsai/bonsai.toml

_delete-target:
    rm -rf target/

_delete-lockfile:
    rm -f Cargo.lock
