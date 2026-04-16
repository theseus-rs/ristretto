//! Object metadata and pointer management for garbage collection.

use crate::Finalize;
use crate::finalizer::FinalizerFn;
use crate::pointers::SafePtr;
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
    drop_fn: Mutex<DropFn>,
    finalizer: Mutex<Option<FinalizerFn>>,
}

impl ObjectMetadata {
    /// Creates a new `ObjectMetadata` instance for a `Gc<T>`.
    pub(crate) fn new_for_gc<T: Send + Sync>(ptr: SafePtr, size: usize) -> Self {
        // Create a type safe drop function that properly deallocates the T data.
        // The pointer was created via Box::into_raw(Box::new(data: T)), so we reconstruct
        // Box<T> to run T::drop and deallocate.
        let ptr_addr = ptr.0;
        let drop_fn: DropFn = Some(Box::new(move || {
            // Safety: This is safe because:
            // 1. The pointer was originally created from Box::into_raw on a Box<T>
            // 2. We're only calling this once during sweep phase
            // 3. The object is unreachable so no other references exist
            // 4. This closure can only be called once due to Option::take()
            unsafe {
                if ptr_addr != 0 {
                    let raw_ptr = ptr_addr as *mut T;
                    if !raw_ptr.is_null() {
                        let _boxed = Box::from_raw(raw_ptr);
                    }
                }
            }
        }));

        Self {
            ptr,
            size,
            marked: AtomicBool::new(INITIAL_MARK_STATE),
            was_ever_marked: AtomicBool::new(false),
            drop_fn: Mutex::new(drop_fn),
            finalizer: Mutex::new(None), // No finalizer by default
        }
    }

    /// Creates a new `ObjectMetadata` instance for a `Gc<T>` with finalizer support.
    pub(crate) fn new_for_gc_with_finalizer<T: Send + Sync + Finalize>(
        ptr: SafePtr,
        size: usize,
    ) -> Self {
        // Create a type-safe drop function that properly deallocates the T data.
        let ptr_addr = ptr.0;
        let drop_fn: DropFn = Some(Box::new(move || {
            // Safety: Same as new_for_gc;  pointer from Box::into_raw(Box::new(data: T))
            unsafe {
                if ptr_addr != 0 {
                    let raw_ptr = ptr_addr as *mut T;
                    if !raw_ptr.is_null() {
                        let _boxed = Box::from_raw(raw_ptr);
                    }
                }
            }
        }));

        // Create finalizer for objects that implement Finalize.
        // The pointer points directly to T data, so we can call T::finalize() on it.
        let finalizer: Option<FinalizerFn> = if ptr_addr != 0 {
            Some(Box::new(move || {
                // Safety: The pointer was valid when the finalizer was created and we're in
                // the finalization phase before object deallocation.
                unsafe {
                    let data_ptr = ptr_addr as *const T;
                    if !data_ptr.is_null() {
                        (*data_ptr).finalize();
                    }
                }
            }))
        } else {
            None
        };

        Self {
            ptr,
            size,
            marked: AtomicBool::new(INITIAL_MARK_STATE),
            was_ever_marked: AtomicBool::new(false),
            drop_fn: Mutex::new(drop_fn),
            finalizer: Mutex::new(finalizer),
        }
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
        match self.drop_fn.try_lock() {
            Ok(mut guard) => {
                if let Some(drop_fn) = guard.take() {
                    std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                        drop_fn();
                    }))
                    .unwrap_or_else(|_| {
                        warn!("Drop function panicked during object cleanup");
                    });
                }
            }
            Err(_) => {
                warn!("Failed to acquire drop_fn lock during object cleanup; drop skipped");
            }
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
            .finish()
    }
}
