//! Allocation-order diagnostics. Enable only for separate counting executions.
use std::sync::atomic::{AtomicU64, Ordering};
static SEQUENCE: AtomicU64 = AtomicU64::new(0);
static REQUESTS: AtomicU64 = AtomicU64::new(0);
static REQUESTED: AtomicU64 = AtomicU64::new(0);
static FREES: AtomicU64 = AtomicU64::new(0);
static RELEASED: AtomicU64 = AtomicU64::new(0);
static REFUNDS: AtomicU64 = AtomicU64::new(0);
static REFUNDED: AtomicU64 = AtomicU64::new(0);
static LAST_FREE: AtomicU64 = AtomicU64::new(0);
static LAST_REFUND: AtomicU64 = AtomicU64::new(0);
#[derive(Clone, Copy, Debug)]
pub struct Snapshot {
    pub requests: u64,
    pub requested: u64,
    pub frees: u64,
    pub released: u64,
    pub refunds: u64,
    pub refunded: u64,
    pub last_free: u64,
    pub last_refund: u64,
}
pub fn snapshot() -> Snapshot {
    Snapshot {
        requests: REQUESTS.load(Ordering::Relaxed),
        requested: REQUESTED.load(Ordering::Relaxed),
        frees: FREES.load(Ordering::Relaxed),
        released: RELEASED.load(Ordering::Relaxed),
        refunds: REFUNDS.load(Ordering::Relaxed),
        refunded: REFUNDED.load(Ordering::Relaxed),
        last_free: LAST_FREE.load(Ordering::Relaxed),
        last_refund: LAST_REFUND.load(Ordering::Relaxed),
    }
}
pub fn allocated(bytes: usize) {
    REQUESTS.fetch_add(1, Ordering::Relaxed);
    REQUESTED.fetch_add(bytes as u64, Ordering::Relaxed);
}
/// Called after the counting allocator has returned the storage to System.
pub fn freed(bytes: usize) {
    FREES.fetch_add(1, Ordering::Relaxed);
    RELEASED.fetch_add(bytes as u64, Ordering::Relaxed);
    LAST_FREE.fetch_max(
        SEQUENCE.fetch_add(1, Ordering::Relaxed) + 1,
        Ordering::Relaxed,
    );
}
pub(crate) fn refunded(bytes: usize) {
    REFUNDS.fetch_add(1, Ordering::Relaxed);
    REFUNDED.fetch_add(bytes as u64, Ordering::Relaxed);
    LAST_REFUND.fetch_max(
        SEQUENCE.fetch_add(1, Ordering::Relaxed) + 1,
        Ordering::Relaxed,
    );
}
