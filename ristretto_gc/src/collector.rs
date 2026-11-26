use crate::config::{Configuration, Statistics};
use crate::metadata::{MetadataRef, ObjectMetadata, Retention};
use crate::pointers::SafePtr;
use crate::work::{MarkItem, MarkQueues};
use crate::{Finalize, Gc, GcRootGuard, Result};
use portable_atomic::AtomicU64;
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet, VecDeque};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::{Arc, Mutex, RwLock, Weak};
#[cfg(not(target_family = "wasm"))]
use std::thread;

#[cfg(test)]
#[path = "storage_tests.rs"]
#[expect(clippy::panic_in_result_fn)]
#[cfg_attr(not(target_family = "wasm"), expect(clippy::unwrap_in_result))]
mod storage_tests;

#[cfg(test)]
#[path = "metadata_owner_tests.rs"]
mod metadata_owner_tests;

#[cfg(test)]
#[path = "relocation_tests.rs"]
#[expect(clippy::panic_in_result_fn)]
mod relocation_tests;

#[cfg(test)]
#[path = "generation_tests.rs"]
mod generation_tests;

#[cfg(test)]
#[path = "phase_tests.rs"]
mod phase_tests;

#[cfg(all(test, not(target_family = "wasm")))]
#[path = "index_progress_tests.rs"]
mod index_progress_tests;

#[cfg(test)]
#[path = "barrier_progress_tests.rs"]
mod barrier_progress_tests;

#[cfg(all(
    test,
    feature = "experimental-pause-recorder",
    target_os = "linux",
    target_has_atomic = "64"
))]
#[path = "pause_recorder_tests.rs"]
mod pause_recorder_tests;

#[path = "phase_work.rs"]
mod phase_work;
use phase_work::Unit;
#[path = "external_trace.rs"]
mod external_trace;
#[path = "weak_publication.rs"]
mod weak_publication;

#[path = "old_generation.rs"]
mod old_generation;
#[cfg(test)]
#[path = "old_generation_tests.rs"]
mod old_generation_tests;

#[path = "publication_state.rs"]
mod publication_state;

pub(crate) struct Publication<'a> {
    _legacy: Option<crate::sync::RecursiveRwLockReadGuard<'a, ()>>,
    admission: Option<publication_state::Admission<'a>>,
}

/// A successful phase try-lock, or the counted policy's intentional gate bypass.
/// Absence of this token means the caller must yield without changing its phase.
struct PhasePublication<'a> {
    // Rust drops fields in declaration order. Release the publication guard
    // before recording release; never timestamp an unlock that has not happened.
    _legacy: Option<crate::sync::RecursiveRwLockWriteGuard<'a, ()>>,
    #[cfg(all(
        feature = "experimental-pause-recorder",
        target_os = "linux",
        target_has_atomic = "64"
    ))]
    _pause: Option<crate::pause_recorder::PauseGuard<'a>>,
}

/// Unwind after index installation must remove the private descriptor before
/// its construction roots are released. The outer constructor owns the payload.
struct PendingBirth<'a> {
    slot: crate::index::Slot<'a>,
    gc: &'a GarbageCollector,
    metadata: &'a MetadataRef,
}

impl Drop for PendingBirth<'_> {
    fn drop(&mut self) {
        if !self.slot.reserved {
            return;
        }
        let address = self.metadata.ptr().0;
        let id = self.metadata.id.to_i64().cast_unsigned();
        #[cfg(not(feature = "experimental-identity-index"))]
        self.gc.identities.remove(id, address);
        #[cfg(feature = "experimental-identity-index")]
        {
            let mut cursor = crate::identity_index::Removal::default();
            while self
                .gc
                .identities
                .remove_step(id, address, &mut cursor)
                .is_none()
            {}
        }
    }
}

impl Publication<'_> {
    fn changed(&self) {
        if let Some(admission) = &self.admission {
            admission.changed();
        }
    }
}

/// A collector-bound token only for paths that pass admission into root creation.
/// Ordinary barrier/allocation guards retain their original compact representation.
pub(crate) struct RootPublication<'a> {
    collector: &'a GarbageCollector,
    _publication: Publication<'a>,
}

/// Page allocation, writer-lock waiting and cancellation happen before/after
/// admission. Only a validated nonzero address is committed under admission.
pub(crate) struct RootReservation<'a> {
    collector: &'a GarbageCollector,
    address: usize,
    context: Option<Arc<crate::context::State>>,
    slot: Option<crate::roots::PreparedSlot>,
}

impl RootReservation<'_> {
    pub(crate) fn protects(&self, collector: &GarbageCollector, address: usize) -> bool {
        std::ptr::eq(self.collector, collector) && self.address == address
    }
}

impl Drop for RootReservation<'_> {
    fn drop(&mut self) {
        if let Some(slot) = self.slot.take() {
            if let Some(context) = &self.context {
                if context.roots.cancel_prepared(slot) {
                    self.collector.wake_context_root_repair(context);
                }
            } else if let Some(roots) = self.collector.scanned_slots() {
                roots.cancel_prepared(slot);
                self.collector.wake_root_repair(roots);
            }
        }
    }
}

/// Removing a queue entry transfers ownership of its existing tree root here.
/// Keep that root through validated acquisition, including error and unwind paths.
struct PendingReferenceRoot<'a> {
    collector: &'a GarbageCollector,
    id: crate::roots::TreeRoot,
}

impl Drop for PendingReferenceRoot<'_> {
    fn drop(&mut self) {
        self.collector.remove_tree_root(self.id);
    }
}

impl RootPublication<'_> {
    pub(crate) fn protects(&self, collector: &GarbageCollector) -> bool {
        std::ptr::eq(self.collector, collector)
    }

    pub(crate) fn register_weak(&self, slot: &Arc<crate::weak::WeakSlot>) {
        self.collector.register_weak(slot);
    }

    /// Retain the descriptor already validated under this same admission.
    pub(crate) fn add_conditional_root(
        &self,
        metadata: &MetadataRef,
        reservation: &mut RootReservation<'_>,
    ) -> crate::roots::Registration {
        let _measurement = self
            .collector
            .diagnostics
            .root_acquire
            .measure(self.collector.diagnostics_enabled());
        assert!(
            reservation.protects(self.collector, metadata.ptr().0),
            "foreign root reservation"
        );
        let registration = self.collector.install_root_registration(reservation);
        // Both domains clamp strong discovery to their own reference stage.
        let _retention = TraceRetention::enter(self.collector, Retention::Strong);
        for index in 0..2 {
            let _context = Context::enter(self.collector, index);
            self.collector.shade_known(metadata);
        }
        registration
    }
}

/// Private logical credits transfer to metadata before it can become visible.
pub(crate) struct AllocationReservation<'a> {
    collector: &'a GarbageCollector,
    pub(crate) size: usize,
    committed: bool,
}

impl<'gc> AllocationReservation<'gc> {
    pub(crate) fn transferred(collector: &'gc GarbageCollector, size: usize) -> Self {
        Self {
            collector,
            size,
            committed: false,
        }
    }

    fn transfer(mut self, metadata: &mut ObjectMetadata) {
        assert_eq!(
            self.size,
            metadata.size(),
            "descriptor charge differs from reservation"
        );
        // No fallible operation separates the ownership transfer and disarming
        // this guard. The descriptor's existing weak collector link carries the
        // charge; it adds neither an allocation nor a strong collector cycle.
        metadata.collector = self.collector.this.clone();
        self.committed = true;
    }
}

impl Drop for AllocationReservation<'_> {
    fn drop(&mut self) {
        if !self.committed {
            self.collector.cancel_allocation(self.size);
        }
    }
}

thread_local! {
    static CLEANUP_HEAPS: std::cell::RefCell<Vec<usize>> = const { std::cell::RefCell::new(Vec::new()) };
    static CONTEXT: std::cell::Cell<Option<(usize, usize)>> = const { std::cell::Cell::new(None) };
    static WORKER: std::cell::Cell<Option<(usize, usize)>> = const { std::cell::Cell::new(None) };
    static SCAN_YOUNG: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
    static TRACE_RETENTION: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
}

// A callback can enter another heap and then try to stop its outer collector.
struct CleanupScope;

impl CleanupScope {
    fn enter(gc: &GarbageCollector) -> Self {
        CLEANUP_HEAPS.with_borrow_mut(|heaps| heaps.push(std::ptr::from_ref(gc).addr()));
        Self
    }

    fn contains(gc: &GarbageCollector) -> bool {
        CLEANUP_HEAPS.with_borrow(|heaps| heaps.contains(&std::ptr::from_ref(gc).addr()))
    }
}

impl Drop for CleanupScope {
    fn drop(&mut self) {
        CLEANUP_HEAPS.with_borrow_mut(|heaps| {
            heaps.pop();
        });
    }
}

// A trace callback can step another heap on this thread. Restore the
// suspended callback's observation when the inner trace returns or unwinds.
struct ScanYoung(bool);

impl ScanYoung {
    fn enter(seen: bool) -> Self {
        Self(SCAN_YOUNG.replace(seen))
    }
}

impl Drop for ScanYoung {
    fn drop(&mut self) {
        SCAN_YOUNG.set(self.0);
    }
}

// Only compare the heap address for identity. No pointer is reconstructed.
const _: () = assert!(align_of::<GarbageCollector>() >= 8);
struct TraceRetention(usize);
impl TraceRetention {
    const RESERVED: usize = 4;

    fn current(gc: &GarbageCollector) -> Option<(Retention, bool)> {
        let state = TRACE_RETENTION.get();
        if state & !7 != std::ptr::from_ref(gc).addr() {
            return None;
        }
        let strength = match state & 3 {
            0 => Retention::Strong,
            1 => Retention::Final,
            _ => Retention::Phantom,
        };
        Some((strength, state & Self::RESERVED != 0))
    }

    fn enter(gc: &GarbageCollector, strength: Retention) -> Self {
        Self(TRACE_RETENTION.replace(std::ptr::from_ref(gc).addr() | strength as usize))
    }

    fn callback(gc: &GarbageCollector, strength: Retention) -> Self {
        Self(
            TRACE_RETENTION
                .replace(std::ptr::from_ref(gc).addr() | strength as usize | Self::RESERVED),
        )
    }
}
impl Drop for TraceRetention {
    fn drop(&mut self) {
        TRACE_RETENTION.set(self.0);
    }
}
struct Context(Option<(usize, usize)>);
impl Context {
    fn enter(gc: &GarbageCollector, index: usize) -> Self {
        Self(CONTEXT.replace(Some((std::ptr::from_ref(gc) as usize, index))))
    }
}
impl Drop for Context {
    fn drop(&mut self) {
        CONTEXT.set(self.0);
    }
}

/// Marking and sweep helpers modify this count for each reserved batch. Keep
/// those writes off the cache line containing the epoch and reference stage.
#[cfg_attr(not(target_family = "wasm"), repr(align(64)))]
#[derive(Debug, Default)]
struct WorkReservations(AtomicUsize);

impl WorkReservations {
    fn count(&self) -> usize {
        self.0.load(Ordering::Acquire)
    }

    fn reserve(&self, count: usize) {
        self.0.fetch_add(count, Ordering::AcqRel);
    }

    fn release(&self, count: usize) {
        self.0.fetch_sub(count, Ordering::AcqRel);
    }
}

#[derive(Debug)]
struct Domain {
    cycle: Mutex<Cycle>,
    publication_clock: Option<(Arc<publication_state::Clock>, usize)>,
    epoch: AtomicU64,
    active: AtomicBool,
    marking: AtomicBool,
    requested: AtomicBool,
    mark_queue: MarkQueues,
    relocations: Option<crate::relocation::Queues>,
    relocation_ready: AtomicBool,
    in_flight: WorkReservations,
    trace_ready: AtomicBool,
    weak_phase: AtomicUsize,
    late_references: crate::weak_registry::LateReferences,
    request_ticket: AtomicU64,
    completed_ticket: AtomicU64,
}

impl Domain {
    /// Returns true when a newly published request needs a worker wakeup.
    fn publish_request(&self) -> bool {
        // A repeated request can carry a newer ticket. Keep every publisher
        // in the release chain consumed by the cycle starter's swap(false).
        !self.requested.swap(true, Ordering::AcqRel)
    }

    fn active(&self) -> bool {
        self.publication_clock.as_ref().map_or_else(
            || self.active.load(Ordering::Acquire),
            |(clock, index)| clock.flags(*index) & 1 != 0,
        )
    }
    fn marking(&self) -> bool {
        self.publication_clock.as_ref().map_or_else(
            || self.marking.load(Ordering::Acquire),
            |(clock, index)| clock.flags(*index) & 2 != 0,
        )
    }
    fn weak_phase(&self) -> usize {
        self.publication_clock.as_ref().map_or_else(
            || self.weak_phase.load(Ordering::Acquire),
            |(clock, index)| usize::from((clock.flags(*index) >> 2).to_le_bytes()[0]),
        )
    }

    fn new(
        configuration: &Configuration,
        index: usize,
        clock: Option<Arc<publication_state::Clock>>,
    ) -> Self {
        let workers = if configuration.work_stealing && !cfg!(target_family = "wasm") {
            configuration.threads.clamp(1, 16)
        } else {
            0
        };
        Self {
            cycle: Mutex::new(Cycle::default()),
            publication_clock: clock.map(|clock| (clock, index)),
            epoch: AtomicU64::new(0),
            active: AtomicBool::new(false),
            marking: AtomicBool::new(false),
            requested: AtomicBool::new(false),
            mark_queue: MarkQueues::new(workers, configuration.metadata_shards),
            relocations: configuration
                .independent_relocation
                .then(|| crate::relocation::Queues::new(workers)),
            relocation_ready: AtomicBool::new(false),
            in_flight: WorkReservations::default(),
            trace_ready: AtomicBool::new(false),
            weak_phase: AtomicUsize::new(0),
            late_references: crate::weak_registry::LateReferences::default(),
            request_ticket: AtomicU64::new(0),
            completed_ticket: AtomicU64::new(0),
        }
    }
}

struct WorkerContext(Option<(usize, usize)>);
impl WorkerContext {
    fn enter(gc: &GarbageCollector, worker: usize) -> Self {
        Self(WORKER.replace(Some((std::ptr::from_ref(gc) as usize, worker))))
    }
}
impl Drop for WorkerContext {
    fn drop(&mut self) {
        WORKER.set(self.0);
    }
}

/// Progress of one resumable object scan.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TraceProgress {
    /// All outgoing references have been visited.
    Complete,
    /// Continue scanning at this offset on a subsequent step.
    Continue(usize),
    /// A mutator owns the object; retry without blocking the collector.
    Busy,
}

#[cfg(test)]
#[derive(Debug)]
struct ClaimPause {
    entered: std::sync::mpsc::Sender<()>,
    resume: std::sync::mpsc::Receiver<()>,
}

/// Constructed only by the batch executor while it owns an in-flight
/// reservation for the non-overlapping full epoch.
pub(crate) struct ReservedFullScan {
    epoch: u64,
    mature: usize,
}

impl ReservedFullScan {
    pub(crate) fn covers(&self, gc: &GarbageCollector, metadata: &ObjectMetadata) -> bool {
        std::ptr::eq(metadata.collector.as_ptr(), gc)
            && metadata.age.load(Ordering::Acquire) >= self.mature
            && metadata.retention(1, self.epoch) == Some(Retention::Strong)
            && metadata.birth.is_live()
    }
}

/// Exclusive ownership of an unregistered descriptor, including clear progress.
/// Moving this task out of a cycle allows that cycle to finish during a callback.
#[derive(Debug)]
struct ReclaimTask {
    metadata: MetadataRef,
    stage: ReclaimStage,
}

#[derive(Debug)]
enum ReclaimStage {
    Clear { offset: usize, finalized: bool },
    Return(Box<crate::metadata::RegionalReturn>),
}

enum RetirementStep {
    Progress(bool),
    Contended,
}

impl ReclaimTask {
    fn new(metadata: MetadataRef) -> Self {
        Self {
            metadata,
            stage: ReclaimStage::Clear {
                offset: 0,
                finalized: false,
            },
        }
    }

    fn advance(&mut self, progress: TraceProgress) {
        if let ReclaimStage::Clear { offset, finalized } = &mut self.stage {
            *finalized = true;
            if let TraceProgress::Continue(next) = progress {
                *offset = next;
            }
        }
    }
}

/// Accumulate useful work without taking the statistics mutex for every object.
#[derive(Default)]
struct WorkCounters {
    marked: usize,
    swept: usize,
    freed: usize,
    logical_freed: usize,
    promoted: usize,
    references: usize,
}

impl WorkCounters {
    fn flush(&mut self, stats: &mut Statistics) {
        let delta = std::mem::take(self);
        stats.objects_marked += delta.marked;
        stats.objects_swept += delta.swept;
        stats.bytes_freed += delta.logical_freed;
        stats.managed_bytes_freed += delta.freed;
        stats.objects_promoted += delta.promoted;
        stats.references_examined += delta.references;
    }
}

/// Metadata that owns VM roots outside the traced object graph.
pub trait HeapOwner: Send + Sync {
    /// Releases VM-owned roots during shutdown. Escaped owning handles remain valid.
    fn release_heap_roots(&self);
}

/// Visit outgoing managed references. Implementations must not allocate Java objects.
///
/// # Safety
/// Implementations must report every managed edge that is not independently
/// rooted, and must not claim a completed scan while required edges are hidden.
/// Capability queries must uphold their documented tracing/barrier exclusions.
/// Strong fields demoted by attachment or `unroot` must regain owning protection
/// in `root` before unrestricted movement. Embedded cells must receive the actual
/// owner's context and detach from it when rooted for movement. Foreign-heap
/// strong edges must retain independent ownership because this heap cannot trace them.
/// Hooks must preserve initialized storage and the lifetime of outstanding
/// borrows during preparation, relocation, clearing and destruction.
///
/// ```compile_fail,E0200
/// use ristretto_gc::{GarbageCollector, Trace};
/// struct Unchecked;
/// impl Trace for Unchecked {
///     fn trace(&self, _: &GarbageCollector) {}
/// }
/// ```
pub unsafe trait Trace {
    /// Whether values of this type can contain managed or conditional edges.
    /// Returning false lets containers omit element tracing. The default preserves
    /// custom tracing hooks; only opt out when no tracing action is required.
    /// This does not permit skipping element destructors.
    #[must_use]
    fn may_contain_gc_references() -> bool
    where
        Self: Sized,
    {
        true
    }

    /// The explicit audited certificate for exactly this payload type.
    #[must_use]
    fn canonical_payload() -> Option<crate::CanonicalPayload<Self>>
    where
        Self: Sized,
    {
        None
    }

    /// Whether the current value requires an owner-mutation notification after a
    /// managed write. False is valid only when this value has no managed or
    /// conditional edges, and no custom tracing action requires that notification.
    ///
    /// Managed write guards query this after `unroot`, while exclusively borrowed.
    /// The query must take constant time and must not allocate, acquire payload
    /// locks, or call the collector. It does not suppress unroot hooks, deletion
    /// barriers, tracing, or relocation bookkeeping. The default is conservative.
    fn needs_write_barrier(&self) -> bool {
        true
    }

    /// Whether all outgoing edges currently carry owning strong registrations and
    /// demotion only converts those same edges using ordinary deletion barriers.
    /// Returning true permits a write guard to publish remembered membership
    /// while those registrations still protect the graph. Conditional edges and
    /// custom trace actions must return false. The query runs under exclusive
    /// access and must not allocate, lock or call into the collector.
    fn edges_rooted_before_unroot(&self) -> bool {
        false
    }

    /// Maximum useful slots per scan, sampled before allocation publication.
    /// A small value still traces correctly if the payload later changes shape.
    fn trace_work(&self) -> usize {
        32
    }

    /// Moves exclusively owned payload storage into a region heap before publication.
    /// # Errors
    /// Returns an allocation error without publishing partially initialized storage.
    fn prepare(&mut self, _heap: &crate::region::RegionHeap) -> Result<()> {
        Ok(())
    }

    /// Prepares a bounded part of an unpublished payload. Existing custom values
    /// retain their synchronous prepare hook unless they override this method.
    /// # Errors
    /// Returns an allocation error while retaining ownership of the source.
    fn prepare_step(
        &mut self,
        heap: &crate::region::RegionHeap,
        _bytes: usize,
    ) -> Result<TraceProgress> {
        self.prepare(heap)?;
        Ok(TraceProgress::Complete)
    }

    /// Relocates a bounded part of an exclusively borrowed payload.
    fn compact_step(&mut self, _heap: &crate::region::RegionHeap, _bytes: usize) -> TraceProgress {
        TraceProgress::Complete
    }

    /// Acquires exclusive payload access without blocking. Ordinary Rust values stay pinned.
    fn compact_guarded(&self, _heap: &crate::region::RegionHeap, _bytes: usize) -> TraceProgress {
        TraceProgress::Complete
    }

    /// Advisory identity of a relocatable payload's region. This creates no borrow
    /// or root; guarded implementations must inspect it without blocking.
    fn relocation_region(&self, _heap: &crate::region::RegionHeap) -> Option<u64> {
        None
    }

    /// Visit outgoing references using `Gc::trace`.
    fn trace(&self, collector: &GarbageCollector);

    /// Visits consecutive elements, with the same edge and retention obligations
    /// as calling each element's trace hook in order.
    fn trace_slice(items: &[Self], collector: &GarbageCollector)
    where
        Self: Sized,
    {
        for item in items {
            item.trace(collector);
        }
    }

    /// Visit a bounded part of an object. Custom implementations default to one whole scan.
    fn trace_step(
        &self,
        collector: &GarbageCollector,
        _offset: usize,
        _budget: usize,
    ) -> TraceProgress {
        self.trace(collector);
        TraceProgress::Complete
    }

    /// Whether retirement must invoke this type's clearing hook.
    /// Only return false if skipping every call to `clear_step` preserves
    /// all edge, resource and callback obligations for every value of this type.
    /// Finalization and ordinary Rust destruction are independent.
    #[must_use]
    fn needs_clear() -> bool
    where
        Self: Sized,
    {
        true
    }

    /// Whether each retirement clearing step and destruction after clearing completes
    /// can run on a collector worker without calling user code, waiting on another
    /// thread, or doing work proportional to payload size, assuming allocator progress.
    /// Returning true only affects scheduling.
    /// The collector still runs both cleanup operations. Separately registered
    /// finalizers and heap owners retain their own callback requirements.
    #[must_use]
    fn has_bounded_cleanup() -> bool
    where
        Self: Sized,
    {
        false
    }

    /// Releases outgoing edges in bounded batches before destruction of unreachable values.
    fn clear_step(&mut self, _offset: usize, _budget: usize) -> TraceProgress {
        TraceProgress::Complete
    }

    /// Bytes of separately allocated storage owned exclusively by this value.
    fn external_size(&self) -> usize {
        0
    }

    /// Returns metadata that holds roots with the VM's lifetime.
    fn heap_owner(&self) -> Option<Arc<dyn HeapOwner>> {
        None
    }

    /// Binds embedded values to their owner and demotes same-heap strong handles.
    /// Foreign-heap strong handles retain independent owning protection.
    ///
    /// # Safety
    /// `owner` must describe this value's actual traced owner. Its graph must
    /// remain protected during conversion, and required mutation barriers must
    /// be published before exposure. No demoted value may escape without `root`.
    ///
    /// ```compile_fail,E0133
    /// use ristretto_gc::{Gc, Trace, TraceOwner};
    /// fn demote(value: &mut Option<Gc<u64>>, owner: TraceOwner<'_>) {
    ///     value.unroot(owner);
    /// }
    /// ```
    unsafe fn unroot(&mut self, _owner: crate::TraceOwner<'_>) {}

    /// Restores owning strong handles before a managed value is exposed through
    /// an unrestricted mutable borrow. Such a borrow may move the whole value,
    /// so ownership must travel with its fields, not only with the write guard.
    /// Implementations which demote strong handles in `unroot` must restore them
    /// here. Conditional/weak references retain their existing semantics. This
    /// hook does not authorize skipping tracing while the payload is locked.
    fn root(&mut self) {}

    /// Associates a value and its embedded cells with their actual traced owner.
    ///
    /// # Safety
    /// The owner and graph must satisfy `unroot`'s contract. Initial attachment
    /// requires construction protection until publication or destruction completes.
    ///
    /// ```compile_fail,E0133
    /// use ristretto_gc::{Gc, Trace, TraceOwner};
    /// fn attach(value: &mut Option<Gc<u64>>, owner: TraceOwner<'_>) {
    ///     value.attach(owner);
    /// }
    /// ```
    unsafe fn attach(&mut self, owner: crate::TraceOwner<'_>) {
        // SAFETY: the caller supplies the allocation's protected ownership context.
        unsafe { self.unroot(owner) };
    }
}

// Each allocation has a non-reusable positive identity and is promoted and
// retired at most once. A Strong transition on a LIVE mature descriptor is
// unique in a pinned full epoch, including after a weaker discovery.
#[derive(Default, Debug)]
struct MatureCensus {
    promotions: AtomicU64,
    retirements: AtomicU64,
    strong: AtomicU64,
}

#[derive(Clone, Copy, Debug)]
enum MatureCoverage {
    Unchecked,
    Incomplete,
    Complete(u64),
}

impl MatureCensus {
    fn coverage(&self, marked_edges: u64) -> MatureCoverage {
        let promotions = self.promotions.load(Ordering::Acquire);
        let retired = self.retirements.load(Ordering::Acquire);
        if promotions != 0
            && promotions.checked_sub(retired)
                == Some(self.strong.load(Ordering::Acquire) + marked_edges)
        {
            MatureCoverage::Complete(promotions)
        } else {
            MatureCoverage::Incomplete
        }
    }

    fn current(&self, proof: MatureCoverage) -> bool {
        matches!(proof, MatureCoverage::Complete(p) if p == self.promotions.load(Ordering::Acquire))
    }
}

/// A collection can reclaim either the young generation or the entire heap.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CollectionKind {
    Young,
    Full,
    /// Conservative old-only pass; explicit full collections retain precise semantics.
    Old,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Phase {
    Idle,
    ResetMarks,
    ResetReferences,
    ContextRoots,
    RootSlots,
    Roots,
    CrossRoots,
    OldHandoff,
    OldReferences,
    Remembered,
    Mark,
    References,
    Sweep,
    SelectRegions,
    Compact,
}

#[derive(Debug)]
struct Cycle {
    phase: Phase,
    kind: CollectionKind,
    cursor: usize,
    young_cursor: crate::index::YoungCursor,
    remembered_cursor: crate::index::RememberedCursor,
    remembered_overflow: crate::index::RememberedOverflow,
    sweep_young_cursor: crate::index::YoungCursor,
    object_cursor: crate::index::Cursor,
    sweep_object_cursor: crate::index::Cursor,
    mature_coverage: MatureCoverage,
    packet_cleanup: Option<usize>,
    young_retirement: crate::index::YoungRetirement,
    #[cfg(not(feature = "experimental-identity-index"))]
    identity_cursor: crate::identity_pages::RetireCursor,
    #[cfg(feature = "experimental-identity-index")]
    identity_removals: [crate::identity_index::Removal; 16],
    #[cfg(feature = "experimental-identity-index")]
    identity_cleanup: crate::identity_index::Cleanup,
    epoch: u64,
    started: Option<std::time::Instant>,
    reference_stage: usize,
    late_reference_turn: bool,
    retry_reference: Option<crate::weak_registry::Entry>,
    ticket: u64,
    relocations: VecDeque<(MetadataRef, usize)>,
    relocation_scan_done: bool,
    phantom_retained: BTreeSet<usize>,
    reclaim: VecDeque<ReclaimTask>,
    refund_selection: Option<crate::retirement_grants::Selection>,
    reclaimed_bytes: usize,
    payload_trim: crate::region::TrimCursor,
    descriptor_trim: crate::region::TrimCursor,
    handoff: Option<(MetadataRef, usize)>,
    root_cursor: crate::roots::Cursor,
    context_cursor: crate::context::RootCursor,
}

impl Default for Cycle {
    fn default() -> Self {
        Self {
            phase: Phase::Idle,
            kind: CollectionKind::Full,
            cursor: 0,
            young_cursor: crate::index::YoungCursor::default(),
            remembered_cursor: crate::index::RememberedCursor::default(),
            remembered_overflow: crate::index::RememberedOverflow::default(),
            sweep_young_cursor: crate::index::YoungCursor::default(),
            object_cursor: crate::index::Cursor::default(),
            sweep_object_cursor: crate::index::Cursor::default(),
            mature_coverage: MatureCoverage::Unchecked,
            packet_cleanup: None,
            young_retirement: crate::index::YoungRetirement::default(),
            #[cfg(not(feature = "experimental-identity-index"))]
            identity_cursor: crate::identity_pages::RetireCursor::default(),
            #[cfg(feature = "experimental-identity-index")]
            identity_removals: std::array::from_fn(|_| crate::identity_index::Removal::default()),
            #[cfg(feature = "experimental-identity-index")]
            identity_cleanup: crate::identity_index::Cleanup::default(),
            epoch: 0,
            started: None,
            reference_stage: 0,
            late_reference_turn: false,
            retry_reference: None,
            ticket: 0,
            relocations: VecDeque::new(),
            relocation_scan_done: false,
            phantom_retained: BTreeSet::new(),
            reclaim: VecDeque::with_capacity(phase_work::SWEEP_BATCH),
            refund_selection: None,
            reclaimed_bytes: 0,
            payload_trim: crate::region::TrimCursor::default(),
            descriptor_trim: crate::region::TrimCursor::default(),
            handoff: None,
            root_cursor: crate::roots::Cursor::default(),
            context_cursor: crate::context::RootCursor::default(),
        }
    }
}

fn lock<T>(mutex: &Mutex<T>) -> std::sync::MutexGuard<'_, T> {
    mutex
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
}

pub(crate) fn try_lock<T>(mutex: &Mutex<T>) -> Option<std::sync::MutexGuard<'_, T>> {
    match mutex.try_lock() {
        Ok(guard) => Some(guard),
        Err(std::sync::TryLockError::Poisoned(error)) => Some(error.into_inner()),
        Err(std::sync::TryLockError::WouldBlock) => None,
    }
}

#[cfg_attr(
    not(all(target_family = "wasm", target_os = "unknown")),
    expect(clippy::unnecessary_wraps, reason = "bare wasm has no host clock")
)]
fn now() -> Option<std::time::Instant> {
    #[cfg(all(target_family = "wasm", target_os = "unknown"))]
    {
        None
    }
    #[cfg(not(all(target_family = "wasm", target_os = "unknown")))]
    {
        Some(std::time::Instant::now())
    }
}

#[cfg(not(target_family = "wasm"))]
#[path = "native_callbacks.rs"]
mod native_callbacks;
#[cfg(not(target_family = "wasm"))]
#[path = "worker_signals.rs"]
mod worker_signals;
#[cfg(not(target_family = "wasm"))]
use worker_signals::WorkerSignals;

/// A collector with stable descriptors, resumable tracing and optional payload compaction.
///
/// Native workers and cooperative callers drive the same state machine. Publication uses
/// either the default recursive gate or counted admission with atomic phase transitions.
/// User tracing never runs with the legacy publication writer held.
/// Generational mutation must call `remember` on the owner and shade overwritten references.
#[derive(Debug)]
pub struct GarbageCollector {
    this: Weak<Self>,
    configuration: Configuration,
    regions: Option<Arc<crate::region::RegionHeap>>,
    descriptor_regions: Option<Arc<crate::region::RegionHeap>>,
    small_allocations: crate::allocation::SmallAllocations,
    statistics: Mutex<Statistics>,
    leaves_marked_at_birth: AtomicUsize,
    scheduler: Option<crate::scheduling::Scheduler>,
    #[cfg(not(target_family = "wasm"))]
    pacing: Option<crate::pacing::Controller>,
    pub(crate) diagnostics: Arc<crate::diagnostics::Counters>,
    pub(crate) recovery: crate::recovery::Recovery,
    pub(crate) grant_registry: crate::retirement_grants::Registry,
    roots: crate::roots::TreeRoots,
    #[cfg(test)]
    tree_root_insert_probe: Mutex<Option<fn(&GarbageCollector)>>,
    pub(crate) contexts: crate::context::Registry,
    root_slots: Option<Box<crate::roots::RootSlots>>,
    typed_slots: once_cell::race::OnceBox<crate::roots::RootSlots>,
    next_root_id: AtomicUsize,
    objects: crate::index::Objects,
    mature_census: MatureCensus,
    #[cfg(feature = "experimental-identity-index")]
    identities: crate::identity_index::Index,
    #[cfg(not(feature = "experimental-identity-index"))]
    identities: crate::identity_pages::Index,
    heap_owners: Mutex<BTreeMap<usize, Weak<dyn HeapOwner>>>,
    remembered: crate::index::Remembered,
    publication: crate::sync::RecursiveRwLock<()>,
    domains: [Domain; 2],
    old_requested: AtomicBool,
    old_active: AtomicBool,
    // Phase commits also certify private births with no traversal obligation
    // in the legacy policy. Its other mutators still use the publication lock.
    phase_clock: Arc<publication_state::Clock>,
    publication_clock: Option<Arc<publication_state::Clock>>,
    coordinator: Mutex<()>,
    retired: arc_swap::ArcSwapOption<crate::publication_queue::Queue<ReclaimTask>>,
    retired_objects: AtomicUsize,
    retired_bytes: AtomicUsize,
    retired_ready: AtomicUsize,
    #[cfg(not(target_family = "wasm"))]
    callback_jobs: AtomicUsize,
    retirement_turn: AtomicBool,
    released_donors: crate::retirement_grants::ReleasedDonors,
    released_turn: AtomicBool,
    barrier_turn: AtomicBool,
    #[cfg(test)]
    barrier_claim_observer: Mutex<Option<std::sync::mpsc::Sender<()>>>,
    #[cfg(test)]
    barrier_prepare_pause: Mutex<Option<ClaimPause>>,
    // Claims hold a reader only while reserving tasks; mark closure takes the writer.
    mark_claims: Option<Box<RwLock<()>>>,
    #[cfg(test)]
    mark_claim_pause: Mutex<Option<ClaimPause>>,
    #[cfg(test)]
    metadata_prepare_pause: Mutex<Option<ClaimPause>>,
    #[cfg(test)]
    metadata_publish_failure: Mutex<Option<Arc<AtomicUsize>>>,
    #[cfg(test)]
    metadata_registration_observer:
        Mutex<Option<std::sync::mpsc::Sender<(usize, crate::ObjectId)>>>,
    #[cfg(test)]
    identity_resolution_pause: Mutex<Option<ClaimPause>>,
    #[cfg(test)]
    pending_transfer_pause: Mutex<Option<ClaimPause>>,
    #[cfg(test)]
    edge_certificate_pause: Mutex<Option<ClaimPause>>,
    relocation_claims: Option<Box<RwLock<()>>>,
    next_domain: AtomicUsize,
    weak_slots: crate::weak_registry::WeakRegistry,
    next_weak_id: AtomicUsize,
    pending_references: once_cell::race::OnceBox<
        crate::publication_queue::Queue<(crate::ObjectId, crate::roots::TreeRoot)>,
    >,
    managed_budget: Arc<crate::allocation_budget::AllocationBudget>,
    allocation_debt: AtomicUsize,
    published_bytes: AtomicUsize,
    published_debt: AtomicUsize,

