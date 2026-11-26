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
        let mut old = self.0.load(Ordering::Relaxed);
        loop {
            let (mode, next) = if old & SHARED != 0 {
                (ReadMode::Shared, old)
            } else if old & WRITER != 0 {
                (ReadMode::Vacant, old)
            } else {
                if old & READERS == READERS {
                    std::process::abort();
                }
                (ReadMode::Primary, old + 1)
            };
            // Validate the observed mode against the current slot even when
            // the reader needs no primary admission. Acquire observes a promoted slot.
            match self
                .0
                .compare_exchange_weak(old, next, Ordering::Acquire, Ordering::Relaxed)
            {
                Ok(_) => return mode,
                Err(current) => old = current,
            }
        }
    }

    /// The caller owns one primary reader and is the sole authorized writer.
    #[cfg(any(
        not(feature = "experimental-identity-index"),
        all(test, not(target_family = "wasm"))
    ))]
    pub(super) fn take_reader(&self) -> bool {
        self.0
            .compare_exchange(1, WRITER, Ordering::AcqRel, Ordering::Relaxed)
            .is_ok()
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
            self.0.fetch_and(!WRITER, Ordering::Release);
            WriteMode::Shared
        } else if previous & READERS == 0 {
            WriteMode::Exclusive
        } else {
            WriteMode::Promote
        }
    }

    /// Only an exclusive writer acquired from zero or from its own sole reader
    /// may finish this way. Observers of WRITER own no count that could be lost.
    pub(super) fn finish_exclusive(&self) {
        self.0.store(0, Ordering::Release);
    }

    pub(super) fn publish_shared(&self) -> bool {
        self.0.fetch_or(SHARED, Ordering::Release);
        self.0.fetch_and(!WRITER, Ordering::Release);
        self.claim_cleanup()
    }

    #[cfg(all(test, not(target_family = "wasm")))]
    pub(super) fn cleaned(&self) -> bool {
        self.0.load(Ordering::Acquire) & CLEANED != 0
    }
}
