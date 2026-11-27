//! Scoped coverage of trace actions. The parent supplies native or Loom atomics.
use super::{AtomicBool, Ordering};

#[derive(Debug, Default)]
pub(crate) struct Certificate(AtomicBool);

impl Certificate {
    /// Only the exclusive writer may publish, after constructing a projection
    /// that keeps every direct strong field independently rooted and needs no
    /// other parent trace actions for its complete lifetime. Empty graphs qualify.
    pub(crate) fn publish(&self) {
        self.0.store(true, Ordering::Release);
    }

    /// Revoke before demoting roots, changing trace actions or unlocking the writer.
    pub(crate) fn revoke(&self) {
        self.0.store(false, Ordering::Release);
    }

    /// This RMW observes the current modification-order value. A stale ordinary
    /// load of a prior guard's certificate must not authorize a later covered scan.
    /// Success linearizes a scan with independently covered edges; it does not
    /// read the borrowed payload. Later root removal retains its normal barriers.
    pub(crate) fn certify(&self) -> bool {
        self.0
            .compare_exchange(true, true, Ordering::Acquire, Ordering::Relaxed)
            .is_ok()
    }
}
