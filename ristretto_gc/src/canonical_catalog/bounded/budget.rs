//! Logical footprint credits, distinct from RSS and allocator bookkeeping.
use super::{
    Arc, AtomicUsize, BoundedCatalog, Capacities, Catalog, ConfigError, Ordering, Participant,
};
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Footprint {
    backing: usize,
    participants: usize,
    total: usize,
}
impl Footprint {
    pub fn for_capacities(capacities: Capacities) -> Result<Self, ConfigError> {
        Self::for_records::<()>(capacities)
    }
    pub fn for_records<R: super::Payload>(capacities: Capacities) -> Result<Self, ConfigError> {
        let overflow = ConfigError::CapacityOverflow;
        let backing = Catalog::<R>::record_backing_bytes(
            capacities.registry,
            capacities.controls,
            capacities.memberships,
            capacities.cells,
        )
        .ok_or(overflow)?
        .checked_add(super::pool::arc_requested_bytes::<BoundedCatalog<R>>().ok_or(overflow)?)
        .ok_or(overflow)?;
        let participants = capacities
            .participants()
            .checked_mul(
                size_of::<Participant<R>>().max(size_of::<Participant<R, super::GuardedCommand>>()),
            )
            .ok_or(overflow)?;
        Ok(Self {
            backing,
            participants,
            total: backing.checked_add(participants).ok_or(overflow)?,
        })
    }
    pub fn backing(self) -> usize {
        self.backing
    }
    pub fn participants(self) -> usize {
        self.participants
    }
    pub fn total(self) -> usize {
        self.total
    }
}

/// Externally supplied budget owner. Its own allocation is outside the catalog's
/// footprint; cloning its `Arc` does not allocate. Reservation makes one CAS attempt.
pub struct Budget {
    limit: usize,
    used: AtomicUsize,
}
impl Budget {
    pub fn new(limit: usize) -> Arc<Self> {
        Arc::new(Self {
            limit,
            used: AtomicUsize::new(0),
        })
    }
    pub fn limit(&self) -> usize {
        self.limit
    }
    pub fn used(&self) -> usize {
        self.used.load(Ordering::Acquire)
    }
    pub(super) fn reserve(self: &Arc<Self>, bytes: usize) -> Result<Charge, ConfigError> {
        let owner = self.clone();
        let old = self.used.load(Ordering::Relaxed);
        let after = old
            .checked_add(bytes)
            .filter(|after| *after <= self.limit)
            .ok_or(ConfigError::BudgetExceeded)?;
        self.used
            .compare_exchange(old, after, Ordering::Acquire, Ordering::Relaxed)
            .map_err(|_| ConfigError::BudgetContended)?;
        Ok(Charge { owner, bytes })
    }
}
pub(super) struct Charge {
    owner: Arc<Budget>,
    bytes: usize,
}
impl Charge {
    pub(super) fn bytes(&self) -> usize {
        self.bytes
    }
}
impl Drop for Charge {
    fn drop(&mut self) {
        let old = self.owner.used.fetch_sub(self.bytes, Ordering::Release);
        debug_assert!(old >= self.bytes, "duplicate budget refund");
    }
}
