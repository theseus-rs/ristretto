//! A sealed private graph survives origin-thread handoff and exact cancellation.
#![expect(clippy::panic_in_result_fn)]
use super::*;
use crate::{Configuration, GcWeak, ReferenceKind, RootStorage, Trace};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

#[path = "publication_shared_tests.rs"]
mod shared;

#[cfg(feature = "experimental-canonical-leaves")]
#[path = "canonical_leaf_tests.rs"]
mod canonical_leaf;

struct Leaf(Arc<AtomicUsize>);
// SAFETY: this value has no outgoing managed references.
unsafe impl Trace for Leaf {
    fn trace(&self, _: &GarbageCollector) {}
}
impl Drop for Leaf {
    fn drop(&mut self) {
        self.0.fetch_add(1, Ordering::Relaxed);
    }
}

struct Parent {
    child: Gc<Leaf>,
    drops: Arc<AtomicUsize>,
}
// SAFETY: the only managed edge participates in tracing and paired ownership hooks.
unsafe impl Trace for Parent {
    fn trace(&self, gc: &GarbageCollector) {
        self.child.trace(gc);
    }
    fn root(&mut self) {
        self.child.root();
    }
    unsafe fn attach(&mut self, owner: crate::TraceOwner<'_>) {
        // SAFETY: the caller protects the initialized private graph.
        unsafe { self.child.attach(owner) };
    }
    unsafe fn unroot(&mut self, owner: crate::TraceOwner<'_>) {
        // SAFETY: the caller protects the owning graph through demotion.
        unsafe { Trace::unroot(&mut self.child, owner) };
    }
}
impl Drop for Parent {
    fn drop(&mut self) {
        self.drops.fetch_add(1, Ordering::Relaxed);
    }
}

fn collector(counted: bool, storage: RootStorage) -> Arc<GarbageCollector> {
    GarbageCollector::with_config(
        Configuration::builder()
            .concurrent_publication(counted)
            .root_storage(storage)
            .diagnostics(true)
            .allocation_threshold(usize::MAX)
            .build(),
    )
}

fn prepare<T: Send + Sync + Trace + 'static>(
    gc: &GarbageCollector,
    value: T,
    rooted: bool,
) -> Publication<'_, T> {
    let mut construction = crate::construction::Roots::new(gc);
    let scope = construction.enter_for::<T>();
    let (handle, allocation, reservation) = Gc::allocate(gc, value);
    let metadata = gc.prepare_allocated_object::<T>(allocation, reservation, handle.id);
    Publication::new(
        gc,
        handle,
        metadata,
        Detached::from_scope(gc, scope),
        rooted,
    )
}

fn graph(
    gc: &Arc<GarbageCollector>,
    child: &Arc<AtomicUsize>,
    parent: &Arc<AtomicUsize>,
) -> Parent {
    Parent {
        child: Gc::new(gc, Leaf(child.clone())).into_rooted(),
        drops: parent.clone(),
    }
}

fn reclaim(gc: &GarbageCollector) -> crate::Result<()> {
    for _ in 0..3 {
        gc.collect_full()?;
    }
    Ok(())
}

#[test]
fn sealed_graph_remains_live_across_collection_and_cancellation() -> crate::Result<()> {
    for counted in [false, true] {
        for storage in [RootStorage::Tree, RootStorage::Slots, RootStorage::Contexts] {
            for rooted in [false, true] {
                let gc = collector(counted, storage);
                let context = gc.register_context();
                let child = Arc::new(AtomicUsize::new(0));
                let parent = Arc::new(AtomicUsize::new(0));
                let publication =
                    context.with(|| prepare(&gc, graph(&gc, &child, &parent), rooted));
                drop(context);
                assert_eq!(
                    Arc::strong_count(&gc),
                    1,
                    "private graph retained a strong collector owner"
                );
                reclaim(&gc)?;
                assert_eq!(
                    child.load(Ordering::Relaxed),
                    0,
                    "sealed construction graph lost"
                );
                assert_eq!(parent.load(Ordering::Relaxed), 0);
                assert_eq!(gc.diagnostics().active_publications, 0);
                let mut unrelated = crate::construction::Roots::new(&gc);
                let unrelated_scope = unrelated.enter();
                drop(publication);
                assert_eq!(parent.load(Ordering::Relaxed), 1);
                reclaim(&gc)?;
                assert_eq!(
                    child.load(Ordering::Relaxed),
                    1,
                    "sealed cleanup adopted helper TLS"
                );
                assert_eq!(gc.managed_bytes(), 0);
                drop(unrelated_scope);
                let weak = Arc::downgrade(&gc);
                drop(gc);
                assert!(weak.upgrade().is_none());
            }
        }
    }
    Ok(())
}

