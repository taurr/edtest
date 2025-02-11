# Best Practice Test

This crate collects the use of [`rstest`](https://crates.io/crates/rstest), [`test-log`](https://crates.io/crates/test-log), [`tokio`](https://crates.io/crates/tokio), [`serial_test`](https://crates.io/crates/serial_test) and [`static_assertions`](https://crates.io/crates/static_assertions) into one test crate.

It utilizes `rstest` to enable use of all its features and `test-log` in order to enable logging via `tracing`.
Furtermore, all assertions from `static_assertions`, as well as `#[serial]`, are re-exported directly.

`proptest` is fully supported.

## Noteworthy crates
- `insta`  
  https://crates.io/crates/insta
- `assert_cmd`
- `mockall`
- `tempfile`
