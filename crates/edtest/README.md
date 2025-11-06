# Easy Desktop Test

The crate streamlines the use of [`rstest`](https://crates.io/crates/rstest) with support for [`tracing`](https://crates.io/crates/tracing) (using [`test-log`](https://crates.io/crates/test-log)). `async` tests are supported using the [`tokio`](https://crates.io/crates/tokio) framework.

The crate re-exports the assertions from [`static-assertions`](https://crates.io/crates/static-assertions), and the `#[serial]` attribute of [`serial_test`](https://crates.io/crates/serial_test) for pure convenience.

## Usage

Unfortunately, due to the nature of macros, besides this crate the user still needs a few dev-dependencies:

cargo.toml:
```toml
[dev-dependencies]
edtest = ...
rstest = ...
test-log = ...
# Only needed if using the #[serial] attribute to make tests not run concurrently
serial_test = ...
```

tests:
```rust
use edtest::test;
use tracing::*;

/// Normal synchronous test.
/// Note that `cargo test` still tries to run these concurrently!
#[test]
fn sync_test() {
    info!("Tracing output is captured and part of the test output");
}

/// Async tests using `tokio` are fully supported - they can even
/// be run using `serial` (non-concurrent)
#[test]
#[edtest::serial]
async fn async_value_test(
    #[values(0, 1, 2, 3, 4, 5)] a: u32,
    #[values(0, 1, 2, 3, 4, 5)] b: u32,
) {
    use edtest::assert_cfg;
    // `static_assertions` are re-exported for convenience
    assert_cfg!(test);

    trace!(a, b);
    let ab = super::add(a, b);
    let ba = b + a;
    assert_eq!(ab, ba);

    tokio::time::sleep(std::time::Duration::from_millis(100)).await;
}
```

## Noteworthy crates
### General
- [`insta`](https://crates.io/crates/insta) 
  Snapshot testing
- [`proptest`](https://crates.io/crates/proptest)
  Property based testing with arbitrary input
- [`mockall`](https://crates.io/crates/mockall)

### Messing with files
- [`assert_fs`](https://crates.io/crates/assert_fs)
  Working with external files during testing
- [`tempfile`](https://crates.io/crates/tempfile)
  Temporary storage and files

### Network
- [`mockito`](https://crates.io/crates/mockito)
  Generating and delivering HTTP mocks
- [`axum-test`](https://crates.io/crates/axum-test)
  Test Axum servers in isolation.

### CLI programs
- [`assert_cmd`](https://crates.io/crates/assert_cmd)
  Simplify integration testing of CLIs
- [`rexpect`](https://crates.io/crates/rexpect)
  Running/testing interactive CLIs

### Tools
- [coverage](https://crates.io/crates/cargo-llvm-cov)
  LLVM Coverage reports the easy way
