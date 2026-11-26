//! Exact immutable selection and completion publication atomics.
use super::{AtomicU8, Ordering, UNDECIDED};

#[inline]
pub(super) fn choose(decision: &AtomicU8, selected: u8) -> Result<u8, u8> {
    decision.compare_exchange(UNDECIDED, selected, Ordering::AcqRel, Ordering::Acquire)
}

#[inline]
pub(super) fn certify(completed: &AtomicU8, selected: u8) {
    completed.store(selected, Ordering::Release);
}
