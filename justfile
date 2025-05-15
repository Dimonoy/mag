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

build-windows:
    cargo build --release --target x86_64-pc-windows-gnu

bundle-linux version:
    cp target/x86_64-unknown-linux-gnu/release/mag .
    tar czvf mag-{{version}}-x86_64.tar.gz assets/ mag
    rm mag

tag version message:
    git tag {{version}} -m "{{message}}"
