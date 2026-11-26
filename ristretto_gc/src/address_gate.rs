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
        // Refuse saturation before changing the ownership bits, even if the
        // overflowing caller is suspended immediately before aborting.
        let mut old = self.0.load(Ordering::Relaxed);
        loop {
            if old & READERS == READERS {
                std::process::abort();
            }
            match self
                .0
                .compare_exchange_weak(old, old + 1, Ordering::Acquire, Ordering::Relaxed)
            {
                Ok(_) => break,
                Err(current) => old = current,
            }
        }
        if old & SHARED != 0 {
            ReadMode::Shared
        } else if old & WRITER != 0 {
            ReadMode::Vacant
        } else {
            ReadMode::Primary
        }
    }

    /// The caller owns one primary reader and is the sole authorized writer.
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

    #[cfg(all(test, not(target_family = "wasm")))]
    pub(super) fn cleaned(&self) -> bool {
        self.0.load(Ordering::Acquire) & CLEANED != 0
    }
}
