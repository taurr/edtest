set dotenv-load := false

# Run formatting, lints, build, and tests locally

fmt:
    cargo fmt --all

fmt-check:
    cargo fmt --all -- --check

clippy:
    cargo clippy --workspace --all-targets --all-features -- -D warnings

build:
    cargo build --workspace --all-targets --all-features

test:
    cargo test --workspace --all-features --no-fail-fast

# Quick checks before committing
check: fmt-check clippy build
    @echo "All checks passed."

# Same suite CI runs
ci: fmt-check clippy build test
    @echo "CI checks passed."

# One-time setup to enable versioned git hooks in this repo
hooks-install:
    git config core.hooksPath .githooks

# Show logs from tests (success cases too)
logs:
    TEST_LOG=1 RUST_LOG=info cargo test --workspace --all-features --no-fail-fast -- --nocapture

example-logs:
    TEST_LOG=1 RUST_LOG=info cargo test -p edtest_example_usage --no-fail-fast -- --nocapture

# Coverage (HTML report under target/llvm-cov/html)
coverage:
    # Workspace coverage excluding examples, test-only crate and proc-macro crate
    cargo llvm-cov --workspace --all-features --exclude-from-report edtest_example_usage --exclude-from-report edtest_tests --exclude-from-report edtest_macros --html

coverage-summary:
    cargo llvm-cov --workspace --all-features --exclude-from-report edtest_example_usage --exclude-from-report edtest_tests --exclude-from-report edtest_macros --summary-only

coverage-open:
    cargo llvm-cov --workspace --all-features --exclude-from-report edtest_example_usage --exclude-from-report edtest_tests --exclude-from-report edtest_macros --open

coverage-example:
    cargo llvm-cov -p edtest_example_usage --html
