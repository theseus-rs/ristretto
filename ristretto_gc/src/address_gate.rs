//! Primary-address ownership. The integration model supplies Loom atomics.
use super::{AtomicUsize, Ordering};

const WRITER: usize = 1 << (usize::BITS - 1);
const SHARED: usize = WRITER >> 1;
const CLEANED: usize = SHARED >> 1;
const READERS: usize = CLEANED - 1;

#[derive(Debug, Default)]
pub(super) struct Gate(AtomicUsize);

#[derive(Debug, PartialEq, Eq)]
pub(super) enum ReadMode {
    Primary,
    Shared,
    Vacant,
}

#[derive(Debug, PartialEq, Eq)]
pub(super) enum WriteMode {
    Exclusive,
    Promote,
    Shared,
}

impl Gate {
    pub(super) fn read(&self) -> ReadMode {
        let old = self.0.fetch_add(1, Ordering::Acquire);
        if old & READERS == READERS {
            std::process::abort();
        }
        if old & SHARED != 0 {
            ReadMode::Shared
        } else if old & WRITER != 0 {
            ReadMode::Vacant
        } else {
            ReadMode::Primary
        }
    }

    pub(super) fn release(&self) -> bool {
        self.0.fetch_sub(1, Ordering::Release) == SHARED | 1 && self.claim_cleanup()
    }

    fn claim_cleanup(&self) -> bool {
        self.0
            .compare_exchange(
                SHARED,
                SHARED | CLEANED,
                Ordering::AcqRel,
                Ordering::Relaxed,
            )
            .is_ok()
    }

    pub(super) fn write(&self) -> WriteMode {
        if self.0.load(Ordering::Acquire) & SHARED != 0 {
            return WriteMode::Shared;
        }
        let previous = self.0.fetch_or(WRITER, Ordering::AcqRel);
        assert_eq!(previous & WRITER, 0, "overlapping address writers");
        if previous & SHARED != 0 {
            self.finish_exclusive();
            WriteMode::Shared
        } else if previous & READERS == 0 {
            WriteMode::Exclusive
        } else {
            WriteMode::Promote
        }
    }

    pub(super) fn finish_exclusive(&self) {
        self.0.fetch_and(!WRITER, Ordering::Release);
    }

    pub(super) fn publish_shared(&self) -> bool {
        self.0.fetch_or(SHARED, Ordering::Release);
        self.finish_exclusive();
        self.claim_cleanup()
    }

    #[cfg(test)]
    pub(super) fn cleaned(&self) -> bool {
        self.0.load(Ordering::Acquire) & CLEANED != 0
    }
}
