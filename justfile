alias b := build
alias c := check
alias d := delete
alias f := fmt
alias r := run

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

# Delete files: data, target, lockfile
delete item="data":
    just _delete-{{ item }}

# Format code
fmt:
    cargo +nightly fmt

# Run the code
run:
    cargo run

_delete-data:
    rm -rf data/

_delete-target:
    rm -rf target/

_delete-lockfile:
    rm -f Cargo.lock
