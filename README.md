# EDTest — Easy Desktop Test

EDTest is a small set of Rust crates that make writing tests with [`rstest`](https://crates.io/crates/rstest) pleasant, with built‑in tracing via [`test-log`](https://crates.io/crates/test-log) and first‑class support for async tests on [`tokio`](https://crates.io/crates/tokio).

Crates in this workspace:

- `edtest` — the user-facing crate exposing the `#[test]` attribute macro, re‑exported `rstest` helpers, `serial_test::serial`, and `static_assertions`.
- `edtest_macros` — proc‑macro implementation for `edtest`.
- `edtest_tests` — external tests/examples for the workspace.

## Quick start

Add the following dev‑dependencies to your crate:

```toml
[dev-dependencies]
edtest = "0.1"
rstest = "0.25"
test-log = { version = "0.2", features = ["trace"] }
# Only if you want non-concurrent tests
serial_test = "3"
```

Write tests as usual, using `edtest::test` in place of `rstest::rstest` and `#[test]`:

```rust
use edtest::test;
use tracing::*;

#[test]
fn sync_test() {
	info!("tracing output is captured");
}

#[test]
async fn async_value_test(
	#[values(0, 1, 2)] a: u32,
	#[values(0, 1, 2)] b: u32,
) {
	use edtest::assert_cfg;
	assert_cfg!(test);
	trace!(a, b);
	assert_eq!(a + b, b + a);
	tokio::time::sleep(std::time::Duration::from_millis(10)).await;
}
```

See the crate README in `crates/edtest/` for more details.

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
