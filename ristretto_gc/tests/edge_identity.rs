//! Heap-edge identity survives reclamation without retaining payloads or the heap.
#![expect(clippy::panic_in_result_fn)]
use ristretto_gc::{
    Configuration, Finalize, GarbageCollector, Gc, GcWeak, ReferenceKind, Result, RootStorage,
    Trace,
};
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

struct Probe {
    drops: Arc<AtomicUsize>,
    finalizations: Arc<AtomicUsize>,
}

// SAFETY: this fixture contains no outgoing GC references.
#[expect(unsafe_code, reason = "Trace requires an explicit ownership contract")]
unsafe impl Trace for Probe {
    fn may_contain_gc_references() -> bool {
        false
    }
    fn trace(&self, _: &GarbageCollector) {}
}
impl Finalize for Probe {
    fn finalize(&self) {
        self.finalizations.fetch_add(1, Ordering::Relaxed);
    }
}
impl Drop for Probe {
    fn drop(&mut self) {
        self.drops.fetch_add(1, Ordering::Relaxed);
    }
}

#[test]
fn retired_edge_identity_does_not_retain_storage_or_resurrect() -> Result<()> {
    for concurrent in [false, true] {
        let gc = GarbageCollector::with_config(
            Configuration::builder()
                .concurrent_publication(concurrent)
                .root_storage(RootStorage::Contexts)
                .allocation_threshold(usize::MAX)
                .build(),
        );
        let drops = Arc::new(AtomicUsize::new(0));
        let finalizations = Arc::new(AtomicUsize::new(0));
        let root = Gc::new_with_finalizer(
            &gc,
            Probe {
                drops: drops.clone(),
                finalizations: finalizations.clone(),
            },
        );
        let mut edge = root.clone_gc();
        // Deliberately retain only identity after retirement; never dereference.
        // SAFETY: The root protects construction of the weak probe; after retirement only identity is used.
        #[expect(unsafe_code, reason = "explicit non-owning edge fixture")]
        unsafe {
            edge.unroot();
        };
        let identity = edge.identity();
        let weak = GcWeak::new(&edge, ReferenceKind::Weak, 0);
        drop(root);
        gc.collect_full()?;
        gc.collect_full()?;
        assert_eq!(gc.statistics()?.live_objects, 0);
        assert_eq!(gc.managed_bytes(), 0);
        assert_eq!(drops.load(Ordering::Relaxed), 1);
        assert_eq!(finalizations.load(Ordering::Relaxed), 1);
        assert!(weak.upgrade().is_none());
        assert!(Gc::<Probe>::from_handle_rooted(&gc, identity.to_i64()).is_err());
        assert_eq!(edge.identity(), identity);
        assert_eq!(identity, edge.identity());
        let heap = Arc::downgrade(&gc);
        drop(gc);
        assert!(heap.upgrade().is_none());
        assert_eq!(edge.identity(), identity);
        drop(edge);
        assert_eq!(drops.load(Ordering::Relaxed), 1);
        assert_eq!(finalizations.load(Ordering::Relaxed), 1);
    }
    Ok(())
}

struct Node {
    next: Option<Gc<ristretto_gc::cell::RwLock<Node>>>,
    drops: Arc<AtomicUsize>,
}
// SAFETY: all strong fields are traced and paired root/demotion hooks forward the owner.
#[expect(unsafe_code, reason = "Trace requires an explicit ownership contract")]
unsafe impl Trace for Node {
    fn root(&mut self) {
        self.next.root();
    }
    fn trace(&self, gc: &GarbageCollector) {
        if let Some(next) = &self.next {
            next.trace(gc);
        }
    }
    unsafe fn unroot(&mut self, owner: ristretto_gc::TraceOwner<'_>) {
        if let Some(next) = &mut self.next {
            unsafe { Trace::unroot(next, owner) };
        }
    }
}
impl Drop for Node {
    fn drop(&mut self) {
        self.drops.fetch_add(1, Ordering::Relaxed);
    }
}

#[test]
fn heap_edges_do_not_turn_payload_cycles_into_reference_count_cycles() -> Result<()> {
    for concurrent in [false, true] {
        let gc = GarbageCollector::with_config(
            Configuration::builder()
                .concurrent_publication(concurrent)
                .allocation_threshold(usize::MAX)
                .build(),
        );
        let drops = Arc::new(AtomicUsize::new(0));
        let first = Gc::try_new(
            &gc,
            ristretto_gc::cell::RwLock::new(Node {
                next: None,
                drops: drops.clone(),
            }),
        )?;
        let second = Gc::try_new(
            &gc,
            ristretto_gc::cell::RwLock::new(Node {
                next: None,
                drops: drops.clone(),
            }),
        )?;
        first.write().next = Some(second.rooted_clone());
        second.write().next = Some(first.rooted_clone());
        let mut edge = first.clone_gc();
        // SAFETY: The root protects construction of the weak probe; after retirement only identity is used.
        #[expect(unsafe_code, reason = "explicit non-owning edge fixture")]
        unsafe {
            edge.unroot();
        };
        let identity = edge.identity();
        let weak = GcWeak::new(&second, ReferenceKind::Weak, 0);
        drop((first, second));
        gc.collect_full()?;
        gc.collect_full()?;
        assert_eq!(drops.load(Ordering::Relaxed), 2);
        assert_eq!(gc.statistics()?.live_objects, 0);
        assert!(weak.upgrade().is_none());
        assert_eq!(edge.identity(), identity);
        drop(edge);
        assert_eq!(drops.load(Ordering::Relaxed), 2);
    }
    Ok(())
}