    #[cfg(not(target_family = "wasm"))]
    worker_signals: arc_swap::ArcSwapOption<WorkerSignals>,
    #[cfg(not(target_family = "wasm"))]
    collector_thread: Mutex<Vec<thread::JoinHandle<()>>>,
    #[cfg(not(target_family = "wasm"))]
    refund_notifier: arc_swap::ArcSwapOption<crate::recovery::RefundNotifier>,
}

impl GarbageCollector {
    fn domain_index(&self) -> usize {
        CONTEXT
            .get()
            .filter(|(address, _)| *address == std::ptr::from_ref(self) as usize)
            .map_or(1, |(_, index)| index)
    }
    fn domain_at(&self, index: usize) -> &Domain {
        if index == 0 {
            &self.domains[0]
        } else {
            &self.domains[1]
        }
    }
    fn domain(&self) -> &Domain {
        self.domain_at(self.domain_index())
    }
    fn local_queue(&self) -> usize {
        if !self.configuration.work_stealing {
            return 0;
        }
        WORKER
            .get()
            .filter(|(address, _)| *address == std::ptr::from_ref(self) as usize)
            .map_or(0, |(_, worker)| {
                self.domain().mark_queue.worker_queue(worker)
            })
    }
    fn in_flight(&self) -> usize {
        // Preserve the conservative cross-generation interlock. Reservations stay
        // charged until callbacks, continuation publication and counter merging finish.
        self.domains
            .iter()
            .map(|domain| domain.in_flight.count())
            .sum()
    }

    fn closure_in_flight(&self) -> usize {
        if (self.old_collection_active() && self.domain_index() == 0)
            || (self.configuration.overlapping_generations && self.domains[1].weak_phase() == 5)
        {
            // Full tracing and reference decisions are closed. Full copy tasks
            // are restricted to its traced snapshot (never post-trace births),
            // whose graphs remain roots for young cycles. Leases alone would
            // protect descriptors, not callback access to their outgoing graph.
            self.domain().in_flight.count()
        } else {
            self.in_flight()
        }
    }

    fn terminal_priority(&self, index: usize) -> bool {
        !(self.configuration.overlapping_generations
            && index == 1
            && self.domains[1].weak_phase() == 5)
    }

    /// Marks a private birth before registry visibility. True assigns this
    /// attempt one traversal in the observed strong epoch.
    fn mark_registration_birth(
        &self,
        metadata: &ObjectMetadata,
        index: usize,
        stage: usize,
    ) -> bool {
        let epoch = self.domain_at(index).epoch.load(Ordering::Acquire);
        if index == 1
            && self.configuration.old_collection_interval != 0
            && self.old_active.load(Ordering::Acquire)
        {
            metadata.old_birth.store(epoch, Ordering::Release);
            metadata.mark_born(index, epoch);
            return false;
        }
        if self.configuration.overlapping_generations && index == 1 && stage == 5 {
            metadata.mark_born(index, epoch);
        } else {
            metadata.mark_black(index, epoch);
        }
        stage == 0
    }
    fn promotion_snapshot(&self, metadata: &MetadataRef) {
        if self.domain_index() == 0 && self.old_collection_active() {
            // Promotion is admitted before publishing the new age. The old pass
            // must own its outgoing old edges, or blacken it after closure.
            let epoch = self.domains[1].epoch.load(Ordering::Acquire);
            if self.domains[1].weak_phase() == 0 {
                metadata.mark(1, epoch, Retention::Strong);
                // Queue publication follows the age store under the same
                // admission; a handoff mark alone is not an old-core traversal.
            } else {
                metadata.mark_black(1, epoch);
            }
            return;
        }
        if self.domain_index() == 0 && self.domains[1].active() {
            let retention = metadata
                .retention(0, self.domains[0].epoch.load(Ordering::Acquire))
                .unwrap_or(Retention::Phantom);
            let _retention = TraceRetention::enter(self, retention);
            let _context = Context::enter(self, 1);
            self.shade_metadata(metadata);
        }
    }
    fn domain_has_work(&self, index: usize) -> bool {
        let domain = self.domain_at(index);
        domain.active()
            || domain.requested.load(Ordering::Acquire)
            || (index == 1 && self.old_requested.load(Ordering::Acquire))
    }

    #[cfg(not(target_family = "wasm"))]
    fn worker_pending(&self) -> bool {
        if self.callback_jobs.load(Ordering::Acquire) == 0 {
            return self.pending();
        }
        // A callback can retain its physical charge indefinitely. A worker
        // still sees every published retirement while that private job sleeps.
        self.pending_with_retirement(self.retired.load().as_ref().is_some_and(|q| !q.is_empty()))
    }

    fn pending(&self) -> bool {
        self.pending_with_retirement(self.retired_ready.load(Ordering::Acquire) != 0)
    }

    fn pending_with_retirement(&self, ready: bool) -> bool {
        self.remembered.dirty.cleanup_pending()
            || ready
            || self.released_donors.has_jobs()
            || self
                .scanned_slots()
                .is_some_and(crate::roots::RootSlots::repair_pending)
            || self.contexts.repairs.pending()
            || self.old_requested.load(Ordering::Acquire)
            || self
                .domains
                .iter()
                .any(|domain| domain.active() || domain.requested.load(Ordering::Acquire))
    }

    /// Physical payload storage, when the compactable backend is enabled.
    #[must_use]
    pub fn region_heap(&self) -> Option<&Arc<crate::region::RegionHeap>> {
        self.regions.as_ref()
    }

    pub(crate) fn prepare_payload(&self, data: &mut impl Trace) -> Result<()> {
        if let Some(heap) = &self.regions {
            data.prepare(heap)?;
        }
        Ok(())
    }

    pub(crate) async fn prepare_payload_async(&self, data: &mut impl Trace) -> Result<()> {
        if let Some(heap) = &self.regions {
            while data.prepare_step(heap, 64 * 1024)? != TraceProgress::Complete {
                crate::recovery::yield_once().await;
            }
        }
        Ok(())
    }

    pub(crate) fn address_for_identity(&self, id: crate::ObjectId) -> Option<usize> {
        let address = self.identities.get(id.to_i64().cast_unsigned());
        #[cfg(test)]
        {
            let pause = lock(&self.identity_resolution_pause).take();
            if let Some(pause) = pause {
                pause.entered.send(()).expect("identity address resolved");
                pause
                    .resume
                    .recv_timeout(std::time::Duration::from_secs(10))
                    .expect("resume identity resolution");
            }
        }
        address
    }

    #[cfg(test)]
    fn set_identity_hint(&self, id: u64, address: Option<usize>) {
        self.identities.test_set(id, address);
    }

