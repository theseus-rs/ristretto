//! Logical footprint credits, distinct from RSS and allocator bookkeeping.
use super::{
    Arc, AtomicUsize, BoundedCatalog, Capacities, Catalog, ConfigError, Ordering, Participant,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Footprint {
    backing: usize,
    participants: usize,
    root_tokens: usize,
    reference_tokens: usize,
    construction_tokens: usize,
    edge_tokens: usize,
    total: usize,
}
impl Footprint {
    pub fn for_capacities(capacities: Capacities) -> Result<Self, ConfigError> {
        Self::for_records::<()>(capacities)
    }
    /// Includes inline record storage and the larger supported participant form.
    /// Allocation owned indirectly by R remains the record owner's responsibility.
    pub fn for_records<R: crate::Record>(capacities: Capacities) -> Result<Self, ConfigError> {
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
            Catalog::<R>::reference_backing_bytes(
                capacities.reference_slots,
                capacities.reference_cells,
                capacities.reference_states,
            )
            .ok_or(overflow)?,
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
        .checked_add(pool::arc_requested_bytes::<BoundedCatalog<R>>().ok_or(overflow)?)
        .ok_or(overflow)?;
        let participants = capacities
            .participants()
            .checked_mul(
                std::mem::size_of::<Participant<R>>()
                    .max(std::mem::size_of::<Participant<R, super::GuardedCommand>>()),
            )
            .ok_or(overflow)?;
        let root_tokens = capacities
            .root_slots
            .checked_mul(std::mem::size_of::<super::RootHandle<R>>())
            .ok_or(overflow)?;
        let reference_tokens = capacities
            .reference_slots
            .checked_mul(std::mem::size_of::<super::ReferenceHandle<R>>())
            .ok_or(overflow)?;
        let construction_tokens = capacities
            .reference_slots
            .checked_mul(std::mem::size_of::<super::ConstructionHandle<R>>())
            .ok_or(overflow)?;
        let edge_tokens = capacities
            .edge_slots
            .checked_mul(std::mem::size_of::<super::EdgeHandle<R>>())
            .ok_or(overflow)?;
        Ok(Self {
            backing,
            participants,
            root_tokens,
            reference_tokens,
            construction_tokens,
            edge_tokens,
            total: backing
                .checked_add(participants)
                .and_then(|v| v.checked_add(root_tokens))
                .and_then(|v| v.checked_add(reference_tokens))
                .and_then(|v| v.checked_add(construction_tokens))
                .and_then(|v| v.checked_add(edge_tokens))
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
    pub fn reference_tokens(self) -> usize {
        self.reference_tokens
    }
    pub fn construction_tokens(self) -> usize {
        self.construction_tokens
    }
    pub fn total(self) -> usize {
        self.total
    }
    pub fn edge_tokens(self) -> usize {
        self.edge_tokens
    }
}

/// Externally supplied budget owner. Its own allocation is outside the catalog's
/// footprint; cloning its Arc does not allocate. Reservation makes one CAS attempt.
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
        #[cfg(feature = "allocation-audit")]
        crate::audit::refunded(self.bytes);
    }
}
