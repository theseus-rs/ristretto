//! Token allocator for the experimental canonical compiled tier.
//!
//! One process-wide source leases disjoint ranges of positive `i64` values. A range
//! owns only two integers, never a managed object or historical range list. IDs
//! are never returned to the source, including when a range is dropped unused.
//!
//! Reservation performs one weak compare-and-exchange, with no retry loop or
//! lock fallback. `Pending` means the host must yield/retry outside generated
//! code and without holding a table borrow, while retaining owning arguments or
//! results. This is an operation-count bound, not a wall-clock latency claim.
//! Local issuance performs no atomic operation. Neither path allocates.
//!
//! Tokens carry no invocation prefix. The frame table accepts only exact, live
//! table entries and keeps its separate frame-registry
//! identity check. A range-membership check is insufficient: an ID in a leased
//! range may be unissued, dead, or discarded. Null remains the integer zero.

use std::fmt;
use std::num::NonZeroU64;
#[cfg(target_has_atomic = "64")]
use std::sync::atomic::{AtomicU64, Ordering};
use std::task::Poll;

/// The first unrepresentable positive `i64`, used only as a frontier sentinel.
#[cfg(target_has_atomic = "64")]
const END_EXCLUSIVE: u64 = 1_u64 << 63;

#[cfg(target_has_atomic = "64")]
static PROCESS_SOURCE: Source = Source {
    next: AtomicU64::new(1),
};

/// Terminal reservation failures. Contention is reported separately as `Pending`.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum LeaseError {
    /// Every positive `i64` has already been leased. This never resets or wraps.
    Exhausted,
    /// The target cannot provide native 64-bit atomics without a lock fallback.
    #[cfg(not(target_has_atomic = "64"))]
    UnsupportedAtomic64,
}

impl fmt::Display for LeaseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Exhausted => formatter.write_str("canonical token identity exhausted"),
            #[cfg(not(target_has_atomic = "64"))]
            Self::UnsupportedAtomic64 => {
                formatter.write_str("canonical tokens require native 64-bit atomics")
            }
        }
    }
}

impl std::error::Error for LeaseError {}

/// Exclusive ownership of a leased ID range; intentionally neither `Copy` nor `Clone`.
#[derive(Debug)]
pub(crate) struct TokenRange {
    next: u64,
    end_exclusive: u64,
}

impl TokenRange {
    /// Try to lease at most `requested` IDs from the process-wide source.
    ///
    /// The final lease can be shorter than requested, but is never empty. Each
    /// call makes at most one weak CAS attempt; contention or spurious failure
    /// returns `Pending` without reserving any IDs. The caller must not spin in
    /// a native callback or a poll with a claimed bounded amount of work.
    pub(crate) fn try_acquire(requested: NonZeroU64) -> Poll<Result<Self, LeaseError>> {
        #[cfg(target_has_atomic = "64")]
        {
            PROCESS_SOURCE.try_acquire(requested)
        }
        #[cfg(not(target_has_atomic = "64"))]
        {
            let _ = requested;
            Poll::Ready(Err(LeaseError::UnsupportedAtomic64))
        }
    }

    /// Issue the next ID, or return `None` when host-side refill is required.
    ///
    /// This neither allocates nor consults the process-wide atomic source. The
    /// caller must finish validation/capacity checks and identity deduplication
    /// before issuing an ID. An ID burned by a later error must never be reused.
    pub(crate) fn issue(&mut self) -> Option<i64> {
        if self.next == self.end_exclusive {
            return None;
        }
        // All constructors enforce 1 <= next < end_exclusive <= 2^63.
        let token = i64::try_from(self.next).ok()?;
        self.next += 1;
        Some(token)
    }

    /// Number of IDs still available without another reservation.
    pub(crate) const fn remaining(&self) -> u64 {
        self.end_exclusive - self.next
    }
}

/// Only the global instance is reachable outside this module. Test sources do
/// not share an identity domain with the running VM and must remain private.
#[cfg(target_has_atomic = "64")]
struct Source {
    next: AtomicU64,
}

#[cfg(target_has_atomic = "64")]
impl Source {
    fn try_acquire(&self, requested: NonZeroU64) -> Poll<Result<TokenRange, LeaseError>> {
        let observed = self.next.load(Ordering::Relaxed);
        self.try_acquire_observed(observed, requested)
    }

    // Kept separate so tests can deterministically inject an intervening lease.
    fn try_acquire_observed(
        &self,
        observed: u64,
        requested: NonZeroU64,
    ) -> Poll<Result<TokenRange, LeaseError>> {
        let Some(available) = END_EXCLUSIVE.checked_sub(observed) else {
            return Poll::Ready(Err(LeaseError::Exhausted));
        };
        if available == 0 {
            return Poll::Ready(Err(LeaseError::Exhausted));
        }
        let count = requested.get().min(available);
        let Some(next) = observed.checked_add(count) else {
            return Poll::Ready(Err(LeaseError::Exhausted));
        };
        // Relaxed ordering is enough: IDs publish no object data. Uniqueness
        // follows from atomic modification order and exclusive range ownership.
        // Weak CAS exposes a failed attempt instead of a hidden LL/SC retry.
        match self
            .next
            .compare_exchange_weak(observed, next, Ordering::Relaxed, Ordering::Relaxed)
        {
            Ok(_) => Poll::Ready(Ok(TokenRange {
                next: observed,
                end_exclusive: next,
            })),
            Err(_) => Poll::Pending,
        }
    }
}

#[cfg(all(test, target_has_atomic = "64"))]
#[path = "token_ranges_tests.rs"]
mod tests;
