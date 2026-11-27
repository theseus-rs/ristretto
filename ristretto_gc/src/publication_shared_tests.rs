//! Shared headers retain no private payload after the last admitted handle.
use super::*;
use crate::gc::publication_owner::shared::AcquireError;

#[test]
fn erased_headers_share_backing_and_preserve_heterogeneous_private_graphs() -> crate::Result<()> {
    for counted in [false, true] {
        for storage in [RootStorage::Tree, RootStorage::Slots, RootStorage::Contexts] {
            let gc = collector(counted, storage);
            let context = gc.register_context();
            let child = Arc::new(AtomicUsize::new(0));
            let parent = Arc::new(AtomicUsize::new(0));
            let leaf = Arc::new(AtomicUsize::new(0));
            let publication = context.with(|| prepare(&gc, graph(&gc, &child, &parent), true));
            let parent_id = publication.metadata().id;
            let (parent_header, parent_origin) = publication.share();
            let publication = context.with(|| prepare(&gc, Leaf(leaf.clone()), false));
            let leaf_id = publication.metadata().id;
            let (leaf_header, leaf_origin) = publication.share();
            let headers = [parent_header.erase(), leaf_header.erase()];
            assert!(parent_header.shares_backing_with(&headers[0]));
            assert!(leaf_header.shares_backing_with(&headers[1]));
            let handles = headers.each_ref().map(|header| {
                header
                    .acquire(&gc)
                    .unwrap_or_else(|_| panic!("open erased header"))
            });
            let parent_origin = parent_origin
                .restore()
                .err()
                .expect("erased helper prevents restore");
            drop(parent_origin);
            drop(leaf_origin);
            drop(parent_header);
            drop(leaf_header);
            drop(context);
            reclaim(&gc)?;
            assert_eq!(handles[0].view().metadata().id, parent_id);
            assert!(handles[0].view().metadata().is_type::<Parent>());
            assert_eq!(handles[1].view().metadata().id, leaf_id);
            assert!(handles[1].view().metadata().is_type::<Leaf>());
            assert_eq!(parent.load(Ordering::Relaxed), 0);
            assert_eq!(child.load(Ordering::Relaxed), 0);
            assert_eq!(leaf.load(Ordering::Relaxed), 0);
            assert_eq!(Arc::strong_count(&gc), 1);
            drop(handles);
            assert_eq!(
                parent.load(Ordering::Relaxed),
                1,
                "erased final handle retained private payload"
            );
            assert_eq!(leaf.load(Ordering::Relaxed), 1);
            reclaim(&gc)?;
            assert_eq!(child.load(Ordering::Relaxed), 1);
            assert_eq!(gc.managed_bytes(), 0);
            for header in &headers {
                assert!(matches!(header.acquire(&gc), Err(AcquireError::Closed)));
            }
            let weak = Arc::downgrade(&gc);
            drop(gc);
            assert!(weak.upgrade().is_none(), "erased header retained the heap");
            drop(headers);
        }
    }
    Ok(())
}

#[test]
fn erased_admission_validates_the_heap_and_avoids_the_result_slot() -> crate::Result<()> {
    for counted in [false, true] {
        let gc = collector(counted, RootStorage::Contexts);
        let foreign = collector(counted, RootStorage::Contexts);
        let drops = Arc::new(AtomicUsize::new(0));
        let (header, origin) = prepare(&gc, Leaf(drops.clone()), true).share();
        let erased = header.erase();
        assert!(matches!(
            erased.acquire(&foreign),
            Err(AcquireError::Foreign)
        ));
        origin.with_result_slot_locked(|| {
            let helper = erased
                .acquire(&gc)
                .unwrap_or_else(|_| panic!("open erased header"));
            assert!(helper.view().metadata().is_type::<Leaf>());
        });
        drop(origin);
        assert_eq!(drops.load(Ordering::Relaxed), 1);
        assert!(matches!(erased.acquire(&gc), Err(AcquireError::Closed)));
        reclaim(&gc)?;
        assert_eq!(gc.managed_bytes(), 0);
    }
    Ok(())
}

