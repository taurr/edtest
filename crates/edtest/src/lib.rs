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

    static COUNTER: AtomicUsize = AtomicUsize::new(0);

    /// Clean a snapshot suffix so it's safe to be used in file names across
    /// Windows, macOS and Linux.
    ///
    /// Strategy:
    /// - Keep ASCII letters and digits.
    /// - Keep '-', '_', '.', '@', '+' as-is (commonly safe on all platforms).
    /// - Convert whitespace and path separators ('/', '\\') to '-'.
    /// - Replace all other characters with '_'.
    /// - Trim trailing dots/spaces (problematic on Windows).
    /// - Avoid reserved Windows device names by prefixing with '_'.
    /// - If the result is empty, return "snapshot".
    #[doc(hidden)]
    pub fn clean_snapshot_suffix(input: &str) -> String {
        let mut out = String::with_capacity(input.len());
        for ch in input.chars() {
            if ch.is_ascii_alphanumeric() || matches!(ch, '-' | '_' | '.' | '@' | '+') {
                out.push(ch);
            } else if ch.is_whitespace() || ch == '/' || ch == '\\' {
                out.push('-');
            } else {
                out.push('_');
            }
        }

        // Windows doesn't like trailing dots or spaces in file names
        while out.ends_with(['.', ' ']) {
            out.pop();
        }

        // Avoid reserved device names on Windows
        // https://learn.microsoft.com/windows/win32/fileio/naming-a-file
        fn is_windows_reserved(name: &str) -> bool {
            const RESERVED: &[&str] = &[
                "CON", "PRN", "AUX", "NUL", "COM1", "COM2", "COM3", "COM4", "COM5", "COM6", "COM7",
                "COM8", "COM9", "LPT1", "LPT2", "LPT3", "LPT4", "LPT5", "LPT6", "LPT7", "LPT8",
                "LPT9",
            ];
            RESERVED.iter().any(|&r| r.eq_ignore_ascii_case(name))
        }

        if out.is_empty() || !out.chars().any(|c| c.is_ascii_alphanumeric()) {
            return "snapshot".to_string();
        }

        if is_windows_reserved(&out) {
            return format!("_{}", out);
        }

        out
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
    fn cleans_suffix_general() {
        use crate::internal::clean_snapshot_suffix as clean;
        // Mix of problem characters including separators and invalid Windows chars
        let s = "a/b\\c:d*e?f|g<h>i\"j k.";
        assert_eq!(clean(s), "a-b-c_d_e_f_g_h_i_j-k");
    }

    #[test]
    fn cleans_suffix_reserved_and_empty() {
        use crate::internal::clean_snapshot_suffix as clean;
        assert_eq!(clean("CON"), "_CON");
        assert_eq!(clean("con"), "_con");
        assert_eq!(clean("   \t\n"), "snapshot");
    }
}