#[test]
fn sealed_graph_can_publish_after_its_original_scope_and_context_end() -> crate::Result<()> {
    for counted in [false, true] {
        for storage in [RootStorage::Tree, RootStorage::Slots, RootStorage::Contexts] {
            let gc = collector(counted, storage);
            let context = gc.register_context();
            let child = Arc::new(AtomicUsize::new(0));
            let parent = Arc::new(AtomicUsize::new(0));
            let publication = context.with(|| prepare(&gc, graph(&gc, &child, &parent), true));
            drop(context);
            reclaim(&gc)?;
            let rooted = publication.finish_rooted(&gc);
            reclaim(&gc)?;
            assert_eq!(rooted.child.0.load(Ordering::Relaxed), 0);
            assert_eq!(parent.load(Ordering::Relaxed), 0);
            drop(rooted);
            reclaim(&gc)?;
            assert_eq!(child.load(Ordering::Relaxed), 1);
            assert_eq!(parent.load(Ordering::Relaxed), 1);
            assert_eq!(gc.managed_bytes(), 0);
        }
    }
    Ok(())
}

#[cfg(not(target_family = "wasm"))]
#[test]
fn a_helper_can_publish_or_cancel_a_sealed_graph_without_origin_tls() -> crate::Result<()> {
    for counted in [false, true] {
        for storage in [RootStorage::Tree, RootStorage::Slots, RootStorage::Contexts] {
            for cancel in [false, true] {
                let gc = collector(counted, storage);
                let context = gc.register_context();
                let child = Arc::new(AtomicUsize::new(0));
                let parent = Arc::new(AtomicUsize::new(0));
                let publication = context.with(|| prepare(&gc, graph(&gc, &child, &parent), true));
                drop(context);
                reclaim(&gc)?;
                std::thread::scope(|threads| {
                    let gc = &gc;
                    let child = &child;
                    threads
                        .spawn(move || -> crate::Result<()> {
                            let mut unrelated = crate::construction::Roots::new(gc);
                            let scope = unrelated.enter();
                            let rooted = if cancel {
                                drop(publication);
                                None
                            } else {
                                Some(publication.finish_rooted(gc))
                            };
                            reclaim(gc)?;
                            assert_eq!(child.load(Ordering::Relaxed), usize::from(cancel));
                            drop(scope);
                            drop(rooted);
                            reclaim(gc)
                        })
                        .join()
                        .expect("publication helper")
                })?;
                assert_eq!(child.load(Ordering::Relaxed), 1);
                assert_eq!(parent.load(Ordering::Relaxed), 1);
                assert_eq!(gc.managed_bytes(), 0);
            }
        }
    }
    Ok(())
}

struct Conditional(GcWeak<Leaf>);
// SAFETY: conditional slot owns its tracing/decision rules and attaches its owner.
unsafe impl Trace for Conditional {
    fn trace(&self, _: &GarbageCollector) {}
    unsafe fn attach(&mut self, owner: crate::TraceOwner<'_>) {
        self.0.set_owner(owner.address());
    }
}

