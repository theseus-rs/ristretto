//! Construction hooks retain their graph through temporary owning registrations.
#![expect(clippy::expect_used, clippy::panic_in_result_fn)]
#![cfg_attr(not(target_family = "wasm"), expect(clippy::unwrap_in_result))]
use ristretto_gc::{Configuration, GarbageCollector, Gc, RootStorage, Trace};
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

struct Child(Arc<AtomicUsize>);
// SAFETY: this fixture contains no outgoing GC references.
#[expect(unsafe_code, reason = "Trace requires an explicit ownership contract")]
unsafe impl Trace for Child {
    fn trace(&self, _: &GarbageCollector) {}
}
impl Drop for Child {
    fn drop(&mut self) {
        self.0.fetch_add(1, Ordering::Relaxed);
    }
}

struct CollectInHook {
    child: Gc<Child>,
    nested: bool,
    panic: bool,
}
// SAFETY: all strong fields are traced and paired root/demotion hooks forward the owner.
#[expect(unsafe_code, reason = "Trace requires an explicit ownership contract")]
unsafe impl Trace for CollectInHook {
    fn root(&mut self) {
        self.child.root();
    }
    unsafe fn unroot(&mut self, owner: ristretto_gc::TraceOwner<'_>) {
        // SAFETY: child belongs to the protected graph supplied by the caller.
        unsafe { Trace::unroot(&mut self.child, owner) };
    }
    fn trace(&self, collector: &GarbageCollector) {
        self.child.trace(collector);
    }
    unsafe fn attach(&mut self, owner: ristretto_gc::TraceOwner<'_>) {
        let collector = owner.collector();
        let collector = collector.upgrade().expect("collector");
        unsafe { Trace::unroot(&mut self.child, owner) };
        if self.nested {
            // Inner scope cleanup transfers its construction roots to the outer
            // scope. The temporary nested parent itself may be collected afterward.
            drop(
                Gc::try_new(
                    &collector,
                    Self {
                        child: self.child.rooted_clone(),
                        nested: false,
                        panic: false,
                    },
                )
                .expect("nested construction"),
            );
        }
        collector.collect_full().expect("collection from a hook");
        assert_eq!(self.child.0.load(Ordering::Relaxed), 0);
        assert!(!self.panic, "injected attachment failure");
    }
}

#[test]
fn nested_attachment_can_collect_without_losing_construction_edges() -> ristretto_gc::Result<()> {
    for concurrent in [false, true] {
        for storage in [RootStorage::Tree, RootStorage::Slots, RootStorage::Contexts] {
            let gc = GarbageCollector::with_config(
                Configuration::builder()
                    .concurrent_publication(concurrent)
                    .root_storage(storage)
                    .barrier_buffer_size(4)
                    .allocation_threshold(usize::MAX)
                    .build(),
            );
            let context = gc.register_context();
            let drops = Arc::new(AtomicUsize::new(0));
            let parent = context.with(|| {
                let child = Gc::try_new(&gc, Child(drops.clone()))?.into_rooted();
                Gc::try_new(
                    &gc,
                    CollectInHook {
                        child,
                        nested: true,
                        panic: false,
                    },
                )
            })?;
            assert_eq!(
                gc.root_statistics().compatibility_roots,
                usize::from(storage == RootStorage::Tree)
            );
            assert_eq!(drops.load(Ordering::Relaxed), 0);
            drop(parent);
            drop(context);
            gc.stop()?;
            assert_eq!(drops.load(Ordering::Relaxed), 1);
            assert_eq!(gc.managed_bytes(), 0);
            assert_eq!(gc.root_statistics().compatibility_roots, 0);
        }
    }
    Ok(())
}

#[cfg(panic = "unwind")]
#[test]
fn attachment_unwind_releases_temporary_roots_and_allocation_credit() -> ristretto_gc::Result<()> {
    for concurrent in [false, true] {
        let gc = GarbageCollector::with_config(
            Configuration::builder()
                .concurrent_publication(concurrent)
                .build(),
        );
        let drops = Arc::new(AtomicUsize::new(0));
        let child = Gc::try_new(&gc, Child(drops.clone()))?.into_rooted();
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let _ = Gc::try_new(
                &gc,
                CollectInHook {
                    child,
                    nested: true,
                    panic: true,
                },
            );
        }));
        assert!(result.is_err());
        assert_eq!(gc.root_statistics().compatibility_roots, 0);
        gc.stop()?;
        assert_eq!(drops.load(Ordering::Relaxed), 1);
        assert_eq!(gc.managed_bytes(), 0);
        // An unwound scope must not capture roots released by later unrelated calls.
        drop(Gc::try_new(&gc, 42_u64)?);
        gc.collect_full()?;
        assert_eq!(gc.managed_bytes(), 0);
    }
    Ok(())
}