    pub(crate) fn identity_for_address(&self, address: usize) -> Option<crate::ObjectId> {
        (address != 0)
            .then(|| self.objects.lookup(address))
            .flatten()
            .map(|m| m.id)
    }
    pub(crate) fn allocation_guard(&self) -> Publication<'_> {
        if let Some(clock) = &self.publication_clock {
            return Publication {
                admission: Some(clock.enter()),
                _legacy: None,
            };
        }
        let guard = if !self.diagnostics_enabled() {
            self.publication.read()
        } else if let Some(guard) = self.publication.try_read() {
            guard
        } else {
            let _measurement = self.diagnostics.publication_read_wait.measure(true);
            self.publication.read()
        };
        Publication {
            admission: None,
            _legacy: Some(guard),
        }
    }

    pub(crate) fn root_publication_guard(&self) -> RootPublication<'_> {
        RootPublication {
            collector: self,
            _publication: self.allocation_guard(),
        }
    }

    fn barrier_guard(&self) -> Publication<'_> {
        if let Some(clock) = &self.publication_clock {
            Publication {
                admission: Some(clock.enter_barrier()),
                _legacy: None,
            }
        } else {
            self.allocation_guard()
        }
    }

    fn try_publication_write(&self) -> Option<PhasePublication<'_>> {
        if self.publication_clock.is_some() {
            return Some(PhasePublication {
                _legacy: None,
                #[cfg(all(
                    feature = "experimental-pause-recorder",
                    target_os = "linux",
                    target_has_atomic = "64"
                ))]
                _pause: None,
            });
        }
        #[cfg(all(
            feature = "experimental-pause-recorder",
            target_os = "linux",
            target_has_atomic = "64"
        ))]
        let requested = self
            .configuration
            .pause_recorder
            .as_ref()
            .map(|recorder| recorder.request());
        if let Some(guard) = self.publication.try_write() {
            return Some(PhasePublication {
                #[cfg(all(
                    feature = "experimental-pause-recorder",
                    target_os = "linux",
                    target_has_atomic = "64"
                ))]
                _pause: self
                    .configuration
                    .pause_recorder
                    .as_ref()
                    .zip(requested)
                    .and_then(|(recorder, requested)| recorder.acquired(requested)),
                _legacy: Some(guard),
            });
        }
        #[cfg(all(
            feature = "experimental-pause-recorder",
            target_os = "linux",
            target_has_atomic = "64"
        ))]
        if let Some(recorder) = &self.configuration.pause_recorder {
            recorder.failed_attempt();
        }
        // Waiting for a stalled reader would retain this step's coordinator and
        // cursor. Leave the admission untouched and release those locks instead.
        if self.diagnostics_enabled() {
            self.diagnostics
                .publication_retries
                .fetch_add(1, Ordering::Relaxed);
        }
        None
    }

    fn observe_phase(&self) -> Option<publication_state::Observation> {
        let observed = self.phase_clock.observe();
        if observed.is_none() && self.diagnostics_enabled() {
            self.diagnostics
                .publication_retries
                .fetch_add(1, Ordering::Relaxed);
        }
        observed
    }

    fn commit_phase(
        &self,
        observed: publication_state::Observation,
        active: bool,
        marking: bool,
        stage: usize,
    ) -> bool {
        let flags = u64::from(active)
            | (u64::from(marking) << 1)
            | (u64::from(stage.to_le_bytes()[0]) << 2);
        let committed = self
            .phase_clock
            .transition(observed, self.domain_index(), flags);
        if self.publication_clock.is_some() && self.diagnostics_enabled() {
            let counter = if committed {
                &self.diagnostics.publication_transitions
            } else {
                &self.diagnostics.publication_retries
            };
            counter.fetch_add(1, Ordering::Relaxed);
        }
        committed
    }

    /// Explicit diagnostic recorder associated before collector construction.
    #[cfg(all(
        feature = "experimental-pause-recorder",
        target_os = "linux",
        target_has_atomic = "64"
    ))]
    #[must_use]
    pub fn pause_recorder(&self) -> Option<&crate::pause_recorder::PauseRecorder> {
        self.configuration.pause_recorder.as_deref()
    }

    pub(crate) fn diagnostics_enabled(&self) -> bool {
        self.configuration.diagnostics
    }

    pub(crate) fn pin_diagnostics(&self) -> Option<Arc<crate::diagnostics::Counters>> {
        (self.diagnostics_enabled()
            && self.configuration.cost_aware_relocation
            && self.regions.is_some())
        .then(|| self.diagnostics.clone())
    }

    /// Returns optional stall and work counters; durations are not global pauses.
    #[must_use]
    pub fn diagnostics(&self) -> crate::Diagnostics {
        let mut snapshot = self.diagnostics.snapshot(self.diagnostics_enabled());
        if snapshot.enabled {
            snapshot.active_publications = self
                .publication_clock
                .as_ref()
                .map_or(0, |clock| clock.active());
            snapshot.allocation_waiters = self.recovery.waiter_count();
            snapshot.execution_contexts = self.contexts.statistics().0;
            snapshot.context_storage_bytes = snapshot
                .execution_contexts
                .saturating_mul(size_of::<crate::context::State>());
            snapshot.pending_barrier_records = self.contexts.pending.load(Ordering::Acquire);
            snapshot.retired_bytes = self.retired_bytes.load(Ordering::Acquire);
            snapshot.retired_objects = self.retired_objects.load(Ordering::Acquire);
            snapshot.retired_ready = self.retired_ready.load(Ordering::Acquire);
            #[cfg(not(target_family = "wasm"))]
            if let Some(pacing) = &self.pacing {
                snapshot.worker_target = pacing.target();
                snapshot.worker_pacing_us = pacing.delay();
            }
            self.contexts.for_each(|context| {
                let cache = lock(&context.allocation);
                snapshot.cached_allocation_bytes += cache.retained_bytes();
                snapshot.cached_allocation_credit += context.available_credit();
                snapshot.local_allocation_hits += cache.hits;
                snapshot.local_allocation_refills += cache.refills;
            });
            if let Some(heap) = &self.descriptor_regions {
                snapshot.descriptor_committed_bytes = heap.statistics().committed_bytes;
                snapshot.descriptor_occupied_bytes = heap.statistics().occupied_bytes;
            }
            for (index, domain) in self.domains.iter().enumerate() {
                let (queued, capacity) = domain.mark_queue.statistics();
                if let Some(value) = snapshot.queued_mark_tasks.get_mut(index) {
                    *value = queued;
                }
                if let Some(value) = snapshot.mark_queue_capacity.get_mut(index) {
                    *value = capacity;
                }
                if let Some(value) = snapshot.in_flight_work.get_mut(index) {
                    *value = domain.in_flight.count();
                }
                if let Some(queues) = &domain.relocations {
                    if let Some(value) = snapshot.queued_relocation_tasks.get_mut(index) {
                        *value = queues.len();
                    }
                    if let Some(value) = snapshot.relocation_queue_capacity.get_mut(index) {
                        *value = queues.capacity();
                    }
                }
            }
        }
        snapshot
    }

    pub(crate) fn recover_allocation(&self) {
        let _measurement = self
            .diagnostics
            .allocation_recovery
            .measure(self.diagnostics_enabled());
        self.wait_full_cycle();
    }

    pub(crate) fn recover_physical(&self, layout: std::alloc::Layout) {
        let _measurement = self
            .diagnostics
            .allocation_recovery
            .measure(self.diagnostics_enabled());
        let Some(heap) = &self.regions else {
            self.wait_full_cycle();
            return;
        };
        let ticket = self.request_recovery_cycle();
        while self.completed_full_ticket() < ticket {
            // An available range is advisory. Advance at least one batch on each
            // retry so a host allocation failure cannot spin on a false positive.
            self.step();
            if heap.can_allocate(layout) {
                break;
            }
        }
    }

    pub(crate) async fn recover_allocation_async(&self) -> Result<()> {
        if crate::cell::has_guards() {
            return Err(crate::Error::CollectionPhaseError(
                "asynchronous recovery requires released managed guards".into(),
            ));
        }
        let _measurement = self
            .diagnostics
            .async_allocation_recovery
            .measure(self.diagnostics_enabled());
        // A failed retry may follow completion of a previous cycle in this poll.
        // Yield before starting another so one allocation poll cannot run two batches.
        crate::recovery::yield_once().await;
        self.collect_full_async().await
    }

    pub(crate) async fn recover_physical_async(&self, layout: std::alloc::Layout) -> Result<()> {
        if self.regions.is_none() {
            return self.recover_allocation_async().await;
        }
        if crate::cell::has_guards() {
            return Err(crate::Error::CollectionPhaseError(
                "asynchronous recovery requires released managed guards".into(),
            ));
        }
        let _measurement = self
            .diagnostics
            .async_allocation_recovery
            .measure(self.diagnostics_enabled());
        // Each retry yields before it can assist again, including after an
        // early physical offer is consumed by a competing allocator.
        crate::recovery::yield_once().await;
        let ticket = self.request_recovery_cycle();
        crate::recovery::CollectionWait::for_physical(self, ticket, layout).await?;
        Ok(())
    }

    pub(crate) fn acquire_typed_root<T: 'static>(
        &self,
        address: usize,
        expected: Option<crate::ObjectId>,
    ) -> Result<(
        std::ptr::NonNull<T>,
        crate::ObjectId,
        crate::roots::Registration,
    )> {
        self.acquire_checked_root(address, expected, ObjectMetadata::is_type::<T>)
    }

    /// An armed slot or its one-shot replacement keeps root acceptance ordered
    /// with scanning. The caller supplies the type proof for its handle source.
    #[expect(
        clippy::expect_used,
        reason = "typed lookup owns a prepared slot until activation"
    )]
    fn acquire_checked_root<T>(
        &self,
        address: usize,
        expected: Option<crate::ObjectId>,
        accepts: impl FnOnce(&ObjectMetadata) -> bool,
    ) -> Result<(
        std::ptr::NonNull<T>,
        crate::ObjectId,
        crate::roots::Registration,
    )> {
        let invalid = || {
            crate::Error::InvalidPointer(
                "unknown, logically dead, or incorrectly typed allocation".into(),
            )
        };
        let _measurement = self
            .diagnostics
            .root_acquire
            .measure(self.diagnostics_enabled());
        let mut reservation = self.reserve_owned_root(address);
        let slot = reservation.slot.get_or_insert_with(|| {
            self.typed_slots
                .get_or_init(|| Box::new(crate::roots::RootSlots::default()))
                .prepare(address)
        });
        slot.arm();
        let observation = self.phase_clock.indexed_root();
        let metadata = self
            .objects
            .lookup(address)
            .filter(|m| expected.is_none_or(|id| m.id == id) && accepts(m))
            .ok_or_else(invalid)?;
        let ptr = std::ptr::NonNull::new(metadata.trace.ptr.cast_mut().cast::<T>())
            .ok_or_else(invalid)?;
        let covered = self.typed_root_covered(observation, &metadata);
        #[cfg(all(test, not(target_family = "wasm"), not(miri)))]
        publication_tests::pause_after_typed_lookup();
        if covered && slot.activate(address) {
            let registration = reservation
                .slot
                .take()
                .expect("activated typed root")
                .finish(reservation.context.take());
            if self.diagnostics_enabled() {
                self.diagnostics
                    .root_registrations
                    .fetch_add(1, Ordering::Relaxed);
            }
            #[cfg(all(test, not(target_family = "wasm"), not(miri)))]
            publication_tests::pause_after_typed_activation();
            return Ok((ptr, metadata.id, registration));
        }

        // The one-shot record is both the registry's decision and the identity
        // retained by epoch-bound traversal proposals. Releasing this original
        // reservation cannot affect a subsequent attempt.
        let context = reservation.context.clone();
        drop(reservation);
        let slots = context.as_ref().map_or_else(
            || self.scanned_slots().expect("typed root slots installed"),
            |context| &context.roots,
        );
        loop {
            let record = slots.prepare_typed_attempt(address);
            let mut attempt = RootReservation {
                collector: self,
                address,
                context: context.clone(),
                slot: Some(crate::roots::PreparedSlot::Overflow(record.clone())),
            };
            let phases = self.phase_clock.indexed_root();
            if !self
                .objects
                .lookup(address)
                .is_some_and(|m| MetadataRef::ptr_eq(&m, &metadata))
            {
                return Err(invalid());
            }
            match self.certify_root_attempt(phases, &metadata, &record) {
                Some(false) => return Err(invalid()),
                None => continue,
                Some(true) => {}
            }
            #[cfg(all(test, not(target_family = "wasm"), not(miri)))]
            publication_tests::pause_after_typed_proposals();
            if record.commit_typed() {
                let registration = attempt
                    .slot
                    .take()
                    .expect("typed attempt registered")
                    .finish(attempt.context.take());
                if self.diagnostics_enabled() {
                    self.diagnostics
                        .root_registrations
                        .fetch_add(1, Ordering::Relaxed);
                }
                #[cfg(all(test, not(target_family = "wasm"), not(miri)))]
                publication_tests::pause_after_typed_activation();
                return Ok((ptr, metadata.id, registration));
            }
        }
    }

    fn certify_root_attempt(
        &self,
        phases: publication_state::Birth,
        metadata: &MetadataRef,
        record: &Arc<crate::roots::Record>,
    ) -> Option<bool> {
        let snapshot = match phases {
            publication_state::Birth::Idle => return Some(true),
            publication_state::Birth::Inline(s) | publication_state::Birth::Epochs(s) => s,
        };
        let epochs = [0, 1].map(|i| {
            snapshot
                .stage(i)
                .and_then(|_| self.domain_at(i).mark_queue.birth_epoch())
        });
        // The same phase flags can recur in a later cycle. Bind them to the
        // captured queue owners before sampling marks; the final validation
        // then covers both the mark reads and any root proposals.
        if !self.phase_clock.same_phases(&snapshot) {
            return None;
        }
        let mut eligible = true;
        let mut linked = false;
        let mature = self.configuration.promotion_age.max(1);
        for (index, epoch) in epochs.iter().enumerate() {
            let Some(stage) = snapshot.stage(index) else {
                continue;
            };
            let epoch = epoch.as_ref()?;
            let young = metadata.age.load(Ordering::Acquire) < mature;
            if index == 0 && !young {
                continue;
            }
            let exempt = epoch.old_only && young;
            let marked = self.visible_mark(metadata, index, epoch.number);
            if stage != 0 && !exempt && !marked {
                #[cfg(test)]
                publication_tests::inspect_rejected_root(self);
                eligible = false;
                break;
            }
            let strength = Retention::at_phase(stage);
            let retention = self.visible_retention(metadata, index, epoch.number);
            if stage < 5
                && retention.is_none_or(|previous| previous > strength)
                && !(exempt && marked && retention.is_none())
            {
                self.domain_at(index).mark_queue.propose_root(
                    epoch,
                    crate::work::RootProposal {
                        metadata: metadata.clone(),
                        record: record.clone(),
                        index,
                        strength,
                        promotion_age: mature,
                    },
                );
                linked = true;
            }
        }
        // A consumer that passed a pending ticket cancelled this exact record.
        // The stamp invalidates an earlier empty observation even when all
        // participating domains otherwise retained their current epoch identity.
        if self.phase_clock.certify_epochs(snapshot, linked)
            && epochs.iter().enumerate().all(|(i, e)| {
                e.as_ref()
                    .is_none_or(|epoch| self.domain_at(i).mark_queue.current_birth_epoch(epoch))
            })
        {
            Some(eligible)
        } else {
            None
        }
    }

    fn typed_root_covered(
        &self,
        observation: publication_state::Birth,
        metadata: &ObjectMetadata,
    ) -> bool {
        let (snapshot, bind_epochs) = match observation {
            publication_state::Birth::Idle => return true,
            publication_state::Birth::Inline(snapshot) => (snapshot, false),
            publication_state::Birth::Epochs(snapshot) => (snapshot, true),
        };
        // An owning inbox identity survives numeric epoch/revision reuse. Take
        // it before reading marks, then check it again after phase validation.
        let epochs = [0, 1].map(|i| {
            if bind_epochs && snapshot.stage(i).is_some() {
                self.domain_at(i).mark_queue.birth_epoch()
            } else {
                None
            }
        });
        let covered = epochs.iter().enumerate().all(|(i, epoch)| {
            snapshot.stage(i).is_none()
                || ((!bind_epochs || epoch.is_some())
                    && self.visible_retention(
                        metadata,
                        i,
                        self.domain_at(i).epoch.load(Ordering::Acquire),
                    ) == Some(Retention::Strong))
        });
        covered
            && if bind_epochs {
                self.phase_clock.certify_epochs(snapshot, false)
                    && epochs.iter().enumerate().all(|(i, epoch)| {
                        epoch
                            .as_ref()
                            .is_none_or(|e| self.domain_at(i).mark_queue.current_birth_epoch(e))
                    })
            } else {
                self.phase_clock.unchanged(snapshot)
            }
    }

    pub(crate) fn resolve_address<T: 'static>(
        &self,
        address: usize,
        retaining: bool,
        expected: Option<crate::ObjectId>,
    ) -> Option<MetadataRef> {
        self.objects.lookup(address).filter(|metadata| {
            metadata.is_type::<T>()
                && expected.is_none_or(|id| metadata.id == id)
                && self.can_root(metadata, retaining)
        })
    }

    fn can_root(&self, metadata: &ObjectMetadata, retaining: bool) -> bool {
        self.domains.iter().enumerate().all(|(index, domain)| {
            let phase = domain.weak_phase();
            phase == 0
                || self.old_excludes_young(index, metadata)
                || (index == 0
                    && metadata.age.load(Ordering::Acquire)
                        >= self.configuration.promotion_age.max(1))
                || self.visible_mark(metadata, index, domain.epoch.load(Ordering::Acquire))
                || (retaining && phase < 5)
        })
    }

    pub(crate) fn weak(&self) -> Weak<Self> {
        self.this.clone()
    }

    /// Borrows the collector's stable weak reference for an ownership hook.
    ///
    /// # Safety
    /// The address and graph protection must satisfy `TraceOwner::new_unchecked`.
    /// The returned context cannot outlive this collector borrow; retained hooks
    /// still clone their own weak reference.
    pub(crate) unsafe fn trace_owner(&self, address: usize) -> crate::TraceOwner<'_> {
        // SAFETY: the caller supplies the actual protected allocation address.
        unsafe { crate::TraceOwner::new_unchecked(&self.this, address) }
    }

    #[cfg(test)]
    pub(crate) fn metadata_for_address(&self, address: usize) -> Option<MetadataRef> {
        self.objects.lock(address).get(&address).cloned()
    }

    fn weak_target_metadata(
        &self,
        slot: &crate::weak::WeakSlot,
        address: usize,
    ) -> Option<MetadataRef> {
        if address == 0 {
            return None;
        }
        self.objects
            .lookup(address)
            .filter(|metadata| metadata.id == slot.target_identity)
    }

    pub(crate) fn snapshot_edge<T>(&self, target: &Gc<T>) {
        if self.domains.iter().any(Domain::marking) {
            self.edge_barrier(target);
        }
    }

    /// The caller pins the exact managed owner and retains every incoming edge
    /// until this returns. Root deletion covers a young scan that already passed
    /// this owner. Later scans see membership before the last root can disappear.
    pub(crate) fn remember_before_unroot(&self, address: usize) -> bool {
        if self.configuration.barrier_buffer_size != 0 {
            return false;
        }
        #[cfg(all(test, not(target_family = "wasm"), not(miri)))]
        barrier_progress_tests::pause_before_remembering();
        // If the shard is busy, the caller keeps the demoted registrations
        // through the ordinary barrier, which can publish to the overflow index.
        self.remembered
            .try_lock(address)
            .and_then(|remembered| self.enroll_unbuffered_owner(address, &remembered))
            .is_some()
    }

    fn enroll_unbuffered_owner(
        &self,
        address: usize,
        remembered: &crate::index::RememberedGuard<'_>,
    ) -> Option<MetadataRef> {
        let _measurement = self
            .diagnostics
            .barrier_record
            .measure(self.diagnostics_enabled());
        // A pinned owner can outlive registry unlink. Enrollment and removal
        // use the never-recycled birth, including if unlink wins this race.
        let metadata = self.objects.lookup(address)?;
        metadata.record_mutation();
        remembered.insert(address, metadata.id);
        if self.objects.lookup_identity(address, metadata.id).is_none() {
            self.remembered.remove(address, metadata.id);
            return None;
        }
        Some(metadata)
    }

    pub(crate) fn remember_address(&self, address: usize) {
        let _publication = self.allocation_guard();
        if self.configuration.barrier_buffer_size == 0 {
            if let Some(remembered) = self.remembered.try_lock(address) {
                if let Some(metadata) = self.enroll_unbuffered_owner(address, &remembered) {
                    self.rescan_dirty_owner(&metadata);
                }
            } else if let Some(metadata) = self.objects.lookup(address) {
                self.remember_contended_owner(&metadata, None);
            }
            return;
        }
        // The managed writer pins this owner throughout the hook. Its address
        // cannot be reused, and the snapshot owns the exact visible descriptor.
        if let Some(metadata) = self.objects.lookup(address) {
            let version = metadata.record_mutation();
            self.barrier(address, Some((&metadata, version)));
        }
    }

    /// Called under publication admission. A future young cycle sees the
    /// directory; an already-started strong snapshot may have passed the owner.
    fn remember_contended_owner(&self, metadata: &MetadataRef, version: Option<usize>) {
        let _measurement = self
            .diagnostics
            .barrier_record
            .measure(self.diagnostics_enabled());
        let id = metadata.id.to_i64().cast_unsigned();
        let address = metadata.ptr().0;
        // Admission pins phase transitions. A terminal white identity can
        // never publish a new version after sweep has removed its membership.
        if self
            .objects
            .lookup(address)
            .is_none_or(|current| current.id != metadata.id)
            || self.logically_retired(metadata)
        {
            return;
        }
        let version = version.unwrap_or_else(|| metadata.record_mutation());
        self.remembered.dirty.publish(id, address, version);
        self.rescan_dirty_owner(metadata);
    }

    /// Ordinary stores can reach either index on consecutive writes. Publish
    /// their active snapshot debt under the same admission as enrollment.
    fn rescan_dirty_owner(&self, metadata: &MetadataRef) {
        let young = &self.domains[0];
        if young.active() && young.weak_phase() == 0 && !self.logically_retired(metadata) {
            self.enqueue_dirty_owner(metadata);
        }
    }

    fn enqueue_dirty_owner(&self, metadata: &MetadataRef) {
        let _context = Context::enter(self, 0);
        // The owner may already have been scanned with its previous fields.
        // Its outstanding mark alone does not cover this new traversal.
        metadata.mark(
            0,
            self.domains[0].epoch.load(Ordering::Acquire),
            Retention::Strong,
        );
        if metadata.request_dirty_scan() {
            self.domains[0]
                .mark_queue
                .push(self.local_queue(), MarkItem::dirty(metadata.clone()));
        }
    }

    /// Creates a collector with default configuration.
    #[must_use]
    pub fn new() -> Arc<Self> {
        Self::with_config(Configuration::default())
    }

    /// Creates a collector with an explicit allocation and scheduling policy.
    #[must_use]
    #[expect(
        clippy::too_many_lines,
        reason = "initialize related collector-owned state together"
    )]
    pub fn with_config(mut configuration: Configuration) -> Arc<Self> {
        if configuration.old_collection_interval != 0 {
            configuration.independent_phases = true;
            configuration.overlapping_generations = true;
        }
        configuration.barrier_buffer_size = configuration
            .barrier_buffer_size
            .min(crate::context::MAX_RECORDS);
        configuration.metadata_shards = crate::index::shard_count(configuration.metadata_shards);
        configuration.region_shards = crate::index::shard_count(configuration.region_shards);
        configuration.relocation_reserve_bytes =
            if configuration.compact_heap && configuration.cost_aware_relocation {
                configuration
                    .relocation_reserve_bytes
                    .min(configuration.max_heap_bytes / 2)
            } else {
                0
            };
        let phase_clock = Arc::new(publication_state::Clock::new(
            publication_state::MAX_REVISION,
        ));
        let publication_clock = configuration
            .concurrent_publication
            .then(|| phase_clock.clone());
        Arc::new_cyclic(|this| {
            let budget = crate::region::Budget::with_reserve(
                configuration.max_heap_bytes,
                configuration.relocation_reserve_bytes,
            );
            let regions = configuration.compact_heap.then(|| {
                crate::region::RegionHeap::with_shards(
                    configuration.region_size,
                    configuration.max_heap_bytes,
                    configuration.relocation_reserve_bytes,
                    budget.clone(),
                    configuration.cost_aware_relocation,
                    configuration.region_shards,
                )
            });
            let descriptor_regions = configuration.local_allocation_buffers.then(|| {
                crate::region::RegionHeap::with_shards(
                    crate::allocation::CHUNK,
                    configuration.max_heap_bytes,
                    0,
                    budget.clone(),
                    false,
                    configuration.region_shards,
                )
            });
            Self {
                this: this.clone(),
                regions,
                descriptor_regions,
                small_allocations: crate::allocation::SmallAllocations::new(budget),
                #[cfg(not(target_family = "wasm"))]
                pacing: configuration.adaptive_workers.then(|| {
                    crate::pacing::Controller::new(
                        configuration.threads,
                        thread::available_parallelism().map_or(1, std::num::NonZeroUsize::get),
                        configuration.max_heap_bytes,
                        configuration.mark_batch_size,
                        configuration.worker_yield_time_us,
                    )
                }),
                typed_slots: once_cell::race::OnceBox::new(),
                root_slots: (configuration.root_storage != crate::RootStorage::Tree)
                    .then(|| Box::new(crate::roots::RootSlots::default())),
                scheduler: (configuration.predictive_collection
                    && configuration.max_heap_bytes != usize::MAX
                    && configuration.max_heap_bytes != 0)
                    .then(|| crate::scheduling::Scheduler::new(configuration.max_heap_bytes)),
                mark_claims: configuration
                    .independent_marking
                    .then(|| Box::new(RwLock::new(()))),
                #[cfg(test)]
                mark_claim_pause: Mutex::new(None),
                #[cfg(test)]
                metadata_prepare_pause: Mutex::new(None),
                #[cfg(test)]
                metadata_publish_failure: Mutex::new(None),
                #[cfg(test)]
                metadata_registration_observer: Mutex::new(None),
                #[cfg(test)]
                identity_resolution_pause: Mutex::new(None),
                #[cfg(test)]
                pending_transfer_pause: Mutex::new(None),
                #[cfg(test)]
                edge_certificate_pause: Mutex::new(None),
                relocation_claims: configuration
                    .independent_relocation
                    .then(|| Box::new(RwLock::new(()))),
                mature_census: MatureCensus::default(),
                // Batched sweeps traverse consecutive cells. Deadline-limited
                // single-entry visits can use smaller leaves for sparse heaps.
                objects: crate::index::Objects::with_sparse_cells(
                    configuration.metadata_shards,
                    configuration.max_pause_time_us != 0,
                ),
                #[cfg(feature = "experimental-identity-index")]
                identities: crate::identity_index::Index::new(4096),
                #[cfg(not(feature = "experimental-identity-index"))]
                identities: crate::identity_pages::Index::new(configuration.metadata_shards),
                remembered: crate::index::Remembered::new(configuration.metadata_shards),
                domains: std::array::from_fn(|index| {
                    Domain::new(&configuration, index, publication_clock.clone())
                }),
                phase_clock: phase_clock.clone(),
                publication_clock: publication_clock.clone(),
                old_requested: AtomicBool::new(false),
                old_active: AtomicBool::new(false),
                managed_budget: Arc::new(
                    crate::allocation_budget::AllocationBudget::with_collector(
                        configuration.max_heap_bytes,
                        this.clone(),
                    ),
                ),
                configuration,
                statistics: Mutex::new(Statistics::default()),
                leaves_marked_at_birth: AtomicUsize::new(0),
                diagnostics: Arc::new(crate::diagnostics::Counters::default()),
                recovery: crate::recovery::Recovery::default(),
                grant_registry: crate::retirement_grants::Registry::default(),
                roots: crate::roots::TreeRoots::default(),
                #[cfg(test)]
                tree_root_insert_probe: Mutex::new(None),
                contexts: crate::context::Registry::default(),
                next_root_id: AtomicUsize::new(1),
                heap_owners: Mutex::new(BTreeMap::new()),

                publication: crate::sync::RecursiveRwLock::new(()),
                coordinator: Mutex::new(()),
                retired: arc_swap::ArcSwapOption::default(),
                retired_objects: AtomicUsize::new(0),
                retired_bytes: AtomicUsize::new(0),
                retired_ready: AtomicUsize::new(0),
                #[cfg(not(target_family = "wasm"))]
                callback_jobs: AtomicUsize::new(0),
                retirement_turn: AtomicBool::new(false),
                released_donors: crate::retirement_grants::ReleasedDonors::default(),
                released_turn: AtomicBool::new(false),
                barrier_turn: AtomicBool::new(false),
                #[cfg(test)]
                barrier_claim_observer: Mutex::new(None),
                #[cfg(test)]
                barrier_prepare_pause: Mutex::new(None),
                next_domain: AtomicUsize::new(0),
                weak_slots: crate::weak_registry::WeakRegistry::default(),
                next_weak_id: AtomicUsize::new(1),
                pending_references: once_cell::race::OnceBox::new(),
                allocation_debt: AtomicUsize::new(0),
                published_bytes: AtomicUsize::new(0),
                published_debt: AtomicUsize::new(0),
                #[cfg(not(target_family = "wasm"))]
                worker_signals: arc_swap::ArcSwapOption::empty(),
                #[cfg(not(target_family = "wasm"))]
                collector_thread: Mutex::new(Vec::new()),
                #[cfg(not(target_family = "wasm"))]
                refund_notifier: arc_swap::ArcSwapOption::empty(),
            }
        })
    }

    pub(crate) fn prepare_allocated_object<T: Send + Sync + Trace + 'static>(
        &self,
        allocation: crate::allocation::Initialized<crate::gc::Allocation<T>>,
        reservation: AllocationReservation<'_>,
        id: crate::ObjectId,
    ) -> MetadataRef {
        let metadata = ObjectMetadata::new_for_gc::<T>(allocation, reservation.size, id);
        self.prepare_typed::<T>(metadata, reservation)
    }

    pub(crate) fn allocate_descriptor(
        &self,
        layout: std::alloc::Layout,
    ) -> Result<crate::allocation::Storage> {
        if let Some(heap) = &self.descriptor_regions
            && layout.size() <= crate::allocation::SMALL
            && layout.align() <= 64
            && let Some(Ok(block)) = crate::context::try_with_allocation(self, |cache| {
                cache.allocate(heap, layout, true)
            })
        {
            return Ok(crate::allocation::Storage::Region(block));
        }
        // Pooling is optional: an exhausted/fragmented arena must not make
        // an otherwise valid descriptor allocation fail prematurely.
        self.default_descriptor(layout)
    }

    pub(crate) fn default_descriptor(
        &self,
        layout: std::alloc::Layout,
    ) -> Result<crate::allocation::Storage> {
        if self.regions.is_none()
            && self.descriptor_regions.is_none()
            && let Some(storage) = self.small_allocations.allocate(layout)
        {
            return Ok(storage);
        }
        crate::allocation::Storage::global(layout)
    }

    pub(crate) fn poll_payload(
        &self,
        layout: std::alloc::Layout,
        probe: &mut crate::allocation::PayloadProbe,
    ) -> std::task::Poll<Result<Option<crate::region::Block>>> {
        let Some(heap) = self.regions.as_ref().filter(|_| layout.size() != 0) else {
            return std::task::Poll::Ready(Ok(None));
        };
        if self.configuration.local_allocation_buffers
            && let Some(result) = crate::context::try_with_allocation(self, |cache| {
                cache.poll_payload(heap, layout, probe)
            })
        {
            return result.map_ok(Some);
        }
        probe.exact.poll(heap, layout).map_ok(Some)
    }

    pub(crate) fn allocate_payload(
        &self,
        layout: std::alloc::Layout,
    ) -> Result<Option<crate::region::Block>> {
        let Some(heap) = &self.regions else {
            return Ok(None);
        };
        if self.configuration.local_allocation_buffers
            && let Some(result) = crate::context::try_with_allocation(self, |cache| {
                cache.allocate(heap, layout, false)
            })
        {
            return result.map(Some);
        }
        heap.allocate(layout, None).map(Some)
    }

    pub(crate) fn prepare_allocated_object_with_finalizer<
        T: Send + Sync + Finalize + Trace + 'static,
    >(
        &self,
        allocation: crate::allocation::Initialized<crate::gc::Allocation<T>>,
        reservation: AllocationReservation<'_>,
        id: crate::ObjectId,
    ) -> MetadataRef {
        let metadata =
            ObjectMetadata::new_for_gc_with_finalizer::<T>(allocation, reservation.size, id);
        self.prepare_typed::<T>(metadata, reservation)
    }

    fn prepare_typed<T: Trace>(
        &self,
        mut metadata: ObjectMetadata,
        reservation: AllocationReservation<'_>,
    ) -> MetadataRef {
        assert!(
            std::ptr::eq(self, reservation.collector),
            "foreign allocation reservation"
        );
        reservation.transfer(&mut metadata);
        if self.configuration.cost_aware_relocation
            && let Some(heap) = &self.regions
        {
            // SAFETY: the typed constructor owns this initialized, unpublished T.
            // No region identity lookup escapes a payload borrow or dereferences an ID.
            let value = unsafe { &*metadata.trace.ptr.cast::<T>() };
            metadata.relocation =
                crate::relocation_history::History::new(value, heap).map(Box::new);
        }
        MetadataRef::allocate(
            metadata,
            (self.regions.is_none() && self.descriptor_regions.is_none())
                .then_some(&self.small_allocations),
        )
    }

    /// The constructor keeps an owner outside this call so private storage cannot
    /// be destroyed by callee unwind before its initial root is removed. Call once
    /// for freshly prepared metadata with construction protection still active.
    /// Borrowing the owner does not make replaying the admitted effects safe.
    pub(crate) fn register_prepared(
        &self,
        metadata: &MetadataRef,
        construction: &crate::construction::Detached<'_>,
    ) {
        assert_eq!(
            self.register_prepared_checked(metadata, construction),
            Ok(()),
            "registration requires a vacant exact write set"
        );
    }

    fn register_prepared_checked(
        &self,
        metadata: &MetadataRef,
        construction: &crate::construction::Detached<'_>,
    ) -> std::result::Result<(), crate::registration_key::Conflict> {
        if !construction.belongs_to(self) || !std::ptr::eq(metadata.collector.as_ptr(), self) {
            return Err(crate::registration_key::Conflict::ForeignCollector);
        }
        let mut rollback = self.index_birth(metadata)?;
        #[cfg(test)]
        publication_tests::inspect_pending_birth(self, metadata);
        #[cfg(all(test, not(target_family = "wasm"), not(miri)))]
        publication_tests::pause_pending_birth();
        loop {
            let published = match self
                .phase_clock
                .indexed_birth(metadata.trace.step_fn.is_none())
            {
                publication_state::Birth::Idle => {
                    #[cfg(all(test, not(target_family = "wasm"), not(miri)))]
                    publication_tests::pause_at_idle_birth();
                    metadata.birth.publish_if_unobserved()
                }
                publication_state::Birth::Inline(snapshot) => {
                    self.publish_observed_birth(metadata, snapshot, false)
                }
                publication_state::Birth::Epochs(snapshot) => {
                    self.publish_observed_birth(metadata, snapshot, true)
                }
            };
            if published {
                break;
            }
            metadata.birth.retry();
        }
        rollback.slot.reserved = false;
        #[cfg(all(test, not(target_family = "wasm"), not(miri)))]
        publication_tests::pause_after_birth_decision();
        #[cfg(all(test, not(target_family = "wasm"), not(miri)))]
        publication_tests::pause_after_registration_indexes();
        #[cfg(test)]
        self.fail_metadata_publication();
        // Construction roots and each slot's pending-owner count survive activation.
        // Binding can wait for a slot without holding phase closure;
        // the owner is already indexed so concurrent set_owner sees its identity.
        construction.publish_reference_owner(metadata);
        Ok(())
    }

    /// Marks remain private until activation. Strong graph births link their
    /// exact traversal in every affected domain before certifying the clock.
    fn publish_observed_birth(
        &self,
        metadata: &MetadataRef,
        snapshot: publication_state::Snapshot,
        bind_epochs: bool,
    ) -> bool {
        let stages = [0, 1].map(|index| snapshot.stage(index));
        let epochs = [0, 1].map(|index| {
            if bind_epochs && snapshot.stage(index).is_some() {
                self.domain_at(index).mark_queue.birth_epoch()
            } else {
                None
            }
        });
        if bind_epochs
            && stages
                .iter()
                .zip(&epochs)
                .any(|(stage, epoch)| stage.is_some() && epoch.is_none())
        {
            return false;
        }
        let trace = [0, 1].map(|index| {
            snapshot
                .stage(index)
                .is_some_and(|stage| self.mark_registration_birth(metadata, index, stage))
        });
        let graph = (trace != [false; 2] && metadata.trace.step_fn.is_some())
            .then(|| crate::work::GraphBirth::new(metadata));
        if let Some(graph) = &graph {
            #[cfg(all(test, not(target_family = "wasm"), not(miri)))]
            publication_tests::pause_before_graph_link();
            for (index, (epoch, required)) in epochs.iter().zip(trace).enumerate() {
                if required && let Some(epoch) = epoch {
                    self.domain_at(index).mark_queue.propose_graph(epoch, graph);
                }
            }
        }
        #[cfg(test)]
        if graph.is_some() {
            publication_tests::inspect_linked_graph(self);
        }
        let valid = if bind_epochs {
            self.phase_clock.certify_epochs(snapshot, graph.is_some())
                && epochs.iter().enumerate().all(|(index, epoch)| {
                    epoch
                        .as_ref()
                        .is_none_or(|e| self.domain_at(index).mark_queue.current_birth_epoch(e))
                })
        } else {
            self.phase_clock.unchanged(snapshot)
        };
        #[cfg(test)]
        publication_tests::inspect_before_activation(self, metadata);
        #[cfg(all(test, not(target_family = "wasm"), not(miri)))]
        publication_tests::pause_before_observed_activation();
        if valid && metadata.birth.publish_if_unobserved() {
            if graph.is_none() && trace != [false; 2] {
                self.leaves_marked_at_birth.fetch_add(
                    usize::from(trace[0]) + usize::from(trace[1]),
                    Ordering::Relaxed,
                );
            }
            return true;
        }
        if let Some(graph) = graph {
            graph.cancel();
        }
        // A revision change or a scanner that passed the private record
        // invalidates this attempt. Erase *both* epochs and the cohort tag:
        // the next observation may see idle domains, even after a wrap.
        for index in 0..2 {
            metadata.reset_mark(index);
        }
        metadata.old_birth.store(0, Ordering::Release);
        false
    }

    fn index_birth<'a>(
        &'a self,
        metadata: &'a MetadataRef,
    ) -> std::result::Result<PendingBirth<'a>, crate::registration_key::Conflict> {
        let address = metadata.ptr().0;
        #[cfg(feature = "experimental-identity-index")]
        let identity = self
            .identities
            .prepare(metadata.id.to_i64().cast_unsigned(), address);
        #[cfg(not(feature = "experimental-identity-index"))]
        let identity = self
            .identities
            .prepare(metadata.id.to_i64().cast_unsigned());
        #[cfg(test)]
        self.pause_metadata_preparation();
        // Reserve the exact address before installing any shared effect. A
        // pending reservation holds no live-object or phase guard. Even another
        // constructor in this page can proceed while this one is parked.
        let slot = self
            .objects
            .reserve(address)
            .ok_or(crate::registration_key::Conflict::Address)?;
        #[cfg(test)]
        if let Some(observer) = lock(&self.metadata_registration_observer).take() {
            let _ = observer.send((address, metadata.id));
        }
        #[cfg(all(test, not(target_family = "wasm"), not(miri)))]
        publication_tests::pause_inside_registration_commit();
        let key = crate::registration_key::Key {
            address,
            id: metadata.id,
        };
        #[cfg(feature = "experimental-identity-index")]
        let previous_identity = self.identities.get(metadata.id.to_i64().cast_unsigned());
        #[cfg(not(feature = "experimental-identity-index"))]
        let previous_identity = identity.as_ref().err().copied().flatten();
        key.vacant(crate::registration_key::Observed {
            object: None,
            identity: previous_identity,
            young: false,
        })?;
        if metadata.birth.indexed() {
            return Err(crate::registration_key::Conflict::AlreadyPresent);
        }
        // The young scan may encounter this exact slot before its descriptor.
        // The following phase-clock RMW covers scans that passed it early.
        slot.mark_young();
        #[cfg(all(test, not(target_family = "wasm"), not(miri)))]
        publication_tests::pause_after_young_slot();
        #[cfg(not(feature = "experimental-identity-index"))]
        identity
            .map_err(|_| crate::registration_key::Conflict::Identity)?
            .install(address);
        #[cfg(feature = "experimental-identity-index")]
        self.identities.insert(identity);
        let pending = PendingBirth {
            slot,
            gc: self,
            metadata,
        };
        metadata.record_publication(self);
        pending.slot.install(metadata.clone());
        Ok(pending)
    }

    #[cfg(test)]
    fn pause_metadata_preparation(&self) {
        let pause = lock(&self.metadata_prepare_pause).take();
        if let Some(pause) = pause {
            pause.entered.send(()).expect("metadata prepared");
            pause
                .resume
                .recv_timeout(std::time::Duration::from_secs(if cfg!(miri) {
                    120
                } else {
                    10
                }))
                .expect("resume metadata registration");
        }
    }

    #[cfg(test)]
    fn fail_metadata_publication(&self) {
        let observer = lock(&self.metadata_publish_failure).take();
        if let Some(observer) = observer {
            observer.store(self.managed_budget.used(), Ordering::Release);
            panic!("injected unwind after descriptor publication");
        }
    }

    fn register_weak(&self, slot: &Arc<crate::weak::WeakSlot>) {
        let address = slot.target.load(Ordering::Acquire);
        let metadata = self.weak_target_metadata(slot, address);
        let young = metadata.as_ref().is_some_and(|m| self.is_young_metadata(m));
        let id = self.weak_slots.insert(&self.next_weak_id, slot, young);
        // A retaining clone inherits its source's birth exemption and drops its
        // owner. That can create an eligible reference behind an active cursor.
        if matches!(
            slot.kind,
            crate::ReferenceKind::Final | crate::ReferenceKind::PhantomRetained
        ) {
            for (i, domain) in self.domains.iter().enumerate() {
                if (i != 0 || young)
                    && slot.birth_stamp(i).load(Ordering::Relaxed)
                        != domain.epoch.load(Ordering::Acquire)
                {
                    domain
                        .late_references
                        .offer_if_eligible(domain.weak_phase(), id, slot);
                }
            }
        }
        if self.old_collection_active()
            && let Some(metadata) = metadata
        {
            let _context = Context::enter(self, 1);
            self.shade_known(&metadata);
        }
    }

    fn reference_owner(&self, slot: &crate::weak::WeakSlot) -> Option<MetadataRef> {
        let identity = slot.owner_identity()?;
        let address = slot.owner.load(Ordering::Acquire);
        self.objects
            .lookup(address)
            .filter(|metadata| metadata.id == identity)
    }

    fn logically_retired(&self, metadata: &ObjectMetadata) -> bool {
        self.domains.iter().enumerate().any(|(index, domain)| {
            if domain.weak_phase() != 5 || self.old_excludes_young(index, metadata) {
                return false;
            }
            let exempt = index == 0
                && metadata.age.load(Ordering::Acquire) >= self.configuration.promotion_age.max(1);
            !exempt && !metadata.marked(index, domain.epoch.load(Ordering::Acquire))
        })
    }

    fn reference_owner_live(&self, slot: &crate::weak::WeakSlot) -> bool {
        self.reference_owner(slot).is_some_and(|metadata| {
            if self.configuration.overlapping_generations && self.logically_retired(&metadata) {
                return false;
            }
            self.visible_mark(
                &metadata,
                self.domain_index(),
                self.domain().epoch.load(Ordering::Acquire),
            ) || ((self.domain_index() == 0)
                && metadata.age.load(Ordering::Acquire) >= self.configuration.promotion_age.max(1))
        })
    }

    fn reference_metadata_live(
        &self,
        metadata: &ObjectMetadata,
        kind: crate::ReferenceKind,
    ) -> bool {
        self.reference_metadata_live_at(
            metadata,
            kind,
            self.domain_index(),
            self.domain().epoch.load(Ordering::Acquire),
        )
    }

    fn reference_metadata_live_at(
        &self,
        metadata: &ObjectMetadata,
        kind: crate::ReferenceKind,
        index: usize,
        epoch: u64,
    ) -> bool {
        let final_retention = matches!(
            kind,
            crate::ReferenceKind::Phantom | crate::ReferenceKind::PhantomRetained
        );
        self.old_excludes_young(index, metadata)
            || metadata.marked_before(index, epoch, final_retention)
            || metadata
                .direct_marks
                .get(index, epoch, &self.domain_at(index).mark_queue)
                .is_some_and(|strength| {
                    strength
                        <= if final_retention {
                            Retention::Final
                        } else {
                            Retention::Strong
                        }
                })
            || (index == 0
                && metadata.age.load(Ordering::Acquire) >= self.configuration.promotion_age.max(1))
    }

    pub(crate) fn weak_is_cleared(&self, slot: &crate::weak::WeakSlot, address: usize) -> bool {
        let metadata = self.weak_target_metadata(slot, address);
        let Some(metadata) = metadata else {
            return true;
        };
        (0..2).any(|index| {
            let _context = Context::enter(self, index);
            if slot.birth_stamp(index).load(Ordering::Acquire)
                == self.domain().epoch.load(Ordering::Acquire)
            {
                return false;
            }
            let phase = self.domain().weak_phase();
            let decision = match slot.kind {
                crate::ReferenceKind::Soft | crate::ReferenceKind::Weak => phase >= 1,
                crate::ReferenceKind::Phantom => phase >= 3,
                crate::ReferenceKind::PhantomRetained | crate::ReferenceKind::Final => false,
            };
            decision && !self.reference_metadata_live(&metadata, slot.kind)
        })
    }

    pub(crate) fn weak_birth_epochs(&self) -> [u64; 2] {
        std::array::from_fn(|index| {
            let domain = self.domain_at(index);
            if domain.weak_phase() != 0 {
                domain.epoch.load(Ordering::Acquire)
            } else {
                0
            }
        })
    }

    /// Returns whether Java reference processing has published work for the Reference Handler.
    #[must_use]
    pub fn has_pending_references(&self) -> bool {
        self.pending_references.get().is_some_and(|queue| {
            // A private append increments the reservation count before linking.
            queue.pending() != 0 && !queue.is_empty()
        })
    }

    /// Waits without polling for reference processing to publish a pending reference.
    pub async fn wait_for_pending_references(&self) {
        self.recovery.wait_for_references(self).await;
    }

    /// Transfers a pending reference's root to the caller.
    ///
    /// # Errors
    /// Returns an error if the caller requests the wrong allocation type.
    pub fn take_pending_reference<T: Trace + 'static>(self: &Arc<Self>) -> Result<Option<Gc<T>>> {
        let Some(queue) = self.pending_references.get() else {
            return Ok(None);
        };
        #[cfg(all(test, not(target_family = "wasm"), not(miri)))]
        let next = queue.test_pop_after_head(reference_tests::pause_after_pending_head);
        #[cfg(not(all(test, not(target_family = "wasm"), not(miri))))]
        let next = queue.pop();
        let Some((identity, id)) = next else {
            return Ok(None);
        };
        // The queue's root remains registered after dequeue. Waiting on the queue
        // or allocating the caller's owning record needs no outer admission;
        // Identity validation rejects a replacement at a revoked owner's former
        // address. Root generations also prevent stale queue cleanup from
        // removing another registration after the public numeric ID wraps.
        let queued_root = PendingReferenceRoot {
            collector: self,
            id,
        };
        #[cfg(test)]
        {
            let pause = lock(&self.pending_transfer_pause).take();
            if let Some(pause) = pause {
                pause
                    .entered
                    .send(())
                    .map_err(|error| crate::Error::SyncError(error.to_string()))?;
                pause
                    .resume
                    .recv_timeout(std::time::Duration::from_secs(if cfg!(miri) {
                        120
                    } else {
                        10
                    }))
                    .map_err(|error| crate::Error::SyncError(error.to_string()))?;
            }
        }
        let result = Gc::from_handle_rooted(self, identity.to_i64());
        drop(queued_root);
        result.map(Some)
    }

    fn enqueue_reference(&self, slot: &crate::weak::WeakSlot) {
        if let Some(metadata) = self.reference_owner(slot) {
            let id = self.insert_root(metadata.ptr().0);
            let queue = self
                .pending_references
                .get_or_init(|| Box::new(crate::publication_queue::Queue::new()));
            #[cfg(all(test, not(target_family = "wasm")))]
            queue.test_push_after_link(
                (metadata.id, id),
                reference_tests::pause_after_pending_notice,
            );
            #[cfg(not(all(test, not(target_family = "wasm"))))]
            queue.push((metadata.id, id));
        }
    }

    /// Returns the number of registered native workers, including sleeping workers.
    /// This snapshot is available without diagnostic instrumentation. It is not a
    /// count of threads currently using CPU; concurrent start/stop can change it.
    /// Cooperative hosts report zero. Native configuration `threads = 0` selects
    /// one worker when `start` is called; `stop` joins workers before returning.
    #[must_use]
    pub fn registered_workers(&self) -> usize {
        self.recovery.registered_workers()
    }

    /// Only a logical allocation that would sleep behind native collection needs
    /// independent wake delivery. Reuse it until this set of workers stops.
    #[cfg(not(target_family = "wasm"))]
    pub(crate) fn ensure_refund_notifier(&self, settlement_only: bool) -> Result<()> {
        if self.refund_notifier.load().is_some() {
            return Ok(());
        }
        let _workers = lock(&self.collector_thread);
        // stop() may already have taken the handles while a worker is still
        // parked. Keep independent delivery available until the workers exit.
        if (!settlement_only && self.recovery.registered_workers() == 0)
            || self.refund_notifier.load().is_some()
        {
            return Ok(());
        }
        let notifier = crate::recovery::RefundNotifier::spawn(self)?;
        self.refund_notifier.store(Some(Arc::new(notifier)));
        Ok(())
    }

    #[cfg(not(target_family = "wasm"))]
    fn close_refund_notifier(&self) {
        if let Some(notifier) = self.refund_notifier.swap(None) {
            notifier.close();
        }
    }

    /// Starts the native worker. Cooperative hosts need no worker.
    #[cfg(not(target_family = "wasm"))]
    pub fn start(&self) {
        let mut worker = lock(&self.collector_thread);
        if !worker.is_empty() {
            return;
        }
        let count = self.configuration.threads.max(1);
        let signals = Arc::new(WorkerSignals::new(count));
        self.worker_signals.store(Some(signals.clone()));
        for index in 0..count {
            let weak = self.this.clone();
            let generation = signals.clone();
            let registration = crate::recovery::WorkerRegistration::new(self);
            worker.push(thread::spawn(move || {
                let _registration = registration;
                native_callbacks::register(&generation);
                let mut contention_delay = 1_u64;
                #[expect(
                    clippy::indexing_slicing,
                    reason = "each worker gets its ordinal in this fixed pool"
                )]
                let signal = &generation.workers[index];
                signal.register();
                let mut retry = true;
                loop {
                    if !retry {
                        signal.wait();
                    }
                    if generation.closed.load(Ordering::Acquire) {
                        break;
                    }
                    let Some(gc) = weak.upgrade() else { break };
                    let _worker = WorkerContext::enter(&gc, index);
                    while gc.worker_pending() {
                        if generation.closed.load(Ordering::Acquire) {
                            break;
                        }
                        if !gc.worker_admitted(index) {
                            thread::sleep(std::time::Duration::from_millis(1));
                            continue;
                        }
                        let claimed = gc.step_inner(true).is_some();
                        if !claimed && gc.configuration.worker_yield_time_us == 0 {
                            // A competing helper owns the available claim. Spread
                            // retries without sleeping or delaying a successful step.
                            for _ in 0..contention_delay {
                                std::hint::spin_loop();
                            }
                            contention_delay = (contention_delay * 2).min(16384);
                        }
                        let pacing = gc.pacing.as_ref().map_or(
                            gc.configuration.worker_yield_time_us,
                            crate::pacing::Controller::delay,
                        );
                        if !claimed && gc.configuration.worker_yield_time_us != 0 {
                            // Failed claims cannot help an allocator. Back off even
                            // under pressure, then decay the delay after useful claims.
                            gc.back_off_worker(&mut contention_delay);
                            continue;
                        }
                        if claimed {
                            // A single successful unit does not imply that contention
                            // with the helper holding the next packet has subsided.
                            // Only failed attempts wait. Continuous useful work runs
                            // immediately and gradually restores prompt retries.
                            contention_delay = if gc.configuration.worker_yield_time_us == 0 {
                                contention_delay.saturating_sub(1).max(1)
                            } else {
                                (contention_delay / 2).max(1)
                            };
                        }
                        let pressure = gc.managed_budget.used()
                            >= gc
                                .configuration
                                .max_heap_bytes
                                .saturating_sub(gc.configuration.max_heap_bytes / 4);
                        // Do not pace a full recovery cycle after sweep has freed
                        // some bytes but an allocation still awaits its completion.
                        if pacing == 0 || pressure || gc.recovery.waiter_count() != 0 {
                            thread::yield_now();
                        } else {
                            thread::sleep(std::time::Duration::from_micros(pacing.min(1000)));
                        }
                    }
                    // A request can look idle while another worker transfers it to
                    // active state. Never block the idle handoff on that worker.
                    retry = if let Ok(_coordinator) = gc.coordinator.try_lock() {
                        gc.worker_pending()
                    } else {
                        gc.back_off_worker(&mut contention_delay);
                        true
                    };
                }
            }));
        }
        // A request may predate worker registration. Poll the authoritative
        // demand once before sleeping, including when start races a request.
        signals.wake();
    }

    #[cfg(not(target_family = "wasm"))]
    fn worker_admitted(&self, index: usize) -> bool {
        let Some(pacing) = &self.pacing else {
            return true;
        };
        pacing.sample(index, || {
            Some(crate::pacing::Demand {
                queued: self.domains.iter().try_fold(0, |count, domain| {
                    domain.mark_queue.queued_sample().map(|queued| {
                        count
                            + queued
                            + domain
                                .relocations
                                .as_ref()
                                .map_or(0, crate::relocation::Queues::len)
                    })
                })?,
                debt: self.allocation_debt.load(Ordering::Relaxed),
                live: self.managed_budget.used(),
                waiters: self.recovery.waiter_count(),
            })
        });
        let admitted = pacing.admits(index);
        if !admitted && self.diagnostics_enabled() {
            self.diagnostics
                .worker_throttles
                .fetch_add(1, Ordering::Relaxed);
        }
        admitted
    }

    #[cfg(not(target_family = "wasm"))]
    fn back_off_worker(&self, contention_delay: &mut u64) {
        let pacing = self.configuration.worker_yield_time_us.min(1000);
        if pacing == 0 {
            thread::yield_now();
            return;
        }
        let delay = (*contention_delay).min(pacing);
        if self.diagnostics_enabled() {
            self.diagnostics
                .worker_backoffs
                .fetch_add(1, Ordering::Relaxed);
        }
        thread::sleep(std::time::Duration::from_micros(delay));
        *contention_delay = delay.saturating_mul(2).min(pacing);
    }

    /// Cooperative hosts execute collection through `step` or `collect_full`.
    #[cfg(target_family = "wasm")]
    pub fn start(&self) {}

    /// Stops the worker and releases unrooted shutdown-owned allocations.
    ///
    /// # Errors
    /// Returns an error if a native worker panics.
    pub fn stop(&self) -> Result<()> {
        if CleanupScope::contains(self) {
            return Err(crate::Error::CollectionPhaseError(
                "cleanup cannot wait for its own collector to stop".into(),
            ));
        }
        let _shutdown = CleanupScope::enter(self);
        #[cfg(not(target_family = "wasm"))]
        let cleanup_failed = {
            let (workers, generation) = {
                let mut workers = lock(&self.collector_thread);
                let generation = self.worker_signals.swap(None);
                if let Some(previous) = &generation {
                    previous.close();
                }
                (std::mem::take(&mut *workers), generation)
            };
            for worker in workers {
                if worker.thread().id() == thread::current().id() {
                    continue;
                }
                worker
                    .join()
                    .map_err(|_| crate::Error::SyncError("collector worker panicked".into()))?;
            }
            let cleanup_failed = if let Some(generation) = generation {
                generation.callbacks.join()?
            } else {
                false
            };
            let notifier = {
                let _workers = lock(&self.collector_thread);
                (self.recovery.registered_workers() == 0)
                    .then(|| self.refund_notifier.swap(None))
                    .flatten()
            };
            if let Some(notifier) = notifier {
                notifier.close();
                notifier.join();
            }
            cleanup_failed
        };
        self.collect_full()?;
        while self.retired_ready.load(Ordering::Acquire) != 0 || self.released_donors.has_jobs() {
            self.step();
        }
        for domain in &self.domains {
            domain.mark_queue.trim_empty();
            if let Some(queues) = &domain.relocations {
                queues.trim_empty();
            }
        }
        if let Some(roots) = self.scanned_slots() {
            roots.trim_empty();
        }
        self.contexts.for_each(|context| {
            context.roots.trim_empty();
            context.release_allocations(self);
        });
        if let Some(heap) = &self.descriptor_regions {
            while !heap.trim(64) {}
        }
        #[cfg(not(target_family = "wasm"))]
        if cleanup_failed {
            return Err(crate::Error::SyncError(
                "collector callback panicked".into(),
            ));
        }
        Ok(())
    }

    /// Registers VM metadata whose roots exist independently of allocated instances.
    pub fn register_heap_owner<T: HeapOwner + 'static>(&self, owner: &Arc<T>) {
        let weak = Arc::downgrade(owner);
        let weak: Weak<dyn HeapOwner> = weak;
        lock(&self.heap_owners).insert(Arc::as_ptr(owner).cast::<()>() as usize, weak);
    }

    /// Releases class mirrors and static roots when the VM that owns this heap shuts down.
    /// Call only after requesting cancellation of its Java tasks.
    pub fn release_vm_roots(&self) {
        let mut owners = BTreeMap::new();
        for (address, owner) in lock(&self.heap_owners).iter() {
            if let Some(owner) = owner.upgrade() {
                owners.insert(*address, owner);
            }
        }
        self.objects.for_each(|metadata| {
            if metadata.birth.is_live()
                && let Some(owner) = metadata.owner.as_ref().and_then(Weak::upgrade)
            {
                owners.insert(Arc::as_ptr(&owner).cast::<()>() as usize, owner);
            }
        });
        for owner in owners.into_values() {
            owner.release_heap_roots();
        }
    }

    fn cleanup_remaining_objects(&self) {
        self.contexts
            .for_each(|context| context.release_allocations(self));
        #[cfg(feature = "experimental-identity-index")]
        self.identities.clear();
        for domain in &self.domains {
            // An if-let scrutinee would retain its guard through the callback
            // body. Transfer the task first, then run all teardown outside it.
            let tasks = std::mem::take(&mut lock(&domain.cycle).reclaim);
            for task in tasks {
                task.metadata.run_finalizer();
                task.metadata.run_drop_fn();
            }
            domain.mark_queue.clear();
            if let Some(queues) = &domain.relocations {
                queues.clear();
            }
        }
        let objects = self.objects.drain();
        #[cfg(not(feature = "experimental-identity-index"))]
        for metadata in &objects {
            self.identities
                .remove(metadata.id.to_i64().cast_unsigned(), metadata.ptr().0);
        }
        for metadata in &objects {
            metadata.retire();
            if metadata.storage_ready() {
                metadata.run_finalizer();
            }
        }
        for metadata in &objects {
            metadata.run_drop_fn();
        }
        self.released_donors.cleanup();
        if let Some(queue) = self.retired.load_full() {
            while let Some(task) = queue.pop() {
                task.metadata.run_finalizer();
                task.metadata.run_drop_fn();
            }
        }
        // An outstanding lease owns its descriptor and storage after heap teardown.
        self.retired_objects.store(0, Ordering::Release);
        self.retired_bytes.store(0, Ordering::Release);
        self.retired_ready.store(0, Ordering::Release);
        self.remembered.clear();
        // Private reservations may outlive the ordinary heap. Each owner refunds its own charge.
    }

    fn wake_context_root_repair(&self, context: &crate::context::State) {
        if self.contexts.repairs.publish(context) {
            self.wake();
        }
    }

    fn wake_root_repair(&self, roots: &crate::roots::RootSlots) {
        if roots.repair_pending() {
            self.wake();
        }
    }

    #[cfg_attr(
        target_family = "wasm",
        expect(
            clippy::unused_self,
            reason = "cooperative hosts have no native workers"
        )
    )]
    fn wake(&self) {
        #[cfg(not(target_family = "wasm"))]
        if let Some(signals) = self.worker_signals.load().as_ref() {
            signals.wake();
        }
    }

    /// Requests a full collection. On cooperative targets, call `step` at safe boundaries.
    pub fn collect(&self) {
        self.request_full_cycle();
    }

    fn request_full_cycle(&self) -> u64 {
        let ticket = self.domains[1]
            .request_ticket
            .fetch_add(1, Ordering::AcqRel)
            + 1;
        self.domains[1].publish_request();
        self.wake();
        ticket
    }

    pub(crate) fn completed_full_ticket(&self) -> u64 {
        self.domains[1].completed_ticket.load(Ordering::Acquire)
    }

    // Recovery can share an outstanding full request. Explicit collection still
    // requests a fresh ticket. No coordinator lock is taken on the allocating task.
    pub(crate) fn request_recovery_cycle(&self) -> u64 {
        let domain = &self.domains[1];
        let requested = domain.request_ticket.load(Ordering::Acquire);
        if requested > domain.completed_ticket.load(Ordering::Acquire) {
            // Help publish a request whose creator was preempted between allocating
            // its ticket and setting the flag. A race with cycle start can request
            // one redundant pass, but cannot strand a waiter or lose a request.
            if !domain.active() {
                domain.publish_request();
            }
            self.wake();
            return requested;
        }
        self.request_full_cycle()
    }

    /// Requests a full collection and suspends only this future until it completes.
    /// Native workers perform collection; without workers each poll performs at most
    /// one configured step. Cancelling the future removes its waiter but leaves the
    /// collection request available to other callers and workers.
    /// # Errors
    /// Returns an error for a failed worker or polling under managed guards.
    pub async fn collect_full_async(&self) -> Result<()> {
        if crate::cell::has_guards() {
            return Err(crate::Error::CollectionPhaseError(
                "asynchronous collection requires released managed guards".into(),
            ));
        }
        if self.diagnostics_enabled() {
            self.diagnostics
                .async_collection_calls
                .fetch_add(1, Ordering::Relaxed);
        }
        let ticket = self.request_full_cycle();
        crate::recovery::CollectionWait::new(self, ticket).await?;
        Ok(())
    }

    /// Runs a full collection to completion on the calling thread.
    ///
    /// # Errors
    /// Reserved for collection errors; currently collection itself is infallible.
    pub fn collect_full(&self) -> Result<()> {
        self.wait_full_cycle();
        #[cfg(not(target_family = "wasm"))]
        {
            let jobs = self.callback_jobs.load(Ordering::Acquire);
            if jobs != 0
                && !CleanupScope::contains(self)
                && let Some(pool) = self.worker_signals.load_full()
            {
                pool.callbacks.help_ready(jobs);
            }
        }
        Ok(())
    }

    fn wait_full_cycle(&self) {
        if self.diagnostics_enabled() {
            self.diagnostics
                .full_collection_calls
                .fetch_add(1, Ordering::Relaxed);
        }
        let ticket = self.request_full_cycle();
        #[cfg(all(test, not(target_family = "wasm"), not(miri)))]
        phase_tests::after_full_request();
        let mut misses = 1;
        while self.domains[1].completed_ticket.load(Ordering::Acquire) < ticket {
            if self.step_inner(false).is_none() {
                for _ in 0..misses {
                    std::hint::spin_loop();
                }
                misses = (misses * 2).min(512);
            } else {
                misses = 1;
            }
        }
    }

    /// Requests a young collection without requiring a native worker.
    pub fn collect_young(&self) {
        if !self.domains[0].publish_request() {
            return;
        }
        self.wake();
    }

    pub(crate) fn reserve_allocation(&self, size: usize) -> Result<()> {
        self.managed_budget.try_charge(size)
    }

    pub(crate) fn reserve_private_bytes(
        &self,
        bytes: usize,
    ) -> Result<crate::allocation_budget::Reservation> {
        self.managed_budget.reserve(bytes)
    }

    pub(crate) fn signal_grants(&self) {
        // Pair with the same-word RMW after a terminal sleeper installed delivery.
        self.managed_budget.used_after_registration();
        self.recovery.grants_released();
        #[cfg(not(target_family = "wasm"))]
        if let Some(notifier) = self.refund_notifier.load().as_ref() {
            notifier.signal();
        }
    }

    pub(crate) fn cancel_allocation(&self, size: usize) {
        let previous = self.managed_budget.release(size);
        let available = self
            .configuration
            .max_heap_bytes
            .saturating_sub(previous - size);
        self.recovery.record_headroom(available);
        #[cfg(not(target_family = "wasm"))]
        if self.recovery.has_fitting_waiter(available)
            && let Some(notifier) = self.refund_notifier.load().as_ref()
        {
            notifier.signal();
        }
    }

    pub(crate) fn headroom_after_registration(&self) -> usize {
        self.configuration
            .max_heap_bytes
            .saturating_sub(self.managed_budget.used_after_registration())
    }

    pub(crate) fn allocation_headroom(&self) -> usize {
        self.configuration
            .max_heap_bytes
            .saturating_sub(self.managed_budget.used())
    }

    pub(crate) fn refresh_allocation_interest(
        &self,
        interest: &mut Option<crate::context::AllocationInterest>,
    ) -> Result<()> {
        if self.configuration.local_allocation_buffers {
            let matches = crate::context::with_state(self, |state| {
                interest
                    .as_ref()
                    .is_some_and(|guard| guard.belongs_to(state))
            })
            .unwrap_or(interest.is_none());
            if !matches {
                // Establish the new interest before releasing the prior context.
                // Moving between threads with the same context needs no update.
                *interest = crate::context::current(self)
                    .map(|state| state.observe_allocations(self))
                    .transpose()?;
            }
        }
        Ok(())
    }

    fn context_credit(&self, size: usize) -> usize {
        if self.configuration.local_allocation_buffers && size <= crate::allocation::SMALL {
            crate::context::with_state(self, crate::context::State::available_credit).unwrap_or(0)
        } else {
            0
        }
    }

    fn context_headroom(&self, size: usize) -> usize {
        self.allocation_headroom()
            .saturating_add(self.context_credit(size))
    }

    pub(crate) fn reserve_exact_bytes(&self, size: usize) -> Result<AllocationReservation<'_>> {
        self.reserve_allocation(size)?;
        Ok(AllocationReservation {
            collector: self,
            size,
            committed: false,
        })
    }

    pub(crate) fn reserve_bytes(&self, size: usize) -> Result<AllocationReservation<'_>> {
        if self.configuration.local_allocation_buffers
            && size <= crate::allocation::SMALL
            && let Some(result) =
                crate::context::with_state(self, |state| state.reserve_cached(self, size))
        {
            match result {
                Ok(unused) => {
                    if unused != 0 {
                        self.cancel_allocation(unused);
                    }
                    return Ok(AllocationReservation {
                        collector: self,
                        size,
                        committed: false,
                    });
                }
                Err(unused) if unused != 0 => self.cancel_allocation(unused),
                Err(_) => {}
            }
        }
        self.reserve_exact_bytes(size)
    }

    pub(crate) fn may_recover_bytes(&self, size: usize) -> bool {
        size <= self.configuration.max_heap_bytes && !crate::cell::has_guards()
    }

    // A shared ticket can already be in sweep when observation starts. Before
    // declaring exhaustion require a full snapshot requested after registration.
    fn request_space_cycle(&self, consecutive_empty: usize) -> u64 {
        if consecutive_empty == 0 {
            self.request_recovery_cycle()
        } else {
            self.request_full_cycle()
        }
    }

    // Capture one generation of publicly released donors. Later arrivals must not
    // extend exhaustion. Capture and help are separate units so an async caller
    // can yield after its deciding collection step and recheck credit and guards.
    fn help_terminal_releases(
        &self,
        released: &mut Option<crate::retirement_grants::ReleasedCheck>,
    ) -> bool {
        let Some(check) = released else {
            *released = Some(self.released_donors.snapshot());
            #[cfg(all(test, not(target_family = "wasm"), not(miri)))]
            crate::refund_tests::terminal_snapshot_captured();
            return true;
        };
        if !check.pending() {
            return false;
        }
        self.reap_retired(false, Some(check));
        true
    }

    /// Reserve as soon as reclamation makes room, without awaiting the rest of a cycle.
    /// Failed races for headroom do not spend the two completed-cycle recovery budget.
    pub(crate) async fn reserve_bytes_async(
        &self,
        size: usize,
    ) -> Result<AllocationReservation<'_>> {
        self.reserve_bytes_with_assistance(size, false).await
    }

    pub(crate) fn reserve_bytes_blocking(&self, size: usize) -> Result<AllocationReservation<'_>> {
        crate::recovery::finish_allocation(self.reserve_bytes_with_assistance(size, true))
    }

    async fn reserve_bytes_with_assistance(
        &self,
        size: usize,
        caller_assists: bool,
    ) -> Result<AllocationReservation<'_>> {
        if size > self.configuration.max_heap_bytes {
            return Err(crate::Error::OutOfMemory);
        }
        if let Ok(reservation) = self.reserve_bytes(size) {
            return Ok(reservation);
        }
        if crate::cell::has_guards() {
            return Err(crate::Error::OutOfMemory);
        }
        let demand = self.grant_registry.register(self, size)?;
        let mut empty_cycles = 0;
        let mut released = None;
        loop {
            if let Some(reservation) = demand.take() {
                return Ok(reservation);
            }
            if let Ok(reservation) = self.reserve_bytes(size) {
                return Ok(reservation);
            }
            if crate::cell::has_guards() {
                return Err(crate::Error::OutOfMemory);
            }
            if empty_cycles == 2 {
                if self.help_terminal_releases(&mut released) {
                    crate::recovery::yield_once().await;
                    continue;
                }
                #[cfg(all(test, not(target_family = "wasm"), not(miri)))]
                crate::refund_tests::before_terminal_close();
                if demand.finish_uncovered() {
                    // Closing the grant demand and reserving general headroom are
                    // separate operations. An unrelated owner can physically refund
                    // storage after our last probe and before this close.
                    return self.reserve_bytes(size);
                }
                // This donor was selected while publicly owned. Its charge is
                // still unavailable until the backing storage is actually freed.
                let mut opportunity = self.observe_space(size)?;
                crate::recovery::CollectionWait::for_space(self, u64::MAX, &mut opportunity)
                    .with_grant(&demand, true, caller_assists)
                    .await?;
                continue;
            }
            let _measurement = self
                .diagnostics
                .async_allocation_recovery
                .measure(self.diagnostics_enabled());
            // Register before yielding so a refund taken by a competitor
            // during that suspension counts as an allocation opportunity.
            let mut opportunity = self.observe_space(size)?;
            crate::recovery::yield_once().await;
            if let Some(reservation) = demand.take() {
                return Ok(reservation);
            }
            if let Ok(reservation) = self.reserve_bytes(size) {
                return Ok(reservation);
            }
            let ticket = self.request_space_cycle(empty_cycles);
            let mut observed_space = false;
            loop {
                observed_space |=
                    crate::recovery::CollectionWait::for_space(self, ticket, &mut opportunity)
                        .with_grant(&demand, false, caller_assists)
                        .await?;
                if let Some(reservation) = demand.take() {
                    return Ok(reservation);
                }
                if let Ok(reservation) = self.reserve_bytes(size) {
                    return Ok(reservation);
                }
                if self.completed_full_ticket() >= ticket {
                    empty_cycles = if observed_space || opportunity.was_offered() {
                        0
                    } else {
                        empty_cycles + 1
                    };
                    break;
                }
                // Another allocator took the advertised space. Re-register for
                // this same cycle; no credits were reserved by its notification.
                crate::recovery::yield_once().await;
            }
        }
    }

    /// Checks headroom before publication, assisting one batch at a time until
    /// space is available or two consecutive cycles complete without offering enough space.
    /// A caller holding a managed guard can use available space but cannot assist.
    /// # Errors
    /// Returns a collection error, or `OutOfMemory` if the live heap cannot accommodate the request.
    pub fn make_room(&self, size: usize) -> Result<()> {
        if size <= self.context_headroom(size) {
            return Ok(());
        }
        if !self.may_recover_bytes(size) {
            return Err(crate::Error::OutOfMemory);
        }
        let mut empty_cycles = 0;
        while empty_cycles < 2 {
            let _measurement = self
                .diagnostics
                .allocation_recovery
                .measure(self.diagnostics_enabled());
            let opportunity = self.observe_space(size)?;
            let ticket = self.request_space_cycle(empty_cycles);
            loop {
                if size <= self.context_headroom(size) {
                    return Ok(());
                }
                if self.completed_full_ticket() >= ticket {
                    break;
                }
                self.step();
            }
            empty_cycles = if opportunity.was_offered() {
                0
            } else {
                empty_cycles + 1
            };
        }
        let mut released = None;
        while size > self.context_headroom(size) {
            if crate::cell::has_guards() || !self.help_terminal_releases(&mut released) {
                return Err(crate::Error::OutOfMemory);
            }
        }
        Ok(())
    }

    /// Checks headroom, returning as soon as enough space is available. At most two consecutive
    /// recovery cycles without an observed or offered opportunity complete before failure.
    /// This is an advisory check, not a reservation; publication must still reserve
    /// its bytes atomically. No collection work runs while holding managed guards.
    /// # Errors
    /// Returns `OutOfMemory` for an unsatisfiable request, or a collection error.
    pub async fn make_room_async(&self, size: usize) -> Result<()> {
        if size > self.configuration.max_heap_bytes {
            return Err(crate::Error::OutOfMemory);
        }
        let mut empty_cycles = 0;
        let mut released = None;
        loop {
            if size <= self.context_headroom(size) {
                return Ok(());
            }
            if crate::cell::has_guards() {
                return Err(crate::Error::OutOfMemory);
            }
            if empty_cycles == 2 {
                if self.help_terminal_releases(&mut released) {
                    crate::recovery::yield_once().await;
                    continue;
                }
                return Err(crate::Error::OutOfMemory);
            }
            let _measurement = self
                .diagnostics
                .async_allocation_recovery
                .measure(self.diagnostics_enabled());
            let mut opportunity = self.observe_space(size)?;
            crate::recovery::yield_once().await;
            if size <= self.context_headroom(size) {
                return Ok(());
            }
            let ticket = self.request_space_cycle(empty_cycles);
            let offered =
                crate::recovery::CollectionWait::for_space(self, ticket, &mut opportunity).await?;
            empty_cycles = if offered || opportunity.was_offered() {
                0
            } else {
                empty_cycles + 1
            };
        }
    }

    /// Records an ordinary allocation's logical live bytes and allocation debt.
    /// Private/canonical reservations use their ownership-bound budget separately.
    pub fn record_allocation(&self, size: usize) {
        self.record_published_bytes(size);
        self.record_allocation_debt(size);
    }

    pub(crate) fn record_published_bytes(&self, size: usize) {
        self.published_bytes.fetch_add(size, Ordering::AcqRel);
        self.published_debt.fetch_add(size, Ordering::AcqRel);
    }

    pub(crate) fn release_published_bytes(&self, size: usize) {
        let previous = self.published_bytes.fetch_sub(size, Ordering::AcqRel);
        debug_assert!(previous >= size, "duplicate published-byte refund");
    }

    pub(crate) fn record_allocation_debt(&self, size: usize) {
        let debt = self.allocation_debt.fetch_add(size, Ordering::AcqRel);
        if debt.saturating_add(size) >= self.configuration.allocation_threshold {
            self.collect_young();
        }
        if let Some(scheduler) = &self.scheduler
            && scheduler.allocated(size, self.managed_budget.used(), self.regions.as_deref())
        {
            self.request_recovery_cycle();
        }
    }

    /// Returns the latest advisory pressure model; disabled by default.
    #[must_use]
    pub fn scheduling_statistics(&self) -> crate::SchedulingStatistics {
        self.scheduler.as_ref().map_or_else(
            crate::SchedulingStatistics::default,
            crate::scheduling::Scheduler::statistics,
        )
    }

    /// Atomically samples charged managed bytes, including pending reservations.
    ///
    /// This is the same Acquire load used by [`Self::statistics`], without taking
    /// statistics or object-index mutexes. A zero snapshot does not certify
    /// quiescence, completed collection, absence of roots, or that another actor
    /// cannot reserve bytes immediately afterward. Callers must establish their
    /// own ownership/lifecycle conditions before treating it as cleanup evidence.
    /// The value excludes catalog/runtime backing and host allocator overhead.
    #[must_use]
    pub fn managed_bytes(&self) -> usize {
        self.managed_budget.used()
    }

    /// Returns live heap accounting and collection statistics.
    ///
    /// # Errors
    /// Reserved for compatibility with callers of the previous collector.
    pub fn statistics(&self) -> Result<Statistics> {
        let mut stats = lock(&self.statistics).clone();
        stats.objects_marked += self.leaves_marked_at_birth.load(Ordering::Relaxed);
        stats.bytes_allocated = self.published_debt.load(Ordering::Acquire);
        stats.bytes_live = self.published_bytes.load(Ordering::Acquire);
        stats.bytes_managed = self.managed_bytes();
        stats.allocation_debt = self.allocation_debt.load(Ordering::Acquire);
        stats.live_objects = self.objects.len();
        Ok(stats)
    }

    /// Returns root occupancy and retained slot-page storage, separately from managed bytes.
    #[must_use]
    pub fn root_statistics(&self) -> crate::RootStatistics {
        let mut result = self.scanned_slots().map_or_else(
            crate::RootStatistics::default,
            crate::roots::RootSlots::statistics,
        );
        result.merge(self.contexts.statistics().1);
        result.compatibility_roots = self.roots.len();
        result
    }

    pub(crate) fn scanned_slots(&self) -> Option<&crate::roots::RootSlots> {
        self.root_slots
            .as_deref()
            .or_else(|| self.typed_slots.get())
    }

    pub(crate) fn reserve_owned_root(&self, address: usize) -> RootReservation<'_> {
        let context = (self.root_slots.is_some()
            && self.configuration.root_storage == crate::RootStorage::Contexts)
            .then(|| crate::context::current(self))
            .flatten();
        let slot = self.root_slots.as_ref().map(|roots| {
            context.as_ref().map_or_else(
                || roots.prepare(address),
                |context| context.roots.prepare(address),
            )
        });
        RootReservation {
            collector: self,
            address,
            context,
            slot,
        }
    }

    /// A managed edge already carries its original Rust type. Resolve its
    /// exact identity in this collector and certify liveness as for a VM handle.
    pub(crate) fn clone_edge_root<T>(&self, root: &Gc<T>) -> Option<crate::roots::Registration> {
        self.acquire_checked_root::<T>(root.ptr.as_ptr().addr(), Some(root.identity()), |_| true)
            .ok()
            .map(|(_, _, registration)| registration)
    }

    pub(crate) fn add_owned_root<T>(&self, root: &Gc<T>) -> crate::roots::Registration {
        let _measurement = self
            .diagnostics
            .root_acquire
            .measure(self.diagnostics_enabled());
        let mut reservation = self.reserve_owned_root(root.ptr.as_ptr() as usize);
        let _publication = self.allocation_guard();
        self.commit_owned_root(root, &mut reservation)
    }

    /// Install the initial root of an exclusively owned, unregistered allocation.
    /// Its address cannot resolve to a live object yet. Descriptor registration
    /// performs the birth/traversal barrier in each active domain before exposing
    /// the handle. Construction roots protect the outgoing edges until then.
    pub(crate) fn add_private_root(&self, metadata: &MetadataRef) -> crate::roots::Registration {
        let _measurement = self
            .diagnostics
            .root_acquire
            .measure(self.diagnostics_enabled());
        self.install_root_registration(&mut self.reserve_owned_root(metadata.ptr().0))
    }

    /// Caller retains this collector's publication admission through commit/shading.
    fn commit_owned_root<T>(
        &self,
        root: &Gc<T>,
        reservation: &mut RootReservation<'_>,
    ) -> crate::roots::Registration {
        let address = root.ptr.as_ptr() as usize;
        assert!(
            reservation.protects(self, address),
            "foreign root reservation"
        );
        let registration = self.install_root_registration(reservation);
        self.shade_all(address);
        registration
    }

    fn install_root_registration(
        &self,
        reservation: &mut RootReservation<'_>,
    ) -> crate::roots::Registration {
        let address = reservation.address;
        let Some(slot) = reservation.slot.take() else {
            return crate::roots::Registration::Tree(self.insert_root(address));
        };
        let registration = slot.commit(address, reservation.context.take());
        if self.diagnostics_enabled() {
            self.diagnostics
                .root_registrations
                .fetch_add(1, Ordering::Relaxed);
        }
        registration
    }

    pub(crate) fn remove_owned_root(&self, id: &crate::roots::Registration) {
        self.remove_owned_root_with_construction::<true>(id);
    }

    /// Release a transferred registration without consulting a helper's ambient
    /// construction scope. The caller must already own this registration and its
    /// graph protection; normal deletion admission, barriers and reuse still apply.
    pub(crate) fn remove_detached_root(&self, id: &crate::roots::Registration) {
        self.remove_owned_root_with_construction::<false>(id);
    }

    fn remove_owned_root_with_construction<const ADOPT: bool>(
        &self,
        id: &crate::roots::Registration,
    ) {
        let _measurement = self
            .diagnostics
            .root_release
            .measure(self.diagnostics_enabled());
        match id {
            crate::roots::Registration::Tree(id) => {
                self.remove_tree_registration::<ADOPT>(id.id, Some(id.generation));
            }
            crate::roots::Registration::Overflow(record) => {
                if let Some(roots) = self.scanned_slots() {
                    self.remove_overflow_root::<ADOPT>(roots, record);
                }
            }
            crate::roots::Registration::ContextOverflow(context, record) => {
                self.remove_overflow_root::<ADOPT>(&context.roots, record);
            }
            crate::roots::Registration::Slot(root) => {
                if self.remove_paged_root::<ADOPT>(root)
                    && let Some(roots) = self.scanned_slots()
                {
                    roots.finish_removal(root);
                    self.wake_root_repair(roots);
                }
            }
            crate::roots::Registration::Context(context, root) => {
                if self.remove_paged_root::<ADOPT>(root) && context.roots.finish_removal(root) {
                    self.wake_context_root_repair(context);
                }
            }
        }
    }

    fn remove_paged_root<const ADOPT: bool>(&self, root: &crate::roots::SlotRoot) -> bool {
        if !ADOPT
            && let Some(address) = root.begin_removal()
            && self.deletion_covered(address, || root.removal_pending())
            && root.finish_removal()
        {
            #[cfg(all(test, not(target_family = "wasm"), not(miri)))]
            publication_tests::record_covered_removal();
            return root.release();
        }
        let _publication = self.allocation_guard();
        if let Some(address) = root.take_address() {
            if ADOPT {
                crate::construction::retain(self, address);
            }
            self.barrier(address, None);
            root.release()
        } else {
            false
        }
    }

    /// The exact removal intent must already be visible to every root scanner.
    fn deletion_covered(&self, address: usize, removal_pending: impl FnOnce() -> bool) -> bool {
        let observation = self.phase_clock.indexed_root();
        // Retirement ignores all deletion shades. If a new domain starts
        // after this sample, its root scan still races the exact removal intent.
        let covered = observation.deletion_closed()
            || self.objects.lookup(address).is_some_and(|metadata| {
                if self.typed_root_covered(observation, &metadata) {
                    return true;
                }
                // Attached contexts already batch deletion barriers in fixed-size
                // buffers. Preserve that path and its allocation behavior.
                if self.configuration.barrier_buffer_size != 0
                    && crate::context::current(self).is_some()
                {
                    return false;
                }
                let phases = self.phase_clock.indexed_root();
                match &phases {
                    publication_state::Birth::Idle => return true,
                    publication_state::Birth::Inline(s) | publication_state::Birth::Epochs(s)
                        if (0..2).any(|i| s.stage(i).is_some_and(|stage| stage != 0)) =>
                    {
                        return false;
                    }
                    _ => {}
                }
                // Each attempt has its own decision. Collectors can cancel a parked
                // publisher when they visit a proposal. Validate the source after
                // linking, while a live matching generation still owns the address.
                let debt = Arc::new(crate::roots::Record::typed(address));
                let certified = self.certify_root_attempt(phases, &metadata, &debt) == Some(true);
                #[cfg(all(test, not(target_family = "wasm"), not(miri)))]
                publication_tests::pause_after_typed_proposals();
                certified && removal_pending() && debt.commit_typed()
            });
        #[cfg(all(test, not(target_family = "wasm"), not(miri)))]
        publication_tests::pause_after_drop_coverage();
        covered
    }

    fn remove_overflow_root<const ADOPT: bool>(
        &self,
        roots: &crate::roots::RootSlots,
        record: &Arc<crate::roots::Record>,
    ) {
        if !ADOPT
            && let Some(address) = record.begin_typed_removal()
            && self.deletion_covered(address, || record.typed_removal_pending())
            && record.finish_typed_removal()
        {
            #[cfg(all(test, not(target_family = "wasm"), not(miri)))]
            publication_tests::record_covered_removal();
            roots.finish_overflow(record);
            return;
        }
        let publication = self.allocation_guard();
        if let Some(address) = record.take_address() {
            if ADOPT {
                crate::construction::retain(self, address);
            }
            self.barrier(address, None);
            drop(publication);
            roots.finish_overflow(record);
        }
    }

    fn insert_root(&self, address: usize) -> crate::roots::TreeRoot {
        self.insert_tree_root(address, true)
    }

    fn insert_tree_root(&self, address: usize, owned: bool) -> crate::roots::TreeRoot {
        #[cfg(test)]
        {
            let probe = *lock(&self.tree_root_insert_probe);
            if let Some(probe) = probe {
                probe(self);
            }
        }
        if self.diagnostics_enabled() {
            self.diagnostics
                .root_registrations
                .fetch_add(1, Ordering::Relaxed);
        }
        if owned {
            self.roots.insert(&self.next_root_id, address)
        } else {
            self.roots.insert_manual(&self.next_root_id, address)
        }
    }

    /// Called under a publication reader before the removed root can cease to
    /// protect its target. Unpublished descriptors have no registry entry yet.
    pub(crate) fn retain_construction_root(
        &self,
        address: usize,
    ) -> Option<crate::roots::TreeRoot> {
        let address = self.objects.lock(address).get(&address)?.ptr().0;
        Some(self.insert_root(address))
    }

    /// A conditional edge is not proof of liveness. Check its allocation identity
    /// and closed weak/retirement decisions before creating a construction root.
    /// Called under short publication admission and the slot's decision lock.
    pub(crate) fn retain_construction_referent(
        &self,
        slot: &crate::weak::WeakSlot,
    ) -> Option<crate::roots::TreeRoot> {
        let address = slot.target.load(Ordering::Acquire);
        if address == 0 || self.weak_is_cleared(slot, address) {
            return None;
        }
        let metadata = self.objects.lock(address).get(&address).cloned()?;
        if metadata.id != slot.target_identity || self.logically_retired(&metadata) {
            return None;
        }
        let id = self.insert_root(metadata.ptr().0);
        // Enumeration may already have passed this root. Publish its graph to
        // each active mark queue before admission ends, respecting phase floors.
        self.shade_all(address);
        Some(id)
    }

    /// Registers a manual root, including while a collection is in progress.
    /// Release the returned ID with `remove_root_by_id`; owning handles and
    /// internal collector registrations are managed independently.
    pub fn add_root<T: Trace>(&self, root: &Gc<T>) -> usize {
        let _publication = self.allocation_guard();
        let id = self.insert_tree_root(root.ptr.as_ptr() as usize, false);
        self.shade_all(root.ptr.as_ptr() as usize);
        id.id
    }

    /// Creates an owning root registration in this allocation's collector.
    ///
    /// # Panics
    /// Panics before registration if the handle belongs to another collector or
    /// is unassociated. Use `Gc::as_root` to receive a mismatch as an error.
    pub fn create_root_guard<T: Trace>(self: &Arc<Self>, root: Gc<T>) -> GcRootGuard<T> {
        assert!(
            root.belongs_to(self),
            "root guard collector does not own this allocation"
        );
        let id = self.add_owned_root(&root);
        GcRootGuard::new(self.clone(), id, root)
    }

    /// Removes a manual root returned by `add_root`, preserving the snapshot
    /// during an active marking cycle. Other IDs cannot revoke owning handles or
    /// internal collector registrations.
    /// Numeric IDs are invalid after removal and can be reused after usize wrap.
    pub fn remove_root_by_id(&self, id: usize) {
        self.remove_tree_registration::<true>(id, None);
    }

    fn remove_tree_root(&self, root: crate::roots::TreeRoot) {
        self.remove_tree_registration::<true>(root.id, Some(root.generation));
    }

    fn remove_tree_registration<const ADOPT: bool>(
        &self,
        id: usize,
        generation: Option<std::num::NonZeroU64>,
    ) {
        if !ADOPT
            && let Some(generation) = generation
            && (self.configuration.barrier_buffer_size == 0
                || crate::context::current(self).is_none())
            && self.roots.remove_covered(
                crate::roots::TreeRoot { id, generation },
                |address, pending| self.deletion_covered(address, pending),
            )
        {
            #[cfg(all(test, not(target_family = "wasm"), not(miri)))]
            publication_tests::record_covered_removal();
            return;
        }
        let _publication = self.allocation_guard();
        let removed = self.roots.remove(id, generation);
        if let Some(address) = removed {
            #[cfg(all(test, not(target_family = "wasm"), not(miri)))]
            pause_after_tree_detach();
            if ADOPT {
                crate::construction::retain(self, address);
            }
            self.barrier(address, None);
        }
    }

    /// Revokes every registration for an allocation, including owning handles.
    ///
    /// Prefer dropping owning handles or removing an `add_root` registration
    /// with `remove_root_by_id`.
    ///
    /// # Safety
    /// The `root` argument must remain valid for this entire call, independently
    /// of the registrations being revoked (for example, while reclamation is
    /// externally excluded).
    /// The caller must prevent access through every affected handle after the
    /// allocation can be reclaimed, including concurrent readers and references
    /// borrowed from those handles. Dropping revoked handles without accessing
    /// their payload is allowed. If handles remain usable, the caller must keep
    /// the allocation and its traced graph independently reachable until a new
    /// registration is established. Revocation does not transfer ownership.
    ///
    /// ```compile_fail,E0133
    /// use ristretto_gc::{GarbageCollector, Gc};
    /// let gc = GarbageCollector::new();
    /// let root = Gc::new(&gc, 42_u64);
    /// gc.remove_root(&**root);
    /// ```
    pub unsafe fn remove_root<T: Trace>(&self, root: &T) {
        // The caller independently protects the payload. Each deletion validates
        // its exact registration and publishes its own barrier; enumeration
        // needs no heap-wide admission across unrelated slots and contexts.
        let ptr = std::ptr::from_ref(root).cast::<()>();
        if let Some(roots) = self.scanned_slots() {
            for record in roots.matching_overflow(ptr as usize) {
                self.remove_owned_root(&crate::roots::Registration::Overflow(record));
            }
            for id in roots.matching(ptr as usize) {
                self.remove_owned_root(&crate::roots::Registration::Slot(id));
            }
        }
        self.contexts.for_each(|context| {
            for record in context.roots.matching_overflow(ptr as usize) {
                self.remove_owned_root(&crate::roots::Registration::ContextOverflow(
                    context.clone(),
                    record,
                ));
            }
            for id in context.roots.matching(ptr as usize) {
                self.remove_owned_root(&crate::roots::Registration::Context(context.clone(), id));
            }
        });
        let ids = self.roots.matching(ptr as usize);
        #[cfg(all(test, not(target_family = "wasm"), not(miri)))]
        publication_tests::pause_after_revocation_scan();
        for id in ids {
            self.remove_tree_root(id);
        }
    }

    fn trace_retention(&self) -> Retention {
        let inherited =
            TraceRetention::current(self).map_or(Retention::Strong, |(strength, _)| strength);
        inherited.max(Retention::at_phase(self.domain().weak_phase()))
    }

    fn shade_metadata(&self, metadata: &MetadataRef) -> bool {
        let epoch = self.domain().epoch.load(Ordering::Acquire);
        let phase = self.domain().weak_phase();
        // Once retirement is admitted, white objects are logically dead. A
        // deletion barrier from clearing a dead parent must not revive a child
        // whose own graph may already have been swept. Fresh allocation has a
        // separate black-publication path; valid rooted handles are already live.
        if phase >= 5
            || (self.configuration.overlapping_generations && self.logically_retired(metadata))
        {
            return false;
        }
        if self.domain_index() == 1
            && self.old_collection_active()
            && metadata.age.load(Ordering::Acquire) < self.configuration.promotion_age.max(1)
        {
            // Initial young objects are handed off separately. Births are not
            // roots of this snapshot merely because their owning handles change.
            if !metadata.marked(1, epoch) || metadata.retention(1, epoch).is_some() {
                return metadata.mark(1, epoch, self.trace_retention());
            }
            return false;
        }
        if metadata.mark(self.domain_index(), epoch, self.trace_retention()) {
            self.domain()
                .mark_queue
                .push(self.local_queue(), MarkItem::fresh(metadata.clone()));
            return true;
        }
        false
    }

    fn shade_all(&self, address: usize) -> bool {
        self.domains.iter().any(Domain::active)
            && self
                .objects
                .lookup(address)
                .is_some_and(|metadata| self.shade_live_domains(&metadata))
    }

    fn shade_live_domains(&self, metadata: &MetadataRef) -> bool {
        // The caller holds publication admission through both domain barriers.
        let _retention = TraceRetention::enter(self, Retention::Strong);
        let mut changed = false;
        for index in 0..2 {
            let _context = Context::enter(self, index);
            changed |= self.shade_known(metadata);
        }
        changed
    }

    fn shade(&self, address: usize) -> bool {
        if !self.domain().active() {
            return false;
        }
        self.objects
            .live(address)
            .is_some_and(|metadata| self.shade_known(&metadata))
    }

    /// The phase unit orders its root read and shade against closure. Resolve
    /// only visible metadata; shading never borrows the target payload.
    fn shade_registered_root(&self, address: usize) {
        if let Some(metadata) = self.objects.lookup(address) {
            let mature_full = self.domain_index() == 1
                && self.domain().weak_phase() == 0
                && !self.is_young_metadata(&metadata)
                && self.trace_retention() == Retention::Strong;
            if self.shade_known(&metadata) && mature_full {
                // A mature full root has no receiving-young publication.
                self.mature_census.strong.fetch_add(1, Ordering::Relaxed);
            }
        }
    }

    /// The caller protects registration/phase ordering; this path does not wait
    /// on the object index while admitted. Metadata owns no payload borrow.
    fn shade_known(&self, metadata: &MetadataRef) -> bool {
        if !self.domain().active() {
            return false;
        }
        let young = metadata.age.load(Ordering::Acquire) < self.configuration.promotion_age.max(1);
        if young {
            SCAN_YOUNG.set(true);
        }
        if self.domain_index() == 0 && !young {
            return false;
        }
        let mut changed = self.shade_metadata(metadata);
        let other = self.domain_at(1 - self.domain_index());
        if other.active() {
            let _retention = TraceRetention::enter(self, self.trace_retention());
            let _context = Context::enter(self, 1 - self.domain_index());
            if self.domain_index() == 1 || young {
                changed |= self.shade_metadata(metadata);
            }
        }
        changed
    }

    fn prune_remembered(
        &self,
        address: usize,
        identity: crate::ObjectId,
        version: usize,
        has_young: bool,
    ) {
        // Pruning is optional. Keep the entry if either shard is busy, so a
        // suspended shard owner cannot strand an in-flight mark task.
        if has_young || version == usize::MAX {
            return;
        }
        // A write after the scan began either has a newer directory version,
        // or will publish it after our CAS. Ignore a lost optional prune.
        self.remembered
            .dirty
            .try_remove(identity.to_i64().cast_unsigned(), version);
        if !self.remembered.may_contain(address) {
            return;
        }
        let Some(objects) = self.objects.try_lock(address) else {
            return;
        };
        // Verify identity and version under the remembered shard. A later
        // buffered update publishes a fresh insertion after this removal.
        if let Some(_writer) = self.remembered.try_lock(address)
            && objects.get(&address).is_some_and(|metadata| {
                metadata.id == identity && metadata.mutation.load(Ordering::Acquire) == version
            })
        {
            self.remembered.remove(address, identity);
        }
    }

    /// Whether the active cycle collects only young allocations.
    #[must_use]
    pub fn is_young_collection(&self) -> bool {
        self.domain_index() == 0
    }

    /// Queries generation metadata without dereferencing an allocation address.
    #[must_use]
    pub fn is_young_allocation(&self, address: usize) -> bool {
        self.objects
            .lock(address)
            .get(&address)
            .is_some_and(|metadata| self.is_young_metadata(metadata))
    }

    fn is_young_metadata(&self, metadata: &ObjectMetadata) -> bool {
        metadata.age.load(Ordering::Acquire) < self.configuration.promotion_age.max(1)
    }

    /// Records mutation of an owner, preserving old-to-young edges across young collections.
    pub fn remember<T>(&self, owner: &Gc<T>) {
        self.remember_address(owner.ptr.as_ptr() as usize);
    }

    /// Shades a reference before it is overwritten. Barrier work is never discarded.
    pub fn write_barrier<T: Trace>(&self, target: &Gc<T>) {
        self.edge_barrier(target);
    }

    /// Certify a no-op against the packed phase/revision clock. Retention only
    /// strengthens within an epoch; all phase transitions stamp the revision.
    /// Metadata ownership permits the check to finish after storage retirement,
    /// but does not access payloads or imply that their graphs remain reachable.
    fn edge_already_satisfied(
        &self,
        metadata: &MetadataRef,
        clock: &publication_state::Clock,
        snapshot: publication_state::Snapshot,
        record: crate::context::Record,
    ) -> bool {
        let satisfied = record.epochs.iter().enumerate().all(|(index, epoch)| {
            epoch.is_none()
                || (index == 0
                    && metadata.age.load(Ordering::Acquire)
                        >= self.configuration.promotion_age.max(1))
                || epoch.is_some_and(|epoch| {
                    metadata.retention(index, epoch) == Some(Retention::Strong)
                })
        });
        #[cfg(test)]
        if satisfied {
            let pause = lock(&self.edge_certificate_pause).take();
            if let Some(pause) = pause {
                pause.entered.send(()).expect("certificate checked");
                pause
                    .resume
                    .recv_timeout(std::time::Duration::from_secs(if cfg!(miri) {
                        120
                    } else {
                        10
                    }))
                    .expect("resume certificate");
            }
        }
        if satisfied && clock.unchanged(snapshot) {
            // Cache only after successful certification. Phase changes after this
            // point merely leave a stale epoch key; rollover disables the cache.
            let record = crate::context::Record {
                identity: metadata.id,
                ..record
            };
            crate::context::with_state(self, |state| state.completed(record));
            true
        } else {
            false
        }
    }

    fn edge_barrier<T>(&self, target: &Gc<T>) {
        self.publish_edge(
            target.ptr.as_ptr().addr(),
            target.identity(),
            target.traced_metadata(),
        );
    }

    fn publish_edge(&self, address: usize, identity: crate::ObjectId, known: Option<&MetadataRef>) {
        let _measurement = self
            .diagnostics
            .barrier_record
            .measure(self.diagnostics_enabled());
        let mut certificate = None;
        let legacy = if let Some(clock) = &self.publication_clock {
            if let Some(snapshot) = clock.snapshot() {
                if (snapshot.flags(0) | snapshot.flags(1)) & 1 == 0 {
                    if clock.unchanged(snapshot) {
                        return;
                    }
                } else {
                    let record = crate::context::Record {
                        address,
                        identity,
                        dirty: false,
                        epochs: std::array::from_fn(|index| {
                            (snapshot.flags(index) & 1 != 0)
                                .then(|| self.domain_at(index).epoch.load(Ordering::Acquire))
                        }),
                    };
                    // Only a concrete birth can have a pending or completed record.
                    // Check it before consulting the address directory.
                    if crate::context::with_state(self, |state| {
                        state.satisfies(&self.contexts, record)
                    }) == Some(true)
                    {
                        if clock.unchanged(snapshot) {
                            return;
                        }
                    } else {
                        certificate = Some((clock, snapshot, record));
                    }
                }
            }
            None
        } else {
            let admission = self.allocation_guard();
            if self.domains.iter().all(|domain| !domain.active()) {
                return;
            }
            Some(admission)
        };
        // A traced edge already owns its original descriptor. An unsafe raw
        // wrapper instead pins its address through the caller's lifetime
        // contract; capture the real identity before any work is deferred.
        let resolved;
        let metadata = if let Some(metadata) = known {
            if !std::ptr::eq(metadata.collector.as_ptr(), self) {
                return;
            }
            metadata
        } else {
            let found = if identity.to_i64() == 0 {
                self.objects.lookup(address)
            } else {
                self.objects.lookup_identity(address, identity)
            };
            let Some(found) = found else { return };
            resolved = found;
            &resolved
        };
        if certificate.is_some_and(|(clock, snapshot, record)| {
            self.edge_already_satisfied(metadata, clock, snapshot, record)
        }) {
            return;
        }
        let publication = legacy.unwrap_or_else(|| self.barrier_guard());
        if self.barrier_inner(metadata, None) {
            publication.changed();
        }
    }

    /// Whether the configured root or barrier policy uses execution contexts.
    #[must_use]
    pub fn execution_contexts_enabled(&self) -> bool {
        self.configuration.root_storage == crate::RootStorage::Contexts
            || self.configuration.barrier_buffer_size != 0
            || self.configuration.local_allocation_buffers
    }

    /// Attach a task-owned root and mutation context. Scoping is explicit; a
    /// suspended context never depends on the OS thread that last polled it.
    #[must_use]
    pub fn register_context(&self) -> crate::ExecutionContext {
        self.contexts.attach(self)
    }

    /// Called under the publication reader. Idle dirty notifications are immediate
    /// so a new young snapshot sees remembered owners even if the context sleeps.
    fn barrier(&self, address: usize, owner: Option<(&MetadataRef, usize)>) -> bool {
        let _measurement = self
            .diagnostics
            .barrier_record
            .measure(self.diagnostics_enabled());
        if let Some((metadata, version)) = owner {
            return self.barrier_inner(metadata, Some(version));
        }
        self.domains.iter().any(Domain::active)
            && self
                .objects
                .lookup(address)
                .is_some_and(|metadata| self.barrier_inner(&metadata, None))
    }

    fn barrier_inner(&self, metadata: &MetadataRef, version: Option<usize>) -> bool {
        if !metadata.birth.is_live() {
            return false;
        }
        if self.configuration.barrier_buffer_size == 0 {
            debug_assert!(
                version.is_none(),
                "an unbuffered owner is enrolled directly"
            );
            return self.shade_live_domains(metadata);
        }
        let owner = version.map(|version| (metadata, version));
        let epochs = std::array::from_fn(|index| {
            let domain = self.domain_at(index);
            domain
                .active()
                .then(|| domain.epoch.load(Ordering::Acquire))
        });
        let record = crate::context::Record {
            address: metadata.trace.ptr.addr(),
            identity: metadata.id,
            epochs,
            dirty: owner.is_some(),
        };
        // After either domain closes strong tracing, publish immediately. This
        // prevents a stream of already-black deletions from continually refilling
        // buffers ahead of reference processing and sweep.
        if self.domains.iter().all(|domain| domain.weak_phase() == 0)
            && epochs.iter().any(Option::is_some)
            && let Some(context) = crate::context::current(self)
        {
            let mut result = context.push(
                &self.contexts,
                record,
                self.configuration.barrier_buffer_size,
            );
            let overflowed = result == crate::context::Push::Full;
            if overflowed {
                if self.diagnostics_enabled() {
                    self.diagnostics
                        .barrier_overflows
                        .fetch_add(1, Ordering::Relaxed);
                }
                // Overflow is ordinary mutator work under admission. Required
                // records keep their reservations when any preparation lock is
                // busy; collector steps can drain them later. Explicit flush()
                // separately retains its synchronous completion contract.
                for _ in 0..crate::context::MAX_RECORDS {
                    if !self.try_apply_buffered_record(&context) {
                        break;
                    }
                }
                // A shared context may refill concurrently. Never spin on overflow.
                result = context.push(
                    &self.contexts,
                    record,
                    self.configuration.barrier_buffer_size,
                );
            }
            let changed = match result {
                crate::context::Push::Full | crate::context::Push::Busy => {
                    self.apply_direct_barrier(record, owner)
                }
                crate::context::Push::Added => {
                    if self.diagnostics_enabled() {
                        self.diagnostics
                            .barrier_buffered
                            .fetch_add(1, Ordering::Relaxed);
                    }
                    true
                }
                crate::context::Push::Coalesced => {
                    if self.diagnostics_enabled() {
                        self.diagnostics
                            .barrier_coalesced
                            .fetch_add(1, Ordering::Relaxed);
                    }
                    false
                }
            };
            // Overflow draining may itself have changed marks/queues. An exact
            // duplicate either owns the original pending reservation or has
            // already established Strong retention in every tagged epoch.
            changed || overflowed
        } else {
            self.apply_direct_barrier(record, owner)
        }
    }

    fn apply_direct_barrier(
        &self,
        record: crate::context::Record,
        owner: Option<(&MetadataRef, usize)>,
    ) -> bool {
        let Some((metadata, version)) = owner else {
            return self.apply_barrier(record);
        };
        // The immediate store already advanced this exact owner's version. All
        // physical retirements revoke its birth while holding the same shard.
        if let Some(remembered) = self.remembered.try_lock(record.address) {
            if !metadata.birth.is_live() {
                return false;
            }
            remembered.insert(record.address, metadata.id);
            drop(remembered);
            self.rescan_dirty_owner(metadata);
        } else {
            self.remember_contended_owner(metadata, Some(version));
        }
        true
    }

    fn buffered_target(&self, record: crate::context::Record) -> Option<MetadataRef> {
        self.objects
            .lookup_identity(record.address, record.identity)
    }

    fn apply_barrier(&self, record: crate::context::Record) -> bool {
        let _retention = TraceRetention::enter(self, Retention::Strong);
        if record.dirty {
            let Some(metadata) = self.buffered_target(record) else {
                return false;
            };
            if self.logically_retired(&metadata) {
                return false;
            }
            if let Some(remembered) = self.remembered.try_lock(record.address) {
                remembered.insert(record.address, metadata.id);
            } else {
                self.remembered.dirty.publish(
                    metadata.id.to_i64().cast_unsigned(),
                    record.address,
                    metadata.mutation.load(Ordering::Acquire),
                );
            }
            self.rescan_buffered_owner(&metadata);
            true
        } else {
            let mut matching =
                record
                    .epochs
                    .into_iter()
                    .enumerate()
                    .filter_map(|(index, epoch)| {
                        let epoch = epoch?;
                        let domain = self.domain_at(index);
                        (domain.active() && domain.epoch.load(Ordering::Acquire) == epoch)
                            .then_some(index)
                    });
            let Some(first) = matching.next() else {
                return false;
            };
            // Publication admission or the buffered record's outstanding debt
            // covers the epoch. Marking needs a visible descriptor, never a
            // payload lease or exclusive access to its address shard.
            let Some(target) = self.buffered_target(record) else {
                return false;
            };
            let mut changed = false;
            for index in std::iter::once(first).chain(matching) {
                let _context = Context::enter(self, index);
                changed |= self.shade_known(&target);
            }
            changed
        }
    }

    pub(crate) fn flush_context(&self, context: &crate::context::State) {
        // Fixed upper bound even when other scopes keep producing into this context.
        for _ in 0..crate::context::MAX_RECORDS {
            if !self.flush_context_record(context) {
                break;
            }
        }
    }

    fn flush_context_record(&self, context: &crate::context::State) -> bool {
        let Some(record) = context.pop(&self.contexts) else {
            return false;
        };
        let _measurement = self
            .diagnostics
            .barrier_drain
            .measure(self.diagnostics_enabled());
        self.apply_buffered_record(context, record);
        true
    }

    fn apply_buffered_record(
        &self,
        context: &crate::context::State,
        record: crate::context::Record,
    ) {
        #[cfg(test)]
        if let Some(observer) = lock(&self.barrier_claim_observer).take() {
            observer.send(()).expect("barrier claim observer");
        }
        #[cfg(all(test, not(target_family = "wasm")))]
        barrier_progress_tests::pause_after_buffer_claim();
        self.apply_barrier(record);
        self.acknowledge_buffered_record(context, record);
    }

    /// Claim only the sampled record. Dirty-owner enrollment prepares its
    /// required indexes first. Clean edges need only the visible descriptor.
    /// A claimed record keeps its pending reservation until its effects publish.
    // Keep this path out of step_inner's code/stack when no context is ready.
    #[inline(never)]
    fn try_apply_buffered_record(&self, context: &crate::context::State) -> bool {
        let Some(record) = context.try_peek() else {
            return false;
        };
        let _measurement = self
            .diagnostics
            .barrier_drain
            .measure(self.diagnostics_enabled());
        #[cfg(test)]
        {
            let pause = record
                .dirty
                .then(|| lock(&self.barrier_prepare_pause).take())
                .flatten();
            if let Some(pause) = pause {
                pause.entered.send(()).expect("barrier prepared");
                pause
                    .resume
                    .recv_timeout(std::time::Duration::from_secs(10))
                    .expect("resume barrier preparation");
            }
        }
        let Some(record) = context.try_pop_matching(&self.contexts, record) else {
            return false;
        };
        self.apply_buffered_record(context, record);
        true
    }

    /// The pending reservation protects this handoff through enqueue. An owner
    /// already marked Strong may have been traced before the buffered write.
    fn rescan_buffered_owner(&self, metadata: &MetadataRef) {
        match self.phase_clock.remembered_stage() {
            Some(0) => self.enqueue_dirty_owner(metadata),
            Some(_) => {
                let _context = Context::enter(self, 0);
                self.shade_metadata(metadata);
            }
            None => {}
        }
    }

    fn acknowledge_buffered_record(
        &self,
        context: &crate::context::State,
        record: crate::context::Record,
    ) {
        if !record.dirty && record.epochs.iter().any(Option::is_some) {
            // A certificate is valid only for the same birth and tagged epochs.
            let strong = self.buffered_target(record).is_some_and(|metadata| {
                record.epochs.iter().enumerate().all(|(index, epoch)| {
                    epoch.is_none_or(|epoch| {
                        (index == 0 && !self.is_young_metadata(&metadata))
                            || metadata.retention(index, epoch) == Some(Retention::Strong)
                    })
                })
            });
            if strong {
                context.completed(record);
            }
        }
        self.contexts.pending.fetch_sub(1, Ordering::AcqRel);
        if self.diagnostics_enabled() {
            self.diagnostics
                .barrier_flushed
                .fetch_add(1, Ordering::Relaxed);
        }
    }

    /// Pending reservations protect popped records through publication. No phase,
    /// cycle, coordinator or claim gate is held while applying or releasing owners.
    fn drain_barrier_step(&self, concurrent: bool) -> Option<bool> {
        let started = now();
        let mut context = self.contexts.try_ready()?;
        let cpu_started = (concurrent && self.diagnostics_enabled())
            .then(crate::diagnostics::thread_cpu_ns)
            .flatten();
        let measurement = if concurrent {
            &self.diagnostics.worker_steps
        } else {
            &self.diagnostics.caller_steps
        }
        .measure_from(self.diagnostics_enabled(), started);
        let mut remaining = self.configuration.incremental_step_size.max(1);
        loop {
            self.try_apply_buffered_record(&context);
            // This can be the final context owner, whose cleanup may wait.
            drop(context);
            remaining -= 1;
            if remaining == 0
                || self.configuration.max_pause_time_us != 0
                    && started.is_some_and(|started| {
                        started.elapsed().as_micros()
                            >= u128::from(self.configuration.max_pause_time_us)
                    })
            {
                break;
            }
            let Some(next) = self.contexts.try_ready() else {
                break;
            };
            context = next;
        }
        self.record_step(
            started,
            cpu_started,
            concurrent,
            &mut WorkCounters::default(),
        );
        drop(measurement);
        self.notify_progress();
        Some(!self.pending())
    }

    pub(crate) fn traced_edge(
        &self,
        address: usize,
        identity: crate::ObjectId,
    ) -> Option<MetadataRef> {
        // Old-only callbacks have an additional cross-generation handoff.
        if self.configuration.old_collection_interval != 0 {
            return None;
        }
        self.objects.lookup(address).filter(|m| m.id == identity)
    }

    /// A traced edge owns inert descriptor metadata only. Retirement invalidates
    /// it before address reuse. Each trace callback still acquires a storage lease.
    pub(crate) fn mark_traced_edge(&self, metadata: &MetadataRef) {
        if std::ptr::eq(metadata.collector.as_ptr(), self) && metadata.birth.is_live() {
            if self.trace_needs_epoch_proposal() {
                self.publish_external_trace(metadata);
            } else {
                self.shade_known(metadata);
            }
        }
    }

    /// Called only with a bounded slice of exact handles. A reserved full scan
    /// pins its epoch. Mature targets have no young-domain marking obligation.
    pub(crate) fn try_mark_mature_slice<'a>(
        &self,
        count: usize,
        edges: impl Iterator<Item = Option<&'a MetadataRef>>,
    ) -> usize {
        let Some((retention, true)) = TraceRetention::current(self) else {
            return 0;
        };
        if self.domain_index() != 1 {
            return 0;
        }
        let full = &self.domains[1];
        let phase = full.weak_phase();
        if phase >= 5 || !full.active() {
            return 0;
        }
        let mature = self.configuration.promotion_age.max(1);
        let epoch = full.epoch.load(Ordering::Acquire);
        let strength = retention.max(Retention::at_phase(phase));
        full.mark_queue.append_mature(
            self.local_queue(),
            count,
            if strength == Retention::Strong {
                epoch
            } else {
                0
            },
            edges,
            |m, known_mature| {
                // A cached group owns the exact eight immutable descriptors and
                // was first admitted by this heap after their last promotion.
                if !known_mature
                    && (!std::ptr::eq(m.collector.as_ptr(), self)
                        || m.age.load(Ordering::Acquire) < mature)
                {
                    return None;
                }
                Some(m.birth.is_live() && m.mark(1, epoch, strength))
            },
        )
    }

    pub(crate) fn mark_object(&self, ptr: SafePtr, identity: crate::ObjectId) {
        if self.trace_needs_epoch_proposal() {
            if self.domain().active()
                && let Some(metadata) = self.objects.lookup(ptr.0)
                && (identity.to_i64() == 0 || metadata.id == identity)
            {
                self.publish_external_trace(&metadata);
            }
            return;
        }
        // With independent young closure, each actual cross-domain publication
        // carries admission; the old callback itself does not park young GC.
        if self.configuration.old_collection_interval == 0 {
            self.shade(ptr.0);
            return;
        }
        let _admission = self.allocation_guard();
        if self.domain().active()
            && let Some(metadata) = self.objects.lookup(ptr.0)
            && (identity.to_i64() == 0 || metadata.id == identity)
        {
            if self.domain_index() == 1 {
                self.retain_old_callback_edge(&metadata);
            }
            self.shade_known(&metadata);
        }
    }

    fn defer_retirement(&self, metadata: MetadataRef) {
        self.retired_objects.fetch_add(1, Ordering::AcqRel);
        if metadata.arm_retirement_notification() {
            // The borrower left before arming. This sweep step already drives
            // worker notification after releasing its coordination guards.
            self.queue_released_storage(metadata);
        }
    }

    #[cfg(all(test, not(target_family = "wasm"), not(miri)))]
    pub(crate) fn with_parked_descriptor_registry(&self, work: impl FnOnce()) {
        if let Some(heap) = &self.descriptor_regions {
            heap.with_parked_registry(work);
        }
    }

    #[cfg(all(test, not(target_family = "wasm"), not(miri)))]
    pub(crate) fn has_released_donor_jobs(&self) -> bool {
        self.released_donors.has_jobs()
    }

    fn queue_released_storage(&self, metadata: MetadataRef) {
        self.retired_ready.fetch_add(1, Ordering::AcqRel);
        if metadata.can_transfer_retired_charge() {
            self.released_donors.publish(metadata);
        } else {
            self.publish_retirement(ReclaimTask::new(metadata));
        }
    }

    /// Publish a continuation before releasing the current callback's ready count.
    fn queue_clearing(&self, task: ReclaimTask) {
        self.retired_objects.fetch_add(1, Ordering::AcqRel);
        self.retired_ready.fetch_add(1, Ordering::AcqRel);
        self.publish_retirement(task);
    }

    fn publish_retirement(&self, task: ReclaimTask) {
        if let Some(queue) = self.retired.load().as_ref() {
            queue.push(task);
            return;
        }
        // Allocate only on the first deferred handoff. A preempted initializer
        // owns private storage and cannot prevent another producer installing a queue.
        let queue = Arc::new(crate::publication_queue::Queue::new());
        let previous = self.retired.compare_and_swap(
            std::ptr::null::<crate::publication_queue::Queue<ReclaimTask>>(),
            Some(queue.clone()),
        );
        previous.as_ref().unwrap_or(&queue).push(task);
    }

    /// The caller owns one ready reservation and the retired byte charge. Once a
    /// callback unwinds, its partially cleared payload can only be destroyed.
    fn clear_retired(
        &self,
        task: &mut ReclaimTask,
        budget: usize,
        counters: &mut WorkCounters,
        selection: Option<crate::retirement_grants::Selection>,
    ) -> TraceProgress {
        use std::panic::{AssertUnwindSafe, catch_unwind, resume_unwind};
        let metadata = &mut task.metadata;
        let callback_panic = if let ReclaimStage::Clear { offset, finalized } = task.stage {
            debug_assert!(metadata.storage_ready());
            match catch_unwind(AssertUnwindSafe(|| {
                if !finalized {
                    metadata.run_finalizer();
                }
                metadata.trace.clear_step(offset, budget)
            })) {
                Ok(TraceProgress::Complete) => None,
                Ok(progress) => return progress,
                Err(panic) => Some(panic),
            }
        } else {
            None
        };
        let destruction = catch_unwind(AssertUnwindSafe(|| match &mut task.stage {
            ReclaimStage::Clear { .. } => {
                if let Some(region) = ObjectMetadata::destroy_retired(metadata, self, selection) {
                    task.stage = ReclaimStage::Return(region);
                    false
                } else {
                    true
                }
            }
            ReclaimStage::Return(region) => region.finish(self),
        }));
        if matches!(destruction, Ok(false)) {
            if let Some(panic) = callback_panic {
                // Preserve sole ownership of deferred backing before propagating
                // the callback panic. The queued job accounts for its own refund.
                let mut next = ReclaimTask::new(metadata.clone());
                std::mem::swap(&mut next.stage, &mut task.stage);
                self.queue_clearing(next);
                self.retired_ready.fetch_sub(1, Ordering::AcqRel);
                self.wake();
                resume_unwind(panic);
            }
            return TraceProgress::Busy;
        }
        self.retired_bytes
            .fetch_sub(metadata.size(), Ordering::AcqRel);
        counters.swept += 1;
        counters.freed += metadata.size();
        counters.logical_freed += metadata.logical_size();
        if let Some(panic) = callback_panic.or(destruction.err()) {
            self.retired_ready.fetch_sub(1, Ordering::AcqRel);
            counters.flush(&mut lock(&self.statistics));
            self.wake();
            resume_unwind(panic);
        }
        TraceProgress::Complete
    }

    #[cfg(not(target_family = "wasm"))]
    fn cleanup_pool(&self, metadata: &ObjectMetadata) -> Option<Arc<WorkerSignals>> {
        if !metadata.has_cleanup()
            || WORKER
                .get()
                .is_none_or(|(heap, _)| heap != std::ptr::from_ref(self).addr())
        {
            return None;
        }
        native_callbacks::pool().filter(|pool| !pool.closed.load(Ordering::Acquire))
    }

    #[cfg(not(target_family = "wasm"))]
    fn offload_cleanup(
        &self,
        task: ReclaimTask,
        budget: usize,
        origin: Option<(usize, u64)>,
    ) -> Option<ReclaimTask> {
        let Some(pool) = self.cleanup_pool(&task.metadata) else {
            return Some(task);
        };
        let batch = native_callbacks::Batch {
            first: (task, budget),
            rest: Vec::new(),
        };
        pool.callbacks
            .send(self, batch, origin)
            .map(|batch| batch.first.0)
    }

    /// Execute a sweep callback with exclusive descriptor ownership and no cycle,
    /// coordinator, or publication lock. A continuation can return to its original
    /// sweep while that exact epoch has space in its bounded continuation queue.
    fn reclaim_swept(
        &self,
        mut task: ReclaimTask,
        index: usize,
        epoch: u64,
        budget: usize,
        counters: &mut WorkCounters,
        selection: Option<crate::retirement_grants::Selection>,
    ) {
        #[cfg(not(target_family = "wasm"))]
        if selection.is_none() {
            let Some(owned) = self.offload_cleanup(task, budget, Some((index, epoch))) else {
                return;
            };
            task = owned;
        }
        let _cleanup = task
            .metadata
            .has_cleanup()
            .then(|| CleanupScope::enter(self));
        let progress = self.clear_retired(&mut task, budget, counters, selection);
        if progress == TraceProgress::Complete {
            self.record_sweep_reclaimed(index, epoch, task.metadata.size());
        } else {
            task.advance(progress);
            if progress != TraceProgress::Busy
                && let Ok(mut cycle) = self.domain_at(index).cycle.try_lock()
                && cycle.epoch == epoch
                && cycle.phase == Phase::Sweep
                && cycle.reclaim.len() < phase_work::SWEEP_BATCH
            {
                cycle.reclaim.push_back(task);
            } else {
                self.queue_clearing(task);
            }
        }
        self.retired_ready.fetch_sub(1, Ordering::AcqRel);
    }

    fn record_sweep_reclaimed(&self, index: usize, epoch: u64, bytes: usize) {
        if let Some(scheduler) = &self.scheduler
            && let Ok(mut cycle) = self.domain_at(index).cycle.try_lock()
            && cycle.epoch == epoch
            && cycle.kind == CollectionKind::Full
            && cycle.phase != Phase::Idle
        {
            if cycle.reclaimed_bytes == 0 {
                scheduler.reclaimed();
            }
            cycle.reclaimed_bytes = cycle.reclaimed_bytes.saturating_add(bytes);
        }
    }

    pub(crate) fn storage_ready(&self, metadata: MetadataRef) {
        self.queue_released_storage(metadata);
        self.wake();
    }

    /// One bounded clearing unit, excluding unrelated leased allocations. Ready
    /// entries stay charged through callbacks and continuation publication.
    fn reap_retired(
        &self,
        concurrent: bool,
        check: Option<&mut crate::retirement_grants::ReleasedCheck>,
    ) -> Option<RetirementStep> {
        // Popping transfers exclusive descriptor ownership to this call. It has
        // already been removed from every generation's registry, so neither the
        // coordinator nor a generation cursor protects this callback's storage.
        let started = now();
        let cpu_started = (concurrent && self.diagnostics_enabled())
            .then(crate::diagnostics::thread_cpu_ns)
            .flatten();
        let timing = if concurrent {
            &self.diagnostics.worker_steps
        } else {
            &self.diagnostics.caller_steps
        };
        let budget = self.configuration.incremental_step_size.clamp(1, 32);
        let queued = || self.retired.load().as_ref()?.pop().map(|task| (task, None));
        let prefer_queue = check.is_none() && self.released_turn.fetch_xor(true, Ordering::Relaxed);
        let mut task = if prefer_queue { queued() } else { None };
        let mut counters = WorkCounters::default();
        if task.is_none() {
            let released = match check {
                Some(check) => {
                    Some(check.step(&self.released_donors, &self.grant_registry, budget)?)
                }
                None => self.released_donors.help(&self.grant_registry, budget),
            };
            match released {
                Some(crate::retirement_grants::ReleasedWork::Progress) => {
                    self.record_step(started, cpu_started, concurrent, &mut counters);
                    drop(timing.measure_from(self.diagnostics_enabled(), started));
                    self.notify_progress();
                    return Some(RetirementStep::Progress(false));
                }
                Some(crate::retirement_grants::ReleasedWork::Claimed(metadata, selection)) => {
                    task = Some((ReclaimTask::new(metadata), Some(selection)));
                }
                None => task = queued(),
            }
        }
        let (mut task, selection) = task?;
        let measurement = timing.measure_from(self.diagnostics_enabled(), started);
        self.retired_objects.fetch_sub(1, Ordering::AcqRel);
        let budget = budget.min(task.metadata.trace.budget);
        #[cfg(not(target_family = "wasm"))]
        if selection.is_none() {
            let Some(owned) = self.offload_cleanup(task, budget, None) else {
                return Some(RetirementStep::Progress(false));
            };
            task = owned;
        }
        let _cleanup = task
            .metadata
            .has_cleanup()
            .then(|| CleanupScope::enter(self));
        let progress = self.clear_retired(&mut task, budget, &mut counters, selection);
        let contended = progress == TraceProgress::Busy;
        if progress == TraceProgress::Complete {
            self.retired_ready.fetch_sub(1, Ordering::AcqRel);
        } else {
            task.advance(progress);
            self.retired_objects.fetch_add(1, Ordering::AcqRel);
            self.publish_retirement(task);
        }
        self.record_step(started, cpu_started, concurrent, &mut counters);
        drop(measurement);
        self.notify_progress();
        Some(if contended {
            RetirementStep::Contended
        } else {
            RetirementStep::Progress(!self.pending())
        })
    }

    /// Performs at most one configured batch. Returns true when no collection is pending.
    ///
    /// The work budget bounds visits, not arbitrary user `Trace` or destructor code. Implement
    /// `Trace::trace_step` for large objects and avoid blocking in tracing/destruction.
    pub fn step(&self) -> bool {
        self.step_inner(false).unwrap_or(false)
    }

    fn claim_mark_batch(&self, claim: &mut crate::work::Claim, remaining: &mut usize) -> usize {
        self.domain().mark_queue.claim_into(
            self.local_queue(),
            self.configuration.mark_batch_size,
            remaining,
            claim,
        );
        #[cfg(test)]
        if claim.count() != 0 {
            let pause = lock(&self.mark_claim_pause).take();
            if let Some(pause) = pause {
                pause.entered.send(()).expect("claim entered");
                pause
                    .resume
                    .recv_timeout(std::time::Duration::from_secs(10))
                    .expect("resume claim");
            }
        }
        if self.diagnostics_enabled() {
            if claim.stolen != 0 {
                self.diagnostics
                    .mark_stolen
                    .fetch_add(claim.stolen as u64, Ordering::Relaxed);
            }
            if claim.misses != 0 {
                self.diagnostics
                    .mark_queue_misses
                    .fetch_add(claim.misses as u64, Ordering::Relaxed);
            }
        }
        claim.count()
    }

    /// A successful independent step never reacquires the coordinator. Reservations
    /// remain outstanding until all continuations and statistics have been published.
    #[cfg(test)]
    fn independent_mark_step(&self, concurrent: bool) -> Option<bool> {
        self.claim_independent_mark(concurrent, &mut false)
    }

    fn claim_independent_mark(&self, concurrent: bool, reserved_miss: &mut bool) -> Option<bool> {
        // A new publication can race this hint; the coordinated fallback and
        // the next step still inspect the authoritative phase and work queues.
        if self
            .domains
            .iter()
            .all(|domain| !domain.trace_ready.load(Ordering::Acquire))
        {
            return None;
        }
        let gate = self.mark_claims.as_ref()?;
        let started = now();
        let Ok(claim) = gate.try_read() else {
            if self.diagnostics_enabled() {
                self.diagnostics
                    .mark_gate_misses
                    .fetch_add(1, Ordering::Relaxed);
            }
            return None;
        };
        // Phase enumeration and newly requested generations need coordinator turns,
        // even when another generation always has a resumable tracing task ready.
        if self.old_requested.load(Ordering::Acquire) && !self.domains[1].active()
            || self.domains.iter().any(|domain| {
                let active = domain.active();
                (!active && domain.requested.load(Ordering::Acquire))
                    || (active
                        && !domain.trace_ready.load(Ordering::Acquire)
                        && domain.weak_phase() == 0)
            })
        {
            return None;
        }
        // A failed fast claim must not advance this hint: the fallback needs to
        // close the empty generation, not alternate back to the busy one again.
        let preferred = self.next_domain.load(Ordering::Relaxed) % 2;
        let terminal =
            self.domains.iter().enumerate().position(|(index, domain)| {
                self.terminal_priority(index) && domain.weak_phase() != 0
            });
        let index = terminal.unwrap_or_else(|| {
            if self
                .domain_at(preferred)
                .trace_ready
                .load(Ordering::Acquire)
            {
                preferred
            } else {
                1 - preferred
            }
        });
        if !self.domain_at(index).trace_ready.load(Ordering::Acquire) {
            return None;
        }
        let context = Context::enter(self, index);
        let mut remaining = self.configuration.incremental_step_size.max(1);
        let mut tasks = crate::work::Claim::default();
        let cpu_started = (concurrent && self.diagnostics_enabled())
            .then(crate::diagnostics::thread_cpu_ns)
            .flatten();
        let count = self.claim_mark_batch(&mut tasks, &mut remaining);
        if count == 0 {
            // There is already a live reservation in precisely the generation
            // we just probed. A coordinator cannot close it. A new request or
            // published continuation is reconsidered by the next helper call.
            *reserved_miss =
                self.domain().in_flight.count() != 0 && !self.domain_has_work(1 - index);
            return None;
        }
        self.next_domain.fetch_add(1, Ordering::Relaxed);
        self.domain().in_flight.reserve(count);
        let mut reserved = count;
        drop(claim);
        let _measurement = if concurrent {
            &self.diagnostics.worker_steps
        } else {
            &self.diagnostics.caller_steps
        }
        .measure_from(self.diagnostics_enabled(), started);
        let mut counters = WorkCounters::default();
        loop {
            let stalled = self.trace_mark_batch(&mut tasks, started, &mut counters);
            if stalled || remaining == 0 {
                break;
            }
            if let Some(started) = started
                && self.configuration.max_pause_time_us != 0
                && started.elapsed().as_micros() >= u128::from(self.configuration.max_pause_time_us)
            {
                break;
            }
            tasks.stolen = 0;
            tasks.misses = 0;
            // A reservation from this step is still outstanding. The interlock means neither generation can
            // close marking, so further claims need no phase lock or epoch recheck.
            let count = self.claim_mark_batch(&mut tasks, &mut remaining);
            if count == 0 {
                break;
            }
            self.domain().in_flight.reserve(count);
            reserved += count;
        }
        if self.diagnostics_enabled() {
            self.diagnostics
                .independent_mark_steps
                .fetch_add(1, Ordering::Relaxed);
        }
        self.record_step(started, cpu_started, concurrent, &mut counters);
        // Release last: completion tickets must never precede this step's counters.
        self.domain().in_flight.release(reserved);
        drop(context);
        self.notify_progress();
        Some(!self.pending())
    }

    fn step_inner(&self, concurrent: bool) -> Option<bool> {
        if self.remembered.dirty.cleanup_pending() {
            self.remembered.dirty.cleanup_step(|id, address| {
                self.objects
                    .lookup(address)
                    .is_some_and(|metadata| metadata.id.to_i64().cast_unsigned() == id)
            });
        }
        let global_repair = self
            .scanned_slots()
            .is_some_and(crate::roots::RootSlots::repair_step);
        let context_repair = self.contexts.repairs.step();
        if (global_repair || context_repair)
            && self.retired_ready.load(Ordering::Acquire) == 0
            && !self.released_donors.has_jobs()
            && !self.domain_has_work(0)
            && !self.domain_has_work(1)
            && self.contexts.pending.load(Ordering::Acquire) == 0
        {
            return Some(!self.pending());
        }
        // A delayed borrower cannot hold a collection ticket open. Once released,
        // give its bounded clearing work alternating turns with active cycles.
        if ((self.retired_objects.load(Ordering::Acquire) != 0
            && self.retired_ready.load(Ordering::Acquire) != 0)
            || self.released_donors.has_jobs())
            && (!self.retirement_turn.fetch_xor(true, Ordering::Relaxed)
                || !self
                    .domains
                    .iter()
                    .any(|domain| domain.active() || domain.requested.load(Ordering::Acquire)))
            && let Some(step) = self.reap_retired(concurrent, None)
        {
            return match step {
                RetirementStep::Progress(done) => Some(done),
                RetirementStep::Contended => None,
            };
        }
        // Alternate drains with tracing/phase work so a replenished buffer cannot
        // starve marking. Busy claims rotate without dropping their reservation.
        if self.contexts.pending.load(Ordering::Acquire) != 0
            && !self.barrier_turn.fetch_xor(true, Ordering::Relaxed)
            && let Some(progress) = self.drain_barrier_step(concurrent)
        {
            return Some(progress);
        }
        let mut waiting = false;
        let result = if let Some(progress) = self.claim_independent_mark(concurrent, &mut waiting) {
            Some(progress)
        } else if waiting {
            None
        } else {
            self.independent_relocation_step(concurrent).or_else(|| {
                match self.independent_phase_step(concurrent) {
                    Ok(Some(progress)) => Some(progress),
                    Err(index) if !self.domain_has_work(1 - index) => None,
                    _ => self.coordinated_step(concurrent),
                }
            })
        };
        // A producer may park after linking while still holding the coordinator.
        // Another ordinary worker or caller can deliver that visible notification.
        if result.is_none() && self.has_pending_references() {
            self.notify_progress();
        }
        result
    }

    fn next_relocation(&self, cycle: &mut Cycle) -> crate::index::Scan<MetadataRef> {
        if self.domain_index() != 0 {
            return match self
                .objects
                .scan(&mut cycle.object_cursor, MetadataRef::clone)
            {
                crate::index::Scan::Entry(_, metadata)
                    if self.old_excludes_young(1, &metadata)
                        || self.configuration.overlapping_generations
                            && metadata.retention(1, cycle.epoch).is_none() =>
                {
                    crate::index::Scan::Advance
                }
                next => next,
            };
        }
        let address = match self.objects.scan_young(&mut cycle.young_cursor) {
            crate::index::Scan::Entry(address, ()) => address,
            crate::index::Scan::Advance => return crate::index::Scan::Advance,
            crate::index::Scan::Busy => return crate::index::Scan::Busy,
            crate::index::Scan::Done => return crate::index::Scan::Done,
        };
        let Some(objects) = self.objects.try_lock(address) else {
            cycle.young_cursor.retry_entry();
            return crate::index::Scan::Busy;
        };
        objects
            .get(&address)
            .filter(|metadata| self.is_young_metadata(metadata))
            .cloned()
            .map_or(crate::index::Scan::Advance, |metadata| {
                crate::index::Scan::Entry(address, metadata)
            })
    }

    fn relocate(
        &self,
        metadata: &MetadataRef,
        heap: &crate::region::RegionHeap,
        attempts: usize,
    ) -> bool {
        if self.old_excludes_young(self.domain_index(), metadata)
            || self.configuration.overlapping_generations
                && (self.logically_retired(metadata)
                    || (self.domain_index() == 1
                        && metadata
                            .retention(1, self.domains[1].epoch.load(Ordering::Acquire))
                            .is_none()))
        {
            return false;
        }
        // Old-only copying excludes remembered scans on this descriptor.
        let Ok(_lease) = ObjectMetadata::borrow_callback(metadata, self.old_collection_active())
        else {
            return false;
        };
        let bytes = self.configuration.relocation_chunk_bytes;
        let limit = metadata
            .size()
            .div_ceil(bytes.max(1))
            .saturating_mul(4)
            .max(4);
        let progress = metadata.trace.compact_guarded(heap, bytes);
        if let Some(history) = &metadata.relocation {
            history.observe(
                progress,
                metadata.trace.ptr,
                heap,
                self.diagnostics_enabled()
                    .then_some(&self.diagnostics.relocation_pin_deferral),
            );
        }
        if self.diagnostics_enabled() {
            self.diagnostics
                .relocation_tasks
                .fetch_add(1, Ordering::Relaxed);
            if progress == TraceProgress::Busy {
                self.diagnostics
                    .relocation_busy
                    .fetch_add(1, Ordering::Relaxed);
            }
        }
        if attempts.saturating_add(1) >= limit && progress != TraceProgress::Complete {
            if self.diagnostics_enabled() {
                self.diagnostics
                    .relocation_retry_limits
                    .fetch_add(1, Ordering::Relaxed);
            }
            let cancelled = metadata.trace.compact_guarded(heap, 0);
            if cancelled == TraceProgress::Busy && self.diagnostics_enabled() {
                self.diagnostics
                    .relocation_cancel_busy
                    .fetch_add(1, Ordering::Relaxed);
            }
        }
        matches!(progress, TraceProgress::Continue(_)) && attempts.saturating_add(1) < limit
    }

    fn relocation_domain(&self) -> Option<usize> {
        if self.configuration.overlapping_generations {
            let preferred = self.next_domain.load(Ordering::Relaxed) % 2;
            let young = &self.domains[0];
            if preferred == 0
                && (young.active() || young.requested.load(Ordering::Acquire))
                && !young.relocation_ready.load(Ordering::Acquire)
            {
                // Roots, mark closure and reference/sweep units need coordinator
                // turns even while the full generation has a persistent copy queue.
                return None;
            }
            if self
                .domain_at(preferred)
                .relocation_ready
                .load(Ordering::Acquire)
            {
                Some(preferred)
            } else if self
                .domain_at(1 - preferred)
                .relocation_ready
                .load(Ordering::Acquire)
            {
                Some(1 - preferred)
            } else {
                None
            }
        } else {
            self.domains
                .iter()
                .position(|domain| domain.relocation_ready.load(Ordering::Acquire))
        }
    }

    fn independent_relocation_step(&self, concurrent: bool) -> Option<bool> {
        let heap = self.regions.as_ref()?;
        let gate = self.relocation_claims.as_ref()?;
        let started = now();
        let claim = gate.try_read().ok()?;
        let index = self.relocation_domain()?;
        let context = Context::enter(self, index);
        let queues = self.domain().relocations.as_ref()?;
        let (task, stolen, misses) = queues.claim(self.local_queue());
        if self.diagnostics_enabled() {
            self.diagnostics
                .relocation_stolen
                .fetch_add(stolen as u64, Ordering::Relaxed);
            self.diagnostics
                .relocation_queue_misses
                .fetch_add(misses as u64, Ordering::Relaxed);
        }
        let mut task = task?;
        if self.configuration.overlapping_generations {
            self.next_domain.fetch_add(1, Ordering::Relaxed);
        }
        self.domain().in_flight.reserve(1);
        drop(claim);
        let cpu_started = (concurrent && self.diagnostics_enabled())
            .then(crate::diagnostics::thread_cpu_ns)
            .flatten();
        let measurement = if concurrent {
            &self.diagnostics.worker_steps
        } else {
            &self.diagnostics.caller_steps
        }
        .measure_from(self.diagnostics_enabled(), started);
        // Keep the reservation through the whole batch, including subsequent
        // dequeue/publish gaps. Closure cannot retire this generation while
        // the batch owns work, even after we release the claim reader.
        for work in 0..self.configuration.incremental_step_size.max(1) {
            if self.relocate(&task.metadata, heap, task.attempts) {
                queues.push(
                    self.local_queue(),
                    crate::relocation::Task {
                        metadata: task.metadata,
                        attempts: task.attempts.saturating_add(1),
                    },
                );
            }
            if work + 1 == self.configuration.incremental_step_size.max(1)
                || (self.configuration.max_pause_time_us != 0
                    && started.is_some_and(|started| {
                        started.elapsed().as_micros()
                            >= u128::from(self.configuration.max_pause_time_us)
                    }))
            {
                break;
            }
            let (next, stolen, misses) = queues.claim(self.local_queue());
            if self.diagnostics_enabled() {
                self.diagnostics
                    .relocation_stolen
                    .fetch_add(stolen as u64, Ordering::Relaxed);
                self.diagnostics
                    .relocation_queue_misses
                    .fetch_add(misses as u64, Ordering::Relaxed);
            }
            let Some(next) = next else { break };
            task = next;
        }
        if self.diagnostics_enabled() {
            self.diagnostics
                .independent_relocation_steps
                .fetch_add(1, Ordering::Relaxed);
        }
        self.record_step(
            started,
            cpu_started,
            concurrent,
            &mut WorkCounters::default(),
        );
        self.domain().in_flight.release(1);
        drop(measurement);
        drop(context);
        self.notify_progress();
        Some(!self.pending())
    }

    /// Finish each claim directly from its queue item. No callback runs under
    /// a phase lock, and the existing reservation covers every publication.
    #[expect(
        clippy::too_many_lines,
        reason = "packet and scalar callbacks share a single claim and tracing budget"
    )]
    fn trace_mark_batch(
        &self,
        claim: &mut crate::work::Claim,
        started: Option<std::time::Instant>,
        counters: &mut WorkCounters,
    ) -> bool {
        let index = self.domain_index();
        let domain = self.domain_at(index);
        let epoch = domain.epoch.load(Ordering::Acquire);
        let lane = self.local_queue();
        let ordinary = !self.configuration.overlapping_generations
            && self.configuration.old_collection_interval == 0;
        let full = (ordinary && index == 1).then_some(ReservedFullScan {
            epoch,
            mature: self.configuration.promotion_age.max(1),
        });
        let may_prune = self.remembered.may_prune();
        let mut executed = 0_u64;
        let mut deferred = 0_u64;
        let mut busy = false;
        let verified = full.is_some() && claim.strong_packet(epoch);
        let expired = || {
            self.configuration.max_pause_time_us != 0
                && started.is_some_and(|time| {
                    time.elapsed().as_micros() >= u128::from(self.configuration.max_pause_time_us)
                })
        };
        if verified
            && !may_prune
            && let Some((packet, last_budget)) = claim.take_packet()
        {
            let _young = ScanYoung::enter(false);
            let _strong = TraceRetention::callback(self, Retention::Strong);
            packet.for_each(last_budget, |metadata, budget| {
                if busy || deferred != 0 || (executed != 0 && expired()) {
                    deferred += 1;
                    domain
                        .mark_queue
                        .push(lane, MarkItem::fresh(metadata.clone()));
                    return;
                }
                // Verified packets contain only fresh work. Dirty state, offset and
                // remembered-version lookup become relevant only on a continuation.
                let progress =
                    ObjectMetadata::trace_ordinary(metadata, self, full.as_ref(), true, 0, budget);
                executed += 1;
                if progress == TraceProgress::Complete {
                    counters.marked += 1;
                } else {
                    let mut next = MarkItem::fresh(metadata.clone());
                    next.version = Some(usize::MAX);
                    next.has_young = SCAN_YOUNG.get();
                    if let TraceProgress::Continue(offset) = progress {
                        next.offset = offset;
                    }
                    busy = progress == TraceProgress::Busy;
                    domain.mark_queue.push(lane, next);
                }
            });
        } else {
            claim.for_each(|mut item, budget| {
                if busy || deferred != 0 || (executed != 0 && expired()) {
                    deferred += 1;
                    domain.mark_queue.push(lane, item.owned());
                    return;
                }
                debug_assert!(
                    index != 1
                        || !self.old_collection_active()
                        || item.metadata.age.load(Ordering::Acquire)
                            >= self.configuration.promotion_age.max(1)
                );
                if item.dirty_scan && item.version.is_none() {
                    item.metadata.begin_dirty_scan();
                }
                let version = item.version.unwrap_or_else(|| {
                    if may_prune {
                        item.metadata.mutation.load(Ordering::Acquire)
                    } else {
                        usize::MAX
                    }
                });
                let progress = if item.metadata.trace.step_fn.is_some() {
                    let _young = ScanYoung::enter(item.has_young);
                    let _retention = {
                        let strength = if verified {
                            Retention::Strong
                        } else {
                            self.visible_retention(&item.metadata, index, epoch)
                                .unwrap_or(Retention::Phantom)
                        };
                        TraceRetention::callback(self, strength)
                    };
                    let result = if ordinary {
                        ObjectMetadata::trace_ordinary(
                            &item.metadata,
                            self,
                            full.as_ref(),
                            verified,
                            item.offset,
                            budget,
                        )
                    } else {
                        ObjectMetadata::trace_step(&item.metadata, self, item.offset, budget)
                    };
                    item.has_young = SCAN_YOUNG.get();
                    result
                } else {
                    TraceProgress::Complete
                };
                executed += 1;
                if progress == TraceProgress::Complete {
                    counters.marked += 1;
                    if version != usize::MAX {
                        self.prune_remembered(
                            item.metadata.ptr().0,
                            item.metadata.id,
                            version,
                            item.has_young,
                        );
                    }
                    if item.dirty_scan && item.metadata.finish_dirty_scan() {
                        domain
                            .mark_queue
                            .push(lane, MarkItem::dirty(item.metadata.into_owned()));
                    }
                } else {
                    if let TraceProgress::Continue(offset) = progress {
                        item.offset = offset;
                    }
                    busy = progress == TraceProgress::Busy;
                    item.version = Some(version);
                    domain.mark_queue.push(lane, item.owned());
                }
            });
        }
        if self.diagnostics_enabled() {
            self.diagnostics
                .mark_batches
                .fetch_add(1, Ordering::Relaxed);
            self.diagnostics
                .trace_tasks
                .fetch_add(executed, Ordering::Relaxed);
            if index == 1 && self.old_collection_active() {
                self.diagnostics
                    .old_trace_tasks
                    .fetch_add(executed, Ordering::Relaxed);
            }
            if deferred != 0 {
                self.diagnostics
                    .trace_deferred
                    .fetch_add(deferred, Ordering::Relaxed);
            }
            if busy {
                self.diagnostics.trace_busy.fetch_add(1, Ordering::Relaxed);
            }
        }
        busy || deferred != 0
    }

    #[expect(clippy::too_many_lines, reason = "resumable phases share one budget")]
    fn coordinated_step(&self, concurrent: bool) -> Option<bool> {
        let started = now();
        let Ok(coordinator) = self.coordinator.try_lock() else {
            if self.diagnostics_enabled() {
                self.diagnostics
                    .coordinator_misses
                    .fetch_add(1, Ordering::Relaxed);
            }
            return None;
        };
        let coordinator = Some(coordinator);
        let _measurement = if concurrent {
            &self.diagnostics.worker_steps
        } else {
            &self.diagnostics.caller_steps
        }
        .measure(self.diagnostics_enabled());
        let cpu_started = (concurrent && self.diagnostics_enabled())
            .then(crate::diagnostics::thread_cpu_ns)
            .flatten();
        let preferred = self.next_domain.fetch_add(1, Ordering::Relaxed) % 2;
        let terminal =
            self.domains.iter().enumerate().position(|(index, domain)| {
                self.terminal_priority(index) && domain.weak_phase() != 0
            });
        let index = terminal.unwrap_or_else(|| {
            if self.domain_at(preferred).active()
                || self.domain_at(preferred).requested.load(Ordering::Acquire)
                || (preferred == 1 && self.old_requested.load(Ordering::Acquire))
            {
                preferred
            } else {
                1 - preferred
            }
        });
        let context = Context::enter(self, index);
        let mut cycle = self.domain().cycle.try_lock().ok()?;
        if cycle.phase == Phase::Idle {
            // Drain an earlier young snapshot before starting a requested full
            // pass. Otherwise its floating roots can be imported into successive
            // full passes after every mutator has already dropped them. New young
            // work yields to that pending start, without suspending mutators.
            // Old-only handoffs additionally exclude young starts throughout
            // precise full tracing because their young retention is conservative.
            if index == 0
                && ((self.domains[1].requested.load(Ordering::Acquire)
                    && !self.old_collection_active())
                    || (self.configuration.old_collection_interval != 0
                        && self.domains[1].active()
                        && !self.old_collection_active()
                        && self.domains[1].weak_phase() < 5))
                || index == 1
                    && self.domain().requested.load(Ordering::Acquire)
                    && self.domains[0].active()
            {
                return Some(false);
            }
        }
        if cycle.phase == Phase::Idle
            && (self.domain().requested.load(Ordering::Acquire)
                || (index == 1 && self.old_requested.load(Ordering::Acquire)))
            && self
                .domain()
                .epoch
                .load(Ordering::Acquire)
                .checked_add(crate::metadata::EPOCH_STRIDE)
                .is_none()
        {
            // Keep this domain inactive and its request pending while resetting
            // one metadata entry per work unit. Publication initializes unmarked
            // storage; neither domain may shade an inactive domain. No old epoch
            // can alias epoch one after the reset finishes.
            self.contexts
                .completed_disabled
                .store(true, Ordering::Release);
            cycle.phase = Phase::ResetMarks;
            cycle.object_cursor = crate::index::Cursor::default();
        }
        if cycle.phase == Phase::Idle {
            let Some(publication) = self.try_publication_write() else {
                return Some(false);
            };
            let Some(observation) = self.observe_phase() else {
                return Some(false);
            };
            let full_request = self.domain().requested.swap(false, Ordering::AcqRel);
            let old_request = index == 1 && self.old_requested.swap(false, Ordering::AcqRel);
            if !full_request && !old_request {
                // An in-flight destructor keeps pending() true, even after its
                // cycle ends. This worker did not claim that task or a request.
                return (!self.pending()).then_some(true);
            }
            let full = index == 1;
            // Capture before publishing any part of the snapshot. A request
            // arriving after this cut leaves the flag set for the next start.
            let ticket = self.domain().request_ticket.load(Ordering::Acquire);
            cycle.kind = if full && !full_request {
                CollectionKind::Old
            } else if full {
                CollectionKind::Full
            } else {
                CollectionKind::Young
            };
            if full {
                self.old_active
                    .store(cycle.kind == CollectionKind::Old, Ordering::Release);
            }
            cycle.epoch =
                self.domain().epoch.load(Ordering::Acquire) + crate::metadata::EPOCH_STRIDE;
            self.domain().epoch.store(cycle.epoch, Ordering::Release);
            self.domain()
                .mark_queue
                .open_graph_inbox(cycle.epoch, cycle.kind == CollectionKind::Old);
            if !self.commit_phase(observation, true, true, 0) {
                self.domain().mark_queue.close_graph_inbox();
                if cycle.kind == CollectionKind::Old {
                    self.old_requested.store(true, Ordering::Release);
                } else {
                    self.domain().publish_request();
                }
                return Some(false);
            }
            #[cfg(all(test, not(target_family = "wasm"), not(miri)))]
            phase_tests::after_snapshot_activation();
            if index == 1 {
                self.mature_census.strong.store(0, Ordering::Release);
                self.domain().mark_queue.reset_mature_strong();
                cycle.mature_coverage = MatureCoverage::Unchecked;
                cycle.packet_cleanup = None;
            }
            cycle.phase = if cycle.kind == CollectionKind::Old {
                Phase::OldHandoff
            } else if self.configuration.root_storage == crate::RootStorage::Contexts
                || self.configuration.local_allocation_buffers
            {
                Phase::ContextRoots
            } else if self.scanned_slots().is_some() {
                Phase::RootSlots
            } else {
                Phase::Roots
            };
            cycle.root_cursor = crate::roots::Cursor::default();
            cycle.context_cursor = crate::context::RootCursor::default();
            cycle.cursor = 0;
            cycle.reference_stage = 0;
            cycle.ticket = ticket;
            cycle.relocations.clear();
            cycle.relocation_scan_done = false;
            debug_assert!(cycle.phantom_retained.is_empty());
            cycle.young_cursor = crate::index::YoungCursor::default();
            cycle.remembered_cursor = crate::index::RememberedCursor::default();
            cycle.remembered_overflow = if cycle.kind == CollectionKind::Young {
                self.remembered.dirty.cursor()
            } else {
                crate::index::RememberedOverflow::default()
            };
            cycle.sweep_young_cursor = crate::index::YoungCursor::default();
            cycle.object_cursor = crate::index::Cursor::default();
            cycle.sweep_object_cursor = crate::index::Cursor::default();
            cycle.young_retirement = crate::index::YoungRetirement::default();
            #[cfg(feature = "experimental-identity-index")]
            {
                cycle.identity_removals =
                    std::array::from_fn(|_| crate::identity_index::Removal::default());
                cycle.identity_cleanup = crate::identity_index::Cleanup::default();
            }
            cycle.started = started;
            cycle.reclaimed_bytes = 0;
            cycle.payload_trim = crate::region::TrimCursor::default();
            cycle.descriptor_trim = crate::region::TrimCursor::default();
            let scheduler_start = self
                .scheduler
                .as_ref()
                .filter(|_| cycle.kind == CollectionKind::Full)
                .map(|scheduler| (scheduler, self.managed_budget.used()));
            self.domain().active.store(true, Ordering::Release);
            self.domain().marking.store(true, Ordering::Release);
            self.allocation_debt.store(0, Ordering::Release);
            self.published_debt.store(0, Ordering::Release);
            // The phase flags and both debts are visible to newly admitted
            // mutators. Keep the cycle serialized through its bookkeeping.
            drop(publication);
            if let Some((scheduler, live)) = scheduler_start {
                scheduler.started(live);
            }
            let mut stats = lock(&self.statistics);
            stats.collections_started += 1;
            stats.last_collection_start = started;
        }
        let budget = self.configuration.incremental_step_size.max(1);
        let mut counters = WorkCounters::default();
        let mut work = budget;
        'units: {
            let mut cycle = cycle;
            let mut coordinator = coordinator;
            while work != 0 {
                if self.configuration.independent_phases {
                    drop(coordinator.take());
                }
                work -= 1;
                // A soft deadline may defer subsequent work, but cannot prevent the
                // first unit after a scheduling delay or expensive phase setup.
                if work + 1 < budget
                    && let Some(started) = started
                    && self.configuration.max_pause_time_us != 0
                    && started.elapsed().as_micros()
                        >= u128::from(self.configuration.max_pause_time_us)
                {
                    break;
                }
                if matches!(
                    cycle.phase,
                    Phase::References | Phase::Sweep | Phase::Compact
                ) && self.contexts.pending.load(Ordering::Acquire) != 0
                {
                    break;
                }
                match cycle.phase {
                    Phase::Idle => break,
                    Phase::OldHandoff => {
                        if !self.old_handoff_unit(&mut cycle, &mut work) {
                            break;
                        }
                    }
                    Phase::OldReferences => {
                        self.old_reference_root_unit(&mut cycle);
                    }
                    Phase::ContextRoots => match self.context_root_unit(&mut cycle) {
                        phase_work::ContextUnit::Progress => {}
                        phase_work::ContextUnit::Cleanup(cleanup) => {
                            drop(cycle);
                            drop(coordinator);
                            cleanup.run(self);
                            break 'units;
                        }
                    },
                    Phase::ResetMarks
                    | Phase::ResetReferences
                    | Phase::RootSlots
                    | Phase::Roots
                    | Phase::CrossRoots
                    | Phase::Remembered => {
                        if !self.root_unit(&mut cycle) {
                            break;
                        }
                    }
                    Phase::Mark => {
                        let claim_admission = self
                            .configuration
                            .independent_phases
                            .then(|| self.allocation_guard());
                        let mut tasks = crate::work::Claim::default();
                        let mut remaining = work + 1;
                        let count = self.claim_mark_batch(&mut tasks, &mut remaining);
                        if count != 0 {
                            work = remaining;
                            // Reserve the whole batch before releasing the coordinator. Neither
                            // generation may finish marking while even an unexecuted task is local.
                            self.domain().in_flight.reserve(count);
                            let epoch = cycle.epoch;
                            drop(claim_admission);
                            drop(cycle);
                            drop(coordinator);
                            let stalled = self.trace_mark_batch(&mut tasks, started, &mut counters);
                            // Another worker can close the epoch as soon as the last
                            // reservation is released. Publish completed mark work first.
                            if counters.marked != 0 {
                                counters.flush(&mut lock(&self.statistics));
                            }
                            self.domain().in_flight.release(count);
                            if stalled || work == 0 {
                                break 'units;
                            }
                            // A reservation protects publication of this batch. Re-enter
                            // only to claim further work in the same mark phase.
                            coordinator = if self.configuration.independent_phases {
                                None
                            } else if let Ok(guard) = self.coordinator.try_lock() {
                                Some(guard)
                            } else {
                                break 'units;
                            };
                            let Ok(next) = self.domain().cycle.try_lock() else {
                                break 'units;
                            };
                            cycle = next;
                            if cycle.epoch != epoch || cycle.phase != Phase::Mark {
                                break;
                            }
                        } else {
                            drop(claim_admission);
                            if coordinator.is_none() {
                                let Ok(guard) = self.coordinator.try_lock() else {
                                    break;
                                };
                                coordinator = Some(guard);
                            }
                            let Some(_publication) = self.try_publication_write() else {
                                break;
                            };
                            let Some(observation) = self.observe_phase() else {
                                break;
                            };
                            if self.contexts.pending.load(Ordering::Acquire) != 0 {
                                break;
                            }
                            // Never wait for an unreserved claim while retaining
                            // the coordinator or the publication writer. The
                            // claim still excludes closure; this caller yields.
                            let _claims = if let Some(gate) = &self.mark_claims {
                                match gate.try_write() {
                                    Ok(guard) => Some(guard),
                                    Err(std::sync::TryLockError::Poisoned(error)) => {
                                        Some(error.into_inner())
                                    }
                                    Err(std::sync::TryLockError::WouldBlock) => break,
                                }
                            } else {
                                None
                            };
                            if self.closure_in_flight() != 0
                                || (index == 1 && self.domains[0].active())
                            {
                                break;
                            }
                            let Some(empty) = self.domain().mark_queue.try_is_empty() else {
                                break;
                            };
                            if empty {
                                // A clone can still enter the previous reference stage while
                                // its graph work is draining. Process those exact slots before
                                // advancing the logical retention floor.
                                let previous_stage = self.domain().weak_phase();
                                let revisit =
                                    self.domain().late_references.has_work(previous_stage);
                                let stage = if cycle.kind == CollectionKind::Old {
                                    5
                                } else if revisit {
                                    previous_stage
                                } else {
                                    cycle.reference_stage.clamp(1, 5)
                                };
                                if !self.commit_phase(observation, true, false, stage) {
                                    break;
                                }
                                self.domain().mark_queue.finish_strong_births();
                                self.domain().trace_ready.store(false, Ordering::Release);
                                self.domain().marking.store(false, Ordering::Release);
                                if stage >= 5 {
                                    self.domain().weak_phase.store(5, Ordering::Release);
                                    cycle.phase = Phase::Sweep;
                                } else {
                                    cycle.reference_stage = stage;
                                    self.domain().weak_phase.store(stage, Ordering::Release);
                                    cycle.cursor = if revisit { usize::MAX } else { 0 };
                                    cycle.phase = Phase::References;
                                }
                            }
                        }
                    }
                    Phase::References => match self.reference_unit(&mut cycle, &mut counters) {
                        Unit::Progress => {}
                        Unit::Busy => break,
                        Unit::Sweep(work) => {
                            let epoch = cycle.epoch;
                            drop(cycle);
                            drop(coordinator);
                            work.run(self, index, epoch, &mut counters);
                            break 'units;
                        }
                        Unit::Boundary => {
                            if coordinator.is_none() {
                                let Ok(guard) = self.coordinator.try_lock() else {
                                    break;
                                };
                                coordinator = Some(guard);
                            }
                            let Some(_publication) = self.try_publication_write() else {
                                break;
                            };
                            let Some(observation) = self.observe_phase() else {
                                break;
                            };
                            // Sample after observing the clock. A registration between
                            // the empty scan and this observation may already have finished.
                            if self
                                .domain()
                                .late_references
                                .has_work(cycle.reference_stage)
                            {
                                continue;
                            }
                            if self.contexts.pending.load(Ordering::Acquire) != 0
                                || self.closure_in_flight() != 0
                            {
                                break;
                            }
                            let next_stage = cycle.reference_stage + 1;
                            let Some(empty) = self.domain().mark_queue.try_is_empty() else {
                                break;
                            };
                            let marking = next_stage == 3 || !empty;
                            let stage = if marking {
                                self.domain().weak_phase()
                            } else {
                                next_stage
                            };
                            if !self.commit_phase(observation, true, marking, stage) {
                                break;
                            }
                            cycle.cursor = 0;
                            cycle.reference_stage = next_stage;
                            self.domain().weak_phase.store(stage, Ordering::Release);
                            if marking {
                                self.domain().marking.store(true, Ordering::Release);
                                cycle.phase = Phase::Mark;
                                self.domain().trace_ready.store(true, Ordering::Release);
                            } else if cycle.reference_stage >= 5 {
                                self.domain().weak_phase.store(5, Ordering::Release);
                                cycle.phase = Phase::Sweep;
                            }
                        }
                    },
                    Phase::SelectRegions => {
                        if matches!(self.select_regions_unit(&mut cycle), Unit::Busy) {
                            break;
                        }
                    }
                    Phase::Compact => {
                        // Stage five can reopen marking, but it cannot revisit
                        // phantom discovery. Charge this cycle for its own set
                        // before taking the final publication writer.
                        if cycle.phantom_retained.pop_first().is_some() {
                            continue;
                        }
                        if let Some(heap) = &self.regions {
                            if let Some(queues) = &self.domain().relocations {
                                let window = self.configuration.threads.clamp(1, 16) * 16;
                                if !cycle.relocation_scan_done && queues.len() < window {
                                    match self.next_relocation(&mut cycle) {
                                        crate::index::Scan::Entry(_, metadata) => {
                                            queues.push(
                                                self.local_queue(),
                                                crate::relocation::Task {
                                                    metadata,
                                                    attempts: 0,
                                                },
                                            );
                                            continue;
                                        }
                                        crate::index::Scan::Advance => continue,
                                        crate::index::Scan::Busy => break,
                                        crate::index::Scan::Done => {
                                            cycle.relocation_scan_done = true;
                                        }
                                    }
                                }
                                if !cycle.relocation_scan_done {
                                    break;
                                }
                                // Claims cannot disappear between dequeue and reservation.
                                // Independent callbacks publish continuations before retiring flight.
                                let _claims = if let Some(gate) = &self.relocation_claims {
                                    match gate.try_write() {
                                        Ok(guard) => Some(guard),
                                        Err(std::sync::TryLockError::Poisoned(error)) => {
                                            Some(error.into_inner())
                                        }
                                        Err(std::sync::TryLockError::WouldBlock) => break,
                                    }
                                } else {
                                    None
                                };
                                if queues.len() != 0 || self.closure_in_flight() != 0 {
                                    break;
                                }
                                self.domain()
                                    .relocation_ready
                                    .store(false, Ordering::Release);
                            } else {
                                let claim_admission = self
                                    .configuration
                                    .independent_phases
                                    .then(|| self.allocation_guard());
                                let next = if let Some(task) = cycle.relocations.pop_front() {
                                    Some(task)
                                } else {
                                    match self.next_relocation(&mut cycle) {
                                        crate::index::Scan::Entry(_, metadata) => {
                                            Some((metadata, 0))
                                        }
                                        crate::index::Scan::Advance => continue,
                                        crate::index::Scan::Busy => break,
                                        crate::index::Scan::Done => None,
                                    }
                                };
                                if let Some((metadata, attempts)) = next {
                                    self.domain().in_flight.reserve(1);
                                    drop(claim_admission);
                                    drop(cycle);
                                    drop(coordinator);
                                    let repeat = self.relocate(&metadata, heap, attempts);
                                    coordinator = if self.configuration.independent_phases {
                                        None
                                    } else {
                                        Some(lock(&self.coordinator))
                                    };
                                    cycle = lock(&self.domain().cycle);
                                    if repeat {
                                        cycle
                                            .relocations
                                            .push_back((metadata, attempts.saturating_add(1)));
                                    }
                                    self.domain().in_flight.release(1);
                                    continue;
                                }
                                drop(claim_admission);
                                if self.closure_in_flight() != 0 {
                                    break;
                                }
                            }
                            if !cycle.payload_trim.trim_accessible(heap, 32) {
                                break;
                            }
                        }
                        if let Some(heap) = &self.descriptor_regions
                            && !cycle.descriptor_trim.trim_accessible(heap, 32)
                        {
                            break;
                        }
                        // Close publication before retiring this snapshot: no stale gray
                        // pointer may survive into a generation that can reclaim its target.
                        if coordinator.is_none() {
                            let Ok(guard) = self.coordinator.try_lock() else {
                                break;
                            };
                            coordinator = Some(guard);
                        }
                        let Some(publication) = self.try_publication_write() else {
                            break;
                        };
                        let Some(observation) = self.observe_phase() else {
                            break;
                        };
                        if self.contexts.pending.load(Ordering::Acquire) != 0 {
                            break;
                        }
                        let Some(empty) = self.domain().mark_queue.try_is_empty() else {
                            break;
                        };
                        if !empty {
                            if !self.commit_phase(observation, true, true, 5) {
                                break;
                            }
                            if let Some(heap) = &self.regions
                                && (!self.configuration.overlapping_generations || index == 1)
                            {
                                heap.end_evacuation();
                            }
                            self.domain().marking.store(true, Ordering::Release);
                            cycle.phase = Phase::Mark;
                            self.domain().trace_ready.store(true, Ordering::Release);
                            continue;
                        }
                        if !self.commit_phase(observation, false, false, 0) {
                            break;
                        }
                        if let Some(heap) = &self.regions
                            && (!self.configuration.overlapping_generations || index == 1)
                        {
                            heap.end_evacuation();
                        }
                        self.domain().mark_queue.close_graph_inbox();
                        self.domain().active.store(false, Ordering::Release);
                        self.domain().marking.store(false, Ordering::Release);
                        self.domain().weak_phase.store(0, Ordering::Release);
                        #[cfg(not(feature = "experimental-identity-index"))]
                        {
                            cycle.identity_cursor = crate::identity_pages::RetireCursor::default();
                        }
                        cycle.phase = Phase::Idle;
                        #[cfg(all(
                            feature = "experimental-pause-recorder",
                            target_os = "linux",
                            target_has_atomic = "64"
                        ))]
                        if let Some(recorder) = &self.configuration.pause_recorder {
                            recorder.collection_completed();
                        }
                        // Roots and barriers may enter the inactive domain.
                        // The coordinator still orders metrics and ticket completion.
                        drop(publication);
                        let mut stats = lock(&self.statistics);
                        counters.flush(&mut stats);
                        stats.collections_completed += 1;
                        match cycle.kind {
                            CollectionKind::Young => {
                                stats.young_collections += 1;
                                // Request exactly at completion: repeated or coalesced
                                // mutator requests must not miss an interval boundary.
                                if stats.young_collections.is_multiple_of(
                                    self.configuration.full_collection_interval.max(1),
                                ) {
                                    self.domains[1].publish_request();
                                } else if self.configuration.old_collection_interval != 0
                                    && stats
                                        .young_collections
                                        .is_multiple_of(self.configuration.old_collection_interval)
                                {
                                    self.old_requested.store(true, Ordering::Release);
                                }
                            }
                            CollectionKind::Old => {
                                stats.old_collections += 1;
                            }
                            CollectionKind::Full => {
                                stats.full_collections += 1;
                                if let Some(scheduler) = &self.scheduler {
                                    scheduler.completed(
                                        cycle.reclaimed_bytes,
                                        self.managed_budget.used(),
                                    );
                                }
                            }
                        }
                        if let Some(started) = cycle.started {
                            let duration = started.elapsed();
                            stats.last_collection_duration = Some(duration);
                            stats.total_collection_time += duration;
                        }
                        if cycle.kind != CollectionKind::Old {
                            self.domain()
                                .completed_ticket
                                .store(cycle.ticket, Ordering::Release);
                        }
                        if index == 0 {
                            self.next_domain.store(1, Ordering::Relaxed);
                        }
                        break;
                    }
                    Phase::Sweep => match self.sweep_unit(&mut cycle, &mut work, &mut counters) {
                        Unit::Progress => {}
                        Unit::Busy => break,
                        Unit::Sweep(external) => {
                            let epoch = cycle.epoch;
                            drop(cycle);
                            drop(coordinator);
                            external.run(self, index, epoch, &mut counters);
                            coordinator = if self.configuration.independent_phases {
                                None
                            } else {
                                let Ok(guard) = self.coordinator.try_lock() else {
                                    break 'units;
                                };
                                Some(guard)
                            };
                            let Ok(next) = self.domain().cycle.try_lock() else {
                                break 'units;
                            };
                            cycle = next;
                            if cycle.epoch != epoch || cycle.phase != Phase::Sweep {
                                break 'units;
                            }
                        }
                        Unit::Boundary => {
                            if coordinator.is_none() {
                                let Ok(guard) = self.coordinator.try_lock() else {
                                    break;
                                };
                                coordinator = Some(guard);
                            }
                            let Some(_publication) = self.try_publication_write() else {
                                break;
                            };
                            if self.contexts.pending.load(Ordering::Acquire) != 0 {
                                break;
                            }
                            let Some(empty) = self.domain().mark_queue.try_is_empty() else {
                                break;
                            };
                            if !empty {
                                let Some(observation) = self.observe_phase() else {
                                    break;
                                };
                                if !self.commit_phase(observation, true, true, 5) {
                                    break;
                                }
                                self.domain().marking.store(true, Ordering::Release);
                                cycle.phase = Phase::Mark;
                                self.domain().trace_ready.store(true, Ordering::Release);
                                continue;
                            }
                            cycle.phase = Phase::Compact;
                            // A full cycle enumerates every payload in a selected region.
                            // Young-only relocation retains the existing packing policy.
                            if cycle.kind != CollectionKind::Young
                                && self
                                    .regions
                                    .as_ref()
                                    .is_some_and(|heap| heap.begin_evacuation())
                            {
                                cycle.phase = Phase::SelectRegions;
                            }
                            cycle.relocation_scan_done = false;
                            self.domain().relocation_ready.store(
                                cycle.phase == Phase::Compact
                                    && self.regions.is_some()
                                    && self.configuration.independent_relocation,
                                Ordering::Release,
                            );
                            cycle.cursor = 0;
                            cycle.object_cursor = crate::index::Cursor::default();
                            cycle.young_cursor = crate::index::YoungCursor::default();
                            break;
                        }
                    },
                }
            }
        }
        drop(context);
        self.record_step(started, cpu_started, concurrent, &mut counters);
        let idle = !self.pending();
        self.notify_progress();
        Some(idle)
    }

    fn record_step(
        &self,
        started: Option<std::time::Instant>,
        cpu_started: Option<u64>,
        concurrent: bool,
        counters: &mut WorkCounters,
    ) {
        if let Some(started) = cpu_started
            && let Some(finished) = crate::diagnostics::thread_cpu_ns()
        {
            self.diagnostics
                .worker_cpu_ns
                .fetch_add(finished.saturating_sub(started), Ordering::Relaxed);
        }
        let mut stats = lock(&self.statistics);
        counters.flush(&mut stats);
        stats.collection_steps += 1;
        if let Some(started) = started {
            let duration = started.elapsed();
            let micros = u64::try_from(duration.as_micros()).unwrap_or(u64::MAX);
            if concurrent {
                stats.concurrent_time_us = stats.concurrent_time_us.saturating_add(micros);
            } else {
                stats.total_pause_time_us = stats.total_pause_time_us.saturating_add(micros);
            }
            stats.max_step_duration = Some(
                stats
                    .max_step_duration
                    .map_or(duration, |old| old.max(duration)),
            );
        }
    }

    fn notify_progress(&self) {
        self.recovery.notify_gc_progress(self);
    }

    /// Pointer used by the compiled runtime context.
    #[must_use]
    pub fn as_context_ptr(&self) -> *const u8 {
        std::ptr::from_ref(self).cast()
    }

    /// Reconstructs a collector from the compiled runtime context.
    ///
    /// # Panics
    /// Panics for a null context.
    /// # Safety
    /// The caller must supply the documented live context for the entire returned lifetime.
    #[must_use]
    #[expect(clippy::cast_ptr_alignment)]
    pub unsafe fn from_raw_ptr<'a>(ptr: *const u8) -> &'a Self {
        assert!(!ptr.is_null(), "null collector context");
        // SAFETY: runtime helpers pass a live collector pointer.
        unsafe { &*ptr.cast::<Self>() }
    }

    /// Reads the collector pointer from the first field of a runtime context.
    ///
    /// # Panics
    /// Panics for a null context.
    /// # Safety
    /// The caller must supply the documented live context for the entire returned lifetime.
    #[must_use]
    #[expect(clippy::cast_ptr_alignment)]
    pub unsafe fn from_context_struct_ptr<'a>(ptr: *const u8) -> &'a Self {
        assert!(!ptr.is_null(), "null runtime context");
        // SAFETY: RuntimeContext has its collector pointer as its first repr(C) field.
        unsafe { Self::from_raw_ptr(*ptr.cast::<*const u8>()) }
    }
}

