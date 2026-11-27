//! Announced storage keeps descriptor identity after private graph cancellation.
use super::*;

#[test]
fn announced_storage_outlives_private_payload_and_heap_shutdown() -> crate::Result<()> {
    for counted in [false, true] {
        for storage in [RootStorage::Tree, RootStorage::Slots, RootStorage::Contexts] {
            for rooted in [false, true] {
                let gc = collector(counted, storage);
                let context = gc.register_context();
                let child = Arc::new(AtomicUsize::new(0));
                let parent = Arc::new(AtomicUsize::new(0));
                let publication =
                    context.with(|| prepare(&gc, graph(&gc, &child, &parent), rooted));
                let identity = publication.metadata().id;
                let (header, origin) = publication.share();
                let erased = header.erase();
                let helper = erased
                    .acquire(&gc)
                    .unwrap_or_else(|_| panic!("open header"));
                let storage = helper.prepare_storage();
                let duplicate = storage.clone();
                drop(context);
                drop(helper);
                let origin = origin
                    .restore()
                    .err()
                    .expect("announced ownership restored into registration");
                let helper = erased
                    .acquire(&gc)
                    .unwrap_or_else(|_| panic!("announced live header"));
                reclaim(&gc)?;
                assert_eq!(
                    storage
                        .view(&helper)
                        .expect("matching header")
                        .metadata()
                        .id,
                    identity
                );
                assert!(
                    duplicate
                        .view(&helper)
                        .expect("matching header")
                        .metadata()
                        .is_type::<Parent>()
                );
                assert_eq!(parent.load(Ordering::Relaxed), 0);
                assert_eq!(child.load(Ordering::Relaxed), 0);
                drop(helper);
                drop(origin);
                assert_eq!(
                    parent.load(Ordering::Relaxed),
                    1,
                    "retained storage delayed private payload cancellation"
                );
                assert!(matches!(header.acquire(&gc), Err(AcquireError::Closed)));
                assert!(matches!(erased.acquire(&gc), Err(AcquireError::Closed)));
                reclaim(&gc)?;
                assert_eq!(child.load(Ordering::Relaxed), 1);
                assert_eq!(gc.managed_bytes(), 0);
                assert_eq!(storage.identity(), identity);
                let weak = Arc::downgrade(&gc);
                drop(gc);
                assert!(weak.upgrade().is_none());
                assert_eq!(duplicate.identity(), identity);
                drop(storage);
                drop(duplicate);
                assert_eq!(parent.load(Ordering::Relaxed), 1);
            }
        }
    }
    Ok(())
}

#[test]
fn storage_views_require_the_exact_header_even_within_one_heap() -> crate::Result<()> {
    let gc = collector(true, RootStorage::Contexts);
    let foreign = collector(true, RootStorage::Contexts);
    let first_drops = Arc::new(AtomicUsize::new(0));
    let (first, first_origin) = prepare(&gc, Leaf(first_drops.clone()), true).share();
    let (other, other_origin) = prepare(&gc, Leaf(Arc::new(AtomicUsize::new(0))), true).share();
    let (foreign_header, foreign_origin) =
        prepare(&foreign, Leaf(Arc::new(AtomicUsize::new(0))), true).share();
    let first_erased = first.erase();
    let other_erased = other.erase();
    let foreign_erased = foreign_header.erase();
    let first_helper = first_erased
        .acquire(&gc)
        .unwrap_or_else(|_| panic!("first header"));
    let other_helper = other_erased
        .acquire(&gc)
        .unwrap_or_else(|_| panic!("other header"));
    let foreign_helper = foreign_erased
        .acquire(&foreign)
        .unwrap_or_else(|_| panic!("foreign header"));
    let storage = first_helper.prepare_storage();
    assert!(
        storage.view(&other_helper).is_none(),
        "different same-heap header admitted a storage view"
    );
    assert!(storage.view(&foreign_helper).is_none());
    drop(first_helper);
    drop(first_origin);
    assert_eq!(first_drops.load(Ordering::Relaxed), 1);
    assert!(storage.view(&other_helper).is_none());
    drop(other_helper);
    drop(other_origin);
    drop(foreign_helper);
    drop(foreign_origin);
    reclaim(&gc)?;
    reclaim(&foreign)?;
    assert_eq!(gc.managed_bytes(), 0);
    assert_eq!(foreign.managed_bytes(), 0);
    Ok(())
}

