//! Object metadata and pointer management for garbage collection.

use crate::Finalize;
use crate::Trace;
use crate::finalizer::FinalizerFn;
use crate::pointers::SafePtr;
use crate::pointers::TracePtr;
use portable_atomic::AtomicU64;
use std::sync::atomic::{AtomicBool, AtomicU8, AtomicUsize, Ordering};
use std::sync::{Arc, Mutex, Weak};
use tracing::warn;

#[path = "birth_visibility.rs"]
mod birth_visibility;

#[path = "storage_state.rs"]
mod storage_state;

#[path = "mark_state.rs"]
mod mark_state;
use mark_state::MarkWord;
pub(crate) use mark_state::{EPOCH_STRIDE, Retention};

/// A storage lease is deliberately separate from logical reachability. It admits
/// no weak upgrade or root resurrection after the collector's retirement decision.
#[cfg(test)]
pub(crate) struct StorageLease(MetadataRef);

/// Conditional references validate object IDs in the registry; they do not need
/// weak metadata ownership. Internal work and storage leases share one counter.
#[path = "metadata_ref.rs"]
mod metadata_ref;
pub(crate) use metadata_ref::MetadataRef;

#[cfg(test)]
impl Drop for StorageLease {
    fn drop(&mut self) {
        if self.0.storage.release()
            && let Some(gc) = self.0.collector.upgrade()
        {
            gc.storage_ready(self.0.clone());
        }
    }
}

/// The task already owns the descriptor. Clone only if the last borrower
/// must publish a retirement notification to its collector.
pub(crate) struct CallbackLease<'a> {
    metadata: &'a MetadataRef,
    scanner: bool,
}

impl Drop for CallbackLease<'_> {
    fn drop(&mut self) {
        let ready = if self.scanner {
            self.metadata.storage.release_scan()
        } else {
            self.metadata.storage.release()
        };
        if ready && let Some(gc) = self.metadata.collector.upgrade() {
            gc.storage_ready(self.metadata.clone());
        }
    }
}

/// Keep shutdown metadata alive through payload destruction, then release its
/// managed roots even if inert descriptor snapshots remain. No separate boxed
/// destructor allocation is introduced.
struct Payload {
    allocation: crate::allocation::Erased,
    owner: Option<Arc<dyn crate::HeapOwner>>,
}
type DropFn = Option<Payload>;

/// Type and callback identity are immutable and shared by all matching allocations.
struct Kind {
    type_id: std::any::TypeId,
    finalizer: Option<FinalizerFn>,
}

/// Metadata for tracking garbage-collected objects
///
/// This struct contains the pointer to the object, its size, manages the marked state, provides
/// type-safe dropping functionality, and supports finalizers.
pub(crate) struct ObjectMetadata {
    pub(crate) id: crate::ObjectId,
    size: usize,
    logical_size: usize,
    pub(crate) birth: birth_visibility::BirthVisibility,
    drop_fn: Mutex<DropFn>,
    kind: &'static Kind,
    finalized: AtomicBool,
    pub(crate) trace: TracePtr,
    epochs: [MarkWord; 2],
    pub(crate) age: AtomicUsize,
    pub(crate) old_birth: AtomicU64,
    pub(crate) mutation: AtomicUsize,
    // Discovery must not turn old descriptor snapshots into managed roots.
    // The matching strong owner lives only in the payload destruction record.
    pub(crate) owner: Option<Weak<dyn crate::HeapOwner>>,
    // Installing this link transfers the reservation's logical charge. It stays
    // charged until this descriptor destroys its payload, even before publication
    // or after removal from the catalog. Old MetadataRefs alone retain no charge.
    pub(crate) collector: Weak<crate::GarbageCollector>,
    storage: storage_state::StorageState,
    pub(crate) relocation: Option<Box<crate::relocation_history::History>>,
}

/// The sole storage destructor refunds on every exit after payload drop glue,
/// including an unwind from diagnostics reporting a panicking destructor.
struct Refund<'a> {
    metadata: &'a ObjectMetadata,
    collector: Option<&'a crate::GarbageCollector>,
}