impl Drop for GarbageCollector {
    fn drop(&mut self) {
        #[cfg(not(target_family = "wasm"))]
        self.close_refund_notifier();
        #[cfg(not(target_family = "wasm"))]
        if let Some(signals) = self.worker_signals.swap(None) {
            signals.close();
        }
        self.cleanup_remaining_objects();
    }
}

// SAFETY: all hooks forward to the present value with the same protected owner.
unsafe impl<T: Trace> Trace for Option<T> {
    fn has_bounded_cleanup() -> bool {
        T::has_bounded_cleanup()
    }

    fn edges_rooted_before_unroot(&self) -> bool {
        self.as_ref().is_some_and(Trace::edges_rooted_before_unroot)
    }

    fn needs_clear() -> bool {
        T::needs_clear()
    }

    fn clear_step(&mut self, offset: usize, budget: usize) -> TraceProgress {
        self.as_mut().map_or(TraceProgress::Complete, |value| {
            value.clear_step(offset, budget)
        })
    }

    fn may_contain_gc_references() -> bool {
        T::may_contain_gc_references()
    }
    unsafe fn unroot(&mut self, owner: crate::TraceOwner<'_>) {
        if let Some(value) = self {
            // SAFETY: this field belongs to the caller's protected owner.
            unsafe { value.unroot(owner) };
        }
    }
    unsafe fn attach(&mut self, owner: crate::TraceOwner<'_>) {
        if let Some(value) = self {
            // SAFETY: construction protection covers the present value.
            unsafe { value.attach(owner) };
        }
    }
    fn root(&mut self) {
        if let Some(value) = self {
            value.root();
        }
    }
    fn trace(&self, collector: &GarbageCollector) {
        if let Some(value) = self {
            value.trace(collector);
        }
    }
}

