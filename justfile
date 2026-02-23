set shell := ["powershell", "-Command"]

ci:
    cargo fmt --all -- --check
    cargo clippy --workspace --all-targets --all-features -- -D warnings
    cargo test --workspace

r:
    just ci
    cargo run -p example --bin task_manager

f:
    cargo fmt

t:
    cargo fmt
    cargo test