#[test]
fn announced_conditional_owners_release_reservations_after_payload_cancellation()
-> crate::Result<()> {
    for counted in [false, true] {
        let gc = collector(counted, RootStorage::Contexts);
        let drops = Arc::new(AtomicUsize::new(0));
        let leaf = Gc::new(&gc, Leaf(drops.clone()));
        let weak = GcWeak::new(&leaf, ReferenceKind::Weak, 0);
        let slot = weak.slot.clone();
        let (header, origin) = prepare(&gc, Conditional(weak), true).share();
        let erased = header.erase();
        let helper = erased
            .acquire(&gc)
            .unwrap_or_else(|_| panic!("open header"));
        let storage = helper.prepare_storage();
        drop(origin);
        drop(leaf);
        reclaim(&gc)?;
        assert_eq!(drops.load(Ordering::Relaxed), 0);
        assert_eq!(slot.unpublished_owners.load(Ordering::Acquire), 1);
        drop(helper);
        assert_eq!(slot.unpublished_owners.load(Ordering::Acquire), 0);
        reclaim(&gc)?;
        assert_eq!(drops.load(Ordering::Relaxed), 1);
        assert!(gc.take_pending_reference::<Conditional>()?.is_none());
        assert_eq!(gc.managed_bytes(), 0);
        drop(storage);
    }
    Ok(())
}

#[test]
fn announced_cancellation_runs_finalizer_and_destructor_once_before_storage_release() {
    struct Finalized {
        finalized: Arc<AtomicUsize>,
        destroyed: Arc<AtomicUsize>,
    }
    // SAFETY: only ordinary counters are retained.
    unsafe impl Trace for Finalized {
        fn trace(&self, _: &GarbageCollector) {}
    }
    impl crate::Finalize for Finalized {
        fn finalize(&self) {
            self.finalized.fetch_add(1, Ordering::Relaxed);
        }
    }
    impl Drop for Finalized {
        fn drop(&mut self) {
            assert_eq!(
                self.finalized.load(Ordering::Relaxed),
                1,
                "private cancellation skipped finalization"
            );
            self.destroyed.fetch_add(1, Ordering::Relaxed);
        }
    }
    for counted in [false, true] {
        let gc = collector(counted, RootStorage::Contexts);
        let finalized = Arc::new(AtomicUsize::new(0));
        let destroyed = Arc::new(AtomicUsize::new(0));
        let mut roots = crate::construction::Roots::new(&gc);
        let scope = roots.enter_for::<Finalized>();
        let (handle, allocation, reservation) = Gc::allocate(
            &gc,
            Finalized {
                finalized: finalized.clone(),
                destroyed: destroyed.clone(),
            },
        );
        let metadata =
            gc.prepare_allocated_object_with_finalizer(allocation, reservation, handle.id);
        let publication = Publication::new(
            &gc,
            handle,
            metadata,
            Detached::from_scope(&gc, scope),
            true,
        );
        let (header, origin) = publication.share();
        let erased = header.erase();
        let helper = erased
            .acquire(&gc)
            .unwrap_or_else(|_| panic!("open header"));
        let storage = helper.prepare_storage();
        drop(origin);
        drop(helper);
        assert_eq!(finalized.load(Ordering::Relaxed), 1);
        assert_eq!(destroyed.load(Ordering::Relaxed), 1);
        assert_eq!(gc.managed_bytes(), 0);
        drop(storage);
        drop(header);
        drop(erased);
        assert_eq!(finalized.load(Ordering::Relaxed), 1);
        assert_eq!(destroyed.load(Ordering::Relaxed), 1);
    }
}

#[test]
fn announced_cancellation_preserves_private_destructor_graph_access() -> crate::Result<()> {
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
            let (header, origin) = publication.share();
            let erased = header.erase();
            let helper = erased
                .acquire(&gc)
                .unwrap_or_else(|_| panic!("open header"));
            let storage = helper.prepare_storage();
            drop(context);
            drop(origin);
            drop(helper);
            assert!(matches!(header.acquire(&gc), Err(AcquireError::Closed)));
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
            drop(storage);
        }
    }
    Ok(())
}

