//! Sealed graph runtime: primitive typed payloads and exclusively catalog-owned
//! edges. It does not admit arbitrary Trace graphs or select a public backend.
use super::allocation::{AllocationError, prepare_record};
use super::maintenance::{LeafMaintenance, MaintenanceError};
use super::root_owner::{PreparedRoot, SharedRoot, backing_bytes};
use super::{LeafAccessError, LeafRecord, LeafView};
use crate::canonical_catalog as gc_catalog;
use crate::{GarbageCollector, Trace};
use gc_catalog::EdgeMembership;
use gc_catalog::bounded::{
    BoundedCatalog, Budget, Capacities, CatalogHandle, ConfigError, EdgeCommand, EdgeHandle,
    EdgeKey, EdgeObservation, Footprint, GuardedCommand, Participant, Progress, RegisterError,
    RequestError, RootHandle,
};
use std::marker::PhantomData;
use std::mem::size_of;
use std::sync::Arc;

#[path = "canonical_graph_access.rs"]
mod access;
pub(in crate::canonical_adapter) use access::{GraphRead, GraphWrite};

type Worker = Participant<LeafRecord, GuardedCommand>;
const MAX_DEGREE: usize = 8;

#[derive(Debug)]
pub(in crate::canonical_adapter) enum GraphError {
    Layout,
    Foreign,
    Index,
    Config(ConfigError),
    Allocation(AllocationError),
    Catalog(RequestError),
    Access(LeafAccessError),
    Finished,
    Protocol,
}

pub(in crate::canonical_adapter) struct GraphRoot<T> {
    root: SharedRoot,
    value: PhantomData<T>,
}
#[cfg(feature = "experimental-canonical-runtime")]
impl<T> GraphRoot<T> {
    pub(in crate::canonical_adapter) fn same_object<U>(&self, other: &GraphRoot<U>) -> bool {
        self.root.observe().target == other.root.observe().target
    }
}
impl<T> Clone for GraphRoot<T> {
    fn clone(&self) -> Self {
        Self {
            root: self.root.clone(),
            value: PhantomData,
        }
    }
}
pub(in crate::canonical_adapter) enum GraphReadValue<T> {
    Null,
    Root(GraphRoot<T>),
}

pub(in crate::canonical_adapter) struct GraphRuntime<'gc> {
    catalog: CatalogHandle<LeafRecord>,
    collector: &'gc GarbageCollector,
    degree: usize,
}

#[derive(Clone, Copy)]
pub(in crate::canonical_adapter) struct GraphFootprint {
    catalog: Footprint,
    runtime: usize,
    total: usize,
}
impl GraphFootprint {
    pub(in crate::canonical_adapter) fn new(
        capacities: Capacities,
        degree: usize,
    ) -> Result<Self, GraphError> {
        if !(1..=MAX_DEGREE).contains(&degree)
            || capacities.registry().checked_mul(degree) != Some(capacities.edge_slots())
            || capacities.reference_slots() != 0
            || capacities.root_slots() == 0
        {
            return Err(GraphError::Layout);
        }
        let catalog =
            Footprint::for_records::<LeafRecord>(capacities).map_err(GraphError::Config)?;
        let backing = backing_bytes().ok_or(GraphError::Config(ConfigError::CapacityOverflow))?;
        let root = size_of::<GraphRoot<()>>()
            .checked_add(backing)
            .and_then(|n| n.checked_sub(size_of::<RootHandle<LeafRecord>>()))
            .ok_or(GraphError::Config(ConfigError::CapacityOverflow))?;
        let request = size_of::<GraphAllocation<'_, '_, ()>>()
            .max(size_of::<GraphRead<'_, '_, ()>>())
            .max(size_of::<GraphWrite<'_, '_>>());
        let mutator = size_of::<GraphMutator<'_>>()
            .checked_sub(size_of::<Worker>())
            .and_then(|v| v.checked_add(request))
            .and_then(|v| v.checked_add(backing));
        let helper = size_of::<LeafMaintenance<'_>>().checked_sub(size_of::<Worker>());
        let runtime = mutator
            .and_then(|v| capacities.mutators().checked_mul(v))
            .and_then(|v| v.checked_add(capacities.helpers().checked_mul(helper?)?))
            .and_then(|v| v.checked_add(capacities.root_slots().checked_mul(root)?))
            .and_then(|v| v.checked_add(size_of::<GraphRuntime<'_>>()))
            .ok_or(GraphError::Config(ConfigError::CapacityOverflow))?;
        let total = catalog
            .total()
            .checked_add(runtime)
            .ok_or(GraphError::Config(ConfigError::CapacityOverflow))?;
        Ok(Self {
            catalog,
            runtime,
            total,
        })
    }
    pub(in crate::canonical_adapter) fn total(self) -> usize {
        self.total
    }
}
impl<'gc> GraphRuntime<'gc> {
    pub(in crate::canonical_adapter) fn new(
        collector: &'gc GarbageCollector,
        capacities: Capacities,
        degree: usize,
        budget: &Arc<Budget>,
    ) -> Result<Self, GraphError> {
        let footprint = GraphFootprint::new(capacities, degree)?;
        let catalog = BoundedCatalog::with_indexed_records_and_storage_in(
            capacities,
            budget,
            footprint.runtime,
            degree,
        )
        .map_err(GraphError::Config)?;
        Ok(Self {
            catalog,
            collector,
            degree,
        })
    }
    pub(in crate::canonical_adapter) fn mutator(&self) -> Result<GraphMutator<'gc>, RegisterError> {
        Ok(GraphMutator {
            prepared: None,
            worker: self.catalog.register_record_mutator()?,
            catalog: self.catalog.clone(),
            collector: self.collector,
            degree: self.degree,
            next_root: 0,
        })
    }
    pub(in crate::canonical_adapter) fn maintenance(
        &self,
    ) -> Result<LeafMaintenance<'gc>, MaintenanceError> {
        LeafMaintenance::new(&self.catalog, self.collector)
    }
    pub(in crate::canonical_adapter) fn borrow<'a, T: 'static>(
        &'a self,
        root: &'a GraphRoot<T>,
    ) -> Result<LeafView<'a, T>, GraphError> {
        if !root.root.belongs_to(&self.catalog) {
            return Err(GraphError::Foreign);
        }
        LeafRecord::borrow_root(&root.root, self.collector).map_err(GraphError::Access)
    }
    #[cfg(feature = "experimental-canonical-runtime")]
    pub(in crate::canonical_adapter) fn configuration(&self) -> (Capacities, usize) {
        (self.catalog.capacities(), self.degree)
    }
    pub(in crate::canonical_adapter) fn registered(&self) -> (usize, usize) {
        self.catalog.registered()
    }
}

