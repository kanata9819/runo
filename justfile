set shell := ["powershell", "-Command"]

ci:
    if (Test-Path target/ci-local) { Remove-Item -Recurse -Force target/ci-local }
    $env:CARGO_TARGET_DIR = "target/ci-local"; cargo fmt --all -- --check
    $env:CARGO_TARGET_DIR = "target/ci-local"; cargo clippy --workspace --all-targets --all-features -- -D warnings
    $env:CARGO_TARGET_DIR = "target/ci-local"; cargo test --workspace

r:
    just ci
    cargo run -p example --bin task_manager

f:
    cargo fmt

t:
    cargo fmt
    cargo test

b:
    cargo fmt
    cargo build

tlc:
    cargo llvm-conv