#[cfg(not(target_family = "wasm"))]
#[test]
fn announced_cancellation_retains_credit_through_stalled_destruction() -> crate::Result<()> {
    struct Resume(std::sync::mpsc::Sender<()>);
    impl Drop for Resume {
        fn drop(&mut self) {
            // Also release the parked destructor if an assertion unwinds.
            let _ = self.0.send(());
        }
    }
    struct StalledDrop {
        child: Gc<Leaf>,
        child_drops: Arc<AtomicUsize>,
        completed: Arc<AtomicBool>,
        entered: std::sync::mpsc::Sender<()>,
        resume: std::sync::Mutex<std::sync::mpsc::Receiver<()>>,
    }
    // SAFETY: the sole managed edge participates in tracing and protected attachment.
    unsafe impl Trace for StalledDrop {
        fn trace(&self, gc: &GarbageCollector) {
            self.child.trace(gc);
        }
        unsafe fn attach(&mut self, owner: crate::TraceOwner<'_>) {
            unsafe { self.child.attach(owner) };
        }
    }
    impl Drop for StalledDrop {
        fn drop(&mut self) {
            self.entered.send(()).expect("private destructor entered");
            self.resume
                .get_mut()
                .expect("exclusive destructor receiver")
                .recv_timeout(std::time::Duration::from_secs(120))
                .expect("resume private destructor");
            // Check outside the managed payload before dereferencing its edge.
            assert_eq!(
                self.child_drops.load(Ordering::Relaxed),
                0,
                "stalled private destructor lost its child"
            );
            assert_eq!(self.child.0.load(Ordering::Relaxed), 0);
            self.completed.store(true, Ordering::Relaxed);
        }
    }
    for counted in [false, true] {
        let gc = collector(counted, RootStorage::Contexts);
        let context = gc.register_context();
        let child = Arc::new(AtomicUsize::new(0));
        let completed = Arc::new(AtomicBool::new(false));
        let (entered, observed) = std::sync::mpsc::channel();
        let (resume, resumed) = std::sync::mpsc::channel();
        let (header, handle) = context.with(|| {
            prepare(
                &gc,
                StalledDrop {
                    child: Gc::new(&gc, Leaf(child.clone())).into_rooted(),
                    child_drops: child.clone(),
                    completed: completed.clone(),
                    entered,
                    resume: std::sync::Mutex::new(resumed),
                },
                true,
            )
            .share()
        });
        let erased = header.erase();
        let helper = erased
            .acquire(&gc)
            .unwrap_or_else(|_| panic!("open header"));
        let storage = helper.prepare_storage();
        drop(helper);
        drop(context);
        let charged = gc.managed_bytes();
        std::thread::scope(|threads| -> crate::Result<()> {
            let cleanup = threads.spawn(move || drop(handle));
            let resume = Resume(resume);
            observed
                .recv_timeout(std::time::Duration::from_secs(120))
                .expect("stalled final-handle cleanup");
            assert!(matches!(header.acquire(&gc), Err(AcquireError::Closed)));
            assert!(!completed.load(Ordering::Relaxed));
            let unrelated = Arc::new(AtomicUsize::new(0));
            drop(Gc::new(&gc, Leaf(unrelated.clone())));
            reclaim(&gc)?;
            assert_eq!(unrelated.load(Ordering::Relaxed), 1);
            assert_eq!(child.load(Ordering::Relaxed), 0);
            assert_eq!(
                gc.managed_bytes(),
                charged,
                "closed header refunded a stalled private payload"
            );
            assert_eq!(gc.diagnostics().active_publications, 0);
            drop(resume);
            cleanup.join().expect("private cleanup completed");
            Ok(())
        })?;
        assert!(completed.load(Ordering::Relaxed));
        reclaim(&gc)?;
        assert_eq!(child.load(Ordering::Relaxed), 1);
        assert_eq!(gc.managed_bytes(), 0);
        drop(storage);
        assert!(matches!(header.acquire(&gc), Err(AcquireError::Closed)));
    }
    Ok(())
}
