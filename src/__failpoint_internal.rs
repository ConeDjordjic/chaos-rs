#[doc(hidden)]
use dashmap::DashSet;
use std::sync::LazyLock;

pub static FAILPOINTS: LazyLock<DashSet<&'static str>> = LazyLock::new(DashSet::new);

pub fn is_failpoint_enabled(tag: &str) -> bool {
    FAILPOINTS.contains(tag)
}

pub fn enable_failpoint(tag: &'static str) {
    FAILPOINTS.insert(tag);
}

pub fn disable_failpoint(tag: &str) {
    FAILPOINTS.remove(tag);
}

pub async fn sleep_async_internal(millis: std::time::Duration) {
    futures_timer::Delay::new(millis).await;
}
