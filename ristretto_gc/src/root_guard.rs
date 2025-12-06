//! RAII-based root reference management for garbage collection. This module provides the
//! `GcRootGuard` type, which allows objects to be registered as roots in the garbage collector. The
//! guard ensures that the root reference is automatically removed when the guard is dropped,
//! preventing memory leaks and ensuring proper cleanup of root references.

use crate::collector::{GarbageCollector, Trace};
use crate::gc::Gc;
use std::hash::{Hash, Hasher};
use std::ops::{Deref, DerefMut};
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

    /// Returns a clone of the underlying `Gc<T>` pointer.
    ///
    /// This allows the `Gc<T>` to be used in data structures or passed to other functions
    /// that expect a `Gc<T>`. Note that the returned `Gc<T>` is NOT rooted by itself;
    /// it relies on the existence of this `GcRootGuard` (or another root) to keep the
    /// object alive.
    #[must_use]
    pub fn clone_gc(&self) -> Gc<T> {
        self.root.clone()
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
    type Target = Gc<T>;

    fn deref(&self) -> &Self::Target {
        &self.root
    }
}

impl<T: Trace> DerefMut for GcRootGuard<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.root
    }
}

impl<T: Trace> Trace for GcRootGuard<T> {
    fn trace(&self, collector: &GarbageCollector) {
        self.root.trace(collector);
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

impl<T: Trace + PartialEq> PartialEq for GcRootGuard<T> {
    fn eq(&self, other: &Self) -> bool {
        *self.root == *other.root
    }
}

impl<T: Trace + Eq> Eq for GcRootGuard<T> {}

impl<T: Trace + Hash> Hash for GcRootGuard<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.root.hash(state);
    }
}

impl<T: Trace + PartialOrd> PartialOrd for GcRootGuard<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.root.partial_cmp(&other.root)
    }
}

impl<T: Trace + Ord> Ord for GcRootGuard<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.root.cmp(&other.root)
    }
}

impl<T: Trace + std::fmt::Debug> std::fmt::Debug for GcRootGuard<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.root.fmt(f)
    }
}

impl<T: Trace + std::fmt::Display> std::fmt::Display for GcRootGuard<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.root.fmt(f)
    }
}
