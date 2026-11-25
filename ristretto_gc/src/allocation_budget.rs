//! Owning managed-payload reservations, independent of allocation-pressure statistics.
use crate::{Error, Result};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Weak};

#[derive(Debug)]
pub(crate) struct AllocationBudget {
    collector: Weak<crate::GarbageCollector>,
    limit: usize,
    used: AtomicUsize,
}

impl AllocationBudget {
    #[cfg(test)]
    pub(crate) fn new(limit: usize) -> Self {
        Self::with_collector(limit, Weak::new())
    }

    pub(crate) fn with_collector(limit: usize, collector: Weak<crate::GarbageCollector>) -> Self {
        Self {
            collector,
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
    #[cfg(not(target_family = "wasm"))]
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

    pub(crate) fn reserve(self: &Arc<Self>, bytes: usize) -> Result<Reservation> {
        self.try_charge(bytes)?;
        Ok(Reservation {
            budget: self.clone(),
            bytes,
        })
    }
}

/// A non-clonable charge. Moving it transfers the obligation to refund once.
pub(crate) struct Reservation {
    budget: Arc<AllocationBudget>,
    bytes: usize,
}

impl Drop for Reservation {
    fn drop(&mut self) {
        if let Some(collector) = self.budget.collector.upgrade() {
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
        let first = budget.reserve(5)?;
        assert!(budget.reserve(4).is_err());
        assert_eq!(budget.used(), 5);
        drop(first);
        assert_eq!(budget.used(), 0);
        let exact = budget.reserve(8)?;
        assert_eq!(budget.used(), 8);
        drop(exact);
        assert_eq!(budget.used(), 0);
        Ok(())
    }

    #[test]
    fn overflow_does_not_wrap_the_budget() -> Result<()> {
        let budget = Arc::new(AllocationBudget::new(usize::MAX));
        let charge = budget.reserve(usize::MAX)?;
        assert!(budget.reserve(1).is_err());
        assert_eq!(budget.used(), usize::MAX);
        drop(charge);
        assert_eq!(budget.used(), 0);
        Ok(())
    }
}
