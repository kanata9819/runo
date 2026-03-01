set shell := ["powershell", "-Command"]

ci:
    cargo fmt --all
    if (Test-Path target/ci-local) { Remove-Item -Recurse -Force target/ci-local }
    $env:CARGO_TARGET_DIR = "target/ci-local"; cargo fmt --all -- --check
    $env:CARGO_TARGET_DIR = "target/ci-local"; cargo clippy --workspace --all-targets --all-features -- -D warnings
    $env:CARGO_TARGET_DIR = "target/ci-local"; cargo test --workspace

r:
    cargo fmt --all
    cargo test
    cargo run -p example --bin task_manager

f:
    cargo fmt --all

t:
    cargo fmt --all
    cargo test

b:
    cargo fmt --all
    cargo build

tlc:
    cargo llvm-conv
