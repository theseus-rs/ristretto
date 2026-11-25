//! Process-unique, address-independent object identities.
use crate::{Error, Result};
use portable_atomic::AtomicU64;
use std::cell::Cell;
use std::sync::atomic::Ordering;

const BATCH: u64 = 64;
static NEXT: AtomicU64 = AtomicU64::new(1);

#[derive(Clone, Copy, Default)]
struct Range {
    next: u64,
    end: u64,
}

impl Range {
    fn reserve(source: &AtomicU64, count: u64) -> Result<Self> {
        // Preserve the original allocator's terminal sentinel. A partial final
        // range is usable; the counter never wraps or publishes zero again.
        source
            .fetch_update(Ordering::Relaxed, Ordering::Relaxed, |next| {
                (next < u64::MAX).then(|| next.saturating_add(count))
            })
            .map(|next| Self {
                next,
                end: next.saturating_add(count),
            })
            .map_err(|_| Error::OutOfMemory)
    }

    fn take(&mut self, source: &AtomicU64) -> Result<ObjectId> {
        if self.next == self.end {
            *self = Self::reserve(source, BATCH)?;
        }
        let id = ObjectId(self.next);
        self.next += 1; // next < end <= MAX: cannot wrap.
        Ok(id)
    }
}

thread_local! {
    // No heap/context ownership and no destructor. Migration can leave holes,
    // but unused identities are never returned or handed to another thread.
    static IDS: Cell<Range> = const { Cell::new(Range { next: 0, end: 0 }) };
}

/// An opaque managed object identity. Zero represents null outside this type.
/// Identities are never recycled, including across heaps.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ObjectId(u64);

impl ObjectId {
    pub(crate) fn allocate() -> Result<Self> {
        IDS.try_with(|cache| {
            let mut range = cache.get();
            let id = range.take(&NEXT)?;
            cache.set(range);
            Ok(id)
        })
        // Some hosts can deny TLS access during teardown. Keep that cold path
        // usable without recovering or recycling any thread's unused range.
        .unwrap_or_else(|_| Range::reserve(&NEXT, 1).map(|range| Self(range.next)))
    }

    /// Encodes an identity for the VM/JIT reference ABI, without exposing an address.
    #[must_use]
    pub fn to_i64(self) -> i64 {
        i64::from_ne_bytes(self.0.to_ne_bytes())
    }

    pub(crate) fn from_i64(value: i64) -> Self {
        Self(u64::from_ne_bytes(value.to_ne_bytes()))
    }
}

#[cfg(test)]
#[expect(clippy::panic_in_result_fn)]
mod tests {
    use super::*;

    #[test]
    fn cached_ids_do_not_touch_the_shared_counter_until_refill() -> Result<()> {
        let counter = AtomicU64::new(1);
        let mut range = Range::default();
        for expected in 1..=BATCH {
            assert_eq!(range.take(&counter)?.0, expected);
            assert_eq!(counter.load(Ordering::Relaxed), BATCH + 1);
        }
        assert_eq!(range.take(&counter)?.0, BATCH + 1);
        assert_eq!(counter.load(Ordering::Relaxed), 2 * BATCH + 1);
        Ok(())
    }

    #[test]
    fn partial_final_ranges_exhaust_without_wrapping_or_invalidating_reserved_ids() -> Result<()> {
        let counter = AtomicU64::new(u64::MAX - BATCH - 2);
        let mut first = Range::default();
        let start = first.take(&counter)?.0;
        let mut last = Range::default();
        assert_eq!(last.take(&counter)?.0, u64::MAX - 2);
        assert_eq!(last.take(&counter)?.0, u64::MAX - 1);
        assert!(last.take(&counter).is_err());
        assert_eq!(counter.load(Ordering::Relaxed), u64::MAX);
        for offset in 1..BATCH {
            assert_eq!(first.take(&counter)?.0, start + offset);
        }
        assert!(first.take(&counter).is_err());
        assert!(Range::reserve(&counter, 1).is_err());
        Ok(())
    }

    #[test]
    fn abandoned_ranges_are_not_recycled() -> Result<()> {
        let counter = AtomicU64::new(1);
        assert_eq!(Range::default().take(&counter)?.0, 1);
        assert_eq!(Range::default().take(&counter)?.0, BATCH + 1);
        Ok(())
    }

    #[cfg(not(target_family = "wasm"))]
    #[test]
    fn concurrent_refills_produce_unique_ids() -> Result<()> {
        let counter = AtomicU64::new(1);
        let mut ids = std::thread::scope(|scope| -> Result<Vec<ObjectId>> {
            let handles: Vec<_> = (0..4)
                .map(|_| {
                    let counter = &counter;
                    scope.spawn(move || {
                        let mut range = Range::default();
                        (0..129)
                            .map(|_| range.take(counter))
                            .collect::<Result<Vec<_>>>()
                    })
                })
                .collect();
            let mut ids = Vec::new();
            for handle in handles {
                ids.extend(
                    handle
                        .join()
                        .map_err(|_| Error::SyncError("identity worker panicked".into()))??,
                );
            }
            Ok(ids)
        })?;
        ids.sort_unstable();
        assert_eq!(ids.len(), 4 * 129);
        assert!(ids.windows(2).all(|pair| pair.first() != pair.get(1)));
        assert!(ids.iter().all(|id| id.0 > 0 && id.0 < u64::MAX));
        Ok(())
    }
}
