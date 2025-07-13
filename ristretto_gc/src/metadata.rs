//! Object metadata and pointer management for garbage collection.

use crate::Finalize;
use crate::finalizer::FinalizerFn;
use crate::gc_box::GcBox;
use crate::pointers::SafePtr;
use std::sync::Mutex;
use std::sync::atomic::{AtomicBool, Ordering};

/// Type-safe drop function for garbage-collected objects
type DropFn = Option<Box<dyn FnOnce() + Send + Sync>>;

/// Metadata for tracking garbage-collected objects
///
/// This struct contains the pointer to the object, its size, manages the marked state, provides
/// type-safe dropping functionality, and supports finalizers.
pub(crate) struct ObjectMetadata {
    ptr: SafePtr,
    size: usize,
    marked: AtomicBool,
    drop_fn: Mutex<DropFn>,
    finalizer: Mutex<Option<FinalizerFn>>,
}

impl ObjectMetadata {
    /// Creates a new `ObjectMetadata` instance for a `GcBox<T>`.
    pub(crate) fn new_for_gcbox<T: Send + Sync>(ptr: SafePtr, size: usize) -> Self {
        // Create a type-safe drop function that properly deallocates the GcBox<T>
        // Convert raw pointer to usize for thread safety
        let ptr_addr = ptr.as_ptr::<GcBox<T>>() as usize;
        let drop_fn: DropFn = Some(Box::new(move || {
            // Safety: This is safe because:
            // 1. The pointer was originally created from Box::into_raw
            // 2. We're only calling this once during sweep phase
            // 3. The object is unreachable so no other references exist
            // 4. This closure can only be called once due to Option::take()
            unsafe {
                if ptr_addr != 0 {
                    let raw_ptr = ptr_addr as *mut GcBox<T>;
                    // Add safety check before dereferencing
                    if !raw_ptr.is_null() {
                        let _boxed = Box::from_raw(raw_ptr);
                        // The Box destructor will properly call T's Drop implementation
                        // and deallocate the memory
                    }
                }
            }
        }));

        Self {
            ptr,
            size,
            marked: AtomicBool::new(false),
            drop_fn: Mutex::new(drop_fn),
            finalizer: Mutex::new(None), // No finalizer by default
        }
    }

    /// Creates a new `ObjectMetadata` instance for a `GcBox<T>` with finalizer support.
    pub(crate) fn new_for_gcbox_with_finalizer<T: Send + Sync + Finalize>(
        ptr: SafePtr,
        size: usize,
    ) -> Self {
        // Create a type-safe drop function that properly deallocates the GcBox<T>
        let ptr_addr = ptr.as_ptr::<GcBox<T>>() as usize;
        let drop_fn: DropFn = Some(Box::new(move ||
            // Safety: This is safe because:
            // 1. The pointer was originally created from Box::into_raw
            // 2. We're only calling this once during sweep phase
            // 3. The object is unreachable so no other references exist
            // 4. This closure can only be called once due to Option::take()
            unsafe {
                if ptr_addr != 0 {
                    let raw_ptr = ptr_addr as *mut GcBox<T>;
                    if !raw_ptr.is_null() {
                        let _boxed = Box::from_raw(raw_ptr);
                    }
                }
            }));

        // Create finalizer for objects that implement Finalize
        let finalizer: Option<FinalizerFn> = if ptr_addr != 0 {
            let raw_ptr = ptr_addr as *const GcBox<T>;
            if raw_ptr.is_null() {
                None
            } else {
                // Safety: This is safe because:
                // 1. raw_ptr points to a valid GcBox<T> during object creation
                // 2. We're only taking the address of the data field, not dereferencing
                // 3. The data field offset is guaranteed by Rust's memory layout
                let data_ptr_addr = unsafe { &raw const (*raw_ptr).data as usize };
                Some(Box::new(move ||
                    // Safety: This is safe because:
                    // 1. The pointer was valid when the finalizer was created
                    // 2. We're in the finalization phase before object deallocation
                    // 3. The object implements Finalize so the method exists
                    // 4. We check for null pointer before dereferencing
                    unsafe {
                        if data_ptr_addr != 0 {
                            let data_ptr = data_ptr_addr as *const T;
                            if !data_ptr.is_null() {
                                (*data_ptr).finalize();
                            }
                        }
                    }))
            }
        } else {
            None
        };

        Self {
            ptr,
            size,
            marked: AtomicBool::new(false),
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
        !self.marked.swap(true, Ordering::AcqRel)
    }

    /// Unmarks the object.
    pub(crate) fn unmark(&self) {
        self.marked.store(false, Ordering::Release);
    }

    /// Drops the object using the type-safe drop function. This first calls the finalizer (if any),
    /// then properly calls the destructor and deallocate s memory. Can only be called once;
    /// subsequent calls are no-ops.
    pub(crate) fn drop_object(&self) {
        // Use atomic operations to ensure this can only be called once and prevent race conditions
        // during concurrent access
        if !self.marked.swap(true, Ordering::AcqRel) {
            // Object was already marked for deletion or is being deleted
            return;
        }

        // First, run the finalizer if present
        if let Ok(mut finalizer_guard) = self.finalizer.try_lock() {
            if let Some(finalizer) = finalizer_guard.take() {
                // Execute finalizer in a safe context
                std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                    finalizer();
                }))
                .unwrap_or_else(|_| {
                    // Log finalizer panic but don't propagate it
                    eprintln!("Warning: Finalizer panicked during object cleanup");
                });
            }
        }

        // Then, drop the object with additional safety checks
        if let Ok(mut drop_fn_guard) = self.drop_fn.try_lock() {
            if let Some(drop_fn) = drop_fn_guard.take() {
                // Execute drop function in a safe context
                std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                    drop_fn();
                }))
                .unwrap_or_else(|_| {
                    // Log drop panic but don't propagate it
                    eprintln!("Warning: Drop function panicked during object cleanup");
                });
            }
        }
    }
}

#[expect(clippy::missing_fields_in_debug)]
impl std::fmt::Debug for ObjectMetadata {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ObjectMetadata")
            .field("ptr", &self.ptr)
            .field("size", &self.size)
            .field("marked", &self.is_marked())
            .finish()
    }
}
