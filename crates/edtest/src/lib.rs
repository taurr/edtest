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
        settings.set_snapshot_suffix(format!($($expr,)*));
        let _guard = settings.bind_to_scope();
    }
}

#[doc(hidden)]
pub mod internal {
    use core::hint::black_box;
    use core::sync::atomic::{AtomicUsize, Ordering};

    static COUNTER: AtomicUsize = AtomicUsize::new(0);

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
}
