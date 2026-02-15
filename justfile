set shell := ["powershell", "-Command"]

r:
    cargo check
    cargo clippy
    cargo fmt
    cargo run
f:
    cargo fmt