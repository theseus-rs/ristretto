//! Exact-generation acquisition; zero is free and one excludes access/reuse.
use super::{AllocationError, AtomicU64, Ordering};

pub(crate) const BITS: u32 = 24;
pub(crate) const MASK: u64 = (1 << BITS) - 1;
pub(crate) const MAX_GENERATION: u64 = u64::MAX >> BITS;
const BUSY: u64 = 1;
const FIRST_OWNER: u64 = 2;

pub(crate) struct State(AtomicU64);
impl State {
    /// Setup encodes the remaining allocation budget; claims use a constant
    /// ceiling. The pool constructor checks 1..=MAX_GENERATION before calling.
    pub(crate) fn with_generation_limit(limit: u64) -> Self {
        debug_assert!((1..=MAX_GENERATION).contains(&limit));
        Self(AtomicU64::new((MAX_GENERATION - limit) << BITS))
    }

    /// One observation and at most one CAS. The returned generation is private
    /// until publish_initialized; no claim changes an exhausted slot.
    #[cfg(test)]
    pub(crate) fn claim(&self) -> Option<u64> {
        self.claim_available().ok()
    }

    pub(crate) fn claim_available(&self) -> Result<u64, AllocationError> {
        let old = self.0.load(Ordering::Relaxed);
        let generation = old >> BITS;
        if generation == MAX_GENERATION {
            return Err(AllocationError::GenerationExhausted);
        }
        if old & MASK != 0 {
            return Err(AllocationError::Unavailable);
        }
        self.0
            .compare_exchange(
                old,
                ((generation + 1) << BITS) | BUSY,
                Ordering::Acquire,
                Ordering::Relaxed,
            )
            .map(|_| generation + 1)
            .map_err(|_| AllocationError::Contended)
    }

    pub(crate) fn publish_initialized(&self, generation: u64) {
        self.0
            .store((generation << BITS) | FIRST_OWNER, Ordering::Release);
    }

    /// One attempt, including when a live generation has competing owners.
    pub(crate) fn acquire(&self, generation: u64) -> bool {
        let old = self.0.load(Ordering::Relaxed);
        let references = old & MASK;
        generation != 0
            && old >> BITS == generation
            && (FIRST_OWNER..MASK).contains(&references)
            && self
                .0
                .compare_exchange(old, old + 1, Ordering::Acquire, Ordering::Relaxed)
                .is_ok()
    }

    /// Caller owns one count. The final decrement leaves BUSY, preventing both
    /// acquisition and reuse throughout destruction; it does not publish free.
    pub(crate) fn release(&self) -> bool {
        let old = self.0.fetch_sub(1, Ordering::Release);
        debug_assert!(old & MASK >= FIRST_OWNER);
        if old & MASK == FIRST_OWNER {
            // The final decrement leaves BUSY and no other writer can change
            // this word until publish_free. Acquire the release sequence of
            // all prior decrements before destruction or reuse. A load also
            // makes this synchronization visible to ThreadSanitizer, which
            // does not model atomic fences.
            self.0.load(Ordering::Acquire);
            true
        } else {
            false
        }
    }

    /// Only the final releaser calls this after the value's drop glue finishes.
    pub(crate) fn publish_free(&self, generation: u64) {
        debug_assert_eq!(self.0.load(Ordering::Relaxed), (generation << BITS) | BUSY);
        self.0.store(generation << BITS, Ordering::Release);
    }

    pub(crate) fn occupied(&self) -> bool {
        self.0.load(Ordering::Acquire) & MASK != 0
    }

    #[cfg(test)]
    pub(crate) fn free_at(generation: u64) -> Self {
        assert!(generation <= MAX_GENERATION);
        Self(AtomicU64::new(generation << BITS))
    }
}
