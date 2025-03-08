# Easy Desktop Test

The crate streamlines the use of `rstest` with logging and support for async tests with the `tokio` framework.

It collects the use of [`rstest`](https://crates.io/crates/rstest), [`test-log`](https://crates.io/crates/test-log), [`serial_test`](https://crates.io/crates/serial_test) and [`static_assertions`](https://crates.io/crates/static_assertions) into one test crate.

Unfortunately, due to the nature of macros, besides this crate the user still needs a few dev-dependencies:

```toml
[dev-dependencies]
edtest = ...
rstest = ...
test-log = ...
# Only needed if using the #[serial] attribute to make tests not run in parallel
serial_test = ...
```

`proptest` is fully supported.

## Noteworthy crates
### General
- [`insta`](https://crates.io/crates/insta) 
  Snapshot testing
- [`proptest`](https://crates.io/crates/proptest)
  Proterty based testing with arbitrary input
- [`mockall`](https://crates.io/crates/mockall)

### Files
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
  Simplify integration testing of CLI's
- [`rexpect`](https://crates.io/crates/rexpect)
  Running/testing interactive CLI's

### Tools
- [coverage](https://crates.io/crates/cargo-llvm-cov)
  LLVM Coverage reports the easy way
