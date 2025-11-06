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