pub(in crate::canonical_adapter) struct GraphMutator<'gc> {
    // The exclusive request borrows this private backing until it hands off a root.
    // Drop it before returning the participant's quota.
    prepared: Option<PreparedRoot>,
    worker: Worker,
    catalog: CatalogHandle<LeafRecord>,
    collector: &'gc GarbageCollector,
    degree: usize,
    next_root: usize,
}
impl<'gc> GraphMutator<'gc> {
    #[cfg(feature = "experimental-canonical-runtime")]
    pub(in crate::canonical_adapter) fn primary_cycles(
        &mut self,
    ) -> Result<(u64, u64), GraphError> {
        primary_cycles(
            self.worker
                .possible_phase_outcomes()
                .map_err(GraphError::Catalog)?,
        )
    }

    fn validate<T>(&self, root: &GraphRoot<T>, index: usize) -> Result<(), GraphError> {
        if !root.root.belongs_to(&self.catalog) {
            return Err(GraphError::Foreign);
        }
        let record = root
            .root
            .borrow_live_record()
            .ok_or(GraphError::Access(LeafAccessError::NotLive))?;
        if index >= usize::from(record.graph_edges) {
            return Err(GraphError::Index);
        }
        Ok(())
    }
    fn observe(
        &mut self,
        source: &RootHandle<LeafRecord>,
        index: usize,
    ) -> Result<EdgeObservation, RequestError> {
        let slot = source.target_slot() * self.degree + index;
        self.worker
            .observe_rooted_edge(source, slot, self.collector)
    }
    pub(in crate::canonical_adapter) fn allocate<T: Trace + Send + Sync + 'static>(
        &mut self,
        value: T,
    ) -> Result<GraphAllocation<'_, 'gc, T>, GraphError> {
        self.allocate_with_edges(value, self.degree)
    }
    pub(in crate::canonical_adapter) fn allocate_with_edges<T: Trace + Send + Sync + 'static>(
        &mut self,
        value: T,
        edges: usize,
    ) -> Result<GraphAllocation<'_, 'gc, T>, GraphError> {
        if edges > self.degree {
            return Err(GraphError::Layout);
        }
        let edge_count = u8::try_from(edges).map_err(|_| GraphError::Layout)?;
        let mut record = prepare_record(self.collector, value).map_err(GraphError::Allocation)?;
        record.graph_edges = edge_count;
        self.prepared.get_or_insert_with(PreparedRoot::new);
        self.worker
            .submit_rooted_record_any(record.address(), record)
            .map_err(|failure| GraphError::Catalog(failure.error))?;
        Ok(GraphAllocation {
            mutator: self,
            root: None,
            edge: None,
            index: 0,
            edge_count,
            stage: AllocationStage::Object,
            finished: false,
            value: PhantomData,
        })
    }
}

