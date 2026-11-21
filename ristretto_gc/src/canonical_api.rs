//! Experimental incremental graph API for admitted callers.
//!
//! Edges live exclusively in the catalog. Payloads must satisfy reference-free
//! admission with an audited `CanonicalPayload<T>` certificate; arbitrary
//! `Trace` graphs and finalized ordinary descriptors are rejected. The current
//! layout reserves 1–8 edge positions per object. This API does not select the VM
//! collector or change ordinary `Gc` construction.
//!
//! Each request poll performs one existing protocol step. Callers schedule helper
//! polls and yield to their host, including on bare Wasm without a clock. Pending
//! work can require capacity or another participant's progress. Dropping a request
//! posts its existing cancellation intent; it does not drive a collection loop.
//! Host allocation, preparation hooks and payload destruction are outside the
//! bounded protocol-work contract. A helper poll can run a payload destructor.
//!
//! Catalog backing is charged separately from payload heap bytes. Retained roots
//! preserve their catalog charge, including after runtime/participant shutdown.
//! Caller-owned aliases, stacks, supplied collector/budget owners and allocator
//! overhead remain separate resources. No pause or sustained-load guarantee is made.
use super::allocation::AllocationError;
use super::graph::{
    GraphAllocation, GraphError, GraphFootprint, GraphMutator, GraphRead, GraphReadValue,
    GraphRoot, GraphRuntime, GraphWrite,
};
use super::maintenance::{LeafMaintenance, MaintenanceError, MaintenanceStep};
use super::{LeafAccessError, LeafView};
use crate::canonical_catalog as gc_catalog;
use crate::{GarbageCollector, Trace};
use gc_catalog::bounded::Capacities;
use std::fmt;
use std::ops::Deref;
use std::sync::Arc;
use std::task::Poll;

/// Reason an exact record could not be admitted to the supplied collector.
pub use gc_catalog::RecordError;
/// Logical catalog budget and explicit configuration/participant/request errors.
/// The collector's payload budget is independent of this catalog budget.
pub use gc_catalog::bounded::{Budget, ConfigError, RegisterError, RequestError};

/// Failure of configuration, admission or an incremental operation.
#[derive(Debug)]
pub enum Error {
    /// The graph layout is outside the admitted range.
    Layout,
    /// A root belongs to a different runtime.
    ForeignRuntime,
    /// An edge index is outside the configured object range.
    Index,
    /// The derived catalog cannot be constructed.
    Configuration(ConfigError),
    /// Participant quota or registration contention prevented admission.
    Registration(RegisterError),
    /// The payload has managed references or unsupported callbacks/finalization.
    UnsupportedPayload,
    /// Payload preparation or heap credit reservation failed.
    Heap(crate::Error),
    /// A catalog operation rejected the request.
    Request(RequestError),
    /// The root no longer permits a new payload view.
    NotLive,
    /// Publication has not certified the payload.
    NotPublished,
    /// The requested child payload type does not match its actual type.
    WrongType,
    /// The supplied collector cannot admit the retained record.
    Admission(RecordError),
    /// The allocation was cancelled.
    Cancelled,
    /// A completed request was polled again.
    Finished,
    /// An internal protocol result was unexpected.
    Protocol,
}
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "canonical runtime: {self:?}")
    }
}
impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Heap(error) => Some(error),
            _ => None,
        }
    }
}
impl From<GraphError> for Error {
    fn from(error: GraphError) -> Self {
        match error {
            GraphError::Layout => Self::Layout,
            GraphError::Foreign => Self::ForeignRuntime,
            GraphError::Index => Self::Index,
            GraphError::Config(error) => Self::Configuration(error),
            GraphError::Allocation(error) => error.into(),
            GraphError::Catalog(error) => Self::Request(error),
            GraphError::Access(error) => match error {
                LeafAccessError::NotLive => Self::NotLive,
                LeafAccessError::NotPublished => Self::NotPublished,
                LeafAccessError::WrongType => Self::WrongType,
                LeafAccessError::Admission(error) => Self::Admission(error),
            },
            GraphError::Finished => Self::Finished,
            GraphError::Protocol => Self::Protocol,
        }
    }
}
impl From<AllocationError> for Error {
    fn from(error: AllocationError) -> Self {
        match error {
            AllocationError::Unsupported => Self::UnsupportedPayload,
            AllocationError::Heap(error) => Self::Heap(error),
            AllocationError::Catalog(error) => Self::Request(error),
            AllocationError::Cancelled => Self::Cancelled,
            AllocationError::Finished => Self::Finished,
            AllocationError::Protocol => Self::Protocol,
        }
    }
}
impl From<MaintenanceError> for Error {
    fn from(error: MaintenanceError) -> Self {
        match error {
            MaintenanceError::ConditionalReferences => Self::Layout,
            MaintenanceError::Register(error) => Self::Registration(error),
            MaintenanceError::Catalog(error) => Self::Request(error),
            MaintenanceError::Protocol => Self::Protocol,
        }
    }
}

