//! RAII-based root reference management for garbage collection. This module provides the
//! `GcRootGuard` type, which allows objects to be registered as roots in the garbage collector. The
//! guard ensures that the root reference is automatically removed when the guard is dropped,
//! preventing memory leaks and ensuring proper cleanup of root references.

use crate::collector::{GarbageCollector, Trace};
use crate::gc::Gc;
use std::ops::Deref;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

/// RAII guard for managing garbage collector root references.
///
/// When a `GcRootGuard` is created, it adds an object as a root to the garbage collector. When the
/// guard is dropped, it automatically removes the root reference, ensuring proper cleanup and
/// preventing memory leaks in the root set.
///
/// `GcRootGuard` acts like an Rc-like smart pointer - it can be cloned to create multiple handles
/// to the same root, and the root is only removed when all guards are dropped.
pub struct GcRootGuard<T: Trace> {
    gc: Arc<GarbageCollector>,
    root: Gc<T>,
    root_id: usize,
    ref_count: Arc<AtomicUsize>,
}

impl<T: Trace> GcRootGuard<T> {
    /// Creates a new root guard that adds the given object as a root. Returns the guard which will
    /// automatically remove the root when dropped.
    pub(crate) fn new(gc: Arc<GarbageCollector>, root: Gc<T>) -> Self {
        let root_id = gc.add_root(&root);
        let ref_count = Arc::new(AtomicUsize::new(1));
        Self {
            gc,
            root,
            root_id,
            ref_count,
        }
    }
}

impl<T: Trace> Clone for GcRootGuard<T> {
    fn clone(&self) -> Self {
        // Increment reference count
        self.ref_count.fetch_add(1, Ordering::Relaxed);
        Self {
            gc: Arc::clone(&self.gc),
            root: self.root.clone(),
            root_id: self.root_id,
            ref_count: Arc::clone(&self.ref_count),
        }
    }
}

impl<T: Trace> Deref for GcRootGuard<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.root
    }
}

impl<T: Trace> Drop for GcRootGuard<T> {
    fn drop(&mut self) {
        // Decrement reference count
        let old_count = self.ref_count.fetch_sub(1, Ordering::Relaxed);

        // If this was the last reference, remove the root
        if old_count == 1 {
            // Ignore errors during cleanup - we can't handle them in Drop
            self.gc.remove_root_by_id(self.root_id);
        }
    }
}
