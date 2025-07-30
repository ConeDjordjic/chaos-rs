/// Returns `Err(tag.into())` or a custom error when the failpoint is enabled.
///
/// # Examples
/// ```rust
/// fn perform_action() -> Result<&'static str, String> {
///     chaos_rs::maybe_fail!("network_fail");
///     Ok("done")
/// }
/// ```
///
/// With custom error:
/// ```rust
/// fn perform_action() -> Result<&'static str, String> {
///     chaos_rs::maybe_fail!("db_fail", "Database unreachable".to_string());
///     Ok("done")
/// }
/// ```
#[macro_export]
macro_rules! maybe_fail {
    ($tag:literal) => {
        #[cfg(feature = "chaos")]
        {
            if $crate::__failpoint_internal::is_failpoint_enabled($tag) {
                return Err($tag.into());
            }
        }
    };
    ($tag:literal, $err:expr) => {
        #[cfg(feature = "chaos")]
        {
            if $crate::__failpoint_internal::is_failpoint_enabled($tag) {
                return Err($err);
            }
        }
    };
}

/// Panics when the failpoint is enabled.
///
/// # Example
/// ```rust
/// fn critical() {
///     chaos_rs::maybe_panic!("unexpected_panic");
/// }
/// ```
#[macro_export]
macro_rules! maybe_panic {
    ($tag:literal) => {
        #[cfg(feature = "chaos")]
        {
            if $crate::__failpoint_internal::is_failpoint_enabled($tag) {
                panic!($tag);
            }
        }
    };
}

/// Sleeps for a given number of milliseconds when the failpoint is enabled.
///
/// # Example
/// ```rust
/// chaos_rs::maybe_sleep!("slow_io", 500);
/// ```
#[macro_export]
macro_rules! maybe_sleep {
    ($tag:literal, $millis:literal) => {
        #[cfg(feature = "chaos")]
        {
            if $crate::__failpoint_internal::is_failpoint_enabled($tag) {
                std::thread::sleep(std::time::Duration::from_millis($millis));
            }
        }
    };
}

/// If the specified failpoint is enabled, this macro will pause the asynchronous
/// execution for a given number of milliseconds.
///
/// # Example
/// ```rust
/// chaos_rs::maybe_sleep_async!("slow_io", 500);
/// ```
#[macro_export]
macro_rules! maybe_sleep_async {
    ($tag:literal, $millis:literal) => {
        #[cfg(feature = "chaos")]
        {
            if $crate::__failpoint_internal::is_failpoint_enabled($tag) {
                let duration = std::time::Duration::from_millis($millis);
                $crate::__failpoint_internal::sleep_async_internal(duration).await;
            }
        }
    };
}

/// Runs a code block with a failpoint enabled and validates its effect.
///
/// Supported modes:
/// - `panic`: Expects the code to panic when the failpoint is active.
/// - `error`: Expects the code to return `Err` when the failpoint is active.
/// - Sleep validation: Verifies that code sleeps somewhere in the range of `min_ms` - `tolerance` and `min_ms` + `tolerance` when failpoint is active.
///
/// # Examples
///
/// Expects a panic:
/// ```rust
/// chaos_rs::with_failpoint!("panic_test", panic, {
///     chaos_rs::maybe_panic!("panic_test");
/// });
/// ```
///
/// Expects an error:
/// ```rust
/// chaos_rs::with_failpoint!("error_test", error, {
///     fn test() -> Result<(), ()> {
///         chaos_rs::maybe_fail!("error_test", ());
///         Ok(())
///     }
///     test()
/// });
/// ```
///
/// Expects the operation to sleep for 200 ± 50ms (150 - 250 range):
/// ```rust
/// chaos_rs::with_failpoint!("sleep_test", 200, 50, {
///     chaos_rs::maybe_sleep!("sleep_test", 200);
/// });
/// ```
#[macro_export]
macro_rules! with_failpoint {
    ($tag:literal, panic, $code:expr) => {{
        #[cfg(feature = "chaos")]
        {
            $crate::__failpoint_internal::enable_failpoint($tag);
            let result = std::panic::catch_unwind(|| $code);
            $crate::__failpoint_internal::disable_failpoint($tag);
            match result {
                Ok(_) => panic!(
                    "Expected panic from failpoint '{}', but none occurred",
                    $tag
                ),
                Err(_) => {}
            }
        }
    }};

    ($tag:literal, error, $code:expr) => {{
        #[cfg(feature = "chaos")]
        {
            $crate::__failpoint_internal::enable_failpoint($tag);
            let result = $code;
            $crate::__failpoint_internal::disable_failpoint($tag);

            match result {
                Err(_) => {}
                Ok(_) => panic!(
                    "Expected error from failpoint '{}', but function returned Ok",
                    $tag
                ),
            }
        }
    }};

    ($tag:literal, $min_ms:literal, $tolerance_ms:literal, $code:expr) => {{
        #[cfg(feature = "chaos")]
        {
            $crate::__failpoint_internal::enable_failpoint($tag);
            let start = std::time::Instant::now();
            $code;
            let elapsed = start.elapsed();
            $crate::__failpoint_internal::disable_failpoint($tag);

            let max = std::time::Duration::from_millis($min_ms + $tolerance_ms);
            let min = std::time::Duration::from_millis($min_ms - $tolerance_ms);

            assert!(
                elapsed <= max && elapsed >= min,
                "Expected sleep between {:?} and {:?} from failpoint '{}', got {:?}",
                min,
                max,
                $tag,
                elapsed
            );
        }
    }};
}
#[macro_export]
macro_rules! with_failpoint_async {
    ($tag:literal, $min_ms:literal, $tolerance_ms:literal, $code:expr) => {{
        #[cfg(feature = "chaos")]
        {
            $crate::__failpoint_internal::enable_failpoint($tag);
            let start = std::time::Instant::now();
            $code.await;
            let elapsed = start.elapsed();
            $crate::__failpoint_internal::disable_failpoint($tag);

            let max = std::time::Duration::from_millis($min_ms + $tolerance_ms);
            let min = std::time::Duration::from_millis($min_ms - $tolerance_ms);

            assert!(
                elapsed <= max && elapsed >= min,
                "Expected sleep between {:?} and {:?} from failpoint '{}', got {:?}",
                min,
                max,
                $tag,
                elapsed
            );
        }
    }};
}
