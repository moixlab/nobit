all:
    @just --list

dev:
    cargo run --release -p nobit

assets TARGET OPTION:
    mkdir -p ./assets/
    cargo run --release -p assets -- {{ TARGET }} {{ OPTION }}

build: format test
    cargo build --release -p nobit

test:
    cargo test --release -p nobit

format:
    cargo fmt
