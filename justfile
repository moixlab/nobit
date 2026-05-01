all:
    @just --list

assets TARGET OPTION:
    mkdir -p ./assets/
    cargo run --release -p assets -- {{ TARGET }} {{ OPTION }}

run:
    cargo run --release -p nobit

build:
    cargo build --release -p nobit
