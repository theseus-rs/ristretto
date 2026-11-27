//! Infallible APIs still own storage and credits when user hooks unwind.
#![cfg(panic = "unwind")]
#![expect(clippy::panic_in_result_fn)]
use ristretto_gc::{Configuration, Finalize, GarbageCollector, Gc, HeapOwner, RootStorage, Trace};
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Hook {
    Attach,
    TraceWork,
    HeapOwner,
    None,
}

struct Value {
    hook: Hook,
    drops: Arc<AtomicUsize>,
    finalizations: Arc<AtomicUsize>,
}
// SAFETY: this fixture contains no outgoing GC references.
#[expect(unsafe_code, reason = "Trace requires an explicit ownership contract")]
unsafe impl Trace for Value {
    fn trace(&self, _: &GarbageCollector) {}
    unsafe fn attach(&mut self, _owner: ristretto_gc::TraceOwner<'_>) {
        assert_ne!(self.hook, Hook::Attach, "injected attach failure");
    }
    fn trace_work(&self) -> usize {
        assert_ne!(
            self.hook,
            Hook::TraceWork,
            "injected trace metadata failure"
        );
        1
    }
    fn heap_owner(&self) -> Option<Arc<dyn HeapOwner>> {
        assert_ne!(
            self.hook,
            Hook::HeapOwner,
            "injected owner metadata failure"
        );
        None
    }
}
impl Finalize for Value {
    fn finalize(&self) {
        self.finalizations.fetch_add(1, Ordering::Relaxed);
    }
}
impl Drop for Value {
    fn drop(&mut self) {
        self.drops.fetch_add(1, Ordering::Relaxed);
    }
}

#[expect(unsafe_code)] // Unrooted fixtures are dropped before any collection can run.
fn construct(gc: &Arc<GarbageCollector>, value: Value, rooted: bool, finalizer: bool) {
    match (rooted, finalizer) {
        (true, false) => drop(Gc::new(gc, value)),
        (true, true) => drop(Gc::new_with_finalizer(gc, value)),
        // SAFETY: no worker is running, collection is not requested by allocation,
        // and the unrooted handle is discarded before the fixture collects.
        (false, false) => drop(unsafe { Gc::with_collector_unrooted(gc, value) }),
        // SAFETY: same unrooted lifetime as the preceding branch.
        (false, true) => drop(unsafe { Gc::with_collector_and_finalizer_unrooted(gc, value) }),
    }
}

#[test]
fn every_legacy_constructor_refunds_hook_unwinds_and_finalizes_success_once()
-> ristretto_gc::Result<()> {
    for concurrent in [false, true] {
        for storage in [RootStorage::Tree, RootStorage::Slots, RootStorage::Contexts] {
            for hook in [Hook::Attach, Hook::TraceWork, Hook::HeapOwner, Hook::None] {
                for rooted in [false, true] {
                    for finalizer in [false, true] {
                        let gc = GarbageCollector::with_config(
                            Configuration::builder()
                                .concurrent_publication(concurrent)
                                .root_storage(storage)
                                .allocation_threshold(usize::MAX)
                                .max_heap_bytes(1024)
                                .build(),
                        );
                        let context = gc.register_context();
                        let drops = Arc::new(AtomicUsize::new(0));
                        let finalizations = Arc::new(AtomicUsize::new(0));
                        let result = context.with(|| {
                            std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                                construct(
                                    &gc,
                                    Value {
                                        hook,
                                        drops: drops.clone(),
                                        finalizations: finalizations.clone(),
                                    },
                                    rooted,
                                    finalizer,
                                );
                            }))
                        });
                        assert_eq!(result.is_err(), hook != Hook::None);
                        if hook != Hook::None {
                            assert_eq!(drops.load(Ordering::Relaxed), 1);
                            assert_eq!(finalizations.load(Ordering::Relaxed), 0);
                            assert_eq!(gc.managed_bytes(), 0);
                        }
                        drop(context);
                        gc.stop()?;
                        assert_eq!(drops.load(Ordering::Relaxed), 1);
                        assert_eq!(
                            finalizations.load(Ordering::Relaxed),
                            usize::from(finalizer && hook == Hook::None)
                        );
                        assert_eq!(gc.managed_bytes(), 0);
                        assert_eq!(gc.statistics()?.live_objects, 0);
                        assert_eq!(gc.root_statistics().compatibility_roots, 0);
                        assert_eq!(gc.root_statistics().slot_roots, 0);
                        assert_eq!(gc.root_statistics().pages, 0);
                    }
                }
            }
        }
    }
    Ok(())
}
