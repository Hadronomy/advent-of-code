_default:
    @just --list

check:
    cargo clippy --locked -- -D warnings
    cargo fmt -- --check

fix:
    cargo clippy --fix --locked -- -D warnings

test:
    cargo test --locked