/// Caller-supplied bounds. Construction checks arithmetic and derives all pools.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Configuration {
    /// Simultaneously discoverable object slots; must be positive.
    pub objects: usize,
    /// Independent logical root permits; copies share their original permit.
    pub roots: usize,
    /// Simultaneously registered mutators and their outstanding requests.
    pub mutators: usize,
    /// Reserved helper participants; must be positive.
    pub helpers: usize,
    /// Catalog-owned positions per object, currently 1 through 8.
    pub edges_per_object: usize,
}
impl Configuration {
    fn capacities(self) -> Result<Capacities, Error> {
        let edges = self
            .objects
            .checked_mul(self.edges_per_object)
            .ok_or(Error::Configuration(ConfigError::CapacityOverflow))?;
        Capacities::derive(self.objects, self.mutators, self.helpers)
            .and_then(|c| c.with_roots(self.roots))
            .and_then(|c| c.with_edges(edges))
            .map_err(Error::Configuration)
    }
    /// Complete logical runtime reservation, including live and pending root backing.
    /// This does not include payloads, external owner variables or allocator overhead.
    /// # Errors
    /// Rejects unsupported layouts and any derived capacity/size overflow.
    pub fn charged_bytes(self) -> Result<usize, Error> {
        GraphFootprint::new(self.capacities()?, self.edges_per_object)
            .map(GraphFootprint::total)
            .map_err(Into::into)
    }
}

/// One collector-bound graph runtime. Its payload reads require this exact owner.
#[repr(transparent)]
pub struct Runtime<'gc>(GraphRuntime<'gc>);
impl<'gc> Runtime<'gc> {
    /// Reserves all catalog/owning-wrapper credit before constructing the runtime.
    /// # Errors
    /// Rejects invalid bounds, insufficient/contended budget or exhausted identities.
    pub fn new(
        collector: &'gc GarbageCollector,
        configuration: Configuration,
        budget: &Arc<Budget>,
    ) -> Result<Self, Error> {
        GraphRuntime::new(
            collector,
            configuration.capacities()?,
            configuration.edges_per_object,
            budget,
        )
        .map(Self)
        .map_err(Into::into)
    }
    /// Registers a mutator with at most one outstanding operation.
    /// # Errors
    /// Fails if the mutator quota is full or its reservation is contended.
    pub fn mutator(&self) -> Result<Mutator<'gc>, Error> {
        self.0.mutator().map(Mutator).map_err(Error::Registration)
    }
    /// Registers a helper independently of mutator quota.
    /// # Errors
    /// Fails if helper quota is full or its reservation is contended.
    pub fn helper(&self) -> Result<Helper<'gc>, Error> {
        self.0.maintenance().map(Helper).map_err(Into::into)
    }
    /// Borrows a live typed payload through a root from this exact runtime.
    /// The view cannot outlive either the root or runtime. It exposes no managed edge.
    /// # Errors
    /// Rejects a foreign, revoked, unpublished, wrongly typed or unadmitted root.
    pub fn borrow<'a, T: 'static>(&'a self, root: &'a Root<T>) -> Result<View<'a, T>, Error> {
        self.0.borrow(&root.0).map(View).map_err(Into::into)
    }
    /// Copies the immutable admitted capacities; this grants no new ownership.
    #[must_use]
    pub fn configuration(&self) -> Configuration {
        let (capacities, degree) = self.0.configuration();
        Configuration {
            objects: capacities.registry(),
            roots: capacities.root_slots(),
            mutators: capacities.mutators(),
            helpers: capacities.helpers(),
            edges_per_object: degree,
        }
    }
    /// Current registered mutator/helper counts; a diagnostic snapshot, not a barrier.
    #[must_use]
    pub fn registered(&self) -> (usize, usize) {
        self.0.registered()
    }
}

/// An owning alias of one logical root. Copies need no `T: Clone` or new root permit.
#[repr(transparent)]
pub struct Root<T>(GraphRoot<T>);
impl<T> Clone for Root<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl<T> Root<T> {
    /// Compares exact catalog-scoped object identities, including independent roots.
    /// This grants no payload access and does not change either root's lifetime.
    #[must_use]
    pub fn same_object<U>(&self, other: &Root<U>) -> bool {
        self.0.same_object(&other.0)
    }
}