impl Drop for Refund<'_> {
    fn drop(&mut self) {
        if let Some(collector) = self.collector {
            self.metadata.refund_publication(collector);
            collector.cancel_allocation(self.metadata.size);
        } else if let Some(collector) = self.metadata.collector.upgrade() {
            self.metadata.refund_publication(&collector);
            collector.cancel_allocation(self.metadata.size);
        }
    }
}

/// Ensure finalizer/diagnostic unwind reaches the guarded payload destructor.
struct DestroyPayload<'a>(&'a ObjectMetadata);

impl Drop for DestroyPayload<'_> {
    fn drop(&mut self) {
        self.0.run_drop_fn();
    }
}

impl ObjectMetadata {
    pub(crate) fn is_type<T: 'static>(&self) -> bool {
        self.kind.type_id == std::any::TypeId::of::<T>()
    }

    #[cfg(test)]
    pub(crate) fn lease(metadata: &MetadataRef) -> Option<StorageLease> {
        metadata
            .storage
            .acquire()
            .then(|| StorageLease(metadata.clone()))
    }

    pub(crate) fn borrow_callback(
        metadata: &MetadataRef,
        scanner: bool,
    ) -> Result<CallbackLease<'_>, crate::TraceProgress> {
        if scanner {
            metadata
                .storage
                .acquire_scan()
                .map_err(|failure| match failure {
                    storage_state::ScanDenied::Busy => crate::TraceProgress::Busy,
                    storage_state::ScanDenied::Retired => crate::TraceProgress::Complete,
                })?;
        } else if !metadata.storage.acquire() {
            return Err(crate::TraceProgress::Complete);
        }
        Ok(CallbackLease { metadata, scanner })
    }

    pub(crate) fn trace_step(
        metadata: &MetadataRef,
        gc: &crate::GarbageCollector,
        offset: usize,
        budget: usize,
    ) -> crate::TraceProgress {
        let Some(step) = metadata.trace.step_fn else {
            return crate::TraceProgress::Complete;
        };
        match Self::borrow_callback(metadata, true) {
            Ok(_lease) => step(metadata.trace.ptr, gc, offset, budget),
            Err(progress) => progress,
        }
    }

    pub(crate) fn retire(&self) -> bool {
        self.storage.retire()
    }

    pub(crate) fn arm_retirement_notification(&self) -> bool {
        self.storage.arm_notification()
    }

    pub(crate) fn storage_ready(&self) -> bool {
        self.storage.ready()
    }

    fn mark_word(&self, index: usize) -> &MarkWord {
        if index == 0 {
            &self.epochs[0]
        } else {
            &self.epochs[1]
        }
    }

    pub(crate) fn marked(&self, index: usize, epoch: u64) -> bool {
        self.mark_word(index).marked(epoch)
    }

    pub(crate) fn retention(&self, index: usize, epoch: u64) -> Option<Retention> {
        self.mark_word(index).retention(epoch)
    }

    /// Liveness at a closed reference boundary, independent of later retention.
    pub(crate) fn marked_before(&self, index: usize, epoch: u64, final_retention: bool) -> bool {
        self.mark_word(index).marked_before(epoch, final_retention)
    }

    pub(crate) fn mark_black(&self, index: usize, epoch: u64) {
        self.mark_word(index).mark_black(epoch);
    }

    pub(crate) fn mark_born(&self, index: usize, epoch: u64) {
        self.mark_word(index).mark_born(epoch);
    }

    pub(crate) fn reset_mark(&self, index: usize) {
        self.mark_word(index).reset();
    }

    /// Callers clamp strength to the domain's phase floor before publication.
    pub(crate) fn mark(&self, index: usize, epoch: u64, strength: Retention) -> bool {
        self.mark_word(index).mark(epoch, strength)
    }

    /// Creates a new `ObjectMetadata` instance for a `Gc<T>`.
    pub(crate) fn new_for_gc<T: Send + Sync + Trace + 'static>(
        allocation: crate::allocation::Initialized<crate::gc::Allocation<T>>,
        size: usize,
        id: crate::ObjectId,
    ) -> Self {
        let ptr = allocation.ptr().cast::<T>();
        // Partial metadata construction destroys this owner on unwind. Successful
        // metadata takes and destroys it exactly once after storage retirement.
        let allocation = allocation.erase();
        let mut trace = TracePtr::new_from_ptr(ptr);
        // SAFETY: the unpublished initialized T remains owned by allocation.
        trace.budget = unsafe { (&*ptr).trace_work().clamp(1, 32) };
        // SAFETY: constructors exclusively own this initialized, unpublished T.
        let owner = unsafe { (&*ptr).heap_owner() };
        let discovery = owner.as_ref().map(Arc::downgrade);
        let drop_fn: DropFn = Some(Payload { allocation, owner });

        Self {
            id,
            size,
            logical_size: size - crate::gc::Allocation::<T>::layout().size() + size_of::<T>(),
            birth: birth_visibility::BirthVisibility::default(),
            drop_fn: Mutex::new(drop_fn),
            kind: &const {
                Kind {
                    type_id: std::any::TypeId::of::<T>(),
                    finalizer: None,
                }
            },
            finalized: AtomicBool::new(false),
            trace,
            epochs: std::array::from_fn(|_| MarkWord::default()),
            age: AtomicUsize::new(0),
            old_birth: AtomicU64::new(0),
            mutation: AtomicUsize::new(0),
            owner: discovery,
            collector: Weak::new(),
            storage: storage_state::StorageState::default(),
            relocation: None,
        }
    }

    /// Creates a new `ObjectMetadata` instance for a `Gc<T>` with finalizer support.
    pub(crate) fn new_for_gc_with_finalizer<T: Send + Sync + Finalize + Trace + 'static>(
        allocation: crate::allocation::Initialized<crate::gc::Allocation<T>>,
        size: usize,
        id: crate::ObjectId,
    ) -> Self {
        let mut metadata = Self::new_for_gc::<T>(allocation, size, id);
        metadata.kind = &const {
            Kind {
                type_id: std::any::TypeId::of::<T>(),
                finalizer: Some(crate::finalizer::finalize::<T>),
            }
        };
        metadata
    }

    /// Returns the pointer to the object.
    pub(crate) fn ptr(&self) -> SafePtr {
        SafePtr::from_ptr(self.trace.ptr)
    }

    /// Returns the size of the object.
    pub(crate) fn size(&self) -> usize {
        self.size
    }

    /// Logical payload bytes exclude the ordinary allocator's inline wrapper.
    pub(crate) fn logical_size(&self) -> usize {
        self.logical_size
    }

    pub(crate) fn record_publication(&self, collector: &crate::GarbageCollector) {
        self.birth.index();
        collector.record_published_bytes(self.logical_size);
    }

    fn refund_publication(&self, collector: &crate::GarbageCollector) {
        // StorageState::destroy and the taken Payload grant exactly one Refund.
        // An indexed descriptor retains its history even after storage retirement.
        if self.birth.indexed() {
            collector.release_published_bytes(self.logical_size);
        }
    }

    /// The initial concrete catalog adapter has no graph or shutdown callbacks.
    /// This does not certify that ordinary Rust destruction is bounded.
    #[cfg(feature = "experimental-canonical-leaves")]
    pub(crate) fn canonical_leaf_eligible(&self) -> bool {
        self.trace.step_fn.is_none() && self.owner.is_none() && self.kind.finalizer.is_none()
    }

    /// Drops the object using the type-safe drop function. This first calls the finalizer (if any),
    /// then properly calls the destructor and deallocates memory. Can only be called once;
    /// subsequent calls are no-ops, guarded by the finalizer claim and destructor take.
    ///
    /// # Caller contract
    ///
    /// Registry retirement must precede this call. Borrow admission is closed
    /// independently of registry visibility; existing leases can delay destruction.
    pub(crate) fn drop_object(&self) {
        self.retire();
        if self.storage_ready() {
            self.run_drop_closures();
        }
    }

    /// Claims the optional type callback once before invoking it outside any lock.
    /// Also used in shutdown phase 1 before destroying the remaining allocations.
    /// The `Finalize` contract still forbids accessing other GC managed objects.
    pub(crate) fn run_finalizer(&self) {
        let Some(finalizer) = self.kind.finalizer else {
            return;
        };
        if self.finalized.swap(true, Ordering::AcqRel) {
            return;
        }
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            // SAFETY: this metadata pairs T's callback with its original pointer.
            // The atomic claim runs it once, before the caller destroys storage.
            unsafe { finalizer(self.trace.ptr) };
        }))
        .unwrap_or_else(|_| {
            warn!("Finalizer panicked during object cleanup");
        });
    }

    /// Destroys the erased allocation owner if present. `take()`-guarded so it runs
    /// at most once. Used in shutdown phase 2 after all finalizers have run.
    ///
    /// Uses `try_lock` to avoid deadlocking on a poisoned mutex from a prior panic.
    pub(crate) fn run_drop_fn(&self) {
        self.run_drop_fn_with_collector(None);
    }

    /// Reclamation already borrows the owning heap. Validate that identity before
    /// claiming storage, then reuse the borrow instead of upgrading a weak owner.
    pub(crate) fn run_drop_fn_in(&self, collector: &crate::GarbageCollector) {
        assert!(
            std::ptr::eq(self.collector.as_ptr(), collector),
            "foreign descriptor refund collector"
        );
        self.run_drop_fn_with_collector(Some(collector));
    }

    pub(crate) fn destroy_retired(metadata: &mut MetadataRef, collector: &crate::GarbageCollector) {
        if let Some(unique) = MetadataRef::get_mut(metadata) {
            assert!(
                std::ptr::eq(unique.collector.as_ptr(), collector),
                "foreign descriptor refund collector"
            );
            if unique.storage.destroy_unique() {
                let payload = unique
                    .drop_fn
                    .get_mut()
                    .unwrap_or_else(std::sync::PoisonError::into_inner)
                    .take();
                unique.destroy_payload(payload, Some(collector));
            }
        } else {
            metadata.run_drop_fn_in(collector);
        }
    }

    fn run_drop_fn_with_collector(&self, collector: Option<&crate::GarbageCollector>) {
        if !self.storage.destroy() {
            return;
        }
        let drop_fn = if let Ok(mut guard) = self.drop_fn.try_lock() {
            guard.take()
        } else {
            warn!("Failed to acquire drop_fn lock during object cleanup; drop skipped");
            return;
        };
        self.destroy_payload(drop_fn, collector);
    }

    fn destroy_payload(&self, drop_fn: DropFn, collector: Option<&crate::GarbageCollector>) {
        if let Some(Payload { allocation, owner }) = drop_fn {
            {
                let _refund = Refund {
                    metadata: self,
                    collector,
                };
                std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                    drop(allocation);
                }))
                .unwrap_or_else(|_| {
                    warn!("Drop function panicked during object cleanup");
                });
                // Refund backing after its destruction, including diagnostic
                // unwind, before releasing any unrelated roots owned by owner.
            }
            // No descriptor mutex or collector coordination guard is acquired
            // here. Metadata snapshots retain only the weak discovery link.
            drop(owner);
        }
    }

    /// Runs the claimed finalizer and taken destructor sequentially, at most once each.
    fn run_drop_closures(&self) {
        let _destruction = DestroyPayload(self);
        self.run_finalizer();
    }
}

