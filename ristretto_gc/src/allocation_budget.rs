//! Owning managed-payload reservations, independent of allocation-pressure statistics.
use crate::{Error, Result};
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

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

    pub(crate) fn reserve(self: &Arc<Self>, bytes: usize) -> Result<Reservation> {
        self.used
            .try_update(Ordering::AcqRel, Ordering::Acquire, |used| {
                used.checked_add(bytes).filter(|total| *total <= self.limit)
            })
            .map_err(|_| Error::OutOfMemory)?;
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
        self.budget.used.fetch_sub(self.bytes, Ordering::AcqRel);
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
