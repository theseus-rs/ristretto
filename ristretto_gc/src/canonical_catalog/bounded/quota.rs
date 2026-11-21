//! Exact bounded registration transitions, also compiled with Loom atomics.
use super::{AtomicUsize, Ordering, RegisterError};
pub(super) fn acquire(count: &AtomicUsize, limit: usize) -> Result<usize, RegisterError> {
    let before = count.load(Ordering::Relaxed);
    if before >= limit {
        return Err(RegisterError::Limit);
    }
    count
        .compare_exchange(before, before + 1, Ordering::Acquire, Ordering::Relaxed)
        .map_err(|_| RegisterError::Contended)
}
pub(super) fn release(count: &AtomicUsize) {
    count.fetch_sub(1, Ordering::Release);
}
