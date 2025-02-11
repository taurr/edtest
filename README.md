# EDTest - Easy Deskktop Test

This set of crates is for making unit and integration testing in Rust using Tokio a little easier.

The crates gathers several other testing crates, and uses a macro `#[edtest::test]` to mark test-functions, as being `rstest` using the `tokio` framework, with enabled `tracing` logs through the `test-log` crate.

It encourages use of `static_assertions` by re-exporting all its assertions, and enables running tests in series (as opposed to the default parallel) through the `#[serial]` attribute.
