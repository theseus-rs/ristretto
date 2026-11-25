//! Construction registrations retain their exact graph across ownership transfer.
#![expect(
    clippy::panic_in_result_fn,
    reason = "construction ownership assertions"
)]
use super::*;
use crate::{Configuration, Gc, RootStorage, Trace};
use std::sync::atomic::AtomicUsize;

struct Leaf(Arc<AtomicUsize>);
// SAFETY: the fixture has no outgoing GC references.
unsafe impl Trace for Leaf {
    fn may_contain_gc_references() -> bool {
        false
    }
    fn trace(&self, _: &GarbageCollector) {}
}
impl Drop for Leaf {
    fn drop(&mut self) {
        self.0.fetch_add(1, Ordering::Relaxed);
    }
}

fn collector(concurrent: bool, storage: RootStorage) -> Arc<GarbageCollector> {
    GarbageCollector::with_config(
        Configuration::builder()
            .concurrent_publication(concurrent)
            .root_storage(storage)
            .barrier_buffer_size(4)
            .allocation_threshold(usize::MAX)
            .build(),
    )
}

fn capture(
    gc: &Arc<GarbageCollector>,
    drops: &Arc<AtomicUsize>,
    count: usize,
) -> crate::Result<Vec<Registration>> {
    let mut roots = Roots::new(gc);
    let scope = roots.enter();
    for _ in 0..count {
        drop(Gc::try_new(gc, Leaf(drops.clone()))?);
    }
    let registrations = std::mem::take(&mut *scope.roots.registrations.borrow_mut());
    assert_eq!(registrations.len(), count);
    drop(scope);
    Ok(registrations)
}

fn release_in_unrelated_scope(
    gc: &Arc<GarbageCollector>,
    registrations: Vec<Registration>,
    drops: &AtomicUsize,
) -> crate::Result<()> {
    let count = registrations.len();
    let mut unrelated = Roots::new(gc);
    let scope = unrelated.enter();
    for registration in &registrations {
        gc.remove_detached_root(registration);
    }
    drop(registrations);
    gc.collect_full()?;
    gc.collect_full()?;
    assert_eq!(
        drops.load(Ordering::Relaxed),
        count,
        "detached cleanup adopted its graph into an unrelated construction scope"
    );
    assert_eq!(gc.managed_bytes(), 0);
    assert_eq!(gc.root_statistics().compatibility_roots, 0);
    assert_eq!(gc.root_statistics().slot_roots, 0);
    assert!(scope.roots.registrations.borrow().is_empty());
    drop(scope);
    Ok(())
}

#[test]
fn detached_cleanup_does_not_adopt_an_unrelated_construction_scope() -> crate::Result<()> {
    for concurrent in [false, true] {
        for storage in [RootStorage::Tree, RootStorage::Slots, RootStorage::Contexts] {
            let gc = collector(concurrent, storage);
            let context = gc.register_context();
            let drops = Arc::new(AtomicUsize::new(0));
            let registrations = context.with(|| capture(&gc, &drops, 3))?;
            gc.collect_full()?;
            assert_eq!(drops.load(Ordering::Relaxed), 0);
            release_in_unrelated_scope(&gc, registrations, &drops)?;
        }
    }
    Ok(())
}

#[cfg(not(target_family = "wasm"))]
#[test]
#[expect(
    clippy::unwrap_in_result,
    reason = "join the registration cleanup helper"
)]
fn detached_context_roots_can_be_released_by_another_thread() -> crate::Result<()> {
    for concurrent in [false, true] {
        for storage in [RootStorage::Tree, RootStorage::Slots, RootStorage::Contexts] {
            let gc = collector(concurrent, storage);
            let context = gc.register_context();
            let drops = Arc::new(AtomicUsize::new(0));
            let registrations = context.with(|| capture(&gc, &drops, 9))?;
            drop(context);
            let helper_gc = gc.clone();
            std::thread::spawn(move || {
                release_in_unrelated_scope(&helper_gc, registrations, &drops)
            })
            .join()
            .expect("root cleanup helper")?;
            gc.stop()?;
            assert_eq!(gc.managed_bytes(), 0);
        }
    }
    Ok(())
}

#[test]
fn nested_batches_preserve_roots_across_a_foreign_construction_scope() -> crate::Result<()> {
    for concurrent in [false, true] {
        for storage in [RootStorage::Tree, RootStorage::Slots, RootStorage::Contexts] {
            for populated in [false, true] {
                let gc = collector(concurrent, storage);
                let foreign = collector(concurrent, storage);
                let context = gc.register_context();
                let drops = Arc::new(AtomicUsize::new(0));
                let mut outer = Roots::new(&gc);
                let outer_scope = outer.enter();
                if populated {
                    context.with(|| Gc::try_new(&gc, Leaf(drops.clone())).map(drop))?;
                }
                let mut foreign_roots = Roots::new(&foreign);
                let foreign_scope = foreign_roots.enter();
                let mut inner = Roots::new(&gc);
                let inner_scope = inner.enter();
                context.with(|| -> crate::Result<()> {
                    for _ in 0..3 {
                        drop(Gc::try_new(&gc, Leaf(drops.clone()))?);
                    }
                    Ok(())
                })?;
                drop(inner_scope);
                drop(context);
                gc.collect_full()?;
                foreign.collect_full()?;
                assert_eq!(
                    drops.load(Ordering::Relaxed),
                    0,
                    "reference-free allocation lost enclosing construction scope"
                );
                let statistics = gc.root_statistics();
                assert_eq!(
                    statistics.compatibility_roots + statistics.slot_roots,
                    3 + usize::from(populated)
                );
                assert_eq!(foreign.root_statistics().compatibility_roots, 0);
                assert_eq!(foreign.root_statistics().slot_roots, 0);
                drop(foreign_scope);
                gc.collect_full()?;
                assert_eq!(drops.load(Ordering::Relaxed), 0);
                drop(outer_scope);
                gc.stop()?;
                assert_eq!(drops.load(Ordering::Relaxed), 3 + usize::from(populated));
                assert_eq!(gc.managed_bytes(), 0);
                assert_eq!(gc.root_statistics().compatibility_roots, 0);
                assert_eq!(gc.root_statistics().slot_roots, 0);
            }
        }
    }
    Ok(())
}

