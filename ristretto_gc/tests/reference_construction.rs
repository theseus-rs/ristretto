//! Unpublished conditional owners retain only their graph, never a whole cycle.
#![expect(clippy::expect_used, clippy::panic_in_result_fn)]
#![cfg_attr(not(target_family = "wasm"), expect(clippy::unwrap_in_result))]
use ristretto_gc::{Configuration, GarbageCollector, Gc, GcWeak, ReferenceKind, Trace};
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

const KINDS: [ReferenceKind; 5] = [
    ReferenceKind::Soft,
    ReferenceKind::Weak,
    ReferenceKind::Final,
    ReferenceKind::Phantom,
    ReferenceKind::PhantomRetained,
];

struct Leaf(Arc<AtomicUsize>);
// SAFETY: this fixture contains no outgoing GC references.
#[expect(unsafe_code, reason = "Trace requires an explicit ownership contract")]
unsafe impl Trace for Leaf {
    fn trace(&self, _: &GarbageCollector) {}
}
impl Drop for Leaf {
    fn drop(&mut self) {
        self.0.fetch_add(1, Ordering::Relaxed);
    }
}

struct Referent(Gc<Leaf>);
// SAFETY: all strong fields are traced and paired root/demotion hooks forward the owner.
#[expect(unsafe_code, reason = "Trace requires an explicit ownership contract")]
unsafe impl Trace for Referent {
    fn root(&mut self) {
        self.0.root();
    }
    fn trace(&self, gc: &GarbageCollector) {
        self.0.trace(gc);
    }
    unsafe fn unroot(&mut self, owner: ristretto_gc::TraceOwner<'_>) {
        unsafe { Trace::unroot(&mut self.0, owner) };
    }
}

struct Conditional {
    link: GcWeak<Referent>,
    hook: Arc<dyn Fn(&GarbageCollector) + Send + Sync>,
    fail: bool,
}
// SAFETY: the only GC link is conditional; its slot handles tracing and owner binding.
#[expect(unsafe_code, reason = "Trace requires an explicit ownership contract")]
unsafe impl Trace for Conditional {
    fn trace(&self, _: &GarbageCollector) {}
    unsafe fn attach(&mut self, owner: ristretto_gc::TraceOwner<'_>) {
        let gc = owner.collector();
        let address = owner.address();
        self.link.set_owner(address);
        // Repeated attachment in the same scope owns one reservation/root.
        self.link.set_owner(address);
        let gc = gc.upgrade().expect("collector");
        (self.hook)(&gc);
        assert!(!self.fail, "injected conditional attachment failure");
    }
}

fn collector(concurrent: bool, independent: bool) -> Arc<GarbageCollector> {
    GarbageCollector::with_config(
        Configuration::builder()
            .concurrent_publication(concurrent)
            .independent_phases(independent)
            .independent_marking(independent)
            .overlapping_generations(independent)
            .metadata_shards(16)
            .diagnostics(true)
            .incremental_step_size(4)
            .allocation_threshold(usize::MAX)
            .build(),
    )
}

fn reference(
    gc: &Arc<GarbageCollector>,
    kind: ReferenceKind,
    drops: &Arc<AtomicUsize>,
) -> ristretto_gc::Result<GcWeak<Referent>> {
    let leaf = Gc::try_new(gc, Leaf(drops.clone()))?.into_rooted();
    let referent = Gc::try_new(gc, Referent(leaf))?;
    Ok(GcWeak::new(&referent, kind, 0))
}

fn finish(
    gc: &Arc<GarbageCollector>,
    owner: Option<ristretto_gc::GcRootGuard<Conditional>>,
    drops: &AtomicUsize,
) -> ristretto_gc::Result<()> {
    gc.collect_full()?;
    if let Some(owner) = &owner {
        assert!(gc.has_pending_references());
        let queued = gc
            .take_pending_reference::<Conditional>()?
            .expect("published owner notification");
        assert_eq!(queued.as_ptr(), owner.as_ptr());
        let retaining = matches!(
            owner.link.upgrade(),
            Some(target) if target.0.0.load(Ordering::Relaxed) == 0
        );
        assert_eq!(drops.load(Ordering::Relaxed), usize::from(!retaining));
        owner.link.clear();
        drop(queued);
    } else {
        assert!(!gc.has_pending_references());
    }
    drop(owner);
    gc.stop()?;
    assert_eq!(drops.load(Ordering::Relaxed), 1);
    assert_eq!(gc.statistics()?.live_objects, 0);
    assert_eq!(gc.managed_bytes(), 0);
    assert_eq!(gc.root_statistics().compatibility_roots, 0);
    assert_eq!(gc.diagnostics().active_publications, 0);
    Ok(())
}