impl Drop for ObjectMetadata {
    fn drop(&mut self) {
        // The last Arc cannot disappear with a lease outstanding. This also
        // cleans retired storage if its collector was destroyed before release.
        if !self.storage.destroyed() {
            self.drop_object();
        }
    }
}

#[expect(clippy::missing_fields_in_debug)]
impl std::fmt::Debug for ObjectMetadata {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ObjectMetadata")
            .field("ptr", &self.trace.ptr)
            .field("size", &self.size)
            .field("epochs", &self.epochs.each_ref().map(MarkWord::word))
            .field("age", &self.age.load(Ordering::Acquire))
            .finish()
    }
}

#[cfg(test)]
#[expect(clippy::panic_in_result_fn)]
mod kind_tests {
    use super::*;
    use crate::{GarbageCollector, Gc, Result};

    struct Probe {
        finalized: Arc<AtomicUsize>,
        dropped: Arc<AtomicUsize>,
        panic: bool,
    }
    // SAFETY: this fixture contains no outgoing GC references.
    unsafe impl Trace for Probe {
        fn may_contain_gc_references() -> bool {
            false
        }
        fn trace(&self, _: &GarbageCollector) {}
    }
    impl Finalize for Probe {
        fn finalize(&self) {
            self.finalized.fetch_add(1, Ordering::Relaxed);
            assert!(!self.panic, "intentional finalizer panic");
        }
    }
    impl Drop for Probe {
        fn drop(&mut self) {
            self.dropped.fetch_add(1, Ordering::Relaxed);
        }
    }