// SAFETY: every initialized element is traced and participates in ownership
// conversion; clearing only destroys initialized elements under exclusive access.
unsafe impl<T: Trace> Trace for Vec<T> {
    fn has_bounded_cleanup() -> bool {
        !std::mem::needs_drop::<T>()
    }

    fn may_contain_gc_references() -> bool {
        T::may_contain_gc_references()
    }
    fn trace_work(&self) -> usize {
        if !T::may_contain_gc_references() && !std::mem::needs_drop::<T>() {
            1
        } else {
            32
        }
    }
    fn clear_step(&mut self, _offset: usize, budget: usize) -> TraceProgress {
        // Clearing elements with no drop glue only changes the length.
        if !std::mem::needs_drop::<T>() {
            self.clear();
            return TraceProgress::Complete;
        }
        self.truncate(self.len().saturating_sub(budget.max(1)));
        if self.is_empty() {
            TraceProgress::Complete
        } else {
            TraceProgress::Continue(0)
        }
    }
    unsafe fn unroot(&mut self, owner: crate::TraceOwner<'_>) {
        for value in self {
            // SAFETY: initialized elements share the caller's protected owner.
            unsafe { value.unroot(owner) };
        }
    }
    unsafe fn attach(&mut self, owner: crate::TraceOwner<'_>) {
        for value in self {
            // SAFETY: construction protection covers every initialized element.
            unsafe { value.attach(owner) };
        }
    }
    fn root(&mut self) {
        for value in self {
            value.root();
        }
    }
    fn trace(&self, collector: &GarbageCollector) {
        if T::may_contain_gc_references() {
            for item in self {
                item.trace(collector);
            }
        }
    }

