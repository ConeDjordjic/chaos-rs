//! # chaos_rs
//!
//! **Minimal chaos testing library for Rust.**
//!
//! `chaos_rs` provides macros for injecting failures, panics, and delays during testing,
//! enabling you to validate your code's resilience under adverse conditions.
//!
//! ## Features
//! - **Fail injection**: Return errors from tagged failpoints (`maybe_fail!`).
//! - **Panic simulation**: Trigger panics when failpoints are enabled (`maybe_panic!`).
//! - **Sleep injection**: Add artificial delays for timing tests (`maybe_sleep!`) and async with
//! (`maybe_sleep_async!`).
//! - **Assertion helpers**: Verify that failpoints behave as expected (`with_failpoint!`) or
//! (`with_failpoint_async!`) for async.
//!
//! ## Example
//! ```rust
//! fn do_work() -> Result<&'static str, String> {
//!     chaos_rs::maybe_fail!("db_error", "Database connection failed".into());
//!     Ok("success")
//! }
//! ```

pub mod __failpoint_internal;
mod macros;

#[cfg(test)]
mod tests {
    use crate::*;
    use std::time::Instant;

    #[test]
    fn test_maybe_fail() {
        fn example() -> Result<&'static str, String> {
            maybe_fail!("fail_test");
            Ok("ok")
        }

        assert_eq!(example().unwrap(), "ok");

        with_failpoint!("fail_test", error, example());
    }

    #[test]
    fn test_maybe_panic() {
        fn risky() {
            maybe_panic!("panic_test");
        }

        risky();

        with_failpoint!("panic_test", panic, risky());
    }

    #[test]
    fn test_maybe_sleep() {
        fn slow() {
            maybe_sleep!("sleep_test", 50);
        }

        let start = Instant::now();
        slow();
        assert!(start.elapsed().as_millis() < 10);

        with_failpoint!("sleep_test", 50, 10, slow());
    }
}
