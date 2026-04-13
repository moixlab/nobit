all:
    @just --list

assets TARGET OPTION:
    @echo "Preparando Assets...\n"
    mkdir -p ./assets/
    cargo run --release -p assets -- {{ TARGET }} {{ OPTION }}