    #[test]
    fn shared_type_identity_preserves_optional_finalization_and_panic_claim() -> Result<()> {
        for panic in [false, true] {
            // These Wasm targets abort on panic; native/Miri exercise unwinding.
            if panic && cfg!(target_family = "wasm") {
                continue;
            }
            let gc = GarbageCollector::new();
            let finalized = Arc::new(AtomicUsize::new(0));
            let dropped = Arc::new(AtomicUsize::new(0));
            let probe = || Probe {
                finalized: finalized.clone(),
                dropped: dropped.clone(),
                panic,
            };
            let plain = Gc::try_new(&gc, probe())?;
            let finalizable = Gc::new_with_finalizer(&gc, probe());
            for (root, expected) in [(&plain, 0), (&finalizable, 1)] {
                let metadata = gc
                    .metadata_for_address(root.as_ptr() as usize)
                    .expect("metadata");
                assert!(metadata.is_type::<Probe>());
                assert!(!metadata.is_type::<u64>());
                assert!(Gc::<Probe>::from_handle_rooted(&gc, root.as_handle_i64()).is_ok());
                assert!(Gc::<u64>::from_handle_rooted(&gc, root.as_handle_i64()).is_err());
                for _ in 0..3 {
                    metadata.run_finalizer();
                }
                assert_eq!(finalized.load(Ordering::Relaxed), expected);
                assert_eq!(dropped.load(Ordering::Relaxed), 0);
            }
            drop((plain, finalizable));
            gc.stop()?;
            assert_eq!(finalized.load(Ordering::Relaxed), 1);
            assert_eq!(dropped.load(Ordering::Relaxed), 2);
            assert_eq!(gc.managed_bytes(), 0);
        }
        Ok(())
    }