/// Shared payload view retaining a borrow of its exact root and runtime.
#[repr(transparent)]
pub struct View<'a, T>(LeafView<'a, T>);
impl<T> Deref for View<'_, T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.0
    }
}

/// Conservative bounds on completed primary-domain full epochs during one call.
/// These do not count reclaimed objects, payload destruction, or completed callbacks.
/// The secondary domain's initial epoch is excluded. Copies retain no GC ownership.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PrimaryCycles {
    /// At least this many primary full epochs had completed at an instant in the call.
    pub lower: u64,
    /// At most this many primary full epochs had completed at that same instant.
    pub upper: u64,
}

/// Registered mutator. Requests exclusively borrow it through completion/drop.
#[repr(transparent)]
pub struct Mutator<'gc>(GraphMutator<'gc>);
impl<'gc> Mutator<'gc> {
    /// Observes primary full-cycle bounds using one controller acquisition.
    /// This requires an idle registered participant and does not help collection.
    /// It does not retry contention or retain an owning snapshot. Bounds need not
    /// be current at return: other participants can advance during the call.
    /// Releasing the temporary owner can run record/payload destruction, so this
    /// is not a destructor-free operation or a wall-time pause bound.
    /// # Errors
    /// Returns `Request(Contended)` on failed acquisition, `Request(Busy)` if the
    /// participant is occupied, or `Protocol` for an unsupported epoch state.
    pub fn primary_cycles(&mut self) -> Result<PrimaryCycles, Error> {
        self.0
            .primary_cycles()
            .map(|(lower, upper)| PrimaryCycles { lower, upper })
            .map_err(Into::into)
    }
    /// Prepares a payload once, then returns an incremental allocation request.
    /// Preparation and host allocation are outside the poll-work bound.
    /// # Errors
    /// Rejects unsupported payloads, heap exhaustion or initial catalog rejection.
    pub fn allocate<T: Trace + Send + Sync + 'static>(
        &mut self,
        value: T,
    ) -> Result<Allocation<'_, 'gc, T>, Error> {
        self.0.allocate(value).map(Allocation).map_err(Into::into)
    }
    /// Allocates a payload with exactly `edges` usable outgoing reference positions.
    /// Zero is valid for payloads with no outgoing references. The runtime's fixed
    /// catalog stride still determines the address range and reservation per object.
    ///
    /// # Errors
    /// Returns `Layout` if `edges` exceeds the configured maximum. Other errors
    /// follow [`Self::allocate`].
    pub fn allocate_with_edges<T: Trace + Send + Sync + 'static>(
        &mut self,
        value: T,
        edges: usize,
    ) -> Result<Allocation<'_, 'gc, T>, Error> {
        self.0
            .allocate_with_edges(value, edges)
            .map(Allocation)
            .map_err(Into::into)
    }
    /// Starts an edge replacement with coupled deletion/insertion barriers.
    /// # Errors
    /// Rejects foreign source/target roots or an out-of-range edge index.
    pub fn write<'a, S, T>(
        &'a mut self,
        source: &'a Root<S>,
        index: usize,
        target: Option<&'a Root<T>>,
    ) -> Result<Write<'a, 'gc>, Error> {
        self.0
            .write(&source.0, index, target.map(|root| &root.0))
            .map(Write)
            .map_err(Into::into)
    }
    /// Starts a child read, acquiring an independent root for a nonnull result.
    /// A full root quota can remain Pending; a later null replacement can complete it.
    /// # Errors
    /// Rejects a foreign source or an out-of-range edge index.
    pub fn child<'a, S, T: 'static>(
        &'a mut self,
        source: &'a Root<S>,
        index: usize,
    ) -> Result<Read<'a, 'gc, T>, Error> {
        self.0.child(&source.0, index).map(Read).map_err(Into::into)
    }
}

/// Cancellable allocation whose successful poll transfers one typed root once.
#[repr(transparent)]
pub struct Allocation<'a, 'gc, T>(GraphAllocation<'a, 'gc, T>);
impl<T> Allocation<'_, '_, T> {
    /// Advances one existing allocation step without an implicit retry loop.
    /// # Errors
    /// Returns a rejected protocol operation or Finished after completion.
    pub fn poll(&mut self) -> Result<Poll<Root<T>>, Error> {
        self.0
            .poll()
            .map(|root| root.map_or(Poll::Pending, |root| Poll::Ready(Root(root))))
            .map_err(Into::into)
    }
}

