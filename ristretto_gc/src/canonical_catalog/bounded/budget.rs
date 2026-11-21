//! Logical footprint credits, distinct from RSS and allocator bookkeeping.
use super::{
    Arc, AtomicUsize, BoundedCatalog, Capacities, Catalog, ConfigError, Ordering, Participant,
};
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Footprint {
    backing: usize,
    participants: usize,
    root_tokens: usize,
    edge_tokens: usize,
    total: usize,
}
impl Footprint {
    pub fn for_capacities(capacities: Capacities) -> Result<Self, ConfigError> {
        Self::for_records::<()>(capacities)
    }
    pub fn for_records<R: super::Payload>(capacities: Capacities) -> Result<Self, ConfigError> {
        if capacities.reference_slots != 0 {
            return Err(ConfigError::UnsupportedReferences);
        }
        let overflow = ConfigError::CapacityOverflow;
        let backing = Catalog::<R>::record_root_backing_bytes(
            capacities.registry,
            capacities.controls,
            capacities.memberships,
            capacities.cells,
            capacities.root_slots,
            capacities.root_cells,
        )
        .ok_or(overflow)?
        .checked_add(
            Catalog::<R>::edge_backing_bytes(
                capacities.edge_slots,
                capacities.edge_cells,
                capacities.edge_states,
            )
            .ok_or(overflow)?,
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
        let root_tokens = capacities
            .root_slots
            .checked_mul(size_of::<super::RootHandle<R>>())
            .ok_or(overflow)?;
        let edge_tokens = capacities
            .edge_slots
            .checked_mul(size_of::<super::EdgeHandle<R>>())
            .ok_or(overflow)?;
        Ok(Self {
            backing,
            participants,
            root_tokens,
            edge_tokens,
            total: backing
                .checked_add(participants)
                .and_then(|bytes| bytes.checked_add(root_tokens))
                .and_then(|bytes| bytes.checked_add(edge_tokens))
                .ok_or(overflow)?,
        })
    }
    pub fn backing(self) -> usize {
        self.backing
    }
    pub fn participants(self) -> usize {
        self.participants
    }
    pub fn root_tokens(self) -> usize {
        self.root_tokens
    }
    pub fn edge_tokens(self) -> usize {
        self.edge_tokens
    }
    pub fn total(self) -> usize {
        self.total
    }
}

/// Externally supplied budget owner. Its own allocation is outside the catalog's
/// footprint; cloning its `Arc` does not allocate. Reservation makes one CAS attempt.
#[derive(Debug)]
pub struct Budget {
    limit: usize,
    used: AtomicUsize,
}
impl Budget {
    #[must_use]
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
