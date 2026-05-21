all:
    @just --list

dev:
    cargo run --release -p nobit

assets TARGET OPTION:
    mkdir -p ./assets/
    cargo run --release -p assets -- {{ TARGET }} {{ OPTION }}

build: format
    cargo build --release -p nobit

format:
    cargo fmt
