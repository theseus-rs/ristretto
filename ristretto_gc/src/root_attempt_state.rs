//! One allocation per slow typed-root attempt. Its decision cannot be reused by
//! another publisher. Removed roots still owe any traversal that was committed.
use super::{AtomicU8, Ordering};
const PENDING: u8 = 0;
const CANCELLED: u8 = 1;
const COMMITTED: u8 = 2;
const RELEASED: u8 = 3;

#[derive(Debug, Default)]
pub(crate) struct Decision(AtomicU8);

impl Decision {
    pub(crate) fn commit(&self) -> bool {
        self.0
            .compare_exchange(PENDING, COMMITTED, Ordering::AcqRel, Ordering::Acquire)
            .is_ok()
    }

    fn cancel_or_observe(&self) -> u8 {
        self.0
            .compare_exchange(PENDING, CANCELLED, Ordering::AcqRel, Ordering::Acquire)
            .unwrap_or_else(|seen| seen)
    }

    pub(crate) fn scan(&self) -> bool {
        self.cancel_or_observe() == COMMITTED
    }
    pub(crate) fn owes_trace(&self) -> bool {
        matches!(self.cancel_or_observe(), COMMITTED | RELEASED)
    }
    pub(crate) fn live(&self) -> bool {
        self.0.load(Ordering::Acquire) == COMMITTED
    }
    pub(crate) fn take(&self) -> bool {
        self.0
            .compare_exchange(COMMITTED, RELEASED, Ordering::AcqRel, Ordering::Acquire)
            .is_ok()
    }
}
