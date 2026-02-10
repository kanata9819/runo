set shell := ["powershell", "-Command"]

r:
    cargo fmt
    cargo run
f:
    cargo fmt