#[test]
fn conditional_attachment_can_collect_and_unwind() -> ristretto_gc::Result<()> {
    for concurrent in [false, true] {
        for independent in [false, true] {
            for kind in KINDS {
                for fail in [false, true] {
                    if fail && cfg!(panic = "abort") {
                        continue;
                    }
                    let gc = collector(concurrent, independent);
                    let drops = Arc::new(AtomicUsize::new(0));
                    let link = reference(&gc, kind, &drops)?;
                    let garbage = Arc::new(AtomicUsize::new(0));
                    drop(Gc::try_new(&gc, Leaf(garbage.clone()))?);
                    let retained = drops.clone();
                    let owner = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                        Gc::try_new(
                            &gc,
                            Conditional {
                                link,
                                fail,
                                hook: Arc::new(move |gc| {
                                    for _ in 0..3 {
                                        gc.collect_full().expect("collection inside hook");
                                        assert_eq!(retained.load(Ordering::Relaxed), 0);
                                        assert_eq!(garbage.load(Ordering::Relaxed), 1);
                                        assert!(!gc.has_pending_references());
                                        assert_eq!(gc.diagnostics().active_publications, 0);
                                    }
                                }),
                            },
                        )
                    }));
                    assert_eq!(owner.is_err(), fail);
                    finish(&gc, owner.ok().transpose()?, &drops)?;
                }
            }
        }
    }
    Ok(())
}

#[cfg(not(target_family = "wasm"))]
#[test]
fn suspended_conditional_attachment_allows_unrelated_reclamation() -> ristretto_gc::Result<()> {
    use std::sync::{Condvar, Mutex, mpsc};
    use std::time::Duration;
    for concurrent in [false, true] {
        for independent in [false, true] {
            for kind in KINDS {
                let gc = collector(concurrent, independent);
                let drops = Arc::new(AtomicUsize::new(0));
                let link = reference(&gc, kind, &drops)?;
                let (entered, observed) = mpsc::channel();
                let release = Arc::new((Mutex::new(false), Condvar::new()));
                let constructor = {
                    let (gc, release) = (gc.clone(), release.clone());
                    std::thread::spawn(move || {
                        Gc::try_new(
                            &gc,
                            Conditional {
                                link,
                                fail: false,
                                hook: Arc::new(move |_| {
                                    entered.send(()).expect("observer");
                                    let (mutex, changed) = &*release;
                                    let mut ready = mutex.lock().expect("release lock");
                                    while !*ready {
                                        ready = changed.wait(ready).expect("release wait");
                                    }
                                }),
                            },
                        )
                    })
                };
                observed
                    .recv_timeout(Duration::from_secs(10))
                    .expect("attachment entered");
                let garbage = Arc::new(AtomicUsize::new(0));
                let (done, completion) = mpsc::channel();
                let worker = {
                    let (gc, garbage) = (gc.clone(), garbage.clone());
                    std::thread::spawn(move || {
                        let result = (|| -> ristretto_gc::Result<()> {
                            for _ in 0..3 {
                                drop(Gc::try_new(&gc, Leaf(garbage.clone()))?);
                                gc.collect_full()?;
                            }
                            Ok(())
                        })();
                        done.send(result).expect("completion observer");
                    })
                };
                let result = completion.recv_timeout(Duration::from_secs(10));
                let reclaimed = garbage.load(Ordering::Relaxed);
                let retained = drops.load(Ordering::Relaxed);
                let admissions = gc.diagnostics().active_publications;
                // Release before assertions so a regression cannot strand workers.
                *release.0.lock().expect("release lock") = true;
                release.1.notify_all();
                let owner = constructor.join().expect("constructor")?;
                worker.join().expect("collector");
                result.expect("collection waited for conditional attachment")?;
                assert_eq!(reclaimed, 3);
                assert_eq!(retained, 0);
                assert_eq!(admissions, 0);
                finish(&gc, Some(owner), &drops)?;
            }
        }
    }
    Ok(())
}