#[test]
fn sealed_conditional_owner_binding_uses_its_exact_batch() -> crate::Result<()> {
    for counted in [false, true] {
        for storage in [RootStorage::Tree, RootStorage::Slots, RootStorage::Contexts] {
            let gc = collector(counted, storage);
            let drops = Arc::new(AtomicUsize::new(0));
            let leaf = Gc::new(&gc, Leaf(drops.clone()));
            let weak = GcWeak::new(&leaf, ReferenceKind::Weak, 0);
            let slot = weak.slot.clone();
            let publication = prepare(&gc, Conditional(weak), true);
            let identity = publication.metadata().id;
            drop(leaf);
            reclaim(&gc)?;
            assert_eq!(drops.load(Ordering::Relaxed), 0);
            assert_eq!(slot.unpublished_owners.load(Ordering::Acquire), 1);
            assert!(slot.owner_identity().is_none());
            let mut unrelated = crate::construction::Roots::new(&gc);
            let scope = unrelated.enter();
            let owner = publication.finish_rooted(&gc);
            assert_eq!(
                slot.owner_identity(),
                Some(identity),
                "sealed conditional owner was not bound"
            );
            assert_eq!(slot.unpublished_owners.load(Ordering::Acquire), 0);
            reclaim(&gc)?;
            assert_eq!(
                drops.load(Ordering::Relaxed),
                1,
                "sealed conditional roots adopted helper TLS"
            );
            let queued = gc
                .take_pending_reference::<Conditional>()?
                .expect("owner notification");
            assert_eq!(queued.identity(), owner.identity());
            drop(scope);
            drop(queued);
            drop(owner);
            reclaim(&gc)?;
            assert_eq!(gc.managed_bytes(), 0);
        }
    }
    Ok(())
}

#[test]
fn cancellation_keeps_children_until_private_payload_destruction_finishes() -> crate::Result<()> {
    struct Destruction {
        child: Gc<Leaf>,
        collector: Weak<GarbageCollector>,
        observed: Arc<AtomicUsize>,
        child_drops: Arc<AtomicUsize>,
        completed: Arc<AtomicBool>,
    }
    // SAFETY: the only edge is traced and demoted during protected attachment.
    unsafe impl Trace for Destruction {
        fn trace(&self, gc: &GarbageCollector) {
            self.child.trace(gc);
        }
        unsafe fn attach(&mut self, owner: crate::TraceOwner<'_>) {
            unsafe { self.child.attach(owner) };
        }
    }
    impl Drop for Destruction {
        fn drop(&mut self) {
            let gc = self
                .collector
                .upgrade()
                .expect("heap borrow excludes shutdown");
            let roots = gc.root_statistics();
            self.observed.store(
                roots.compatibility_roots + roots.slot_roots,
                Ordering::Relaxed,
            );
            reclaim(&gc).expect("collection inside private destructor");
            assert_eq!(
                self.child_drops.load(Ordering::Relaxed),
                0,
                "child died before private destructor access"
            );
            assert_eq!(
                self.child.0.load(Ordering::Relaxed),
                0,
                "child died inside private destruction"
            );
            self.completed.store(true, Ordering::Relaxed);
        }
    }
    for counted in [false, true] {
        for storage in [RootStorage::Tree, RootStorage::Slots, RootStorage::Contexts] {
            let gc = collector(counted, storage);
            let context = gc.register_context();
            let child = Arc::new(AtomicUsize::new(0));
            let observed = Arc::new(AtomicUsize::new(usize::MAX));
            let completed = Arc::new(AtomicBool::new(false));
            let publication = context.with(|| {
                prepare(
                    &gc,
                    Destruction {
                        child: Gc::new(&gc, Leaf(child.clone())).into_rooted(),
                        collector: Arc::downgrade(&gc),
                        observed: observed.clone(),
                        child_drops: child.clone(),
                        completed: completed.clone(),
                    },
                    true,
                )
            });
            drop(context);
            drop(publication);
            assert!(
                completed.load(Ordering::Relaxed),
                "private destructor did not finish graph access"
            );
            assert_eq!(
                observed.load(Ordering::Relaxed),
                1,
                "parent root survived into private destructor"
            );
            reclaim(&gc)?;
            assert_eq!(child.load(Ordering::Relaxed), 1);
            assert_eq!(gc.managed_bytes(), 0);
        }
    }
    Ok(())
}
