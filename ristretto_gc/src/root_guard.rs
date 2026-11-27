//! RAII-based root reference management for garbage collection. This module provides the
//! `GcRootGuard` type, which allows objects to be registered as roots in the garbage collector. The
//! guard ensures that the root reference is automatically removed when the guard is dropped,
//! preventing memory leaks and ensuring proper cleanup of root references.

use crate::collector::{GarbageCollector, Trace};
use crate::gc::{Gc, OwnedRoot};
use std::hash::{Hash, Hasher};
use std::mem::ManuallyDrop;
use std::ops::{Deref, DerefMut};
use std::sync::Arc;
use triomphe::Arc as RootArc;

/// RAII guard for managing garbage collector root references.
///
/// When a `GcRootGuard` is created, it adds an object as a root to the garbage collector. When the
/// guard is dropped, it automatically removes the root reference, ensuring proper cleanup and
/// preventing memory leaks in the root set.
///
/// `GcRootGuard` acts like an Rc-like smart pointer; it can be cloned to create multiple handles
/// to the same root, and the root is only removed when all guards are dropped.
pub struct GcRootGuard<T: Trace> {
    root: Gc<T>,
    registration: RootArc<OwnedRoot>,
}

impl<T: Trace> GcRootGuard<T> {
    /// Transfers ownership into a handle that keeps the allocation and collector rooted.
    #[must_use]
    pub fn into_rooted(self) -> Gc<T> {
        let guard = ManuallyDrop::new(self);
        // SAFETY: consume both fields exactly once without running this guard's
        // custom Drop. The local handle and registration own normal destruction,
        // including if replacement-handle rooting unwinds below.
        let (root, registration) = unsafe {
            (
                std::ptr::read(&raw const guard.root),
                std::ptr::read(&raw const guard.registration),
            )
        };
        root.with_registration(registration)
    }
    /// Creates a new root guard that adds the given object as a root. Returns the guard which will
    /// automatically remove the root when dropped.
    pub(crate) fn new(
        collector: Arc<GarbageCollector>,
        root_id: crate::roots::Registration,
        root: Gc<T>,
    ) -> Self {
        let registration = RootArc::new(OwnedRoot {
            collector,
            id: root_id,
            target: root.identity(),
        });
        Self { root, registration }
    }

    /// Compatibility alias for an owning handle. The allocation is released after the last
    /// owning handle is dropped and a subsequent collection finds it unreachable.
    #[must_use]
    pub fn into_retained(self) -> Gc<T> {
        self.into_rooted()
    }

    /// Returns an owning clone of the underlying `Gc<T>` pointer.
    ///
    /// This allows the `Gc<T>` to be used in data structures or passed to other functions
    /// that expect a `Gc<T>`. The clone shares this guard's registration and keeps
    /// the allocation alive after the guard is dropped. If the guard's inner
    /// handle has been replaced, the clone protects that replacement instead.
    #[must_use]
    pub fn clone_gc(&self) -> Gc<T> {
        self.root.clone_with_registration(self.registration.clone())
    }
}

impl<T: Trace> Drop for GcRootGuard<T> {
    fn drop(&mut self) {
        // Fields drop afterward in declaration order. The registration remains
        // owned while the inner handle drops; its last owner publishes removal.
        self.root.suppress_guard_edge_barrier(&self.registration);
    }
}

impl<T: Trace> Clone for GcRootGuard<T> {
    fn clone(&self) -> Self {
        let registration = self.registration.clone();
        Self {
            // SAFETY: this guard retains the registration alongside the handle.
            root: unsafe { self.root.clone_for_guard(&registration) },
            registration,
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
        // The caller may move the entire inner handle out with mem::replace.
        // Its lifetime must then follow the handle, independently of this guard.
        self.root = self.root.clone_with_registration(self.registration.clone());
        &mut self.root
    }
}

// SAFETY: the guard always retains an independent root; it never demotes itself.
unsafe impl<T: Trace> Trace for GcRootGuard<T> {
    fn trace(&self, collector: &GarbageCollector) {
        self.root.trace(collector);
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

#[cfg(all(test, not(target_family = "wasm")))]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};

    struct Probe(Arc<AtomicUsize>);
    // SAFETY: this fixture contains no outgoing GC references.
    unsafe impl Trace for Probe {
        fn trace(&self, _: &GarbageCollector) {}
    }
    impl Drop for Probe {
        fn drop(&mut self) {
            self.0.fetch_add(1, Ordering::Relaxed);
        }
    }

    #[test]
    #[expect(clippy::panic_in_result_fn, reason = "root transfer unwind invariants")]
    fn failed_replacement_transfer_releases_the_original_registration_once() -> crate::Result<()> {
        for concurrent in [false, true] {
            for storage in [
                crate::RootStorage::Tree,
                crate::RootStorage::Slots,
                crate::RootStorage::Contexts,
            ] {
                let gc = GarbageCollector::with_config(
                    crate::Configuration::builder()
                        .concurrent_publication(concurrent)
                        .root_storage(storage)
                        .allocation_threshold(usize::MAX)
                        .build(),
                );
                let context = gc.register_context();
                context.with(|| -> crate::Result<()> {
                    let old_drops = Arc::new(AtomicUsize::new(0));
                    let replacement_drops = Arc::new(AtomicUsize::new(0));
                    let mut original = Gc::try_new(&gc, Probe(old_drops.clone()))?;
                    let replacement = Gc::try_new(&gc, Probe(replacement_drops.clone()))?;
                    // SAFETY: replacement remains rooted through this entire test.
                    // A raw wrapper has no collector association, so trying to
                    // root that replacement must unwind rather than transfer the
                    // original allocation's registration to the wrong identity.
                    *original = unsafe { Gc::from_raw(replacement.as_ptr())? };
                    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                        original.into_rooted()
                    }));
                    assert!(result.is_err());
                    gc.collect_full()?;
                    assert_eq!(old_drops.load(Ordering::Relaxed), 1);
                    assert_eq!(replacement_drops.load(Ordering::Relaxed), 0);
                    assert_eq!(gc.statistics()?.live_objects, 1);
                    drop(replacement);
                    gc.collect_full()?;
                    assert_eq!(old_drops.load(Ordering::Relaxed), 1);
                    assert_eq!(replacement_drops.load(Ordering::Relaxed), 1);
                    Ok(())
                })?;
                drop(context);
                gc.stop()?;
                assert_eq!(gc.managed_bytes(), 0);
            }
        }
        Ok(())
    }
}
