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
/// - Perform long-running operations (this blocks the GC)
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

/// Internal wrapper for finalizer functions
pub(crate) type FinalizerFn = Box<dyn FnOnce() + Send + Sync>;

/// Creates a finalizer function for an object that implements Finalize
pub(crate) fn create_finalizer_for<T: Finalize>(obj_ptr: *const T) -> FinalizerFn {
    // Convert raw pointer to usize for thread safety
    let ptr_addr = obj_ptr as usize;
    Box::new(move || {
        // Safety: This is safe because:
        // 1. We're in the sweep phase, so the object is unreachable but not yet deallocated
        // 2. The finalizer is called exactly once before deallocation
        // 3. The pointer was valid when the finalizer was created
        // 4. We convert back from usize to maintain the original pointer validity
        unsafe {
            if ptr_addr != 0 {
                let obj_ptr = ptr_addr as *const T;
                (*obj_ptr).finalize();
            }
        }
    })
}