/// Cancellable edge replacement retaining source/target borrows until it leaves.
#[repr(transparent)]
pub struct Write<'a, 'gc>(GraphWrite<'a, 'gc>);
impl Write<'_, '_> {
    /// Advances one existing write step; completion certifies the coupled barrier.
    /// # Errors
    /// Returns a rejected protocol operation or Finished after completion.
    pub fn poll(&mut self) -> Result<Poll<()>, Error> {
        self.0
            .poll()
            .map(|done| if done { Poll::Ready(()) } else { Poll::Pending })
            .map_err(Into::into)
    }
}

/// Cancellable independently rooted child read; Ready(None) means a null edge.
#[repr(transparent)]
pub struct Read<'a, 'gc, T>(GraphRead<'a, 'gc, T>);
impl<T: 'static> Read<'_, '_, T> {
    /// Advances one existing read step without waiting for root capacity.
    /// # Errors
    /// Rejects protocol/type/admission failures or polling after completion.
    pub fn poll(&mut self) -> Result<Poll<Option<Root<T>>>, Error> {
        self.0
            .poll()
            .map(|value| match value {
                None => Poll::Pending,
                Some(GraphReadValue::Null) => Poll::Ready(None),
                Some(GraphReadValue::Root(root)) => Poll::Ready(Some(Root(root))),
            })
            .map_err(Into::into)
    }
}

/// Result of one helper visit. No variant implies a globally drained heap.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Work {
    /// A request remains in progress or a bounded observation was contended.
    Pending,
    /// One bounded scan visit completed.
    Visited,
    /// A protocol operation committed.
    Committed,
    /// One object discovery slot was released.
    ReleasedObjectSlot,
    /// One root discovery slot was released.
    ReleasedRootSlot,
    /// One edge discovery slot was released.
    ReleasedEdgeSlot,
}
/// Reserved helper that can outlive its original runtime while the collector lives.
#[repr(transparent)]
pub struct Helper<'gc>(LeafMaintenance<'gc>);
impl Helper<'_> {
    /// Advances one bounded protocol visit, potentially destroying a retired payload.
    /// Payload Drop and host allocator delays are not bounded by this API.
    /// # Errors
    /// Returns catalog rejection or an unexpected protocol result.
    pub fn poll(&mut self) -> Result<Work, Error> {
        self.0
            .poll()
            .map(|step| match step {
                MaintenanceStep::Pending => Work::Pending,
                MaintenanceStep::Visited => Work::Visited,
                MaintenanceStep::Committed => Work::Committed,
                MaintenanceStep::ReleasedObjectSlot => Work::ReleasedObjectSlot,
                MaintenanceStep::ReleasedRootSlot => Work::ReleasedRootSlot,
                MaintenanceStep::ReleasedEdgeSlot => Work::ReleasedEdgeSlot,
            })
            .map_err(Into::into)
    }
}

// Debug never borrows a payload, polls the collector or invokes user formatting.
macro_rules! opaque_debug {
    ($name:ident <$($parameter:tt),+>) => {
        impl<$($parameter),+> fmt::Debug for $name<$($parameter),+> {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.debug_struct(stringify!($name)).finish_non_exhaustive()
            }
        }
    };
}
opaque_debug!(Runtime<'gc>);
opaque_debug!(Root<T>);
opaque_debug!(View<'a, T>);
opaque_debug!(Mutator<'gc>);
opaque_debug!(Allocation<'a, 'gc, T>);
opaque_debug!(Write<'a, 'gc>);
opaque_debug!(Read<'a, 'gc, T>);
opaque_debug!(Helper<'gc>);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn public_owners_preserve_the_charged_internal_representations() {
        macro_rules! same_layout {
            ($public:ty, $inner:ty) => {
                assert_eq!(size_of::<$public>(), size_of::<$inner>());
                assert_eq!(align_of::<$public>(), align_of::<$inner>());
            };
        }
        same_layout!(Runtime<'static>, GraphRuntime<'static>);
        same_layout!(Mutator<'static>, GraphMutator<'static>);
        same_layout!(Helper<'static>, LeafMaintenance<'static>);
        same_layout!(Root<u64>, GraphRoot<u64>);
        same_layout!(View<'static, u64>, LeafView<'static, u64>);
        same_layout!(
            Allocation<'static, 'static, u64>,
            GraphAllocation<'static, 'static, u64>
        );
        same_layout!(Write<'static, 'static>, GraphWrite<'static, 'static>);
        same_layout!(
            Read<'static, 'static, u64>,
            GraphRead<'static, 'static, u64>
        );
    }
}