#[test]
fn detached_cleanup_preserves_deletion_barriers_and_exact_registration_identity()
-> crate::Result<()> {
    for concurrent in [false, true] {
        for storage in [RootStorage::Tree, RootStorage::Slots, RootStorage::Contexts] {
            let gc = collector(concurrent, storage);
            let context = gc.register_context();
            let drops = Arc::new(AtomicUsize::new(0));
            let registrations = context.with(|| capture(&gc, &drops, 1))?;
            gc.collect();
            assert!(!gc.step());
            for registration in &registrations {
                gc.remove_detached_root(registration);
            }
            while !gc.step() {}
            assert_eq!(drops.load(Ordering::Relaxed), 0, "snapshot edge lost");
            gc.collect_full()?;
            assert_eq!(drops.load(Ordering::Relaxed), 1);
            let replacement = context.with(|| Gc::try_new(&gc, Leaf(drops.clone())))?;
            for registration in &registrations {
                gc.remove_detached_root(registration);
            }
            gc.collect_full()?;
            assert_eq!(drops.load(Ordering::Relaxed), 1);
            drop(replacement);
            drop(context);
            gc.stop()?;
            assert_eq!(drops.load(Ordering::Relaxed), 2);
            assert_eq!(gc.managed_bytes(), 0);
        }
    }
    Ok(())
}

#[cfg(panic = "unwind")]
#[test]
fn failed_private_parent_root_is_not_adopted_by_an_outer_constructor() -> crate::Result<()> {
    struct Private(Arc<AtomicUsize>);
    // SAFETY: no outgoing managed references; metadata hook intentionally unwinds.
    unsafe impl Trace for Private {
        fn trace(&self, _: &GarbageCollector) {}
        fn trace_work(&self) -> usize {
            panic!("private metadata hook failure");
        }
    }
    impl Drop for Private {
        fn drop(&mut self) {
            self.0.fetch_add(1, Ordering::Relaxed);
        }
    }
    for concurrent in [false, true] {
        for storage in [RootStorage::Tree, RootStorage::Slots, RootStorage::Contexts] {
            let gc = collector(concurrent, storage);
            let context = gc.register_context();
            let drops = Arc::new(AtomicUsize::new(0));
            let mut outer = Roots::new(&gc);
            let outer_scope = outer.enter();
            let result = context.with(|| {
                std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                    Gc::try_new(&gc, Private(drops.clone()))
                }))
            });
            assert!(result.is_err());
            assert_eq!(drops.load(Ordering::Relaxed), 1);
            assert_eq!(gc.statistics()?.live_objects, 0);
            assert_eq!(gc.managed_bytes(), 0);
            let roots = gc.root_statistics();
            assert_eq!(
                roots.compatibility_roots + roots.slot_roots,
                0,
                "destroyed unpublished parent left a root in the enclosing constructor"
            );
            drop(outer_scope);
        }
    }
    Ok(())
}

#[test]
fn sealed_small_and_large_batches_preserve_exact_graph_ownership() -> crate::Result<()> {
    for concurrent in [false, true] {
        for storage in [RootStorage::Tree, RootStorage::Slots, RootStorage::Contexts] {
            for count in [0, 4, 5, 9] {
                let gc = collector(concurrent, storage);
                let context = gc.register_context();
                let drops = Arc::new(AtomicUsize::new(0));
                let mut roots = Roots::new(&gc);
                let scope = roots.enter();
                context.with(|| -> crate::Result<()> {
                    for _ in 0..count {
                        drop(Gc::try_new(&gc, Leaf(drops.clone()))?);
                    }
                    Ok(())
                })?;
                let detached = scope.detach();
                drop(context);
                gc.collect_full()?;
                assert_eq!(
                    drops.load(Ordering::Relaxed),
                    0,
                    "sealed batch lost a captured root"
                );
                let mut unrelated = Roots::new(&gc);
                let unrelated_scope = unrelated.enter();
                drop(detached);
                gc.collect_full()?;
                gc.collect_full()?;
                assert_eq!(
                    drops.load(Ordering::Relaxed),
                    count,
                    "sealed small or large roots leaked"
                );
                assert_eq!(gc.managed_bytes(), 0);
                let roots = gc.root_statistics();
                assert_eq!(roots.compatibility_roots + roots.slot_roots, 0);
                assert!(unrelated_scope.roots.registrations.borrow().is_empty());
                drop(unrelated_scope);
            }
        }
    }
    Ok(())
}
