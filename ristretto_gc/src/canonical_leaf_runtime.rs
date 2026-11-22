//! Heap-bound internal leaf API and a prepaid census of runtime wrapper storage.
//! Catalog/payload work remains subject to the leaf protocol's callback limits.
use super::allocation::{AllocationError, LeafAllocation, LeafRoot};
use super::maintenance::{LeafMaintenance, MaintenanceError};
use super::root_owner::backing_bytes;
use super::{LeafAccessError, LeafRecord, LeafView};
use crate::canonical_catalog as gc_catalog;
use crate::{GarbageCollector, Trace};
use gc_catalog::RecordError;
use gc_catalog::bounded::{
    BoundedCatalog, Budget, Capacities, CatalogHandle, ConfigError, Footprint, GuardedCommand,
    Participant, RegisterError, RootHandle,
};
use std::mem::size_of;
use std::sync::Arc;

type Worker = Participant<LeafRecord, GuardedCommand>;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(in crate::canonical_adapter) enum RuntimeError {
    ConditionalReferences,
    Config(ConfigError),
}

/// Base catalog reservations plus bounded wrapper storage. Payload backing,
/// external collector/budget owners, allocator metadata and RSS are separate.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(in crate::canonical_adapter) struct RuntimeFootprint {
    catalog: Footprint,
    runtime: usize,
    total: usize,
}
impl RuntimeFootprint {
    pub(in crate::canonical_adapter) fn for_capacities(
        capacities: Capacities,
    ) -> Result<Self, RuntimeError> {
        if capacities.reference_slots() != 0 {
            return Err(RuntimeError::ConditionalReferences);
        }
        let overflow = RuntimeError::Config(ConfigError::CapacityOverflow);
        let catalog =
            Footprint::for_records::<LeafRecord>(capacities).map_err(RuntimeError::Config)?;
        // The base footprint already charges at least this Worker per quota.
        // Each mutator can have one exclusive-borrow allocation wrapper at once.
        let mutator = size_of::<LeafMutator<'_>>()
            .checked_sub(size_of::<Worker>())
            .and_then(|n| n.checked_add(size_of::<LeafAllocation<'_, ()>>()))
            .ok_or(overflow)?;
        let helper = size_of::<LeafMaintenance<'_>>()
            .checked_sub(size_of::<Worker>())
            .ok_or(overflow)?;
        let backing = backing_bytes().ok_or(overflow)?;
        let root = size_of::<LeafRoot<()>>()
            .checked_add(backing)
            .and_then(|n| n.checked_sub(size_of::<RootHandle<LeafRecord>>()))
            .ok_or(overflow)?;
        let runtime = capacities
            .mutators()
            .checked_mul(mutator.checked_add(backing).ok_or(overflow)?)
            .and_then(|n| capacities.helpers().checked_mul(helper)?.checked_add(n))
            .and_then(|n| capacities.root_slots().checked_mul(root)?.checked_add(n))
            .and_then(|n| n.checked_add(size_of::<LeafRuntime<'_>>()))
            .ok_or(overflow)?;
        let total = catalog.total().checked_add(runtime).ok_or(overflow)?;
        Ok(Self {
            catalog,
            runtime,
            total,
        })
    }

    pub(in crate::canonical_adapter) fn catalog(self) -> Footprint {
        self.catalog
    }

    pub(in crate::canonical_adapter) fn runtime(self) -> usize {
        self.runtime
    }

    pub(in crate::canonical_adapter) fn total(self) -> usize {
        self.total
    }
}

/// The raw catalog stays private: every returned wrapper uses the charged quotas
/// and this collector. Neither the runtime nor its participants are cloneable.
pub(in crate::canonical_adapter) struct LeafRuntime<'gc> {
    catalog: CatalogHandle<LeafRecord>,
    collector: &'gc GarbageCollector,
}
impl<'gc> LeafRuntime<'gc> {
    pub(in crate::canonical_adapter) fn new(
        collector: &'gc GarbageCollector,
        capacities: Capacities,
        budget: &Arc<Budget>,
    ) -> Result<Self, RuntimeError> {
        let footprint = RuntimeFootprint::for_capacities(capacities)?;
        let catalog =
            BoundedCatalog::with_records_and_storage_in(capacities, budget, footprint.runtime())
                .map_err(RuntimeError::Config)?;
        Ok(Self { catalog, collector })
    }

    pub(in crate::canonical_adapter) fn mutator(&self) -> Result<LeafMutator<'gc>, RegisterError> {
        Ok(LeafMutator {
            worker: self.catalog.register_record_mutator()?,
            collector: self.collector,
        })
    }

    pub(in crate::canonical_adapter) fn maintenance(
        &self,
    ) -> Result<LeafMaintenance<'gc>, MaintenanceError> {
        LeafMaintenance::new(&self.catalog, self.collector)
    }

    pub(in crate::canonical_adapter) fn borrow<'a, T: 'static>(
        &'a self,
        root: &'a LeafRoot<T>,
    ) -> Result<LeafView<'a, T>, LeafAccessError> {
        if !root.belongs_to(&self.catalog) {
            return Err(LeafAccessError::Admission(RecordError::Foreign));
        }
        root.borrow(self.collector)
    }

    pub(in crate::canonical_adapter) fn charged_bytes(&self) -> usize {
        self.catalog.charged_bytes()
    }

    pub(in crate::canonical_adapter) fn registered(&self) -> (usize, usize) {
        self.catalog.registered()
    }
}

pub(in crate::canonical_adapter) struct LeafMutator<'gc> {
    worker: Worker,
    collector: &'gc GarbageCollector,
}
impl LeafMutator<'_> {
    /// Incremental slot selection preserves the payload across object/root
    /// collisions. A full discovery array stays pending; drive maintenance or
    /// cancel. A full/contended root quota also retains the request. Preparation
    /// and callback limits are unchanged.
    pub(in crate::canonical_adapter) fn begin_any<T: Trace + Send + Sync + 'static>(
        &mut self,
        value: T,
    ) -> Result<LeafAllocation<'_, T>, AllocationError> {
        LeafAllocation::begin_any(&mut self.worker, self.collector, value)
    }

    /// Explicit slots retain the existing collision/error semantics. This does
    /// not imply an automatic search/retry protocol for a consumed payload.
    pub(in crate::canonical_adapter) fn begin<T: Trace + Send + Sync + 'static>(
        &mut self,
        slot: usize,
        root_slot: usize,
        value: T,
    ) -> Result<LeafAllocation<'_, T>, AllocationError> {
        LeafAllocation::begin(&mut self.worker, self.collector, slot, root_slot, value)
    }
}
