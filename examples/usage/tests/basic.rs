use edtest::rstest;
use tracing::*;

/// Normal synchronous test. Tracing output is captured by `test-log`.
#[rstest]
fn sync_test() {
    info!("hello from sync test");
    assert_eq!(2 + 2, 4);
}

/// Parameterized test using `rstest`'s `#[values]` via `edtest::rstest`.
#[rstest]
fn param_test(#[values(0, 1, 2)] a: u32, #[values(3, 4)] b: u32) {
    let sum = edtest_example_usage::add(a, b);
    assert_eq!(sum, a + b);
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
