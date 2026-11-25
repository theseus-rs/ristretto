//! Admission and retirement of descriptor borrows. The model tests compile this
//! same implementation with Loom atomics supplied by the parent module.
use super::{AtomicUsize, Ordering};

const RETIRED: usize = 1 << (usize::BITS - 1);
const DESTROYED: usize = RETIRED | (RETIRED >> 1);
const NOTIFY: usize = RETIRED >> 2;
const SCANNING: usize = NOTIFY >> 1;
const MAX_READERS: usize = SCANNING - 1;

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum ScanDenied {
    Busy,
    Retired,
}

#[derive(Debug, Default)]
pub(super) struct StorageState(AtomicUsize);

impl StorageState {
    /// The successful CAS admits a storage borrower, without establishing a root.
    pub(super) fn acquire(&self) -> bool {
        let mut state = self.0.load(Ordering::Acquire);
        while state < RETIRED && state & MAX_READERS != MAX_READERS {
            match self.0.compare_exchange_weak(
                state,
                state + 1,
                Ordering::AcqRel,
                Ordering::Acquire,
            ) {
                Ok(_) => return true,
                Err(observed) => state = observed,
            }
        }
        false
    }

    /// A scanner borrows storage and excludes other graph scans in one CAS.
    pub(super) fn acquire_scan(&self) -> Result<(), ScanDenied> {
        let mut state = self.0.load(Ordering::Acquire);
        loop {
            if state >= RETIRED {
                return Err(ScanDenied::Retired);
            }
            if state & SCANNING != 0 {
                return Err(ScanDenied::Busy);
            }
            match self.0.compare_exchange_weak(
                state,
                state | SCANNING,
                Ordering::AcqRel,
                Ordering::Acquire,
            ) {
                Ok(_) => return Ok(()),
                Err(actual) => state = actual,
            }
        }
    }

    pub(super) fn release_scan(&self) -> bool {
        self.0.fetch_and(!SCANNING, Ordering::AcqRel) == RETIRED | NOTIFY | SCANNING
    }

    /// The last borrower owns the notification if the sweeper has armed it.
    pub(super) fn release(&self) -> bool {
        self.0.fetch_sub(1, Ordering::AcqRel) == (RETIRED | NOTIFY) + 1
    }

    /// Closes admission permanently. True when no storage borrower remains.
    pub(super) fn retire(&self) -> bool {
        self.0.fetch_or(RETIRED, Ordering::AcqRel) == 0
    }

    /// Arm exactly once after charging deferred work. If the last borrower
    /// already left, the sweeper itself must publish the ready task.
    pub(super) fn arm_notification(&self) -> bool {
        let previous = self.0.fetch_or(NOTIFY, Ordering::AcqRel);
        debug_assert_eq!(previous & !(MAX_READERS | SCANNING), RETIRED);
        previous == RETIRED
    }

    pub(super) fn ready(&self) -> bool {
        self.0.load(Ordering::Acquire) & !NOTIFY == RETIRED
    }

    /// Exactly one caller may destroy storage, after observing every borrow end.
    pub(super) fn destroy(&self) -> bool {
        match self
            .0
            .compare_exchange(RETIRED, DESTROYED, Ordering::AcqRel, Ordering::Acquire)
        {
            Ok(_) => true,
            Err(state) if state == RETIRED | NOTIFY => self
                .0
                .compare_exchange(state, DESTROYED, Ordering::AcqRel, Ordering::Acquire)
                .is_ok(),
            Err(_) => false,
        }
    }
}
