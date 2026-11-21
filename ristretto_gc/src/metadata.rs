//! Object metadata and pointer management for garbage collection.

use crate::Finalize;
use crate::finalizer::FinalizerFn;
use crate::pointers::SafePtr;
use crate::private_allocation::Allocation;
use std::sync::Mutex;
use std::sync::atomic::{AtomicBool, Ordering};
use tracing::warn;

/// Type-safe drop function for garbage-collected objects
type DropFn = Option<Box<dyn FnOnce() + Send + Sync>>;

/// New objects start marked (`true`) so they survive any in-flight GC cycle at allocation time.
/// This is called "allocation-color-black" in GC literature. Changing this to `false` would
/// cause use-after-free when objects are allocated concurrently with a collection cycle.
const INITIAL_MARK_STATE: bool = true;

/// Metadata for tracking garbage-collected objects
///
/// This struct contains the pointer to the object, its size, manages the marked state, provides
/// type-safe dropping functionality, and supports finalizers.
pub(crate) struct ObjectMetadata {
    ptr: SafePtr,
    size: usize,
    marked: AtomicBool,
    /// Tracks whether this object was ever marked as reachable during a GC mark phase.
    /// Objects that were never reachable from roots (e.g., VM internal objects held only
    /// by Rust containers like `ClassLoader`) are never swept. Only objects that were
    /// previously traceable from a root and subsequently became unreachable are collected.
    was_ever_marked: AtomicBool,
    /// Host-owned allocations are released during shutdown, even if once traced.
    retained: AtomicBool,
    drop_fn: Mutex<DropFn>,
    finalizer: Mutex<Option<FinalizerFn>>,
}

impl ObjectMetadata {
    /// Owns the initialized allocation before any ordinary index can discover it.
    pub(crate) fn new_for_gc(allocation: Allocation) -> Self {
        let ptr = SafePtr::from_ptr(allocation.pointer().get());
        let size = allocation.size();
        Self {
            ptr,
            size,
            marked: AtomicBool::new(INITIAL_MARK_STATE),
            was_ever_marked: AtomicBool::new(false),
            retained: AtomicBool::new(false),
            drop_fn: Mutex::new(Some(Box::new(move || drop(allocation)))),
            finalizer: Mutex::new(None),
        }
    }

    /// # Safety
    /// The allocation must contain the same initialized T used at construction.
    pub(crate) unsafe fn new_for_gc_with_finalizer<T: Send + Sync + Finalize>(
        allocation: Allocation,
    ) -> Self {
        let pointer = allocation.pointer();
        let mut metadata = Self::new_for_gc(allocation);
        metadata.finalizer = Mutex::new(Some(Box::new(move || {
            // SAFETY: the matching unique allocation remains in metadata until
            // after finalizers finish; this wrapper preserves pointer provenance.
            unsafe { (&*pointer.get().cast::<T>()).finalize() };
        })));
        metadata
    }

    /// Returns the pointer to the object.
    pub(crate) fn ptr(&self) -> &SafePtr {
        &self.ptr
    }

    /// Returns the size of the object.
    pub(crate) fn size(&self) -> usize {
        self.size
    }

    /// Returns whether the object is marked.
    pub(crate) fn is_marked(&self) -> bool {
        self.marked.load(Ordering::Acquire)
    }

    /// Marks the object as reachable.
    /// Returns true if this call actually marked the object.
    pub(crate) fn mark(&self) -> bool {
        self.was_ever_marked.store(true, Ordering::Release);
        !self.marked.swap(true, Ordering::AcqRel)
    }

    /// Returns whether this object was ever marked as reachable during a GC mark phase.
    pub(crate) fn was_ever_marked(&self) -> bool {
        self.was_ever_marked.load(Ordering::Acquire)
    }

    /// Keeps this allocation alive until collector shutdown, independently of tracing.
    pub(crate) fn retain(&self) {
        self.retained.store(true, Ordering::Release);
    }

    pub(crate) fn is_retained(&self) -> bool {
        self.retained.load(Ordering::Acquire)
    }

    /// Unmarks the object.
    pub(crate) fn unmark(&self) {
        self.marked.store(false, Ordering::Release);
    }

    /// Drops the object using the type-safe drop function. This first calls the finalizer (if any),
    /// then properly calls the destructor and deallocates memory. Can only be called once;
    /// subsequent calls are no-ops (guarded by `Option::take()` inside the closures).
    ///
    /// # Caller contract
    ///
    /// This must only be called after the `ObjectMetadata` has been **removed** from the shared
    /// `objects` `DashMap`, giving the caller exclusive ownership. Because the metadata is no
    /// longer visible to any other thread, no concurrent re-mark race is possible and no
    /// atomic guard is needed here.
    pub(crate) fn drop_object(&self) {
        self.run_drop_closures();
    }

    /// Runs only the finalizer closure if present. `take()`-guarded so it executes at most once.
    /// Used in shutdown phase 1: all finalizers run while every object's memory is still valid,
    /// so finalizers may safely access other GC managed objects.
    ///
    /// Uses `try_lock` to avoid deadlocking on a poisoned mutex from a prior panic.
    pub(crate) fn run_finalizer(&self) {
        match self.finalizer.try_lock() {
            Ok(mut guard) => {
                if let Some(finalizer) = guard.take() {
                    std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                        finalizer();
                    }))
                    .unwrap_or_else(|_| {
                        warn!("Finalizer panicked during object cleanup");
                    });
                }
            }
            Err(_) => {
                warn!("Failed to acquire finalizer lock during object cleanup; finalizer skipped");
            }
        }
    }

    /// Runs only the drop/deallocation closure if present. `take()`-guarded so it executes at
    /// most once. Used in shutdown phase 2: after all finalizers have run, drop closures
    /// deallocate object memory.
    ///
    /// Uses `try_lock` to avoid deadlocking on a poisoned mutex from a prior panic.
    pub(crate) fn run_drop_fn(&self) {
        let drop_fn = {
            let Ok(mut guard) = self.drop_fn.try_lock() else {
                warn!("Failed to acquire drop_fn lock during object cleanup; drop skipped");
                return;
            };
            guard.take()
        };
        if let Some(drop_fn) = drop_fn {
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(drop_fn))
                .unwrap_or_else(|_| warn!("Drop function panicked during object cleanup"));
        }
    }

    /// Runs the finalizer and drop closures sequentially. Each closure is `take()`-guarded so
    /// it runs at most once.
    fn run_drop_closures(&self) {
        self.run_finalizer();
        self.run_drop_fn();
    }
}

#[expect(clippy::missing_fields_in_debug)]
impl std::fmt::Debug for ObjectMetadata {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ObjectMetadata")
            .field("ptr", &self.ptr)
            .field("size", &self.size)
            .field("marked", &self.is_marked())
            .field("was_ever_marked", &self.was_ever_marked())
            .field("retained", &self.is_retained())
            .finish()
    }
}
