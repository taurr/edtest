use tracing::*;

// Ensure the public macro from this crate correctly drives the runtime hooks.
#[edtest::rstest]
fn macro_smoke(#[values(1, 2)] x: u32) {
    info!(x, "macro_smoke running");
    assert!(x >= 1);
}

// Ensure the snapshot suffix is hashed by the macro so paths are valid on all OSes.
#[test]
fn macro_suffix_hashed() {
    // Contains path separators, Windows-forbidden characters, quotes, and trailing dot/space
    let raw = "a/b\\c:d*e?f|g<h>i\"j k. ";
    let expected = edtest::internal::clean_snapshot_suffix(raw);

    // This should set a cleaned suffix under the hood without panicking or producing invalid paths
    edtest::set_snapshot_suffix!("{}", raw);

    // Taking an inline snapshot exercises insta name/path generation while avoiding filesystem writes.
    // If suffix cleaning is wrong, insta can error on some platforms. Inline keeps the test deterministic.
    let value = "ok";
    insta::assert_snapshot!(value, @"ok");

    // Also assert the hashing behavior directly for determinism across platforms.
    assert_eq!(expected.len(), 16);
    assert!(expected
        .chars()
        .all(|c| c.is_ascii_hexdigit() && (c.is_ascii_lowercase() || c.is_ascii_digit())));
    // Different input -> likely different hash (not guaranteed, but highly probable for 64-bit hash)
    let different = edtest::internal::clean_snapshot_suffix("different input");
    assert_ne!(expected, different);
}