#[cfg(not(target_family = "wasm"))]
#[test]
fn an_erased_helper_can_inspect_and_close_on_another_thread() -> crate::Result<()> {
    for counted in [false, true] {
        let gc = collector(counted, RootStorage::Contexts);
        let child = Arc::new(AtomicUsize::new(0));
        let parent = Arc::new(AtomicUsize::new(0));
        let (header, origin) = prepare(&gc, graph(&gc, &child, &parent), true).share();
        let erased = header.erase();
        let helper = erased
            .acquire(&gc)
            .unwrap_or_else(|_| panic!("open erased header"));
        drop(origin);
        std::thread::scope(|threads| {
            let worker = threads.spawn(move || {
                assert!(helper.view().metadata().is_type::<Parent>());
                drop(helper);
            });
            worker.join().expect("erased helper joined");
        });
        assert_eq!(parent.load(Ordering::Relaxed), 1);
        reclaim(&gc)?;
        assert_eq!(child.load(Ordering::Relaxed), 1);
        assert_eq!(gc.managed_bytes(), 0);
    }
    Ok(())
}

#[test]
fn metadata_views_borrow_the_private_graph_without_owning_its_storage() -> crate::Result<()> {
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
                let address = publication.metadata().ptr();
                let bytes = publication.metadata().size();
                let (header, origin) = publication.share();
                drop(context);
                let helper = header
                    .acquire(&gc)
                    .unwrap_or_else(|_| panic!("open header"));
                drop(origin);
                {
                    let view = helper.view();
                    reclaim(&gc)?;
                    let metadata = view.metadata();
                    assert_eq!(metadata.id, identity);
                    assert_eq!(metadata.ptr(), address);
                    assert_eq!(metadata.size(), bytes);
                    assert!(metadata.is_type::<Parent>());
                    assert_eq!(parent.load(Ordering::Relaxed), 0);
                    assert_eq!(child.load(Ordering::Relaxed), 0);
                    // SAFETY: the view borrows an admitted handle; the exact private
                    // payload and its transitive construction protection remain live.
                    unsafe { metadata.trace.trace(&gc) };
                }
                drop(helper);
                assert_eq!(
                    parent.load(Ordering::Relaxed),
                    1,
                    "view retained private storage"
                );
                reclaim(&gc)?;
                assert_eq!(child.load(Ordering::Relaxed), 1);
                assert_eq!(gc.managed_bytes(), 0);
                assert!(matches!(header.acquire(&gc), Err(AcquireError::Closed)));
            }
        }
    }
    Ok(())
}

#[test]
fn metadata_views_do_not_acquire_the_private_result_slot() -> crate::Result<()> {
    for counted in [false, true] {
        let gc = collector(counted, RootStorage::Contexts);
        let child = Arc::new(AtomicUsize::new(0));
        let parent = Arc::new(AtomicUsize::new(0));
        let publication = prepare(&gc, graph(&gc, &child, &parent), true);
        let identity = publication.metadata().id;
        let (header, origin) = publication.share();
        origin.with_result_slot_locked(|| {
            let helper = header
                .acquire(&gc)
                .unwrap_or_else(|_| panic!("open header"));
            let view = helper.view();
            assert_eq!(view.metadata().id, identity);
        });
        drop(origin);
        reclaim(&gc)?;
        assert_eq!(parent.load(Ordering::Relaxed), 1);
        assert_eq!(child.load(Ordering::Relaxed), 1);
    }
    Ok(())
}