    fn trace_step(
        &self,
        collector: &GarbageCollector,
        offset: usize,
        budget: usize,
    ) -> TraceProgress {
        if !T::may_contain_gc_references() {
            return TraceProgress::Complete;
        }
        let end = offset.saturating_add(budget.max(1)).min(self.len());
        if let Some(items) = self.get(offset..end) {
            T::trace_slice(items, collector);
        }
        if end == self.len() {
            TraceProgress::Complete
        } else {
            TraceProgress::Continue(end)
        }
    }

    fn external_size(&self) -> usize {
        self.capacity()
            .saturating_mul(size_of::<T>())
            .saturating_add(self.iter().map(Trace::external_size).sum::<usize>())
    }
}

// SAFETY: keys remain independently owning because the map cannot expose them
// mutably for ownership conversion. Values are traced and converted in place.
unsafe impl<K: Trace, V: Trace, S: ::std::hash::BuildHasher> Trace for HashMap<K, V, S> {
    fn root(&mut self) {
        for value in self.values_mut() {
            value.root();
        }
    }
    unsafe fn unroot(&mut self, owner: crate::TraceOwner<'_>) {
        for value in self.values_mut() {
            // SAFETY: values belong to this exclusively borrowed traced owner.
            unsafe { value.unroot(owner) };
        }
    }
    unsafe fn attach(&mut self, owner: crate::TraceOwner<'_>) {
        for value in self.values_mut() {
            // SAFETY: construction protection covers the embedded values.
            unsafe { value.attach(owner) };
        }
    }
    fn trace(&self, collector: &GarbageCollector) {
        for (key, value) in self {
            key.trace(collector);
            value.trace(collector);
        }
    }
}

