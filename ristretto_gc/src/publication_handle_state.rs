//! A zero handle count is terminal even if announcement and headers survive.
use super::{AtomicUsize, Ordering};

const ANNOUNCED: usize = 1 << (usize::BITS - 1);
const COUNT_MASK: usize = ANNOUNCED - 1;

pub(super) struct State(AtomicUsize);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum AcquireError {
    Foreign,
    Closed,
    Contended,
    Exhausted,
}

impl State {
    pub(super) fn new() -> Self {
        Self(AtomicUsize::new(1))
    }

    #[cfg(test)]
    pub(super) fn with_count_for_test(count: usize, announced: bool) -> Self {
        assert!(count <= COUNT_MASK);
        Self(AtomicUsize::new(
            count | if announced { ANNOUNCED } else { 0 },
        ))
    }

    /// One load and at most one CAS. A failed attempt grants no resource access.
    pub(super) fn acquire(&self) -> Result<(), AcquireError> {
        let current = self.0.load(Ordering::Acquire);
        if current & COUNT_MASK == 0 {
            return Err(AcquireError::Closed);
        }
        if current & COUNT_MASK == COUNT_MASK {
            return Err(AcquireError::Exhausted);
        }
        let next = current + 1;
        self.0
            .compare_exchange(current, next, Ordering::Acquire, Ordering::Relaxed)
            .map(|_| ())
            .map_err(|_| AcquireError::Contended)
    }

    /// The last release acquires all preceding releases before extracting resources.
    pub(super) fn release(&self) -> bool {
        let previous = self.0.fetch_sub(1, Ordering::AcqRel);
        assert!(
            previous & COUNT_MASK != 0,
            "released a closed publication handle"
        );
        previous & COUNT_MASK == 1
    }

    /// Couple irreversible announcement to the count so a helper's acquire/release
    /// cannot restore the plain-one value expected by exclusive restoration.
    pub(super) fn announce(&self) {
        let previous = self.0.fetch_or(ANNOUNCED, Ordering::AcqRel);
        assert!(
            previous & COUNT_MASK != 0,
            "announced a closed publication handle"
        );
    }

    pub(super) fn announced(&self) -> bool {
        self.0.load(Ordering::Acquire) & ANNOUNCED != 0
    }

    /// Only the sole unannounced handle can recover exclusive ownership.
    pub(super) fn close_if_unique(&self) -> bool {
        self.0
            .compare_exchange(1, 0, Ordering::AcqRel, Ordering::Relaxed)
            .is_ok()
    }
}
