_default:
    @just --list

check:
    cargo clippy --locked
    cargo fmt -- --check

fix:
    cargo clippy --fix --locked -- -D warnings

test day:
    cargo nextest run --locked -p {{day}}

create day:
    cargo generate --path ./daily-template --name {{day}}