// SAFETY: set keys are never demoted and remain independently owning; tracing
// still visits them. Their ownership cannot be mutated through the set API.
unsafe impl<T: Trace, S: ::std::hash::BuildHasher> Trace for HashSet<T, S> {
    fn trace(&self, collector: &GarbageCollector) {
        for item in self {
            item.trace(collector);
        }
    }
}

// SAFETY: every component is traced and receives the same ownership context.
unsafe impl<A: Trace, B: Trace> Trace for (A, B) {
    fn needs_clear() -> bool {
        A::needs_clear() || B::needs_clear()
    }
    fn root(&mut self) {
        self.0.root();
        self.1.root();
    }
    unsafe fn unroot(&mut self, owner: crate::TraceOwner<'_>) {
        // SAFETY: both components belong to the protected tuple owner.
        unsafe {
            self.0.unroot(owner);
            self.1.unroot(owner);
        }
    }
    unsafe fn attach(&mut self, owner: crate::TraceOwner<'_>) {
        // SAFETY: construction protection covers both components.
        unsafe {
            self.0.attach(owner);
            self.1.attach(owner);
        }
    }
    fn trace(&self, collector: &GarbageCollector) {
        self.0.trace(collector);
        self.1.trace(collector);
    }
}

// SAFETY: every component is traced and receives the same ownership context.
unsafe impl<A: Trace, B: Trace, C: Trace> Trace for (A, B, C) {
    fn needs_clear() -> bool {
        A::needs_clear() || B::needs_clear() || C::needs_clear()
    }
    fn root(&mut self) {
        self.0.root();
        self.1.root();
        self.2.root();
    }
    unsafe fn unroot(&mut self, owner: crate::TraceOwner<'_>) {
        // SAFETY: all components belong to the protected tuple owner.
        unsafe {
            self.0.unroot(owner);
            self.1.unroot(owner);
            self.2.unroot(owner);
        }
    }
    unsafe fn attach(&mut self, owner: crate::TraceOwner<'_>) {
        // SAFETY: construction protection covers all components.
        unsafe {
            self.0.attach(owner);
            self.1.attach(owner);
            self.2.attach(owner);
        }
    }
    fn trace(&self, collector: &GarbageCollector) {
        self.0.trace(collector);
        self.1.trace(collector);
        self.2.trace(collector);
    }
}

// SAFETY: strings contain no managed edges and retain ordinary Rust storage.
unsafe impl Trace for String {
    fn has_bounded_cleanup() -> bool {
        true
    }

    fn may_contain_gc_references() -> bool {
        false
    }
    fn trace_work(&self) -> usize {
        1
    }
    fn trace(&self, _collector: &GarbageCollector) {}
    fn external_size(&self) -> usize {
        self.capacity()
    }
}

// SAFETY: borrowed string data contains no managed edges.
unsafe impl Trace for &str {
    fn may_contain_gc_references() -> bool {
        false
    }
    fn trace_work(&self) -> usize {
        1
    }
    fn trace(&self, _collector: &GarbageCollector) {}
}

macro_rules! impl_trace_primitive {
    ($($t:ty),*) => {
        $(
            // SAFETY: primitive scalars contain no managed edges.
            unsafe impl Trace for $t {
                fn may_contain_gc_references() -> bool { false }
                fn needs_clear() -> bool { false }

                fn canonical_payload() -> Option<crate::CanonicalPayload<Self>> {
                    // SAFETY: primitive scalars have no graph, owner, backing or callbacks.
                    Some(unsafe { crate::CanonicalPayload::new() })
                }

                fn trace_work(&self) -> usize { 1 }
                fn trace(&self, _collector: &GarbageCollector) {}
            }
        )*
    };
}

impl_trace_primitive!(
    bool, char, u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64
);

// SAFETY: payload access is locked and fields are never demoted. Ordinary mutex
// mutation lacks managed barriers, so payload fields remain independently owning.
unsafe impl<T> Trace for Mutex<T>
where
    T: Trace,
{
    fn has_bounded_cleanup() -> bool {
        T::has_bounded_cleanup()
    }

    fn needs_clear() -> bool {
        T::needs_clear()
    }

    fn clear_step(&mut self, offset: usize, budget: usize) -> TraceProgress {
        self.get_mut()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .clear_step(offset, budget)
    }

    fn may_contain_gc_references() -> bool {
        T::may_contain_gc_references()
    }
    fn trace(&self, collector: &GarbageCollector) {
        if !T::may_contain_gc_references() {
            return;
        }
        match self.lock() {
            Ok(guard) => guard.trace(collector),
            Err(poisoned) => poisoned.into_inner().trace(collector),
        }
    }

    fn trace_step(
        &self,
        collector: &GarbageCollector,
        offset: usize,
        budget: usize,
    ) -> TraceProgress {
        if !T::may_contain_gc_references() {
            return TraceProgress::Complete;
        }
        match self.try_lock() {
            Ok(guard) => guard.trace_step(collector, offset, budget),
            Err(std::sync::TryLockError::Poisoned(error)) => {
                error.into_inner().trace_step(collector, offset, budget)
            }
            Err(std::sync::TryLockError::WouldBlock) => TraceProgress::Busy,
        }
    }

    fn external_size(&self) -> usize {
        lock(self).external_size()
    }
}

// SAFETY: payload access is locked and fields remain independently owning.
unsafe impl<T> Trace for RwLock<T>
where
    T: Trace,
{
    fn has_bounded_cleanup() -> bool {
        T::has_bounded_cleanup()
    }

    fn needs_clear() -> bool {
        T::needs_clear()
    }

    fn clear_step(&mut self, offset: usize, budget: usize) -> TraceProgress {
        self.get_mut()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .clear_step(offset, budget)
    }

    fn may_contain_gc_references() -> bool {
        T::may_contain_gc_references()
    }
    fn trace(&self, collector: &GarbageCollector) {
        if !T::may_contain_gc_references() {
            return;
        }
        match self.read() {
            Ok(guard) => guard.trace(collector),
            Err(poisoned) => poisoned.into_inner().trace(collector),
        }
    }

    fn trace_step(
        &self,
        collector: &GarbageCollector,
        offset: usize,
        budget: usize,
    ) -> TraceProgress {
        if !T::may_contain_gc_references() {
            return TraceProgress::Complete;
        }
        match self.try_read() {
            Ok(guard) => guard.trace_step(collector, offset, budget),
            Err(std::sync::TryLockError::Poisoned(error)) => {
                error.into_inner().trace_step(collector, offset, budget)
            }
            Err(std::sync::TryLockError::WouldBlock) => TraceProgress::Busy,
        }
    }

    fn external_size(&self) -> usize {
        self.read()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .external_size()
    }
}

// SAFETY: payload access is locked and fields remain independently owning.
unsafe impl<T> Trace for crate::sync::Mutex<T>
where
    T: Trace,
{
    fn has_bounded_cleanup() -> bool {
        T::has_bounded_cleanup()
    }

    fn needs_clear() -> bool {
        T::needs_clear()
    }

    fn clear_step(&mut self, offset: usize, budget: usize) -> TraceProgress {
        self.get_mut().clear_step(offset, budget)
    }

    fn may_contain_gc_references() -> bool {
        T::may_contain_gc_references()
    }
    fn trace(&self, collector: &GarbageCollector) {
        if T::may_contain_gc_references() {
            self.lock().trace(collector);
        }
    }

    fn trace_step(
        &self,
        collector: &GarbageCollector,
        offset: usize,
        budget: usize,
    ) -> TraceProgress {
        if !T::may_contain_gc_references() {
            return TraceProgress::Complete;
        }
        self.try_lock().map_or(TraceProgress::Busy, |value| {
            value.trace_step(collector, offset, budget)
        })
    }

    fn external_size(&self) -> usize {
        self.lock().external_size()
    }
}

#[cfg(test)]
mod tests {
    #![expect(clippy::panic_in_result_fn)]
    use super::*;

    #[cfg(all(not(target_family = "wasm"), not(miri)))]
    #[test]
    fn abandoning_a_contended_regional_return_releases_its_range_once() -> Result<()> {
        struct CountDrop(Arc<AtomicUsize>);
        // SAFETY: the host counter owns no managed edges.
        unsafe impl Trace for CountDrop {
            fn trace(&self, _: &GarbageCollector) {}
        }
        impl Drop for CountDrop {
            fn drop(&mut self) {
                self.0.fetch_add(1, Ordering::Relaxed);
            }
        }
        for heap_gone in [false, true] {
            let drops = Arc::new(AtomicUsize::new(0));
            let gc = GarbageCollector::with_config(
                Configuration::builder()
                    .local_allocation_buffers(true)
                    .max_heap_bytes(8)
                    .region_shards(1)
                    .allocation_threshold(usize::MAX)
                    .build(),
            );
            let heap = gc.descriptor_regions.clone().expect("descriptor heap");
            let root = gc
                .register_context()
                .with(|| Gc::try_new(&gc, CountDrop(drops.clone())))?;
            assert_eq!(heap.statistics().occupied_bytes, 8);
            let meta = gc
                .metadata_for_address(root.as_ptr().addr())
                .expect("live allocation");
            let lease = ObjectMetadata::borrow_callback(&meta, false).map_err(|_| {
                crate::Error::CollectionPhaseError("expected a live descriptor".to_owned())
            })?;
            drop(root);
            gc.collect_full()?;
            drop(lease);
            heap.with_parked_registry(|| {
                assert!(matches!(
                    gc.reap_retired(false, None),
                    Some(RetirementStep::Contended)
                ));
            });
            assert_eq!(gc.managed_bytes(), 8);
            assert_eq!(drops.load(Ordering::Acquire), 1);
            let job = gc
                .retired
                .load()
                .as_ref()
                .and_then(|queue| queue.pop())
                .expect("queued return");
            assert!(matches!(job.stage, ReclaimStage::Return(_)));
            let weak = gc.weak();
            let live = if heap_gone {
                drop(gc);
                None
            } else {
                Some(gc)
            };
            assert_eq!(weak.upgrade().is_none(), heap_gone);
            drop(job);
            if let Some(heap) = &live {
                assert_eq!(heap.managed_bytes(), 0);
            }
            assert_eq!(heap.statistics().occupied_bytes, 0);
            while !heap.trim(1) {}
            assert_eq!(heap.statistics().shared_committed_bytes, 0);
            assert_eq!(drops.load(Ordering::Acquire), 1);
        }
        Ok(())
    }

    #[cfg(not(target_family = "wasm"))]
    #[test]
    fn a_space_offer_lost_to_another_allocator_does_not_spend_recovery_cycles() -> Result<()> {
        use std::future::Future;
        use std::task::{Context, Waker};
        let gc = GarbageCollector::with_config(
            Configuration::builder()
                .max_heap_bytes(8)
                .allocation_threshold(usize::MAX)
                .build(),
        );
        // Model an active worker while the fixture drives completion deterministically.
        let worker = crate::recovery::WorkerRegistration::new(&gc);
        gc.reserve_allocation(8)?;
        let mut allocation = std::pin::pin!(gc.reserve_bytes_async(8));
        let mut context = Context::from_waker(Waker::noop());
        for offer in [false, true, false, true, true] {
            for _ in 0..8 {
                assert!(allocation.as_mut().poll(&mut context).is_pending());
                if gc.recovery.waiter_count() == 1 {
                    break;
                }
            }
            assert_eq!(gc.recovery.waiter_count(), 1);
            if offer {
                gc.cancel_allocation(8);
                gc.recovery.notify_progress(&gc); // Enough headroom was offered to this waiter.
                assert_eq!(gc.recovery.waiter_count(), 0);
                gc.reserve_allocation(8)?; // Another allocator wins before the waiter is polled.
            }
            gc.collect_full()?;
            assert!(allocation.as_mut().poll(&mut context).is_pending());
        }
        gc.cancel_allocation(8);
        let std::task::Poll::Ready(reservation) = allocation.as_mut().poll(&mut context) else {
            panic!("available credit must be reserved on the next poll");
        };
        drop(reservation?);
        drop(worker);
        assert_eq!(gc.managed_bytes(), 0);
        assert_eq!(gc.recovery.waiter_count(), 0);
        Ok(())
    }

    pub(super) fn prepare_full_mark(gc: &GarbageCollector) {
        gc.collect();
        for _ in 0..1000 {
            gc.step();
            if gc.domains[1].trace_ready.load(Ordering::Acquire) {
                return;
            }
        }
        assert!(
            gc.domains[1].trace_ready.load(Ordering::Acquire),
            "fixture did not reach marking"
        );
    }

    #[test]
    fn a_trace_started_before_remembered_enrollment_keeps_later_membership() -> Result<()> {
        struct TwoSteps;
        // SAFETY: the fixture has no outgoing references.
        unsafe impl Trace for TwoSteps {
            fn trace(&self, _: &GarbageCollector) {}
            fn trace_step(&self, _: &GarbageCollector, offset: usize, _: usize) -> TraceProgress {
                if offset == 0 {
                    TraceProgress::Continue(1)
                } else {
                    TraceProgress::Complete
                }
            }
        }

        for counted in [false, true] {
            let gc = GarbageCollector::with_config(
                Configuration::builder()
                    .concurrent_publication(counted)
                    .independent_marking(true)
                    .incremental_step_size(1)
                    .max_pause_time_us(0)
                    .allocation_threshold(usize::MAX)
                    .build(),
            );
            let owner = Gc::try_new(&gc, TwoSteps)?;
            prepare_full_mark(&gc);
            let address = owner.as_ptr().addr();
            let context = Context::enter(&gc, 1);
            let mut tasks = crate::work::Claim::default();
            assert!(!gc.remembered.may_prune());
            assert_eq!(gc.claim_mark_batch(&mut tasks, &mut 1), 1);
            gc.trace_mark_batch(&mut tasks, None, &mut WorkCounters::default());
            gc.remembered.lock(address).insert(
                address,
                gc.identity_for_address(address).expect("member identity"),
            );
            assert!(gc.remembered.may_prune());
            assert_eq!(gc.claim_mark_batch(&mut tasks, &mut 1), 1);
            gc.trace_mark_batch(&mut tasks, None, &mut WorkCounters::default());
            assert!(gc.remembered.lock(address).contains(address));
            drop(context);
            drop(owner);
            gc.collect_full()?;
            assert_eq!(gc.managed_bytes(), 0);
        }
        Ok(())
    }

    #[test]
    fn publication_after_all_sweep_shards_survives_and_retires_once() -> Result<()> {
        struct Counted(Arc<AtomicUsize>);
        // SAFETY: this fixture contains no outgoing GC references.
        unsafe impl Trace for Counted {
            fn trace(&self, _: &GarbageCollector) {}
        }
        impl Drop for Counted {
            fn drop(&mut self) {
                self.0.fetch_add(1, Ordering::Relaxed);
            }
        }
        for (shards, independent, index) in [(1, false), (16, false), (1, true), (16, true)]
            .into_iter()
            .flat_map(|(shards, independent)| (0..2).map(move |index| (shards, independent, index)))
        {
            let gc = GarbageCollector::with_config(
                Configuration::builder()
                    .metadata_shards(shards)
                    .independent_marking(independent)
                    .incremental_step_size(1)
                    .max_pause_time_us(0)
                    .full_collection_interval(usize::MAX)
                    .allocation_threshold(usize::MAX)
                    .build(),
            );
            let drops = Arc::new(AtomicUsize::new(0));
            let old = Gc::new(&gc, Counted(drops.clone()));
            let old_id = old.identity();
            drop(old);
            if index == 0 {
                gc.collect_young();
            } else {
                gc.collect();
            }
            for _ in 0..1000 {
                gc.step();
                if lock(&gc.domain_at(index).cycle).phase == Phase::Compact {
                    break;
                }
            }
            assert_eq!(lock(&gc.domain_at(index).cycle).phase, Phase::Compact);
            assert_eq!(drops.load(Ordering::Relaxed), 1);
            assert!(gc.address_for_identity(old_id).is_none());
            // Every shard's sweep cursor has passed. Publication must mark this
            // allocation black even if the allocator reuses a retired address.
            let late = Gc::new(&gc, Counted(drops.clone()));
            let late_id = late.identity();
            while !gc.step() {}
            assert!(gc.address_for_identity(late_id).is_some());
            assert_eq!(gc.statistics()?.live_objects, 1);
            assert_eq!(drops.load(Ordering::Relaxed), 1);
            drop(late);
            gc.stop()?;
            assert!(gc.address_for_identity(late_id).is_none());
            assert_eq!(gc.statistics()?.live_objects, 0);
            assert_eq!(drops.load(Ordering::Relaxed), 2);
        }
        Ok(())
    }

