//! Owning managed-payload reservations, independent of allocation-pressure statistics.
use crate::{Error, Result};
use std::sync::atomic::{AtomicUsize, Ordering};
#[cfg(test)]
use std::sync::{Arc, Weak};

#[derive(Debug)]
pub(crate) struct AllocationBudget {
    limit: usize,
    used: AtomicUsize,
}

impl AllocationBudget {
    pub(crate) fn new(limit: usize) -> Self {
        Self {
            limit,
            used: AtomicUsize::new(0),
        }
    }

    pub(crate) fn used(&self) -> usize {
        self.used.load(Ordering::Acquire)
    }

    /// Publish waiter registration into the same modification order as refunds.
    /// A later release acquires that registration; an earlier refund is visible
    /// in the returned charge. An acquire-only load would allow both sides to miss.
    pub(crate) fn used_after_registration(&self) -> usize {
        self.used.fetch_or(0, Ordering::AcqRel)
    }

    pub(crate) fn try_charge(&self, bytes: usize) -> Result<()> {
        self.used
            .try_update(Ordering::AcqRel, Ordering::Acquire, |used| {
                used.checked_add(bytes).filter(|total| *total <= self.limit)
            })
            .map_err(|_| Error::OutOfMemory)?;
        Ok(())
    }

    pub(crate) fn release(&self, bytes: usize) -> usize {
        self.used.fetch_sub(bytes, Ordering::AcqRel)
    }

    pub(crate) fn record(&self, bytes: usize) {
        self.used.fetch_add(bytes, Ordering::AcqRel);
    }

    #[cfg(test)]
    pub(crate) fn reserve(
        self: &Arc<Self>,
        bytes: usize,
        collector: Weak<crate::GarbageCollector>,
    ) -> Result<Reservation> {
        self.try_charge(bytes)?;
        Ok(Reservation {
            collector,
            budget: self.clone(),
            bytes,
        })
    }
}

/// A non-clonable charge. Moving it transfers the obligation to refund once.
#[cfg(test)]
pub(crate) struct Reservation {
    collector: Weak<crate::GarbageCollector>,
    budget: Arc<AllocationBudget>,
    bytes: usize,
}

#[cfg(test)]
impl Drop for Reservation {
    fn drop(&mut self) {
        if let Some(collector) = self.collector.upgrade() {
            // A private refund makes space available to ordinary allocation waiters.
            collector.cancel_allocation(self.bytes);
        } else {
            self.budget.release(self.bytes);
        }
    }
}

#[cfg(test)]
#[expect(
    clippy::panic_in_result_fn,
    reason = "tests propagate setup errors and assert invariants"
)]
mod tests {
    use super::*;

    #[test]
    fn failed_and_cancelled_reservations_preserve_exact_credit() -> Result<()> {
        let budget = Arc::new(AllocationBudget::new(8));
        let first = budget.reserve(5, Weak::new())?;
        assert!(budget.reserve(4, Weak::new()).is_err());
        assert_eq!(budget.used(), 5);
        drop(first);
        assert_eq!(budget.used(), 0);
        let exact = budget.reserve(8, Weak::new())?;
        assert_eq!(budget.used(), 8);
        drop(exact);
        assert_eq!(budget.used(), 0);
        Ok(())
    }

    #[cfg(not(target_family = "wasm"))]
    #[test]
    fn a_refund_and_a_new_observer_cannot_both_miss_the_registration() {
        use loom::sync::Arc;
        use loom::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
        loom::model(|| {
            let used = Arc::new(AtomicUsize::new(2));
            let registered = Arc::new(AtomicBool::new(false));
            let observed = Arc::new(AtomicBool::new(false));
            let refund = {
                let (used, registered, observed) =
                    (used.clone(), registered.clone(), observed.clone());
                loom::thread::spawn(move || {
                    used.fetch_sub(2, Ordering::AcqRel);
                    if registered.load(Ordering::Acquire) {
                        observed.store(true, Ordering::Release);
                    }
                    // A thief takes half the space. A later two-byte request
                    // cannot use the earlier offer.
                    used.fetch_add(1, Ordering::AcqRel);
                })
            };
            registered.store(true, Ordering::Release);
            let charge = used.fetch_or(0, Ordering::AcqRel);
            refund.join().expect("refund finished");
            if charge != 1 {
                assert!(charge == 0 || observed.load(Ordering::Acquire));
            }
        });
    }

    #[test]
    fn overflow_does_not_wrap_the_budget() -> Result<()> {
        let budget = Arc::new(AllocationBudget::new(usize::MAX));
        let charge = budget.reserve(usize::MAX, Weak::new())?;
        assert!(budget.reserve(1, Weak::new()).is_err());
        assert_eq!(budget.used(), usize::MAX);
        drop(charge);
        assert_eq!(budget.used(), 0);
        Ok(())
    }
}