#[cfg(not(target_family = "wasm"))]
#[test]
fn a_stalled_result_owner_does_not_block_descriptor_preparation() -> crate::Result<()> {
    struct Resume(std::sync::mpsc::Sender<()>);
    impl Drop for Resume {
        fn drop(&mut self) {
            let _ = self.0.send(());
        }
    }
    for counted in [false, true] {
        let gc = collector(counted, RootStorage::Contexts);
        let child = Arc::new(AtomicUsize::new(0));
        let parent = Arc::new(AtomicUsize::new(0));
        let publication = prepare(&gc, graph(&gc, &child, &parent), true);
        let identity = publication.metadata().id;
        let (header, origin) = publication.share();
        std::thread::scope(|threads| -> crate::Result<()> {
            let (entered, entry) = std::sync::mpsc::channel();
            let (resume, resumed) = std::sync::mpsc::channel();
            let stopped = threads.spawn(move || {
                origin.with_result_slot_locked(|| {
                    entered.send(()).expect("result slot held");
                    resumed
                        .recv_timeout(std::time::Duration::from_secs(120))
                        .expect("resume result owner");
                });
            });
            let resume = Resume(resume);
            entry
                .recv_timeout(std::time::Duration::from_secs(120))
                .expect("stopped result owner");
            let helper = header
                .acquire(&gc)
                .unwrap_or_else(|_| panic!("open header"));
            let view = helper.view();
            assert_eq!(view.metadata().id, identity);
            let garbage = Arc::new(AtomicUsize::new(0));
            drop(Gc::new(&gc, Leaf(garbage.clone())));
            reclaim(&gc)?;
            assert_eq!(garbage.load(Ordering::Relaxed), 1);
            assert_eq!(child.load(Ordering::Relaxed), 0);
            assert_eq!(parent.load(Ordering::Relaxed), 0);
            assert_eq!(gc.diagnostics().active_publications, 0);
            drop(resume);
            stopped.join().expect("result owner joined");
            assert_eq!(view.metadata().id, identity);
            Ok(())
        })?;
        reclaim(&gc)?;
        assert_eq!(child.load(Ordering::Relaxed), 1);
        assert_eq!(parent.load(Ordering::Relaxed), 1);
        assert_eq!(gc.managed_bytes(), 0);
    }
    Ok(())
}

#[test]
fn shared_conditional_reservations_survive_until_binding_or_cancellation() -> crate::Result<()> {
    for counted in [false, true] {
        for cancel in [false, true] {
            let gc = collector(counted, RootStorage::Contexts);
            let drops = Arc::new(AtomicUsize::new(0));
            let leaf = Gc::new(&gc, Leaf(drops.clone()));
            let weak = GcWeak::new(&leaf, ReferenceKind::Weak, 0);
            let slot = weak.slot.clone();
            let publication = prepare(&gc, Conditional(weak), true);
            let identity = publication.metadata().id;
            let (header, origin) = publication.share();
            let helper = header
                .acquire(&gc)
                .unwrap_or_else(|_| panic!("open header"));
            drop(origin);
            drop(leaf);
            reclaim(&gc)?;
            assert_eq!(drops.load(Ordering::Relaxed), 0);
            assert_eq!(slot.unpublished_owners.load(Ordering::Acquire), 1);
            let owner = if cancel {
                drop(helper);
                None
            } else {
                let publication = helper.restore().unwrap_or_else(|_| panic!("unique helper"));
                let owner = publication.finish_rooted(&gc);
                assert_eq!(slot.owner_identity(), Some(identity));
                Some(owner)
            };
            assert!(matches!(header.acquire(&gc), Err(AcquireError::Closed)));
            assert_eq!(slot.unpublished_owners.load(Ordering::Acquire), 0);
            reclaim(&gc)?;
            assert_eq!(drops.load(Ordering::Relaxed), 1);
            let queued = gc.take_pending_reference::<Conditional>()?;
            if cancel {
                assert!(queued.is_none());
            } else {
                assert_eq!(queued.as_ref().map(Gc::identity), Some(identity));
            }
            drop(queued);
            drop(owner);
            reclaim(&gc)?;
            assert_eq!(gc.managed_bytes(), 0);
        }
    }
    Ok(())
}

