set shell := ["powershell", "-Command"]

ci:
    cargo fmt --all -- --check
    cargo clippy --workspace --all-targets --all-features -- -D warnings
    cargo test --workspace

r:
    just ci
    cargo run

f:
    cargo fmt

t:
    cargo test
