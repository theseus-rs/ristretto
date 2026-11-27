mod barriers;
mod marking;
mod phases;
mod reclamation;
mod references;
mod relocation_work;
mod roots;
mod trace_batch;
mod trace_impls;
mod workers;

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
    #[cfg(not(target_family = "wasm"))]
    static SPARE_FULL_CLEANUP: std::cell::Cell<Option<Arc<AtomicUsize>>> = const { std::cell::Cell::new(None) };
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
    #[cfg(not(target_family = "wasm"))]
    pending_full_cleanup: arc_swap::ArcSwapOption<AtomicUsize>,
    request_ticket: AtomicU64,
    completed_ticket: AtomicU64,
}

impl Domain {
    fn issue_ticket(&self) -> u64 {
        self.request_ticket.fetch_add(1, Ordering::AcqRel) + 1
    }

    #[cfg(not(target_family = "wasm"))]
    fn enroll_full_cleanup(&self) -> Arc<AtomicUsize> {
        if let Some(pending) = self.pending_full_cleanup.load_full() {
            return pending;
        }
        // The initial reservation belongs to the full cycle. An already
        // completed numerical ticket cannot satisfy an enrolment still pending.
        let fresh = SPARE_FULL_CLEANUP
            .try_with(std::cell::Cell::take)
            .ok()
            .flatten()
            .unwrap_or_else(|| Arc::new(AtomicUsize::new(1)));
        let previous = self
            .pending_full_cleanup
            .compare_and_swap(std::ptr::null::<AtomicUsize>(), Some(fresh.clone()));
        previous.as_ref().cloned().unwrap_or(fresh)
    }

    #[cfg(not(target_family = "wasm"))]
    fn capture_full_cleanup(&self, cycle: &mut Cycle) {
        if self.pending_full_cleanup.load().is_none() {
            return;
        }
        if cycle.active_full_cleanup.is_some() {
            // A failed phase commit retained its original group. Leave a full
            // request for the callers who enrolled since the unsuccessful cut.
            self.publish_request();
        } else {
            cycle.active_full_cleanup = self.pending_full_cleanup.swap(None).map(FullCleanup);
        }
    }

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
            #[cfg(not(target_family = "wasm"))]
            pending_full_cleanup: arc_swap::ArcSwapOption::empty(),
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
    #[cfg(not(target_family = "wasm"))]
    full_cleanup: Option<FullCleanup>,
}

/// A synchronous full caller owns a count of cleanup tasks admitted by its snapshot.
/// Moving a task to another helper transfers the same obligation.
#[cfg(not(target_family = "wasm"))]
#[derive(Debug)]
struct FullCleanup(Arc<AtomicUsize>);

#[cfg(not(target_family = "wasm"))]
impl FullCleanup {
    fn reserve(&self) -> Self {
        self.0.fetch_add(1, Ordering::Relaxed);
        Self(self.0.clone())
    }

    fn recycle(mut completed: Arc<AtomicUsize>) {
        // Other callers can still be observing this group's terminal zero.
        // Only its final owner may prepare it for another request, possibly on another heap.
        if let Some(unique) = Arc::get_mut(&mut completed) {
            debug_assert_eq!(*unique.get_mut(), 0);
            *unique.get_mut() = 1;
            let _ = SPARE_FULL_CLEANUP.try_with(|spare| spare.set(Some(completed)));
        }
    }
}

#[cfg(not(target_family = "wasm"))]
impl Drop for FullCleanup {
    fn drop(&mut self) {
        self.0.fetch_sub(1, Ordering::AcqRel);
    }
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
            #[cfg(not(target_family = "wasm"))]
            full_cleanup: None,
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