#[cfg(not(target_family = "wasm"))]
#[test]
fn unwinding_a_final_handle_leaves_only_an_inert_header() -> crate::Result<()> {
    let gc = collector(true, RootStorage::Contexts);
    let child = Arc::new(AtomicUsize::new(0));
    let parent = Arc::new(AtomicUsize::new(0));
    let (header, origin) = prepare(&gc, graph(&gc, &child, &parent), true).share();
    let failed = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let _handle = origin;
        panic!("injected private handle unwind");
    }));
    assert!(failed.is_err());
    assert!(matches!(header.acquire(&gc), Err(AcquireError::Closed)));
    assert_eq!(parent.load(Ordering::Relaxed), 1);
    reclaim(&gc)?;
    assert_eq!(child.load(Ordering::Relaxed), 1);
    assert_eq!(gc.managed_bytes(), 0);
    Ok(())
}

#[test]
fn retained_headers_are_inert_before_collector_shutdown() -> crate::Result<()> {
    for counted in [false, true] {
        for storage in [RootStorage::Tree, RootStorage::Slots, RootStorage::Contexts] {
            for rooted in [false, true] {
                let gc = collector(counted, storage);
                let context = gc.register_context();
                let child = Arc::new(AtomicUsize::new(0));
                let parent = Arc::new(AtomicUsize::new(0));
                let publication =
                    context.with(|| prepare(&gc, graph(&gc, &child, &parent), rooted));
                let (header, origin) = publication.share();
                drop(context);
                let helper = header
                    .acquire(&gc)
                    .unwrap_or_else(|_| panic!("open header"));
                let origin = origin
                    .restore()
                    .err()
                    .expect("helper prevents exclusive restore");
                drop(origin);
                reclaim(&gc)?;
                assert_eq!(
                    parent.load(Ordering::Relaxed),
                    0,
                    "live helper lost private payload"
                );
                assert_eq!(
                    child.load(Ordering::Relaxed),
                    0,
                    "live helper lost private graph"
                );
                assert_eq!(
                    Arc::strong_count(&gc),
                    1,
                    "header retained the collector strongly"
                );
                assert_eq!(gc.diagnostics().active_publications, 0);
                let mut unrelated = crate::construction::Roots::new(&gc);
                let scope = unrelated.enter();
                drop(helper);
                assert_eq!(
                    parent.load(Ordering::Relaxed),
                    1,
                    "retained header preserved private payload"
                );
                assert!(
                    matches!(header.acquire(&gc), Err(AcquireError::Closed)),
                    "closed header admitted a helper"
                );
                reclaim(&gc)?;
                assert_eq!(
                    child.load(Ordering::Relaxed),
                    1,
                    "shared cleanup adopted helper TLS"
                );
                assert_eq!(gc.managed_bytes(), 0);
                drop(scope);
                let weak = Arc::downgrade(&gc);
                drop(gc);
                assert!(
                    weak.upgrade().is_none(),
                    "inert header kept the collector alive"
                );
                drop(header);
                assert_eq!(parent.load(Ordering::Relaxed), 1);
                assert_eq!(child.load(Ordering::Relaxed), 1);
            }
        }
    }
    Ok(())
}

#[test]
fn a_unique_helper_restores_and_publishes_exact_ownership() -> crate::Result<()> {
    for counted in [false, true] {
        for storage in [RootStorage::Tree, RootStorage::Slots, RootStorage::Contexts] {
            let gc = collector(counted, storage);
            let child = Arc::new(AtomicUsize::new(0));
            let parent = Arc::new(AtomicUsize::new(0));
            let (header, origin) = prepare(&gc, graph(&gc, &child, &parent), true).share();
            let helper = header
                .acquire(&gc)
                .unwrap_or_else(|_| panic!("open header"));
            drop(origin);
            let publication = helper.restore().unwrap_or_else(|_| panic!("unique helper"));
            assert!(matches!(header.acquire(&gc), Err(AcquireError::Closed)));
            reclaim(&gc)?;
            let rooted = publication.finish_rooted(&gc);
            reclaim(&gc)?;
            assert_eq!(child.load(Ordering::Relaxed), 0);
            assert_eq!(rooted.child.0.load(Ordering::Relaxed), 0);
            drop(rooted);
            reclaim(&gc)?;
            assert_eq!(parent.load(Ordering::Relaxed), 1);
            assert_eq!(child.load(Ordering::Relaxed), 1);
            assert_eq!(gc.managed_bytes(), 0);
            drop(header);
        }
    }
    Ok(())
}

