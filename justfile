set windows-shell := ["pwsh", "-c"]

build:
    cargo build

lint:
    cargo clippy

format:
    cargo fmt

test:
    cargo test

run day=("0"):
    cargo run -- --input ./inputs/day-{{day}}.txt {{day}}

help:
    cargo run -- --help