#[derive(Clone, Copy)]
enum AllocationStage {
    Object,
    Create,
    Creating,
    Publish,
    Publishing,
    Handoff,
}
pub(in crate::canonical_adapter) struct GraphAllocation<'a, 'gc, T> {
    mutator: &'a mut GraphMutator<'gc>,
    root: Option<RootHandle<LeafRecord>>,
    edge: Option<EdgeHandle<LeafRecord>>,
    index: u8,
    edge_count: u8,
    stage: AllocationStage,
    finished: bool,
    value: PhantomData<T>,
}
impl<T> GraphAllocation<'_, '_, T> {
    pub(in crate::canonical_adapter) fn poll(
        &mut self,
    ) -> Result<Option<GraphRoot<T>>, GraphError> {
        if self.finished {
            return Err(GraphError::Finished);
        }
        let result = self.step();
        if result.is_err() {
            self.finished = true;
            self.mutator.worker.discard_request();
            self.edge = None;
            self.root = None;
        }
        result
    }
    fn step(&mut self) -> Result<Option<GraphRoot<T>>, GraphError> {
        let worker = &mut self.mutator.worker;
        let collector = self.mutator.collector;
        match self.stage {
            AllocationStage::Object => match worker.poll_in(collector) {
                Progress::Pending => {}
                Progress::RootCreated(_) => {
                    self.root = Some(worker.take_root().ok_or(GraphError::Protocol)?);
                    self.stage = AllocationStage::Create;
                }
                Progress::Rejected(e) => return Err(GraphError::Catalog(e)),
                _ => return Err(GraphError::Protocol),
            },
            AllocationStage::Create => {
                if self.index == self.edge_count {
                    self.finished = true;
                    return Ok(Some(GraphRoot {
                        root: self
                            .mutator
                            .prepared
                            .take()
                            .ok_or(GraphError::Protocol)?
                            .finish(self.root.take().ok_or(GraphError::Protocol)?),
                        value: PhantomData,
                    }));
                }
                let source = self
                    .root
                    .as_ref()
                    .ok_or(GraphError::Protocol)?
                    .observe()
                    .target;
                let slot = source.slot() * self.mutator.degree + usize::from(self.index);
                match worker.submit_edge_creation(slot) {
                    Ok(()) => self.stage = AllocationStage::Creating,
                    Err(RequestError::Contended | RequestError::EdgeLimit) => {}
                    Err(e) => return Err(GraphError::Catalog(e)),
                }
            }
            AllocationStage::Creating => match worker.poll_in(collector) {
                Progress::Pending => {}
                Progress::EdgeCreated(_) => {
                    self.edge = Some(worker.take_edge().ok_or(GraphError::Protocol)?);
                    self.stage = AllocationStage::Publish;
                }
                Progress::Rejected(RequestError::Conflict) => self.stage = AllocationStage::Create,
                Progress::Rejected(e) => return Err(GraphError::Catalog(e)),
                _ => return Err(GraphError::Protocol),
            },
            AllocationStage::Publish => {
                worker
                    .submit_edge(EdgeCommand::Publish {
                        edge: self.edge.as_ref().ok_or(GraphError::Protocol)?.key(),
                        source: self.root.as_ref().ok_or(GraphError::Protocol)?.key(),
                        target: None,
                    })
                    .map_err(GraphError::Catalog)?;
                self.stage = AllocationStage::Publishing;
            }
            AllocationStage::Publishing => match worker.poll_in(collector) {
                Progress::Pending => {}
                Progress::Committed => self.stage = AllocationStage::Handoff,
                Progress::Rejected(e) => return Err(GraphError::Catalog(e)),
                _ => return Err(GraphError::Protocol),
            },
            AllocationStage::Handoff => {
                let edge = self.edge.take().ok_or(GraphError::Protocol)?;
                match worker.release_edge_to_source(edge, collector) {
                    Ok(_) => {
                        self.index += 1;
                        self.stage = AllocationStage::Create;
                    }
                    Err(failure) => {
                        self.edge = Some(failure.edge);
                        if failure.error != RequestError::Contended {
                            return Err(GraphError::Catalog(failure.error));
                        }
                    }
                }
            }
        }
        Ok(None)
    }
}
impl<T> Drop for GraphAllocation<'_, '_, T> {
    fn drop(&mut self) {
        if !self.finished {
            self.mutator.worker.discard_request();
        }
    }
}

#[cfg(test)]
#[path = "canonical_graph_tests.rs"]
mod tests;

#[cfg(feature = "experimental-canonical-runtime")]
fn primary_cycles(outcomes: [gc_catalog::Phase; 2]) -> Result<(u64, u64), GraphError> {
    let completed = |phase: gc_catalog::Phase| {
        let domain = phase.domains[0];
        if domain.epoch == 0 {
            return Err(GraphError::Protocol);
        }
        // The sealed graph has edges and only permits Full starts. A generic
        // bounded catalog can contain Young history, so mapping belongs here.
        match domain.mode {
            gc_catalog::DomainMode::Full => domain.epoch.checked_sub(1).ok_or(GraphError::Protocol),
            gc_catalog::DomainMode::Inactive => Ok(domain.epoch),
            _ => Err(GraphError::Protocol),
        }
    };
    let a = completed(outcomes[0])?;
    let b = completed(outcomes[1])?;
    Ok((a.min(b), a.max(b)))
}
