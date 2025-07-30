# chaos-rs

[![Crates.io](https://img.shields.io/crates/v/chaos-rs.svg)](https://crates.io/crates/chaos-rs)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

Minimal chaos testing library for Rust.

## Overview

`chaos-rs` provides macros for injecting failures, panics, and delays during testing, enabling you to validate your code's resilience under adverse conditions.

## Features

- **Fail injection**: Return errors from tagged failpoints
- **Panic simulation**: Trigger panics when failpoints are enabled
- **Sleep injection**: Add artificial delays for timing tests (sync and async)
- **Assertion helpers**: Verify that failpoints behave as expected

## Usage

Add to your `Cargo.toml`:

```toml
[dependencies]
chaos-rs = "0.1.4"
```

### Basic Examples

**Inject failures:**

```rust
fn do_work() -> Result<&'static str, String> {
    chaos_rs::maybe_fail!("db_error", "Database connection failed".into());
    Ok("success")
}
```

**Simulate panics:**

```rust
fn critical_section() {
    chaos_rs::maybe_panic!("unexpected_panic");
    // ... critical code
}
```

**Add delays:**

```rust
fn slow_operation() {
    chaos_rs::maybe_sleep!("slow_io", 500);
    // ... operation
}

async fn async_operation() {
    chaos_rs::maybe_sleep_async!("slow_async", 200);
    // ... async operation
}
```

### Testing Failpoints

```rust
#[test]
fn test_error_handling() {
    chaos_rs::with_failpoint!("db_error", error, do_work());
}

#[test]
fn test_panic_handling() {
    chaos_rs::with_failpoint!("unexpected_panic", panic, {
        critical_section();
    });
}

#[test]
fn test_timing() {
    chaos_rs::with_failpoint!("slow_io", 500, 50, {
        slow_operation(); // Should sleep 450-550ms
    });
}
```

## License

MIT
