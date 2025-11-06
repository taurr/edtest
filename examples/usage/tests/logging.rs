use edtest::test;
use tracing::{debug, error, info, span, warn, Level};

/// Demonstrates capturing structured logs via `tracing`.
/// The `test-log` integration from `edtest` ensures logs appear in test output.
#[test]
fn logs_are_captured() {
    let outer = span!(Level::INFO, "outer", version = %env!("CARGO_PKG_VERSION"));
    let _guard = outer.enter();

    debug!("this is a debug log");
    info!(answer = 42, "structured info");
    warn!(event = "something_minor", code = 1001);
    error!(reason = "only an example");

    use std::time::{Duration, Instant};
    let start = Instant::now();
    assert!(start.elapsed() >= Duration::from_micros(0));
}
