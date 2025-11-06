use tracing::*;

// Ensure the public macro from this crate correctly drives the runtime hooks.
#[edtest::test]
fn macro_smoke(#[values(1, 2)] x: u32) {
    info!(x, "macro_smoke running");
    assert!(x >= 1);
}
