# Run all code quality checks
check:
    cargo fmt --all -- --check
    cargo clippy -- -D warnings
    cargo test
    cargo audit