    /// Whether releasing a managed write guard can omit both `unroot` and the
    /// owner notification for the current value. The guard holds exclusive access.
    /// Returning true requires an empty strong and conditional graph, a no-op
    /// `unroot` (including custom, nested and relocation hooks), and a false
    /// `needs_write_barrier` result after that demotion. These guarantees
    /// describe the current payload, including after replacing a different value.
    /// The collector still revokes trace coverage, unlocks and accounts for the
    /// payload pin. The query must not allocate, lock or call the collector.
    fn can_skip_unroot_after_write(&self) -> bool {
        false
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

    /// Whether dropping any initialized value directly, without calling
    /// `clear_step` first, can finish in bounded work without invoking user
    /// callbacks or waiting on another thread, assuming allocator progress.
    /// Containers use this when truncation runs their elements' Rust destructors.
    #[must_use]
    fn has_bounded_drop() -> bool
    where
        Self: Sized,
    {
        !std::mem::needs_drop::<Self>()
    }

    /// Releases outgoing edges in bounded batches before destruction of unreachable values.
    fn clear_step(&mut self, _offset: usize, _budget: usize) -> TraceProgress {
        TraceProgress::Complete
    }

    /// Bytes of separately allocated storage owned exclusively by this value.
    fn external_size(&self) -> usize {
        0
    }

    /// Total separately owned storage for the initialized elements of a slice.
    /// An unrepresentable sum saturates so allocation admission reports exhaustion.
    #[must_use]
    fn slice_external_size(items: &[Self]) -> usize
    where
        Self: Sized,
    {
        items
            .iter()
            .map(Self::external_size)
            .fold(0, usize::saturating_add)
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
    #[cfg(not(target_family = "wasm"))]
    active_full_cleanup: Option<FullCleanup>,
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
            #[cfg(not(target_family = "wasm"))]
            active_full_cleanup: None,
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
pub(crate) use native_callbacks::Turn as CallbackTurn;
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
                managed_budget: Arc::new(crate::allocation_budget::AllocationBudget::new(
                    configuration.max_heap_bytes,
                )),
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
        #[cfg(feature = "experimental-identity-index")]
        let previous_identity = self.identities.get(metadata.id.to_i64().cast_unsigned());
        #[cfg(not(feature = "experimental-identity-index"))]
        let previous_identity = identity.as_ref().err().copied().flatten();
        crate::registration_key::validate(address, metadata.id, previous_identity)?;
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

    /// Requests a full collection. On cooperative targets, call `step` at safe boundaries.
    pub fn collect(&self) {
        self.request_full_cycle();
    }

    fn request_full_cycle(&self) -> u64 {
        let ticket = self.domains[1].issue_ticket();
        self.publish_full_request();
        ticket
    }

    fn publish_full_request(&self) {
        self.domains[1].publish_request();
        self.wake();
    }

    #[cfg(not(target_family = "wasm"))]
    fn request_full_cleanup(&self) -> (u64, Arc<AtomicUsize>) {
        let domain = &self.domains[1];
        let ticket = domain.issue_ticket();
        let cleanup = domain.enroll_full_cleanup();
        self.publish_full_request();
        (ticket, cleanup)
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
    /// Native callers help finish ready host cleanup retired by their snapshot.
    /// A storage lease or a contended cleanup can keep its charge until a later step.
    ///
    /// # Errors
    /// Reserved for collection errors; currently collection itself is infallible.
    pub fn collect_full(&self) -> Result<()> {
        #[cfg(not(target_family = "wasm"))]
        if !CleanupScope::contains(self) {
            let (ticket, cleanup) = self.request_full_cleanup();
            self.wait_full_ticket(ticket);
            loop {
                let jobs = self.callback_jobs.load(Ordering::Acquire);
                if jobs != 0
                    && let Some(pool) = self.worker_signals.load_full()
                {
                    pool.callbacks.help_ready(jobs);
                }
                if cleanup.load(Ordering::Acquire) == 0 {
                    FullCleanup::recycle(cleanup);
                    return Ok(());
                }
                self.step_inner(false);
                thread::yield_now();
            }
        }
        self.wait_full_cycle();
        Ok(())
    }

    fn wait_full_cycle(&self) {
        self.wait_full_ticket(self.request_full_cycle());
    }

    fn wait_full_ticket(&self, ticket: u64) {
        if self.diagnostics_enabled() {
            self.diagnostics
                .full_collection_calls
                .fetch_add(1, Ordering::Relaxed);
        }
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

    #[cfg(test)]
    pub(crate) fn reserve_private_bytes(
        &self,
        bytes: usize,
    ) -> Result<crate::allocation_budget::Reservation> {
        self.managed_budget.reserve(bytes, self.this.clone())
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

    #[cfg(not(target_family = "wasm"))]
    fn terminal_callback_turn(&self) -> Option<native_callbacks::Turn> {
        if self.callback_jobs.load(Ordering::Acquire) == 0 {
            return None;
        }
        let pool = self.worker_signals.load();
        let pool = pool.as_ref()?;
        pool.callbacks.terminal_turn(pool)
    }

    #[cfg(not(target_family = "wasm"))]
    async fn wait_for_dispatch(
        &self,
        dispatch: &CallbackTurn,
        request: &crate::retirement_grants::Request,
        size: usize,
    ) -> Result<()> {
        self.ensure_refund_notifier(false)?;
        if let Some(notifier) = self.refund_notifier.load_full() {
            return notifier.wait_for_turn(self, dispatch, request, size).await;
        }
        if dispatch.pending() {
            return Err(crate::Error::SyncError(
                "no callback dispatch worker".into(),
            ));
        }
        Ok(())
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
        #[cfg(not(target_family = "wasm"))]
        if let Some(pool) = self.worker_signals.load().as_ref() {
            pool.callbacks.wake_pressure();
        }
        #[cfg(not(target_family = "wasm"))]
        let mut terminal_service = None;
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
                #[cfg(not(target_family = "wasm"))]
                {
                    // Enroll once per demand and keep checking real funds on each
                    // poll. A running destructor, unavailable worker or empty
                    // queue cannot extend the wait for the user callback to return.
                    let turn =
                        terminal_service.get_or_insert_with(|| self.terminal_callback_turn());
                    if let Some(dispatch) = turn.as_ref().filter(|turn| turn.pending()) {
                        self.wait_for_dispatch(dispatch, &demand.request, size)
                            .await?;
                        continue;
                    }
                    if turn.take().is_some() {
                        empty_cycles = 1;
                        continue;
                    }
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

    /// Records a published allocation's logical live bytes and allocation debt.
    /// Private reservations are charged to the managed budget before publication.
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
    /// The value excludes host allocator overhead and JVM storage outside the managed budget.
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

#[cfg(test)]
mod tests;

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
