//! One allocation per slow typed-root attempt. Its decision cannot be reused by
//! another publisher. Removed roots still owe any traversal that was committed.
use super::{AtomicU8, Ordering};
const PENDING: u8 = 0;
const CANCELLED: u8 = 1;
const COMMITTED: u8 = 2;
const RELEASED: u8 = 3;
const REMOVING: u8 = 4;
const VETOED: u8 = 5;

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
        match self.cancel_or_observe() {
            // A scanner that reports the root defeats an in-progress fast drop.
            REMOVING => matches!(
                self.0
                    .compare_exchange(REMOVING, VETOED, Ordering::AcqRel, Ordering::Acquire),
                Ok(_) | Err(VETOED)
            ),
            COMMITTED | VETOED => true,
            _ => false,
        }
    }

    pub(crate) fn owes_trace(&self) -> bool {
        matches!(
            self.cancel_or_observe(),
            COMMITTED | REMOVING | VETOED | RELEASED
        )
    }

    pub(crate) fn live(&self) -> bool {
        matches!(
            self.0.load(Ordering::Acquire),
            COMMITTED | REMOVING | VETOED
        )
    }

    pub(crate) fn begin_removal(&self) -> bool {
        self.0
            .compare_exchange(COMMITTED, REMOVING, Ordering::AcqRel, Ordering::Acquire)
            .is_ok()
    }

    pub(crate) fn removal_pending(&self) -> bool {
        self.0
            .compare_exchange(REMOVING, REMOVING, Ordering::AcqRel, Ordering::Acquire)
            .is_ok()
    }

    pub(crate) fn finish_removal(&self) -> bool {
        self.0
            .compare_exchange(REMOVING, RELEASED, Ordering::AcqRel, Ordering::Acquire)
            .is_ok()
    }

    pub(crate) fn take(&self) -> bool {
        let mut state = self.0.load(Ordering::Acquire);
        while matches!(state, COMMITTED | REMOVING | VETOED) {
            match self
                .0
                .compare_exchange(state, RELEASED, Ordering::AcqRel, Ordering::Acquire)
            {
                Ok(_) => return true,
                Err(seen) => state = seen,
            }
        }
        false
    }
}
