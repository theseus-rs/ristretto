//! Pointer wrappers and utilities for garbage collection.

use crate::GarbageCollector;
use crate::collector::Trace;

type TraceStep = fn(*const (), &GarbageCollector, usize, usize) -> crate::TraceProgress;

/// Immutable type operations are shared by every descriptor and root of that type.
#[derive(Debug)]
struct TraceTable {
    trace: fn(*const (), &GarbageCollector),
    compact: fn(*const (), &crate::region::RegionHeap, usize) -> crate::TraceProgress,
    clear: fn(*const (), usize, usize) -> crate::TraceProgress,
}

const MIN_TRACE_ADDRESS: usize = 4096;

/// Thread-safe pointer wrapper for object tracking
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub(crate) struct SafePtr(pub(crate) usize);

// Safety: SafePtr only stores a usize representation of a pointer address.
// It doesn't actually dereference the pointer or provide direct access to the data,
// so it's safe to send between threads. The actual memory safety is handled by
// the garbage collector's synchronization mechanisms.
unsafe impl Send for SafePtr {}

// Safety: SafePtr is just a usize that represents a pointer address.
// Multiple threads can safely read this value since it's just an integer.
// Any actual dereferencing happens through controlled mechanisms in the GC.
unsafe impl Sync for SafePtr {}

impl SafePtr {
    /// Creates a new `SafePtr` from a raw pointer.
    pub(crate) fn from_ptr<T>(ptr: *const T) -> Self {
        Self(ptr as usize)
    }

    /// Converts the `SafePtr` back to a raw pointer.
    pub(crate) fn as_ptr<T>(self) -> *const T {
        self.0 as *const T
    }
}

/// Safe wrapper for raw pointers to `Trace` objects
#[derive(Clone, Debug)]
pub(crate) struct TracePtr {
    pub(crate) ptr: *const (),
    pub(crate) budget: usize,
    // None denotes a type that explicitly permits omission of all tracing hooks.
    // Use the function-pointer niche instead of enlarging every descriptor/root.
    pub(crate) step_fn: Option<TraceStep>,
    table: &'static TraceTable,
}

// Safety: TracePtr can be sent between threads because:
// 1. The raw pointer is only used for identity comparison and tracing
// 2. The operations table is immutable and contains only function pointers
// 3. Actual dereferencing only happens during controlled GC phases
// 4. The pointed-to objects are guaranteed to be Send + Sync by the GC API
unsafe impl Send for TracePtr {}

// Safety: TracePtr can be shared between threads because:
// 1. All fields are immutable after publication
// 2. The raw pointer is read-only and only dereferenced during tracing
// 3. Function pointers are inherently Sync
// 4. The GC ensures proper synchronization during object access
unsafe impl Sync for TracePtr {}

impl TracePtr {
    fn trace_impl<T: Trace>(ptr: *const (), collector: &GarbageCollector) {
        if ptr as usize >= MIN_TRACE_ADDRESS {
            // SAFETY: callers preserve a live descriptor of the original T.
            // The address check is only a defensive guard, not lifetime validation.
            unsafe { (&*ptr.cast::<T>()).trace(collector) };
        }
    }

    fn compact<T: Trace>(
        ptr: *const (),
        heap: &crate::region::RegionHeap,
        bytes: usize,
    ) -> crate::TraceProgress {
        // SAFETY: collection owns the descriptor until all work completes. The callback
        // acquires its own exclusive payload guard before accessing movable storage.
        unsafe { (&*ptr.cast::<T>()).compact_guarded(heap, bytes) }
    }

    fn clear<T: Trace>(ptr: *const (), offset: usize, budget: usize) -> crate::TraceProgress {
        // SAFETY: called only after removing an unreachable allocation from the registry.
        unsafe { (&mut *ptr.cast_mut().cast::<T>()).clear_step(offset, budget) }
    }
    fn step<T: Trace>(
        ptr: *const (),
        collector: &GarbageCollector,
        offset: usize,
        budget: usize,
    ) -> crate::TraceProgress {
        // SAFETY: the registry owns the allocation until tracing and sweeping finish.
        unsafe { (&*ptr.cast::<T>()).trace_step(collector, offset, budget) }
    }
    /// Creates a new `TracePtr` from a reference to a `Trace` object.
    pub(crate) fn new<T: Trace>(obj: &T) -> Self {
        Self::new_from_ptr(std::ptr::from_ref(obj))
    }

    /// Creates a new `TracePtr` directly from a `Gc` pointer.
    /// This is used for roots to avoid storing pointers to temporary `Gc<T>` structs.
    pub(crate) fn new_from_ptr<T: Trace>(gc_ptr: *const T) -> Self {
        Self {
            ptr: gc_ptr.cast::<()>(),
            budget: 32,
            step_fn: T::may_contain_gc_references().then_some(Self::step::<T>),
            table: const {
                &TraceTable {
                    trace: Self::trace_impl::<T>,
                    clear: Self::clear::<T>,
                    compact: Self::compact::<T>,
                }
            },
        }
    }

    pub(crate) fn clear_step(&self, offset: usize, budget: usize) -> crate::TraceProgress {
        (self.table.clear)(self.ptr, offset, budget)
    }

    pub(crate) fn compact_guarded(
        &self,
        heap: &crate::region::RegionHeap,
        bytes: usize,
    ) -> crate::TraceProgress {
        (self.table.compact)(self.ptr, heap, bytes)
    }

    /// Returns the raw pointer to the `Trace` object.
    pub(crate) fn as_raw_ptr(&self) -> *const () {
        self.ptr
    }

    /// Trace the object pointed to by this pointer. This is only safe if the object is still alive
    /// and the pointer is valid.
    pub(crate) unsafe fn trace(&self, collector: &GarbageCollector) {
        (self.table.trace)(self.ptr, collector);
    }
}
