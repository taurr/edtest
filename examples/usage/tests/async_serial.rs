use edtest::test;
use std::time::Duration;

/// Async test runs on the Tokio runtime and can be marked `serial` to avoid concurrency.
#[test]
#[edtest::serial]
async fn async_value_test(#[values(0, 1)] a: u32, #[values(0, 1)] b: u32) {
    let sum = edtest_example_usage::add(a, b);
    tokio::time::sleep(Duration::from_millis(10)).await;
    assert_eq!(sum, a + b);
}
