//! The exact owning-link transition, also exercised with Loom atomics.
use super::{AtomicU64, Ordering};

pub(super) fn replace(token: &AtomicU64, old: u64, new: u64) -> bool {
    token
        .compare_exchange(old, new, Ordering::AcqRel, Ordering::Acquire)
        .is_ok()
}
