//! Pointer wrappers and utilities for garbage collection.

use crate::GarbageCollector;
use crate::collector::Trace;

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
    pub(crate) trace_fn: fn(*const (), &GarbageCollector),
}

// Safety: TracePtr can be sent between threads because:
// 1. The raw pointer is only used for identity comparison and tracing
// 2. The trace_fn is a function pointer which is inherently Send
// 3. Actual dereferencing only happens during controlled GC phases
// 4. The pointed-to objects are guaranteed to be Send + Sync by the GC API
unsafe impl Send for TracePtr {}

// Safety: TracePtr can be shared between threads because:
// 1. Both fields are immutable after construction
// 2. The raw pointer is read-only and only dereferenced during tracing
// 3. Function pointers are inherently Sync
// 4. The GC ensures proper synchronization during object access
unsafe impl Sync for TracePtr {}

impl TracePtr {
    /// Creates a new `TracePtr` from a reference to a `Trace` object.
    pub(crate) fn new<T: Trace>(obj: &T) -> Self {
        fn trace_impl<T: Trace>(ptr: *const (), collector: &GarbageCollector) {
            if ptr.is_null() {
                return;
            }

            // Basic safety check: ensure pointer is in reasonable range
            let ptr_addr = ptr as usize;
            if ptr_addr < 4096 {
                return;
            }

            // Safety: This is safe because:
            // 1. The pointer was created from a valid reference in new()
            // 2. We've added a basic null check and address validation
            // 3. T implements Trace so the cast is valid
            // 4. We only call this during controlled GC phases
            unsafe {
                let object = &*ptr.cast::<T>();
                object.trace(collector);
            }
        }

        Self {
            ptr: std::ptr::from_ref::<T>(obj).cast::<()>(),
            trace_fn: trace_impl::<T>,
        }
    }

    /// Creates a new `TracePtr` directly from a `Gc` pointer.
    /// This is used for roots to avoid storing pointers to temporary `Gc<T>` structs.
    pub(crate) fn new_from_ptr<T: Trace>(gc_ptr: *const T) -> Self {
        fn trace_impl<T: Trace>(ptr: *const (), collector: &GarbageCollector) {
            if ptr.is_null() {
                return;
            }

            // Basic safety check: ensure pointer is in reasonable range
            let ptr_addr = ptr as usize;
            if ptr_addr < 4096 {
                return;
            }

            // Safety: This is safer because:
            // 1. The pointer was created from a valid Gc pointer
            // 2. We've added a basic null check and address validation
            // 3. T implements Trace so the cast is valid
            // 4. The object is managed by the GC and should be alive during tracing
            unsafe {
                let object = &*ptr.cast::<T>();
                object.trace(collector);
            }
        }

        Self {
            ptr: gc_ptr.cast::<()>(),
            trace_fn: trace_impl::<T>,
        }
    }

    /// Returns the raw pointer to the `Trace` object.
    pub(crate) fn as_raw_ptr(&self) -> *const () {
        self.ptr
    }

    /// Trace the object pointed to by this pointer. This is only safe if the object is still alive
    /// and the pointer is valid.
    pub(crate) unsafe fn trace(&self, collector: &GarbageCollector) {
        (self.trace_fn)(self.ptr, collector);
    }
}
