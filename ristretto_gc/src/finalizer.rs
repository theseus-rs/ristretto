//! Finalizer support for garbage-collected objects.

/// Trait for objects that need custom finalization before being dropped.
///
/// Objects implementing this trait will have their `finalize` method called
/// during the garbage collection sweep phase, before the object is deallocated.
/// This allows for custom cleanup logic such as closing file handles, releasing
/// system resources, or performing other cleanup operations.
///
/// # Safety
///
/// The `finalize` method should not:
/// - Access other garbage-collected objects (they may already be finalized)
/// - Allocate new Gc objects
/// - Perform long-running operations (this stalls the executing helper and retains the allocation)
/// - Panic (this will abort the program)
///
/// The finalize method is called at most once per object.
pub trait Finalize {
    /// Perform custom cleanup before the object is deallocated.
    ///
    /// This method is called during the sweep phase of garbage collection,
    /// after the object has been determined to be unreachable.
    fn finalize(&self);
}

/// A typed, non-owning operation. Metadata supplies the original live value pointer.
pub(crate) type FinalizerFn = unsafe fn(*const ());

/// # Safety
/// `ptr` must point to a live T whose finalizer has not previously run.
pub(crate) unsafe fn finalize<T: Finalize>(ptr: *const ()) {
    // SAFETY: the caller retains the original typed allocation until this returns.
    unsafe { (&*ptr.cast::<T>()).finalize() };
}
