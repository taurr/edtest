use tracing::*;

// Ensure the public macro from this crate correctly drives the runtime hooks.
#[edtest::rstest]
fn macro_smoke(#[values(1, 2)] x: u32) {
    info!(x, "macro_smoke running");
    assert!(x >= 1);
}

// Ensure the snapshot suffix is cleaned by the macro so paths are valid on all OSes.
#[test]
fn macro_suffix_cleaned() {
    // Contains path separators, Windows-forbidden characters, quotes, and trailing dot/space
    let raw = "a/b\\c:d*e?f|g<h>i\"j k. ";
    let expected = edtest::internal::clean_snapshot_suffix(raw);

    // This should set a cleaned suffix under the hood without panicking or producing invalid paths
    edtest::set_snapshot_suffix!("{}", raw);

    // Taking an inline snapshot exercises insta name/path generation while avoiding filesystem writes.
    // If suffix cleaning is wrong, insta can error on some platforms. Inline keeps the test deterministic.
    let value = "ok";
    insta::assert_snapshot!(value, @"ok");

    // Also assert the sanitizer behavior directly for determinism across platforms.
    assert_eq!(expected, "a-b-c_d_e_f_g_h_i_j-k.-");
}
