//! Private publication cancellation must release roots before payload destruction.
#![cfg(all(panic = "unwind", not(target_family = "wasm")))]
#![expect(clippy::panic_in_result_fn)]
use super::*;

struct Watch {
    collector: Weak<GarbageCollector>,
    observed_roots: Arc<AtomicUsize>,
}
// SAFETY: no outgoing managed references.
unsafe impl Trace for Watch {
    fn trace(&self, _: &GarbageCollector) {}
}
impl Finalize for Watch {
    fn finalize(&self) {}
}
impl Drop for Watch {
    fn drop(&mut self) {
        if let Some(gc) = self.collector.upgrade() {
            let roots = gc.root_statistics();
            self.observed_roots.store(
                roots.compatibility_roots + roots.slot_roots,
                Ordering::Release,
            );
        }
    }
}

#[test]
fn private_payload_destruction_observes_initial_root_already_removed() -> Result<()> {
    use std::sync::mpsc;
    for counted in [false, true] {
        for storage in [
            crate::RootStorage::Tree,
            crate::RootStorage::Slots,
            crate::RootStorage::Contexts,
        ] {
            for constructor in 0..3 {
                let gc = GarbageCollector::with_config(
                    Configuration::builder()
                        .concurrent_publication(counted)
                        .root_storage(storage)
                        .allocation_threshold(usize::MAX)
                        .build(),
                );
                let context = gc.register_context();
                let observed_roots = Arc::new(AtomicUsize::new(usize::MAX));
                let (entered, observed) = mpsc::channel();
                let (resume, receiver) = mpsc::channel();
                drop(resume);
                *lock(&gc.metadata_prepare_pause) = Some(ClaimPause {
                    entered,
                    resume: receiver,
                });
                let result = context.with(|| {
                    std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                        let value = Watch {
                            collector: Arc::downgrade(&gc),
                            observed_roots: observed_roots.clone(),
                        };
                        match constructor {
                            0 => drop(Gc::try_new(&gc, value)),
                            1 => drop(Gc::new(&gc, value)),
                            _ => drop(Gc::new_with_finalizer(&gc, value)),
                        }
                    }))
                });
                assert!(result.is_err());
                assert!(observed.try_recv().is_ok());
                assert_eq!(gc.managed_bytes(), 0);
                assert_eq!(gc.statistics()?.live_objects, 0);
                assert_eq!(
                    observed_roots.load(Ordering::Acquire),
                    0,
                    "private payload destruction ran before initial-root cancellation"
                );
                let roots = gc.root_statistics();
                assert_eq!(roots.compatibility_roots + roots.slot_roots, 0);
            }
        }
    }
    Ok(())
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Hook {
    TraceWork,
    HeapOwner,
    Relocation,
}

struct Child(Arc<AtomicUsize>);
// SAFETY: no outgoing managed references.
unsafe impl Trace for Child {
    fn trace(&self, _: &GarbageCollector) {}
}
impl Drop for Child {
    fn drop(&mut self) {
        self.0.fetch_add(1, Ordering::Relaxed);
    }
}

struct Parent {
    child: Gc<Child>,
    hook: Hook,
    _watch: Watch,
    drops: Arc<AtomicUsize>,
}
// SAFETY: attachment and tracing visit the only outgoing managed edge.
unsafe impl Trace for Parent {
    fn trace(&self, gc: &GarbageCollector) {
        self.child.trace(gc);
    }
    unsafe fn attach(&mut self, owner: crate::TraceOwner<'_>) {
        // SAFETY: the caller protects the complete graph during attachment.
        unsafe { self.child.attach(owner) };
    }
    fn trace_work(&self) -> usize {
        assert_ne!(self.hook, Hook::TraceWork, "injected trace-work failure");
        1
    }
    fn heap_owner(&self) -> Option<Arc<dyn HeapOwner>> {
        assert_ne!(self.hook, Hook::HeapOwner, "injected heap-owner failure");
        None
    }
    fn relocation_region(&self, _: &crate::region::RegionHeap) -> Option<u64> {
        assert_ne!(self.hook, Hook::Relocation, "injected relocation failure");
        None
    }
}
impl Finalize for Parent {
    fn finalize(&self) {}
}
impl Drop for Parent {
    fn drop(&mut self) {
        self.drops.fetch_add(1, Ordering::Relaxed);
    }
}

#[test]
fn metadata_hook_unwind_removes_parent_but_preserves_construction_children() -> Result<()> {
    for counted in [false, true] {
        for storage in [
            crate::RootStorage::Tree,
            crate::RootStorage::Slots,
            crate::RootStorage::Contexts,
        ] {
            for constructor in 0..3 {
                for hook in [Hook::TraceWork, Hook::HeapOwner, Hook::Relocation] {
                    let gc = GarbageCollector::with_config(
                        Configuration::builder()
                            .concurrent_publication(counted)
                            .root_storage(storage)
                            .compact_heap(true)
                            .cost_aware_relocation(true)
                            .allocation_threshold(usize::MAX)
                            .build(),
                    );
                    let context = gc.register_context();
                    let child_drops = Arc::new(AtomicUsize::new(0));
                    let parent_drops = Arc::new(AtomicUsize::new(0));
                    let observed_roots = Arc::new(AtomicUsize::new(usize::MAX));
                    let mut outer = crate::construction::Roots::new(&gc);
                    let outer_scope = outer.enter();
                    let result = context.with(|| {
                        std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                            let child = Gc::new(&gc, Child(child_drops.clone())).into_rooted();
                            let value = Parent {
                                child,
                                hook,
                                _watch: Watch {
                                    collector: Arc::downgrade(&gc),
                                    observed_roots: observed_roots.clone(),
                                },
                                drops: parent_drops.clone(),
                            };
                            match constructor {
                                0 => drop(Gc::try_new(&gc, value)),
                                1 => drop(Gc::new(&gc, value)),
                                _ => drop(Gc::new_with_finalizer(&gc, value)),
                            }
                        }))
                    });
                    assert!(result.is_err(), "{hook:?} did not run");
                    assert_eq!(parent_drops.load(Ordering::Relaxed), 1);
                    assert_eq!(observed_roots.load(Ordering::Acquire), 1);
                    let roots = gc.root_statistics();
                    assert_eq!(roots.compatibility_roots + roots.slot_roots, 1);
                    gc.collect_full()?;
                    assert_eq!(child_drops.load(Ordering::Relaxed), 0);
                    assert_eq!(gc.statistics()?.live_objects, 1);
                    drop(outer_scope);
                    drop(context);
                    gc.stop()?;
                    assert_eq!(child_drops.load(Ordering::Relaxed), 1);
                    assert_eq!(gc.managed_bytes(), 0);
                    let roots = gc.root_statistics();
                    assert_eq!(roots.compatibility_roots + roots.slot_roots, 0);
                }
            }
        }
    }
    Ok(())
}
