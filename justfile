set shell := ["powershell", "-Command"]

r:
    cargo check
    cargo fmt
    cargo run
f:
    cargo fmt