#[cfg(not(target_family = "wasm"))]
#[test]
fn helper_handles_can_close_on_another_thread() -> crate::Result<()> {
    for counted in [false, true] {
        let gc = collector(counted, RootStorage::Contexts);
        let child = Arc::new(AtomicUsize::new(0));
        let parent = Arc::new(AtomicUsize::new(0));
        let (header, origin) = prepare(&gc, graph(&gc, &child, &parent), true).share();
        std::thread::scope(|threads| {
            let gc = &gc;
            let header = &header;
            let child = &child;
            let parent = &parent;
            let (ready, received) = std::sync::mpsc::channel();
            let (resume, resumed) = std::sync::mpsc::channel();
            let helper = threads.spawn(move || -> crate::Result<()> {
                let handle = header.acquire(gc).unwrap_or_else(|_| panic!("open header"));
                ready.send(()).expect("helper entered");
                resumed
                    .recv_timeout(std::time::Duration::from_secs(120))
                    .expect("resume helper");
                assert_eq!(parent.load(Ordering::Relaxed), 0);
                assert_eq!(child.load(Ordering::Relaxed), 0);
                reclaim(gc)?;
                drop(handle);
                Ok(())
            });
            received
                .recv_timeout(std::time::Duration::from_secs(120))
                .expect("live helper");
            drop(origin);
            // A parked private helper owns no global admission. Unrelated garbage
            // can be reclaimed while the exact prepared graph stays protected.
            let garbage_drops = Arc::new(AtomicUsize::new(0));
            drop(Gc::new(gc, Leaf(garbage_drops.clone())));
            reclaim(gc)?;
            assert_eq!(garbage_drops.load(Ordering::Relaxed), 1);
            assert_eq!(parent.load(Ordering::Relaxed), 0);
            resume.send(()).expect("resume");
            helper.join().expect("helper joined")
        })?;
        reclaim(&gc)?;
        assert_eq!(parent.load(Ordering::Relaxed), 1);
        assert_eq!(child.load(Ordering::Relaxed), 1);
        assert!(matches!(header.acquire(&gc), Err(AcquireError::Closed)));
    }
    Ok(())
}

#[test]
fn a_foreign_heap_cannot_acquire_private_resources() -> crate::Result<()> {
    let gc = collector(true, RootStorage::Contexts);
    let foreign = collector(true, RootStorage::Contexts);
    let child = Arc::new(AtomicUsize::new(0));
    let parent = Arc::new(AtomicUsize::new(0));
    let (header, handle) = prepare(&gc, graph(&gc, &child, &parent), true).share();
    assert!(matches!(
        header.acquire(&foreign),
        Err(AcquireError::Foreign)
    ));
    drop(handle);
    reclaim(&gc)?;
    assert_eq!(parent.load(Ordering::Relaxed), 1);
    assert_eq!(child.load(Ordering::Relaxed), 1);
    Ok(())
}

#[test]
fn final_handle_cleanup_preserves_private_destructor_graph_access() -> crate::Result<()> {
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
            let helper = header
                .acquire(&gc)
                .unwrap_or_else(|_| panic!("open header"));
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
        }
    }
    Ok(())
}

#[cfg(not(target_family = "wasm"))]
#[test]
fn a_closed_header_does_not_refund_stalled_private_destruction() -> crate::Result<()> {
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
        assert!(matches!(header.acquire(&gc), Err(AcquireError::Closed)));
    }
    Ok(())
}

#[path = "publication_storage_tests.rs"]
mod storage;

#[path = "publication_record_tests.rs"]
mod records;