    #[test]
    fn independent_marking_needs_neither_coordinator_nor_cycle_lock() -> Result<()> {
        let gc = GarbageCollector::with_config(
            Configuration::builder()
                .metadata_shards(16)
                .independent_marking(true)
                .diagnostics(true)
                .incremental_step_size(1)
                .max_pause_time_us(0)
                .allocation_threshold(usize::MAX)
                .build(),
        );
        let roots: Vec<_> = (0..16).map(|_| Gc::new(&gc, 42_u64)).collect();
        prepare_full_mark(&gc);
        let coordinator = lock(&gc.coordinator);
        let cycle = lock(&gc.domains[1].cycle);
        for _ in 0..16 {
            assert!(!gc.step());
        }
        assert_eq!(gc.diagnostics().independent_mark_steps, 16);
        assert_eq!(gc.statistics()?.objects_marked, 16);
        assert_eq!(gc.in_flight(), 0);
        assert!(!gc.step()); // Empty queues fall back without blocking on the held coordinator.
        drop((cycle, coordinator));
        while !gc.step() {}
        assert_eq!(gc.statistics()?.full_collections, 1);
        drop(roots);
        gc.stop()?;
        assert_eq!(gc.statistics()?.live_objects, 0);
        Ok(())
    }

    #[test]
    fn a_departed_worker_leaves_its_continuation_available_to_callers() -> Result<()> {
        struct Resumable(Mutex<Vec<usize>>);
        // SAFETY: this fixture contains no outgoing GC references.
        unsafe impl Trace for Resumable {
            fn trace_work(&self) -> usize {
                1
            }
            fn trace(&self, _: &GarbageCollector) {}
            fn trace_step(&self, _: &GarbageCollector, offset: usize, _: usize) -> TraceProgress {
                lock(&self.0).push(offset);
                if offset == 0 {
                    TraceProgress::Continue(1)
                } else {
                    TraceProgress::Complete
                }
            }
        }
        for independent in [false, true] {
            let gc = GarbageCollector::with_config(
                Configuration::builder()
                    .work_stealing(true)
                    .independent_marking(independent)
                    .threads(4)
                    .diagnostics(true)
                    .incremental_step_size(1)
                    .max_pause_time_us(0)
                    .allocation_threshold(usize::MAX)
                    .build(),
            );
            let root = Gc::new(&gc, Resumable(Mutex::new(Vec::new())));
            {
                let _worker = WorkerContext::enter(&gc, 3);
                prepare_full_mark(&gc);
                assert!(!gc.step());
            }
            assert_eq!(*lock(&root.0), [0]);
            assert_eq!(gc.diagnostics().queued_mark_tasks, [0, 1]);
            assert_eq!(gc.diagnostics().in_flight_work, [0, 0]);
            // No thread remains assigned to that queue. A cooperative caller must
            // steal its continuation before the generation can close.
            while !gc.step() {}
            assert_eq!(*lock(&root.0), [0, 1]);
            assert_eq!(gc.statistics()?.objects_marked, 1);
            if !cfg!(target_family = "wasm") {
                assert_eq!(gc.diagnostics().mark_stolen, 1);
            }
            drop(root);
            gc.stop()?;
            let diagnostics = gc.diagnostics();
            assert_eq!(diagnostics.queued_mark_tasks, [0, 0]);
            assert_eq!(diagnostics.in_flight_work, [0, 0]);
            assert_eq!(diagnostics.mark_queue_capacity, [0, 0]);
            assert_eq!(gc.statistics()?.live_objects, 0);
        }
        Ok(())
    }

    #[cfg(not(target_family = "wasm"))]
    #[test]
    #[expect(clippy::unwrap_in_result)]
    #[expect(clippy::too_many_lines, reason = "one forced dequeue/closure schedule")]
    fn a_claim_cannot_disappear_between_dequeue_and_reservation() -> Result<()> {
        struct Block {
            entered: std::sync::mpsc::Sender<()>,
            resume: Mutex<std::sync::mpsc::Receiver<()>>,
        }
        // SAFETY: this fixture contains no outgoing GC references.
        unsafe impl Trace for Block {
            fn trace_work(&self) -> usize {
                1
            }
            fn trace(&self, gc: &GarbageCollector) {
                assert_eq!(gc.in_flight(), 1);
                self.entered.send(()).expect("callback entered");
                lock(&self.resume)
                    .recv_timeout(std::time::Duration::from_secs(10))
                    .expect("resume");
            }
        }
        let gc = GarbageCollector::with_config(
            Configuration::builder()
                .metadata_shards(16)
                .independent_marking(true)
                .work_stealing(true)
                .threads(4)
                .diagnostics(true)
                .incremental_step_size(1)
                .max_pause_time_us(0)
                .allocation_threshold(usize::MAX)
                .build(),
        );
        let (entered, callback) = std::sync::mpsc::channel();
        let (resume, release) = std::sync::mpsc::channel();
        let root = Gc::new(
            &gc,
            Block {
                entered,
                resume: Mutex::new(release),
            },
        );
        {
            let _worker = WorkerContext::enter(&gc, 0);
            prepare_full_mark(&gc);
        }
        // Pause the actual claim after dequeue and before its in-flight reservation.
        // Claims now carry exact metadata and need no object-index lookup.
        let (entered, claimed) = std::sync::mpsc::channel();
        let (release_claim, resume_claim) = std::sync::mpsc::channel();
        *lock(&gc.mark_claim_pause) = Some(ClaimPause {
            entered,
            resume: resume_claim,
        });
        let marker = {
            let gc = gc.clone();
            thread::spawn(move || {
                let _worker = WorkerContext::enter(&gc, 1);
                // Force the independent path this test asserts. A general step
                // may miss a nonblocking steal and claim under the coordinator,
                // which safely excludes closure using a different lock.
                loop {
                    if let Some(idle) = gc.independent_mark_step(false) {
                        break idle;
                    }
                    thread::yield_now();
                }
            })
        };
        claimed
            .recv_timeout(std::time::Duration::from_secs(10))
            .expect("claim entered");
        assert_eq!(gc.domains[1].mark_queue.try_is_empty(), Some(true));
        assert_eq!(gc.in_flight(), 0);
        let gate = gc.mark_claims.as_ref().expect("claim gate");
        assert!(
            gate.try_write().is_err(),
            "the unreserved claim must exclude closure"
        );
        let (finished, closure) = std::sync::mpsc::channel();
        let closer = {
            let gc = gc.clone();
            thread::spawn(move || {
                loop {
                    // The observer below also probes the coordinator. Missing its
                    // brief guard is allowed; retry until the closer actually runs.
                    if let Some(idle) = gc.coordinated_step(true) {
                        finished.send(idle).expect("closure observer");
                        break Some(idle);
                    }
                    thread::yield_now();
                }
            })
        };
        let yielded = closure.recv_timeout(std::time::Duration::from_secs(1));
        let coordinator_free = gc.coordinator.try_lock().is_ok();
        let publication_free = gc.publication.try_write().is_some();
        release_claim.send(()).expect("release claim");
        callback
            .recv_timeout(std::time::Duration::from_secs(10))
            .expect("entered");
        assert_eq!(closer.join().expect("closer"), Some(false));
        assert_eq!(lock(&gc.domains[1].cycle).phase, Phase::Mark);
        assert_eq!(gc.domains[1].weak_phase(), 0);
        assert_eq!(gc.diagnostics().in_flight_work, [0, 1]);
        assert_eq!(gc.diagnostics().mark_stolen, 1);
        assert_eq!(gc.statistics()?.collections_completed, 0);
        resume.send(()).expect("release");
        marker.join().expect("marker");
        assert_eq!(yielded, Ok(false), "claim wait held global coordination");
        assert!(coordinator_free && publication_free);
        while !gc.step() {}
        assert_eq!(gc.statistics()?.objects_marked, 1);
        assert_eq!(gc.statistics()?.collections_completed, 1);
        drop(root);
        gc.stop()?;
        assert_eq!(gc.statistics()?.live_objects, 0);
        Ok(())
    }

    #[test]
    fn retired_mark_work_cannot_trace_or_prune_a_reused_address() -> Result<()> {
        struct Probe(Arc<AtomicUsize>);
        // SAFETY: this fixture contains no outgoing GC references.
        unsafe impl Trace for Probe {
            fn trace(&self, _: &GarbageCollector) {
                panic!("retired task executed a callback");
            }
        }
        impl Drop for Probe {
            fn drop(&mut self) {
                self.0.fetch_add(1, Ordering::Relaxed);
            }
        }
        for concurrent in [false, true] {
            let gc = GarbageCollector::with_config(
                Configuration::builder()
                    .concurrent_publication(concurrent)
                    .incremental_step_size(1)
                    .mark_batch_size(1)
                    .max_pause_time_us(0)
                    .allocation_threshold(usize::MAX)
                    .build(),
            );
            let drops = Arc::new(AtomicUsize::new(0));
            let root = Gc::try_new(&gc, Probe(drops.clone()))?;
            let address = root.as_ptr() as usize;
            prepare_full_mark(&gc);
            drop(root);
            // Simulate retirement by another generation with a queued task left.
            let retired = gc
                .objects
                .replace_for_test(address, None)
                .expect("metadata");
            #[cfg(feature = "experimental-identity-index")]
            {
                let mut cursor = crate::identity_index::Removal::default();
                while gc
                    .identities
                    .remove_step(retired.id.to_i64().cast_unsigned(), address, &mut cursor)
                    .is_none()
                {}
            }
            #[cfg(not(feature = "experimental-identity-index"))]
            gc.identities
                .remove(retired.id.to_i64().cast_unsigned(), address);
            gc.objects.set_young_for_test(address, false);
            retired.drop_object();
            assert_eq!(drops.load(Ordering::Relaxed), 1);
            assert!(ObjectMetadata::lease(&retired).is_none());

            let replacement = Gc::try_new(&gc, 42_u64)?;
            let replacement_address = replacement.as_ptr() as usize;
            let metadata = gc
                .objects
                .lock(replacement_address)
                .get(&replacement_address)
                .cloned()
                .expect("replacement");
            // Force the old address to resolve to a different type and identity,
            // regardless of the host allocator's reuse policy. Never dereference
            // this synthetic index alias; only the queued identity may be used.
            gc.objects.replace_for_test(address, Some(metadata.clone()));
            let context = Context::enter(&gc, 1);
            let mut tasks = crate::work::Claim::default();
            assert_eq!(gc.claim_mark_batch(&mut tasks, &mut 1), 1);
            let record = &tasks.items[0].as_ref().expect("claim").0;
            assert_eq!(record.metadata.id, retired.id);
            metadata
                .mutation
                .store(retired.mutation.load(Ordering::Acquire), Ordering::Release);
            gc.remembered.lock(address).insert(
                address,
                gc.identity_for_address(address).expect("member identity"),
            );
            let mut counters = WorkCounters::default();
            assert!(!gc.trace_mark_batch(&mut tasks, now(), &mut counters));
            assert_eq!(counters.marked, 1);
            assert!(gc.remembered.lock(address).contains(address));
            if address != replacement_address {
                gc.objects.replace_for_test(address, None);
            }
            gc.remembered.lock(address).remove(address);
            drop(context);
            while !gc.step() {}
            drop(replacement);
            gc.stop()?;
            assert_eq!(drops.load(Ordering::Relaxed), 1);
            assert_eq!(gc.statistics()?.live_objects, 0);
            assert_eq!(gc.managed_bytes(), 0);
        }
        Ok(())
    }

    #[cfg(not(target_family = "wasm"))]
    #[test]
    #[expect(clippy::unwrap_in_result)]
    fn a_reserved_batch_prevents_termination_in_both_generations() -> Result<()> {
        for independent in [false, true] {
            struct Leaf;
            // SAFETY: this fixture contains no outgoing GC references.
            unsafe impl Trace for Leaf {
                fn trace_work(&self) -> usize {
                    1
                }
                fn trace(&self, _: &GarbageCollector) {}
            }
            struct Block {
                entered: std::sync::mpsc::Sender<()>,
                resume: Mutex<std::sync::mpsc::Receiver<()>>,
            }
            // SAFETY: this fixture contains no outgoing GC references.
            unsafe impl Trace for Block {
                fn trace_work(&self) -> usize {
                    1
                }
                fn trace(&self, gc: &GarbageCollector) {
                    if gc.domain_index() == 1 {
                        assert!(gc.in_flight() >= 8);
                    }
                    self.entered.send(()).expect("entered");
                    lock(&self.resume)
                        .recv_timeout(std::time::Duration::from_secs(10))
                        .expect("resume");
                }
            }
            let gc = GarbageCollector::with_config(
                Configuration::builder()
                    .metadata_shards(16)
                    .work_stealing(true)
                    .independent_marking(independent)
                    .diagnostics(true)
                    .mark_batch_size(8)
                    .incremental_step_size(64)
                    .max_pause_time_us(0)
                    .allocation_threshold(usize::MAX)
                    .full_collection_interval(usize::MAX)
                    .build(),
            );
            let (entered, ready) = std::sync::mpsc::channel();
            let (resume, released) = std::sync::mpsc::channel();
            let blocked = Gc::new(
                &gc,
                Block {
                    entered,
                    resume: Mutex::new(released),
                },
            );
            let roots: Vec<_> = (0..15).map(|_| Gc::new(&gc, Leaf)).collect();
            drop(Gc::new(&gc, 99_u64));
            gc.collect();
            let worker = {
                let gc = gc.clone();
                thread::spawn(move || gc.step())
            };
            ready
                .recv_timeout(std::time::Duration::from_secs(10))
                .expect("first callback");
            for _ in 0..16 {
                assert!(!gc.step());
            }
            assert_eq!(gc.domains[1].mark_queue.try_is_empty(), Some(true));
            assert_eq!(gc.in_flight(), 8);
            assert_eq!(gc.statistics()?.objects_marked, 8);
            assert_eq!(gc.statistics()?.collections_completed, 0);
            assert_eq!(gc.statistics()?.objects_swept, 0);
            // A second generation may scan other tasks but cannot retire either snapshot.
            gc.collect_young();
            for _ in 0..16 {
                assert!(!gc.step());
            }
            assert_eq!(gc.statistics()?.collections_completed, 0);
            assert_eq!(gc.statistics()?.objects_swept, 0);
            // Both generations will execute Block once; release both before draining.
            resume.send(()).expect("release full");
            resume.send(()).expect("release young");
            assert!(!worker.join().expect("worker"));
            while !gc.step() {}
            assert_eq!(gc.in_flight(), 0);
            assert_eq!(gc.statistics()?.collections_completed, 2);
            if independent {
                assert!(gc.diagnostics().independent_mark_steps > 0);
            }
            assert_eq!(gc.statistics()?.objects_marked, 32);
            drop((roots, blocked));
            gc.stop()?;
            assert_eq!(gc.statistics()?.live_objects, 0);
        }
        Ok(())
    }

    #[test]
    fn idle_steps_repair_attached_contexts_while_another_context_writer_is_parked() -> Result<()> {
        let gc = GarbageCollector::with_config(
            Configuration::builder()
                .root_storage(crate::RootStorage::Contexts)
                .allocation_threshold(usize::MAX)
                .build(),
        );
        let parked = gc.register_context();
        let peer = gc.register_context();
        let reserve = || {
            let keep: Vec<_> = (0..crate::roots::SLOTS)
                .map(|_| gc.reserve_owned_root(0x1000))
                .collect();
            (keep, gc.reserve_owned_root(0x1000))
        };
        let (first, cancel_first) = parked.with(reserve);
        let (second, cancel_second) = peer.with(reserve);
        parked.0.roots.with_all_writers_held(|| {
            drop(cancel_first);
            peer.0.roots.with_all_writers_held(|| drop(cancel_second));
            assert!(parked.0.roots.repair_pending() && peer.0.roots.repair_pending());
            for _ in 0..256 {
                gc.step();
            }
            assert!(!peer.0.roots.repair_pending(), "idle peer never repaired");
            assert!(parked.0.roots.repair_pending());
            assert_eq!(peer.0.roots.statistics().pages, 1);
        });
        for _ in 0..256 {
            if gc.step() {
                break;
            }
        }
        assert!(!parked.0.roots.repair_pending());
        assert_eq!(parked.0.roots.statistics().pages, 1);
        assert_eq!(gc.statistics()?.collections_completed, 0);
        drop((first, second, parked, peer));
        gc.stop()
    }

    #[test]
    fn removing_an_escaped_typed_root_services_its_context_without_a_collection() -> Result<()> {
        let gc = GarbageCollector::with_config(
            Configuration::builder()
                .root_storage(crate::RootStorage::Contexts)
                .allocation_threshold(usize::MAX)
                .build(),
        );
        let owner = Gc::try_new(&gc, 13_u64)?;
        let context = gc.register_context();
        let mut aliases = context.with(|| {
            (0..=crate::roots::SLOTS)
                .map(|_| Gc::<u64>::from_handle_rooted(&gc, owner.as_handle_i64()))
                .collect::<Result<Vec<_>>>()
        })?;
        let last = aliases.pop().unwrap();
        context.0.roots.with_all_writers_held(|| drop(last));
        assert!(context.0.roots.repair_pending());
        assert!((0..256).any(|_| gc.step()));
        assert!(!context.0.roots.repair_pending() && !gc.contexts.repairs.pending());
        assert_eq!(context.0.roots.statistics().pages, 1);
        assert_eq!(gc.statistics()?.collections_completed, 0);
        assert!(
            aliases
                .iter()
                .all(|edge| edge.identity() == owner.identity())
        );
        drop((aliases, owner, context));
        gc.stop()
    }

    #[cfg(not(target_family = "wasm"))]
    #[test]
    fn a_new_worker_pool_finds_context_repairs_published_between_generations() -> Result<()> {
        let gc = GarbageCollector::with_config(
            Configuration::builder()
                .root_storage(crate::RootStorage::Contexts)
                .build(),
        );
        gc.start();
        let old_pool = gc.worker_signals.load_full().unwrap();
        gc.stop()?;
        let context = gc.register_context();
        let reserve = || context.with(|| gc.reserve_owned_root(0x1000));
        let keep: Vec<_> = (0..crate::roots::SLOTS).map(|_| reserve()).collect();
        let cancel = reserve();
        context.0.roots.with_all_writers_held(|| drop(cancel));
        assert!(gc.contexts.repairs.pending());
        let completed = gc.statistics()?.collections_completed;
        gc.start();
        old_pool.wake();
        let started = std::time::Instant::now();
        while gc.contexts.repairs.pending() {
            assert!(started.elapsed() < std::time::Duration::from_secs(5));
            thread::yield_now();
        }
        assert_eq!(context.0.roots.statistics().pages, 1);
        assert_eq!(gc.statistics()?.collections_completed, completed);
        drop(keep);
        gc.stop()
    }

    #[cfg(not(target_family = "wasm"))]
    #[test]
    fn cancelled_root_reservation_wakes_an_idle_worker_to_repair_pages() -> Result<()> {
        let gc = GarbageCollector::with_config(
            Configuration::builder()
                .root_storage(crate::RootStorage::Slots)
                .build(),
        );
        let roots = gc.scanned_slots().expect("paged root fixture");
        // Fill the cached page so cancelling the next reservation releases a page.
        let reservations: Vec<_> = (0..crate::roots::SLOTS)
            .map(|_| gc.reserve_owned_root(0x1000))
            .collect();
        let cancelled = gc.reserve_owned_root(0x1000);
        gc.start();
        let signals = gc.worker_signals.load_full().expect("worker pool");
        let wait_until = |condition: &dyn Fn() -> bool| {
            let start = std::time::Instant::now();
            while !condition() {
                assert!(start.elapsed() < std::time::Duration::from_secs(5));
                thread::yield_now();
            }
        };
        wait_until(&|| {
            signals
                .workers
                .first()
                .is_some_and(|worker| worker.waiting.load(Ordering::Acquire))
        });
        roots.with_all_writers_held(|| {
            drop(cancelled);
            assert!(roots.repair_pending());
        });
        wait_until(&|| !roots.repair_pending());
        assert_eq!(roots.statistics().pages, 1);
        drop(reservations);
        gc.stop()
    }

    #[cfg(not(target_family = "wasm"))]
    #[test]
    fn idle_worker_does_not_block_on_a_busy_coordinator() -> Result<()> {
        let gc = GarbageCollector::with_config(Configuration::builder().diagnostics(true).build());
        let coordinator = lock(&gc.coordinator);
        gc.start();
        gc.wake();
        let started = std::time::Instant::now();
        while gc.diagnostics().worker_backoffs == 0
            && started.elapsed() < std::time::Duration::from_secs(1)
        {
            thread::sleep(std::time::Duration::from_millis(1));
        }
        let backed_off = gc.diagnostics().worker_backoffs != 0;
        drop(coordinator);
        gc.stop()?;
        assert!(
            backed_off,
            "idle handoff must back off without blocking on the coordinator"
        );
        Ok(())
    }

    #[test]
    fn root_published_after_slot_scan_is_shaded_in_each_generation() -> Result<()> {
        for full in [false, true] {
            let gc = GarbageCollector::with_config(
                Configuration::builder()
                    .root_storage(crate::RootStorage::Slots)
                    .incremental_step_size(1)
                    .max_pause_time_us(0)
                    .allocation_threshold(usize::MAX)
                    .build(),
            );
            // SAFETY: the fixture drives only root enumeration before rooting this allocation.
            let value = unsafe { Gc::with_collector_unrooted(&gc, 7_u64) };
            if full {
                gc.collect();
            } else {
                gc.collect_young();
            }
            let index = usize::from(full);
            for _ in 0..100 {
                gc.step();
                if lock(&gc.domain_at(index).cycle).phase == Phase::Roots {
                    break;
                }
            }
            assert_eq!(lock(&gc.domain_at(index).cycle).phase, Phase::Roots);
            let root = gc.create_root_guard(value);
            while !gc.step() {}
            assert_eq!(**root, 7);
            assert_eq!(gc.statistics()?.live_objects, 1);
            drop(root);
            gc.stop()?;
            assert_eq!(gc.statistics()?.live_objects, 0);
            assert_eq!(gc.root_statistics().pages, 0);
        }
        Ok(())
    }

    #[test]
    fn context_records_cover_both_snapshots_and_late_dirty_owners() -> Result<()> {
        let gc = GarbageCollector::with_config(
            Configuration::builder()
                .root_storage(crate::RootStorage::Contexts)
                .barrier_buffer_size(64)
                .diagnostics(true)
                .incremental_step_size(1)
                .max_pause_time_us(0)
                .allocation_threshold(usize::MAX)
                .full_collection_interval(usize::MAX)
                .build(),
        );
        let context = gc.register_context();
        let parent = context
            .with(|| Gc::try_new(&gc, crate::sync::RwLock::new(Vec::<Gc<u64>>::new())))?
            .into_rooted();
        gc.collect_full()?;
        gc.collect_full()?;
        assert!(!gc.is_young_allocation(parent.as_ptr() as usize));
        gc.collect_young();
        gc.collect();
        for _ in 0..1000 {
            gc.step();
            if lock(&gc.domains[0].cycle).phase == Phase::Mark {
                break;
            }
        }
        assert_eq!(lock(&gc.domains[0].cycle).phase, Phase::Mark);
        assert!(gc.domains.iter().all(Domain::active));
        let child = context.with(|| Gc::try_new(&gc, 42_u64))?.into_rooted();
        let weak = crate::GcWeak::new(&child, crate::ReferenceKind::Weak, 0);
        let address = parent.as_ptr() as usize;
        let version = gc
            .objects
            .lock(address)
            .get(&address)
            .expect("parent")
            .mutation
            .load(Ordering::Acquire);
        context.with(|| parent.write().push(child));
        assert!(gc.contexts.pending.load(Ordering::Acquire) > 0);
        assert!(
            gc.objects
                .lock(address)
                .get(&address)
                .expect("parent")
                .mutation
                .load(Ordering::Acquire)
                > version
        );
        while !gc.step() {}
        assert_eq!(gc.contexts.pending.load(Ordering::Acquire), 0);
        gc.collect_young();
        while !gc.step() {}
        assert_eq!(weak.upgrade().as_deref(), Some(&42));
        context.with(|| parent.write().clear());
        drop(parent);
        gc.stop()?;
        assert!(weak.upgrade().is_none());
        assert_eq!(gc.statistics()?.live_objects, 0);
        Ok(())
    }

    #[cfg(not(target_family = "wasm"))]
    #[test]
    fn remembered_pruning_does_not_wait_for_unrelated_publication() -> Result<()> {
        let gc =
            GarbageCollector::with_config(Configuration::builder().metadata_shards(16).build());
        let root = Gc::try_new(&gc, 42_u64)?;
        let address = root.as_ptr() as usize;
        let identity = root.identity();
        gc.remember(&root);
        let version = gc
            .objects
            .lock(address)
            .get(&address)
            .expect("root")
            .mutation
            .load(Ordering::Acquire);
        let publication = gc.allocation_guard();
        let (send, receive) = std::sync::mpsc::channel();
        let worker = {
            let gc = gc.clone();
            thread::spawn(move || {
                gc.prune_remembered(address, identity, version, false);
                send.send(()).expect("prune completion");
            })
        };
        let completed = receive
            .recv_timeout(std::time::Duration::from_secs(1))
            .is_ok();
        drop(publication); // let a failing legacy writer finish before joining
        worker
            .join()
            .unwrap_or_else(|error| std::panic::resume_unwind(error));
        assert!(
            completed,
            "pruning waited for the heap-wide publication writer"
        );
        assert!(!gc.remembered.lock(address).contains(address));
        // A later mutation must invalidate the old scan and remain remembered.
        gc.remember(&root);
        gc.prune_remembered(address, identity, version, false);
        assert!(gc.remembered.lock(address).contains(address));
        drop(root);
        gc.stop()?;
        Ok(())
    }

    #[cfg(all(not(target_family = "wasm"), not(miri)))]
    #[test]
    fn optional_pruning_defers_while_either_required_shard_is_parked() -> Result<()> {
        use std::sync::mpsc;
        let gc = GarbageCollector::new();
        let root = Gc::try_new(&gc, 42_u64)?;
        let address = root.as_ptr().addr();
        let id = root.identity();
        gc.remember(&root);
        let version = gc
            .objects
            .lookup(address)
            .unwrap()
            .mutation
            .load(Ordering::Acquire);
        for object_shard in [false, true] {
            let objects = object_shard.then(|| gc.objects.lock(address));
            let remembered = (!object_shard).then(|| gc.remembered.lock(address));
            let (done, result) = mpsc::channel();
            let completed = thread::scope(|scope| {
                let heap = &gc;
                scope.spawn(move || {
                    heap.prune_remembered(address, id, version, false);
                    done.send(()).unwrap();
                });
                let completed = result.recv_timeout(std::time::Duration::from_secs(2));
                drop((objects, remembered));
                completed
            });
            assert!(completed.is_ok(), "optional pruning waited on a shard");
            assert!(gc.remembered.lock(address).contains(address));
        }
        gc.prune_remembered(address, id, version, false);
        assert!(!gc.remembered.lock(address).contains(address));
        Ok(())
    }

    #[cfg(not(target_family = "wasm"))]
    #[test]
    fn remembered_barriers_progress_with_unrelated_generation_shards_held() -> Result<()> {
        let gc =
            GarbageCollector::with_config(Configuration::builder().metadata_shards(16).build());
        let root = Gc::try_new(&gc, 42_u64)?;
        let address = root.as_ptr() as usize;
        let other = crate::index::address_bucket(address) ^ 1;
        let objects = gc.objects.lock_bucket(other);
        let remembered = gc.remembered.lock_bucket(other);
        let (send, receive) = std::sync::mpsc::channel();
        let worker = {
            let gc = gc.clone();
            thread::spawn(move || {
                gc.remember(&root);
                send.send(root).expect("barrier completion");
            })
        };
        let completed = receive.recv_timeout(std::time::Duration::from_secs(1));
        drop((objects, remembered));
        worker
            .join()
            .unwrap_or_else(|error| std::panic::resume_unwind(error));
        let root = completed.map_err(|error| crate::Error::SyncError(error.to_string()))?;
        assert!(gc.remembered.lock(address).contains(address));
        assert!(gc.objects.contains_young(address));
        drop(root);
        gc.stop()?;
        assert_eq!(gc.statistics()?.live_objects, 0);
        Ok(())
    }

    #[cfg(not(target_family = "wasm"))]
    #[test]
    fn conditional_owner_construction_releases_weak_decision_admission() -> Result<()> {
        let gc = GarbageCollector::new();
        let child = Gc::try_new(&gc, 42_u64)?;
        let link = crate::GcWeak::new(&child, crate::ReferenceKind::Final, 0);
        let mut construction = crate::construction::Roots::new(&gc);
        let scope = construction.enter();
        assert!(gc.publication.try_write().is_some());
        link.set_owner(usize::MAX); // pending owner has no registered metadata yet
        assert!(gc.publication.try_write().is_some());
        drop(scope);
        assert!(gc.publication.try_write().is_some());
        drop(link);
        drop(child);
        gc.stop()?;
        Ok(())
    }

    #[cfg(not(target_family = "wasm"))]
    #[test]
    fn buffered_drain_does_not_wait_for_unrelated_publication() -> Result<()> {
        let gc = GarbageCollector::with_config(
            Configuration::builder()
                .barrier_buffer_size(4)
                .incremental_step_size(1)
                .max_pause_time_us(0)
                .allocation_threshold(usize::MAX)
                .build(),
        );
        let context = gc.register_context();
        let root = Gc::try_new(&gc, 42_u64)?;
        gc.collect();
        // Drive to tracing without allowing the one-unit step to close it.
        while lock(&gc.domains[1].cycle).phase != Phase::Mark {
            gc.step();
        }
        context.with(|| gc.write_barrier(&root));
        assert_eq!(gc.contexts.pending.load(Ordering::Acquire), 1);
        let publication = gc.allocation_guard();
        let (send, receive) = std::sync::mpsc::channel();
        let worker = {
            let gc = gc.clone();
            thread::spawn(move || {
                gc.step();
                send.send(()).expect("drain completion");
            })
        };
        let completed = receive
            .recv_timeout(std::time::Duration::from_secs(5))
            .is_ok();
        drop(publication);
        worker
            .join()
            .unwrap_or_else(|error| std::panic::resume_unwind(error));
        assert!(
            completed,
            "buffer draining waited for an exclusive publication gate"
        );
        assert_eq!(gc.contexts.pending.load(Ordering::Acquire), 0);
        drop(root);
        drop(context);
        gc.stop()?;
        Ok(())
    }

    #[cfg(not(target_family = "wasm"))]
    #[test]
    fn a_popped_context_record_excludes_closure_without_holding_publication() -> Result<()> {
        let gc = GarbageCollector::with_config(
            Configuration::builder()
                .barrier_buffer_size(4)
                .incremental_step_size(1)
                .max_pause_time_us(0)
                .allocation_threshold(usize::MAX)
                .build(),
        );
        let context = gc.register_context();
        let root = Gc::try_new(&gc, 42_u64)?;
        gc.collect();
        while lock(&gc.domains[1].cycle).phase != Phase::Mark {
            gc.step();
        }
        context.with(|| gc.write_barrier(&root));
        let objects = gc.objects.lock(root.as_ptr() as usize);
        let (finished, done) = std::sync::mpsc::channel();
        let (claimed, claim) = std::sync::mpsc::channel();
        let (release, resume) = std::sync::mpsc::channel();
        let flusher = thread::spawn(move || {
            barrier_progress_tests::park_next_claim(ClaimPause {
                entered: claimed,
                resume,
            });
            context.flush();
            finished.send(()).expect("clean record acknowledged");
        });
        let claimed = claim.recv_timeout(std::time::Duration::from_secs(10));
        let pending = gc.contexts.pending.load(Ordering::Acquire);
        let publication_free = gc.publication.try_write().is_some();
        gc.step();
        let active = gc.domains[1].active();
        let ticket = gc.completed_full_ticket();
        assert!(release.send(()).is_ok(), "resume flusher");
        let without_shard = done.recv_timeout(std::time::Duration::from_secs(2));
        drop(objects);
        flusher
            .join()
            .unwrap_or_else(|error| std::panic::resume_unwind(error));
        assert!(claimed.is_ok());
        assert_eq!(without_shard, Ok(()));
        assert_eq!(pending, 1);
        assert!(publication_free && active);
        assert_eq!(ticket, 0);
        assert_eq!(gc.contexts.pending.load(Ordering::Acquire), 0);
        gc.collect_full()?;
        assert_eq!(**root, 42);
        drop(root);
        gc.stop()?;
        Ok(())
    }

    #[test]
    fn root_and_weak_identifiers_skip_live_entries_after_wrap() -> Result<()> {
        let gc = GarbageCollector::new();
        let first = Gc::new(&gc, 1_u64);
        let first_weak = crate::GcWeak::new(&first, crate::ReferenceKind::Weak, 0);
        gc.next_root_id.store(usize::MAX, Ordering::Relaxed);
        gc.next_weak_id.store(usize::MAX, Ordering::Relaxed);
        let second = Gc::new(&gc, 2_u64);
        let third = Gc::new(&gc, 3_u64);
        let second_weak = crate::GcWeak::new(&second, crate::ReferenceKind::Weak, 0);
        let third_weak = crate::GcWeak::new(&third, crate::ReferenceKind::Weak, 0);
        gc.collect_full()?;
        assert_eq!(gc.statistics()?.live_objects, 3);
        drop((first, second, third));
        gc.collect_full()?;
        assert!(first_weak.upgrade().is_none());
        assert!(second_weak.upgrade().is_none());
        assert!(third_weak.upgrade().is_none());
        Ok(())
    }
}

#[cfg(test)]
#[path = "reference_tests.rs"]
mod reference_tests;

#[cfg(all(test, not(target_family = "wasm"), not(miri)))]
pub(crate) use publication_tests::{pause_after_tree_detach, pause_while_graph_slot_owned};
#[cfg(test)]
#[path = "publication_tests.rs"]
mod publication_tests;

#[cfg(test)]
#[path = "allocation_credit_tests.rs"]
mod allocation_credit_tests;

#[cfg(test)]
#[path = "initial_root_tests.rs"]
mod initial_root_tests;

#[cfg(test)]
#[path = "private_allocation_tests.rs"]
mod private_allocation_tests;

#[cfg(test)]
#[path = "tree_root_progress_tests.rs"]
mod tree_root_progress_tests;
