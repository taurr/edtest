#![doc = include_str!("../README.md")]

/// Generate a test function using `rstest`. If used on a `async` function
/// the test will use the `tokio` runtime.
/// See the [rstest documentation](https://docs.rs/rstest/latest/rstest).
pub use edtest_macros::rstest;

/// Creation of test-fixtures. see the
/// [fixture documentation](https://docs.rs/rstest/latest/rstest/attr.fixture.html).
pub use rstest::fixture;

pub use serial_test::serial;

pub use static_assertions::*;

/// Helper macro to set an `insta` snapshot suffix for the current scope.
///
/// Example:
///
/// set_snapshot_suffix!("{}", input);
///
/// Expands to code that clones the current `insta::Settings`, sets the
/// snapshot suffix and binds the settings to the current scope so snapshots
/// get the provided suffix for the duration of the scope.
#[macro_export]
macro_rules! set_snapshot_suffix {
    ($($expr:expr),*) => {
        let mut settings = insta::Settings::clone_current();
        let raw_suffix = format!($($expr,)*);
        let cleaned = $crate::internal::clean_snapshot_suffix(&raw_suffix);
        settings.set_snapshot_suffix(cleaned);
        let _guard = settings.bind_to_scope();
    }
}

#[doc(hidden)]
pub mod internal {
    use core::hint::black_box;
    use core::sync::atomic::{AtomicUsize, Ordering};
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    static COUNTER: AtomicUsize = AtomicUsize::new(0);

    /// Convert any provided snapshot suffix into a short, filesystem-safe hash.
    ///
    /// Strategy:
    /// - Use the standard library's default hasher (currently SipHash-based) over the input bytes.
    /// - Format the resulting u64 as 16 lowercase hex characters.
    /// - Deterministic within a given Rust version/implementation; ASCII-only and cross-platform safe.
    #[doc(hidden)]
    pub fn clean_snapshot_suffix(input: &str) -> String {
        let mut hasher = DefaultHasher::new();
        input.hash(&mut hasher);
        let h: u64 = hasher.finish();
        format!("{:016x}", h)
    }

    // Tiny side effects to ensure coverage sees executed regions and prevent full optimization.
    pub fn on_test_enter(name: &str) {
        black_box(name.len());
        COUNTER.fetch_add(1, Ordering::Relaxed);
    }

    pub fn on_test_exit() {
        COUNTER.fetch_add(1, Ordering::Relaxed);
    }
}

/// A guard that calls `internal::on_test_exit()` when dropped.
/// Used by the macro expansion to ensure an exit hook runs even if the test panics.
#[doc(hidden)]
pub struct TestGuard;

#[allow(clippy::new_without_default)]
impl TestGuard {
    #[inline(always)]
    pub fn new() -> Self {
        Self
    }
}

impl Drop for TestGuard {
    fn drop(&mut self) {
        internal::on_test_exit();
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn internal_hooks_execute() {
        crate::internal::on_test_enter("sample");
        let _g = crate::TestGuard::new();
        crate::internal::on_test_exit();
    }

    #[test]
    fn hashes_suffix_general() {
        use crate::internal::clean_snapshot_suffix as clean;
        let s = "a/b\\c:d*e?f|g<h>i\"j k.";
        let h = clean(s);
        assert_eq!(h.len(), 16);
        assert!(h
            .chars()
            .all(|c| c.is_ascii_hexdigit() && (c.is_ascii_lowercase() || c.is_ascii_digit())));
        // Deterministic for the same input
        assert_eq!(h, clean(s));
        // Likely different for a different input
        assert_ne!(h, clean("different"));
    }

    #[test]
    fn hash_properties_reserved_and_empty() {
        use crate::internal::clean_snapshot_suffix as clean;
        // Case sensitivity preserved via hashing
        assert_ne!(clean("CON"), clean("con"));
        // Empty still yields a deterministic hex string (for a given implementation)
        let e = clean("");
        assert_eq!(e.len(), 16);
        assert!(e
            .chars()
            .all(|c| c.is_ascii_hexdigit() && (c.is_ascii_lowercase() || c.is_ascii_digit())));
    }
}