    #[cfg(not(target_family = "wasm"))]
    #[test]
    fn concurrent_finalizer_claims_execute_once() -> Result<()> {
        let gc = GarbageCollector::new();
        let finalized = Arc::new(AtomicUsize::new(0));
        let dropped = Arc::new(AtomicUsize::new(0));
        let root = Gc::new_with_finalizer(
            &gc,
            Probe {
                finalized: finalized.clone(),
                dropped: dropped.clone(),
                panic: false,
            },
        );
        let metadata = gc
            .metadata_for_address(root.as_ptr() as usize)
            .expect("metadata");
        let start = Arc::new(std::sync::Barrier::new(5));
        let mut workers = Vec::new();
        for _ in 0..4 {
            let metadata = metadata.clone();
            let start = start.clone();
            workers.push(std::thread::spawn(move || {
                start.wait();
                metadata.run_finalizer();
            }));
        }
        start.wait();
        for worker in workers {
            worker
                .join()
                .map_err(|_| crate::Error::SyncError("finalizer claimant panicked".into()))?;
        }
        assert_eq!(finalized.load(Ordering::Relaxed), 1);
        assert_eq!(dropped.load(Ordering::Relaxed), 0);
        drop((metadata, root));
        gc.stop()?;
        assert_eq!(finalized.load(Ordering::Relaxed), 1);
        assert_eq!(dropped.load(Ordering::Relaxed), 1);
        Ok(())
    }
}
