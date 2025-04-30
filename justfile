alias bl := build-linux
alias c := check
alias r := run

default:
    @just --list

run:
    cargo run --features dev

check:
    cargo check --features dev

build-linux:
    cargo build --release --target x86_64-unknown-linux-gnu

bundle-linux:
    cp target/x86_64-unknown-linux-gnu/release/mag .
    zip -r mag-x86_64-unknown-linux-gnu.zip assets/ mag
    rm mag

tag version message:
    git tag {{version}} -m "{{message}}"
