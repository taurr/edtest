# EDTest — Easy Desktop Test

[![CI](https://github.com/taurr/edtest/actions/workflows/ci.yml/badge.svg)](https://github.com/taurr/edtest/actions/workflows/ci.yml) ![Coverage](https://raw.githubusercontent.com/taurr/edtest/main/badges/coverage.svg) [![docs.rs](https://docs.rs/edtest/badge.svg)](https://docs.rs/edtest)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE-MIT)
[![License: Apache-2.0](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](LICENSE-APACHE)

EDTest is a small set of Rust crates that make writing tests with [`rstest`](https://crates.io/crates/rstest) pleasant, with built‑in tracing via [`test-log`](https://crates.io/crates/test-log) and first‑class support for async tests on [`tokio`](https://crates.io/crates/tokio).

Crates in this workspace:

- `edtest` — the user-facing crate exposing the `#[rstest]` attribute macro, re‑exported `rstest` helpers, `serial_test::serial`, and `static_assertions`.
- `edtest_macros` — proc‑macro implementation for `edtest`.
- `edtest_tests` — external tests/examples for the workspace.

## Quick start

Add the following dev‑dependencies to your crate:

```toml
[dev-dependencies]
edtest = "0.5"
rstest = "0.26"
test-log = { version = "0.2", features = ["trace"] }
# Only if you want non-concurrent tests
serial_test = "3"
```

Write tests as usual, using `edtest::rstest` in place of `rstest::rstest` and `#[test]`:

```rust
use edtest::rstest;
#[allow(unused)]
use tracing::*;

#[rstest]
fn sync_test() {
	info!("tracing output is captured");
}

#[rstest]
#[edtest::serial]
#[case(0)]
#[case(1)]
#[case(2)]
async fn async_value_test(#[case] a: u32, #[values(0, 1, 2)] b: u32) {
    use edtest::assert_cfg;
    assert_cfg!(test);

    debug!(event = "something_minor", code = 1001);
    tokio::time::sleep(std::time::Duration::from_millis(10)).await;
    let ab = a + b;
    let ba = b + a;
    assert_eq!(ab, ba);
}
```

See the crate README in `crates/edtest/` for more details.

## Examples

This repo includes a small, runnable example crate in `examples/usage` that demonstrates:

- Synchronous tests with tracing
- Parameterized tests with `#[values]`
- Async tests on Tokio, optionally `#[edtest::serial]`
- Using `#[fixture]` for shared setup

Run the examples:

```bash
cargo test -p edtest_example_usage --no-fail-fast
```

### Show logs from passing tests

By default, `test-log` only prints logs on failures. To always show logs (useful when exploring the examples), enable logging and disable output capture:

```bash
TEST_LOG=1 RUST_LOG=info cargo test -p edtest_example_usage -- --nocapture
```

If you use `just`, convenient recipes are provided:

```bash
just example-logs    # logs for the examples crate
just logs            # logs for the whole workspace
```

## Development

- Requires Rust 1.74+ (edition 2021).
- Run all tests: `cargo test -q` at the workspace root.
- Optional: run `cargo clippy --all-targets --all-features -D warnings`.

## License

Licensed under either of

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