#[cfg(not(target_family = "wasm"))]
#[test]
fn a_stalled_attachment_does_not_hold_collection_open() -> ristretto_gc::Result<()> {
    for concurrent in [false, true] {
        use ristretto_gc::{GcWeak, ReferenceKind};
        use std::sync::{Condvar, Mutex, mpsc};
        use std::time::Duration;
        struct Stalled {
            child: Gc<Child>,
            entered: mpsc::Sender<()>,
            release: Arc<(Mutex<bool>, Condvar)>,
        }
        // SAFETY: all strong fields are traced and paired root/demotion hooks forward the owner.
        #[expect(unsafe_code, reason = "Trace requires an explicit ownership contract")]
        unsafe impl Trace for Stalled {
            fn root(&mut self) {
                self.child.root();
            }
            unsafe fn unroot(&mut self, owner: ristretto_gc::TraceOwner<'_>) {
                // SAFETY: child belongs to the protected graph supplied by the caller.
                unsafe { Trace::unroot(&mut self.child, owner) };
            }
            fn trace(&self, collector: &GarbageCollector) {
                self.child.trace(collector);
            }
            unsafe fn attach(&mut self, owner: ristretto_gc::TraceOwner<'_>) {
                unsafe { Trace::unroot(&mut self.child, owner) };
                self.entered.send(()).expect("observer");
                let (mutex, changed) = &*self.release;
                let mut released = mutex.lock().expect("release lock");
                while !*released {
                    released = changed.wait(released).expect("release wait");
                }
            }
        }
        for storage in [RootStorage::Tree, RootStorage::Slots, RootStorage::Contexts] {
            for fallible in [false, true] {
                let gc = GarbageCollector::with_config(
                    Configuration::builder()
                        .concurrent_publication(concurrent)
                        .root_storage(storage)
                        .barrier_buffer_size(4)
                        .allocation_threshold(usize::MAX)
                        .build(),
                );
                let context = gc.register_context();
                let drops = Arc::new(AtomicUsize::new(0));
                let child = context
                    .with(|| Gc::try_new(&gc, Child(drops.clone())))?
                    .into_rooted();
                let weak = GcWeak::new(&child, ReferenceKind::Weak, 0);
                drop(Gc::try_new(&gc, Child(drops.clone()))?);
                let (entered, observed) = mpsc::channel();
                let release = Arc::new((Mutex::new(false), Condvar::new()));
                let constructor = {
                    let (gc, release) = (gc.clone(), release.clone());
                    std::thread::spawn(move || {
                        context.with(|| {
                            let data = Stalled {
                                child,
                                entered,
                                release,
                            };
                            if fallible {
                                Gc::try_new(&gc, data)
                            } else {
                                Ok(Gc::new(&gc, data))
                            }
                        })
                    })
                };
                observed
                    .recv_timeout(Duration::from_secs(10))
                    .expect("attachment entered");
                let (completed, completion) = mpsc::channel();
                let collector = {
                    let gc = gc.clone();
                    std::thread::spawn(move || {
                        completed
                            .send(gc.collect_full())
                            .expect("completion observer");
                    })
                };
                let result = completion.recv_timeout(Duration::from_secs(10));
                // Always release/join before asserting so a regression cannot strand threads.
                *release.0.lock().expect("release lock") = true;
                release.1.notify_all();
                let parent = constructor.join().expect("constructor")?;
                collector.join().expect("collector");
                result.expect("collection waited for arbitrary attachment")?;
                assert_eq!(drops.load(Ordering::Relaxed), 1);
                assert!(weak.upgrade().is_some());
                assert_eq!(parent.child.0.load(Ordering::Relaxed), 1);
                drop(parent);
                gc.stop()?;
                assert_eq!(drops.load(Ordering::Relaxed), 2);
                assert_eq!(gc.managed_bytes(), 0);
                assert_eq!(gc.root_statistics().compatibility_roots, 0);
            }
        }
    }
    Ok(())
}
