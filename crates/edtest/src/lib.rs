#![doc = include_str!("../README.md")]

/// Generate a test function using `rstest`. If used on a `async` function
/// the test will use the `tokio` runtime.
/// See the [rstest documentation](https://docs.rs/rstest/latest/rstest).
pub use edtest_macros::test;

/// Creation of test-fixtures. see the
/// [fixture documentation](https://docs.rs/rstest/latest/rstest/attr.fixture.html).
pub use rstest::fixture;

pub use serial_test::serial;

pub use static_assertions::*;
