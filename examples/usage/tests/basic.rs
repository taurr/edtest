use edtest::test;
use tracing::*;

/// Normal synchronous test. Tracing output is captured by `test-log`.
#[test]
fn sync_test() {
    info!("hello from sync test");
    assert_eq!(2 + 2, 4);
}

/// Parameterized test using `rstest`'s `#[values]` via `edtest::test`.
#[test]
fn param_test(#[values(0, 1, 2)] a: u32, #[values(3, 4)] b: u32) {
    let sum = edtest_example_usage::add(a, b);
    assert_eq!(sum, a + b);
}
