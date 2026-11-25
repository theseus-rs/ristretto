//! Forced reference-boundary and mark-epoch rollover schedules.
#![expect(clippy::panic_in_result_fn)]
use super::*;
use crate::{GcWeak, ReferenceKind, RootStorage};

#[cfg(not(target_family = "wasm"))]
thread_local! {
    static PENDING_NOTICE_PAUSE: std::cell::Cell<Option<ClaimPause>> = const { std::cell::Cell::new(None) };
}

#[cfg(not(target_family = "wasm"))]
pub(super) fn pause_after_pending_notice() {
    if let Some(pause) = PENDING_NOTICE_PAUSE.take() {
        pause.entered.send(()).expect("observed pending notice");
        pause
            .resume
            .recv_timeout(std::time::Duration::from_secs(15))
            .expect("resume notifier");
    }
}

#[cfg(not(target_family = "wasm"))]
#[test]
fn published_reference_can_notify_while_the_first_notifier_is_parked() -> Result<()> {
    use std::future::Future;
    use std::sync::mpsc;
    use std::task::{Context, Wake, Waker};
    use std::time::Duration;

    struct Signal(AtomicBool);
    impl Wake for Signal {
        fn wake(self: Arc<Self>) {
            self.0.store(true, Ordering::Release);
        }
    }
    let gc = GarbageCollector::new();
    let target = Gc::try_new(&gc, 42_u64)?;
    let owner = Gc::try_new(&gc, 7_u64)?;
    let weak = GcWeak::new(&target, ReferenceKind::Weak, owner.as_ptr() as usize);
    drop(target);
    let mut wait = std::pin::pin!(gc.wait_for_pending_references());
    let signal = Arc::new(Signal(AtomicBool::new(false)));
    let waker = Waker::from(signal.clone());
    assert!(
        wait.as_mut()
            .poll(&mut Context::from_waker(&waker))
            .is_pending()
    );
    let (entered, parked) = mpsc::channel();
    let (resume, release) = mpsc::channel();
    thread::scope(|scope| -> Result<()> {
        let collector = gc.clone();
        let first = scope.spawn(move || {
            PENDING_NOTICE_PAUSE.set(Some(ClaimPause {
                entered,
                resume: release,
            }));
            collector.collect_full()
        });
        parked
            .recv_timeout(Duration::from_secs(10))
            .expect("published reference");
        assert!(gc.has_pending_references());
        gc.notify_progress();
        let delivered = signal.0.load(Ordering::Acquire);
        resume.send(()).expect("resume first notifier");
        first.join().expect("first notifier joined")?;
        assert!(
            delivered,
            "published notification was stranded with the first notifier"
        );
        Ok(())
    })?;
    assert!(
        wait.as_mut()
            .poll(&mut Context::from_waker(&waker))
            .is_ready()
    );
    assert_eq!(
        *gc.take_pending_reference::<u64>()?.expect("pending owner"),
        7
    );
    assert!(weak.upgrade().is_none());
    Ok(())
}

#[test]
fn a_revoked_pending_owner_cannot_deliver_or_unroot_a_replacement() -> Result<()> {
    for concurrent in [false, true] {
        for storage in [RootStorage::Tree, RootStorage::Slots, RootStorage::Contexts] {
            let gc = GarbageCollector::with_config(
                Configuration::builder()
                    .concurrent_publication(concurrent)
                    .root_storage(storage)
                    .allocation_threshold(usize::MAX)
                    .build(),
            );
            let context = gc.register_context();
            context.with(|| -> Result<()> {
                let target = Gc::try_new(&gc, 42_u64)?;
                let owner = Gc::try_new(&gc, 7_u64)?;
                let weak = GcWeak::new(&target, ReferenceKind::Weak, owner.as_ptr() as usize);
                drop(target);
                gc.collect_full()?;
                let (identity, queued) =
                    *lock(&gc.pending_references).front().expect("queued owner");
                // SAFETY: only drop follows; the stale queued owner must validate
                // its identity before any subsequent payload access.
                unsafe { gc.remove_root(&**owner) };
                drop(owner);
                gc.collect_full()?;
                assert_eq!(gc.statistics()?.live_objects, 0);

                let replacement = Gc::try_new(&gc, 99_u64)?;
                gc.next_root_id.store(queued.id, Ordering::Relaxed);
                let replacement_root = gc.add_root(&replacement);
                assert_eq!(replacement_root, queued.id);
                let root_count = gc.root_statistics().compatibility_roots;
                // Force a stale address hint without depending on host allocator
                // reuse. Transfer must still validate the queued allocation ID.
                gc.set_identity_hint(
                    identity.to_i64().cast_unsigned(),
                    Some(replacement.as_ptr() as usize),
                );
                let delivered = gc.take_pending_reference::<u64>();
                gc.set_identity_hint(identity.to_i64().cast_unsigned(), None);
                assert!(
                    delivered.is_err(),
                    "queued identity followed a replacement address"
                );
                assert!(!gc.has_pending_references());
                assert_eq!(
                    gc.root_statistics().compatibility_roots,
                    root_count,
                    "stale queue cleanup removed a replacement root"
                );
                assert_eq!(gc.root_statistics().reserved_slots, 0);
                gc.collect_full()?;
                assert_eq!(**replacement, 99);
                gc.remove_root_by_id(replacement_root);
                drop((replacement, weak));
                Ok(())
            })?;
            drop(context);
            gc.stop()?;
            assert_eq!(gc.managed_bytes(), 0);
            assert_eq!(gc.root_statistics().compatibility_roots, 0);
            assert_eq!(gc.root_statistics().resident_pages, 0);
        }
    }
    Ok(())
}

#[cfg(all(not(target_family = "wasm"), not(miri)))]
#[test]
#[expect(
    clippy::unwrap_in_result,
    reason = "forced-schedule synchronization assertions"
)]
fn a_paused_pending_transfer_keeps_its_graph_without_holding_collection_open() -> Result<()> {
    use std::sync::mpsc;
    use std::time::Duration;
    for concurrent in [false, true] {
        let gc = GarbageCollector::with_config(
            Configuration::builder()
                .concurrent_publication(concurrent)
                .root_storage(RootStorage::Contexts)
                .barrier_buffer_size(4)
                .allocation_threshold(usize::MAX)
                .build(),
        );
        let context = gc.register_context();
        let (owner, weak) = context.with(|| -> Result<_> {
            let target = Gc::try_new(&gc, 42_u64)?;
            let child = Gc::try_new(&gc, 7_u64)?.into_rooted();
            let owner = Gc::try_new(&gc, child)?;
            let weak = GcWeak::new(&target, ReferenceKind::Weak, owner.as_ptr() as usize);
            drop(target);
            Ok((owner, weak))
        })?;
        gc.collect_full()?;
        assert!(gc.has_pending_references());
        assert!(weak.upgrade().is_none());
        drop((owner, context));
        let (entered_tx, entered_rx) = mpsc::channel();
        let (resume_tx, resume_rx) = mpsc::channel();
        *lock(&gc.pending_transfer_pause) = Some(ClaimPause {
            entered: entered_tx,
            resume: resume_rx,
        });
        let handler_gc = gc.clone();
        let handler = thread::spawn(move || {
            let context = handler_gc.register_context();
            context.with(|| handler_gc.take_pending_reference::<Gc<u64>>())
        });
        entered_rx
            .recv_timeout(Duration::from_secs(5))
            .expect("dequeued reference");
        let (completed_tx, completed_rx) = mpsc::channel();
        let cycle_gc = gc.clone();
        let cycler = thread::spawn(move || -> Result<()> {
            for _ in 0..3 {
                drop(Gc::try_new(&cycle_gc, 99_u64)?);
                cycle_gc.collect_full()?;
                assert_eq!(cycle_gc.statistics()?.live_objects, 2);
            }
            completed_tx.send(()).expect("completed full cycles");
            Ok(())
        });
        let progressed = completed_rx.recv_timeout(Duration::from_secs(5)).is_ok();
        // Resume and join before asserting progress, so a regression releases
        // its publication guard instead of stranding the collection thread.
        resume_tx.send(()).expect("resume handler");
        let delivered = handler
            .join()
            .unwrap_or_else(|error| std::panic::resume_unwind(error))?
            .expect("pending reference");
        cycler
            .join()
            .unwrap_or_else(|error| std::panic::resume_unwind(error))?;
        assert!(progressed, "pending transfer held collection open");
        assert_eq!(**delivered, 7);
        assert!(!gc.has_pending_references());
        drop(delivered);
        gc.collect_full()?;
        gc.stop()?;
        assert_eq!(gc.statistics()?.live_objects, 0);
        assert_eq!(gc.managed_bytes(), 0);
        assert_eq!(gc.root_statistics().pages, 0);
        assert_eq!(gc.root_statistics().compatibility_roots, 0);
    }
    Ok(())
}

#[test]
fn rejected_pending_transfer_releases_its_queue_root() -> Result<()> {
    for concurrent in [false, true] {
        for storage in [RootStorage::Tree, RootStorage::Slots, RootStorage::Contexts] {
            let gc = GarbageCollector::with_config(
                Configuration::builder()
                    .concurrent_publication(concurrent)
                    .root_storage(storage)
                    .allocation_threshold(usize::MAX)
                    .build(),
            );
            let context = gc.register_context();
            context.with(|| -> Result<()> {
                let target = Gc::try_new(&gc, 42_u64)?;
                let owner = Gc::try_new(&gc, 7_u64)?;
                let weak = GcWeak::new(&target, ReferenceKind::Weak, owner.as_ptr() as usize);
                drop(target);
                gc.collect_full()?;
                assert!(weak.upgrade().is_none());
                drop(owner);
                assert!(gc.take_pending_reference::<bool>().is_err());
                assert!(!gc.has_pending_references());
                assert!(gc.take_pending_reference::<u64>()?.is_none());
                Ok(())
            })?;
            drop(context);
            gc.collect_full()?;
            gc.stop()?;
            assert_eq!(gc.statistics()?.live_objects, 0);
            assert_eq!(gc.managed_bytes(), 0);
            assert_eq!(gc.root_statistics().compatibility_roots, 0);
            assert_eq!(gc.root_statistics().pages, 0);
        }
    }
    Ok(())
}

fn at_reference_stage(gc: &GarbageCollector, index: usize, stage: usize) {
    for _ in 0..10_000 {
        if gc.domains[index].weak_phase.load(Ordering::Acquire) == stage {
            assert_eq!(lock(&gc.domains[index].cycle).phase, Phase::References);
            return;
        }
        gc.step();
    }
    panic!("reference boundary was never reached");
}

fn finish(gc: &GarbageCollector) {
    for _ in 0..10_000 {
        if gc.step() {
            return;
        }
    }
    panic!("cycle did not finish");
}

#[cfg(all(not(target_family = "wasm"), not(miri)))]
#[test]
fn reference_decision_finishes_while_target_shard_is_parked() -> Result<()> {
    use std::sync::mpsc;
    use std::time::Duration;
    for counted in [false, true] {
        for (keep, cleared) in [(false, false), (true, false), (true, true)] {
            let gc = GarbageCollector::with_config(
                Configuration::builder()
                    .concurrent_publication(counted)
                    .incremental_step_size(1)
                    .max_pause_time_us(0)
                    .allocation_threshold(usize::MAX)
                    .build(),
            );
            let target = Gc::try_new(&gc, 17_u64)?;
            let address = target.as_ptr().addr();
            let weak = GcWeak::new(&target, ReferenceKind::Weak, 0);
            let survivor = keep.then(|| target.rooted_clone());
            if cleared {
                weak.clear();
            }
            drop(target);
            gc.collect();
            at_reference_stage(&gc, 1, 1);
            let held = gc.objects.lock(if cleared { 0 } else { address });
            let (done, received) = mpsc::channel();
            let finished = thread::scope(|scope| {
                let heap = &gc;
                scope.spawn(move || {
                    heap.step();
                    done.send(()).unwrap();
                });
                let result = received.recv_timeout(Duration::from_secs(2));
                drop(held);
                result
            });
            assert!(
                finished.is_ok(),
                "object shard stalled the reference decision"
            );
            assert_eq!(
                weak.slot.target.load(Ordering::Acquire) != 0,
                keep && !cleared
            );
            finish(&gc);
            assert_eq!(
                weak.upgrade().as_deref().copied(),
                (keep && !cleared).then_some(17)
            );
            drop(survivor);
        }
    }
    Ok(())
}

#[cfg(all(not(target_family = "wasm"), not(miri)))]
#[test]
fn host_weak_registration_clone_and_upgrade_progress_with_parked_object_shard() -> Result<()> {
    use std::sync::mpsc;
    use std::time::Duration;
    for counted in [false, true] {
        for active in [false, true] {
            let gc = GarbageCollector::with_config(
                Configuration::builder()
                    .concurrent_publication(counted)
                    .incremental_step_size(1)
                    .max_pause_time_us(0)
                    .allocation_threshold(usize::MAX)
                    .build(),
            );
            let original = Gc::try_new(&gc, 913_u64)?;
            if active {
                gc.collect();
                assert!((0..128).any(|_| {
                    gc.step();
                    gc.domains[1].active() && gc.domains[1].weak_phase() == 0
                }));
            }
            let held = gc.objects.lock(original.as_ptr().addr());
            let (done, received) = mpsc::channel();
            let finished = thread::scope(|scope| {
                let target = &original;
                let work = scope.spawn(move || {
                    let weak = GcWeak::new(target, ReferenceKind::Weak, 0);
                    let copy = weak.clone();
                    let result = copy.upgrade().expect("strong target still live");
                    done.send(()).unwrap();
                    (weak, copy, result)
                });
                let finished = received.recv_timeout(Duration::from_secs(2));
                drop(held);
                (finished, work.join().unwrap())
            });
            let (finished, (weak, copy, result)) = finished;
            assert!(
                finished.is_ok(),
                "weak publication blocked on the object shard"
            );
            assert_eq!(result.identity(), original.identity());
            assert_eq!(*result, 913);
            drop((original, result));
            gc.collect_full()?;
            gc.collect_full()?;
            assert!(weak.upgrade().is_none() && copy.upgrade().is_none());
        }
    }
    Ok(())
}

#[test]
fn direct_trace_during_final_processing_is_immediately_visible() -> Result<()> {
    for counted in [false, true] {
        let gc = GarbageCollector::with_config(
            Configuration::builder()
                .concurrent_publication(counted)
                .incremental_step_size(1)
                .max_pause_time_us(0)
                .allocation_threshold(usize::MAX)
                .build(),
        );
        let target = Gc::try_new(&gc, 17_u64)?;
        let child = Gc::try_new(&gc, 42_u64)?;
        let owned = GcWeak::new(&target, ReferenceKind::Final, child.as_ptr().addr());
        let parent = Gc::try_new(&gc, vec![child.into_rooted()])?;
        let outer = GcWeak::new(&parent, ReferenceKind::Final, 0);
        drop((target, parent));
        gc.collect();
        at_reference_stage(&gc, 1, 2);
        let parent = outer.upgrade().expect("final-retained parent");
        let edge = parent.first().unwrap();
        let handle = edge.as_handle_i64();
        assert!(Gc::<u64>::from_handle_rooted(&gc, handle).is_err());
        edge.trace(&gc);
        let restored = Gc::<u64>::from_handle_rooted(&gc, handle)?;
        assert_eq!(*restored, 42);
        finish(&gc);
        assert!(owned.slot.activated.load(Ordering::Acquire));
        assert_eq!(*owned.upgrade().expect("final referent"), 17);
    }
    Ok(())
}

#[test]
fn young_final_retention_must_not_become_strong_in_an_old_snapshot() -> Result<()> {
    struct Hold(Arc<AtomicBool>);
    // SAFETY: this fixture contains no outgoing GC references.
    unsafe impl Trace for Hold {
        fn trace(&self, _: &GarbageCollector) {}
        fn trace_step(&self, gc: &GarbageCollector, _: usize, _: usize) -> TraceProgress {
            if !gc.is_young_collection() && self.0.load(Ordering::Acquire) {
                TraceProgress::Busy
            } else {
                TraceProgress::Complete
            }
        }
    }
    for (concurrent, resurrect) in [(false, false), (true, false), (false, true), (true, true)] {
        let gc = GarbageCollector::with_config(
            Configuration::builder()
                .concurrent_publication(concurrent)
                .promotion_age(1)
                .full_collection_interval(usize::MAX)
                .allocation_threshold(usize::MAX)
                .incremental_step_size(1)
                .max_pause_time_us(0)
                .build(),
        );
        let held = Arc::new(AtomicBool::new(false));
        let blocker = Gc::try_new(&gc, Hold(held.clone()))?;
        let child = Gc::try_new(&gc, 42_u64)?;
        gc.collect_full()?;
        assert!(!gc.is_young_allocation(child.as_ptr() as usize));
        let weak = GcWeak::new(&child, ReferenceKind::Weak, 0);
        let parent = Gc::try_new(&gc, vec![child.clone_gc()])?;
        let retained = GcWeak::new(&parent, ReferenceKind::Final, 0);
        drop((child, parent));
        held.store(true, Ordering::Release);
        gc.collect();
        for _ in 0..1000 {
            gc.step();
            if gc.domains[1].trace_ready.load(Ordering::Acquire) {
                break;
            }
        }
        assert!(gc.domains[1].trace_ready.load(Ordering::Acquire));
        let completed = gc.statistics()?.young_collections;
        gc.collect_young();
        for _ in 0..10000 {
            gc.step();
            if gc.statistics()?.young_collections > completed {
                break;
            }
        }
        assert!(gc.statistics()?.young_collections > completed);
        assert!(gc.domains[1].active());
        held.store(false, Ordering::Release);
        let root = if resurrect {
            let child = gc
                .metadata_for_address(weak.slot.target.load(Ordering::Acquire))
                .expect("retained metadata");
            let epoch = gc.domains[1].epoch.load(Ordering::Acquire);
            for _ in 0..1000 {
                gc.step();
                if child.retention(1, epoch) == Some(Retention::Final) {
                    break;
                }
            }
            assert_eq!(child.retention(1, epoch), Some(Retention::Final));
            assert_eq!(gc.domains[1].weak_phase(), 0);
            // The parent has already traced at final strength. Root acquisition
            // must retrace its earlier children at strong strength before closure.
            Some(retained.upgrade().expect("resurrection before closure"))
        } else {
            None
        };
        finish(&gc);
        assert_eq!(
            weak.upgrade().is_some(),
            resurrect,
            "wrong cross-generation strength"
        );
        let parent = root
            .or_else(|| retained.upgrade())
            .expect("retained parent");
        assert_eq!(**parent.first().expect("retained child"), 42);
        retained.clear();
        drop((parent, blocker));
        gc.stop()?;
        assert_eq!(gc.statistics()?.live_objects, 0);
    }
    Ok(())
}

#[test]
fn retention_cannot_reverse_a_logical_reference_decision() -> Result<()> {
    for concurrent in [false, true] {
        for storage in [RootStorage::Tree, RootStorage::Slots, RootStorage::Contexts] {
            for index in 0..2 {
                for (kind, retaining, boundary) in [
                    (ReferenceKind::Soft, ReferenceKind::Final, 1),
                    (ReferenceKind::Weak, ReferenceKind::Final, 1),
                    (ReferenceKind::Phantom, ReferenceKind::PhantomRetained, 3),
                ] {
                    let gc = GarbageCollector::with_config(
                        Configuration::builder()
                            .concurrent_publication(concurrent)
                            .root_storage(storage)
                            .metadata_shards(16)
                            .barrier_buffer_size(4)
                            .incremental_step_size(1)
                            .max_pause_time_us(0)
                            .allocation_threshold(usize::MAX)
                            .full_collection_interval(usize::MAX)
                            .build(),
                    );
                    gc.register_context().with(|| -> Result<()> {
                        let target = Gc::try_new(&gc, 42_u64)?;
                        let owner = Gc::try_new(&gc, 7_u64)?;
                        let weak = GcWeak::new(&target, kind, owner.as_ptr() as usize);
                        let retained = GcWeak::new(&target, retaining, 0);
                        drop(target);
                        if index == 0 {
                            gc.collect_young();
                        } else {
                            gc.collect();
                        }
                        at_reference_stage(&gc, index, boundary);
                        assert_ne!(weak.slot.target.load(Ordering::Acquire), 0);
                        assert!(weak.upgrade().is_none());
                        let resurrected = retained.upgrade().expect("retaining edge");
                        assert!(weak.upgrade().is_none(), "logical clearing reversed");
                        assert!(weak.clone().upgrade().is_none());
                        // A new reference to a resurrected object did not exist at
                        // the earlier boundary and must remain usable in this cycle.
                        let after = GcWeak::new(&resurrected, kind, 0);
                        assert!(after.upgrade().is_some());
                        assert!(after.clone().upgrade().is_some());
                        // Publication after the boundary still creates a black object.
                        let fresh = Gc::try_new(&gc, 11_u64)?;
                        let fresh_weak = GcWeak::new(&fresh, kind, 0);
                        assert!(fresh_weak.upgrade().is_some());
                        finish(&gc);
                        assert_eq!(weak.slot.target.load(Ordering::Acquire), 0);
                        assert!(after.upgrade().is_some());
                        assert_eq!(gc.take_pending_reference::<u64>()?.as_deref(), Some(&7));
                        assert!(!gc.has_pending_references());
                        gc.collect_full()?;
                        assert!(weak.upgrade().is_none());
                        assert!(fresh_weak.upgrade().is_some());
                        assert!(!gc.has_pending_references());
                        retained.clear();
                        drop((resurrected, owner, fresh));
                        Ok(())
                    })?;
                    gc.stop()?;
                    assert_eq!(gc.statistics()?.live_objects, 0);
                }
            }
        }
    }
    Ok(())
}

#[test]
fn final_graph_tracing_cannot_reverse_weak_descendant_clearing() -> Result<()> {
    for concurrent in [false, true] {
        let gc = GarbageCollector::with_config(
            Configuration::builder()
                .concurrent_publication(concurrent)
                .incremental_step_size(1)
                .max_pause_time_us(0)
                .allocation_threshold(usize::MAX)
                .build(),
        );
        let child = Gc::try_new(&gc, 42_u64)?;
        let weak = GcWeak::new(&child, ReferenceKind::Weak, 0);
        let parent = Gc::try_new(&gc, crate::sync::RwLock::new(vec![child.into_rooted()]))?;
        let retained = GcWeak::new(&parent, ReferenceKind::Final, 0);
        drop(parent);
        gc.collect();
        at_reference_stage(&gc, 1, 1);
        assert!(weak.upgrade().is_none());
        let resurrected = retained.upgrade().expect("final parent");
        finish(&gc);
        assert!(weak.upgrade().is_none());
        assert_eq!(**resurrected.read().first().unwrap(), 42);
        gc.collect_full()?;
        assert!(weak.upgrade().is_none());
        retained.clear();
        drop(resurrected);
        gc.stop()?;
        assert_eq!(gc.statistics()?.live_objects, 0);
    }
    Ok(())
}

#[test]
fn logically_retired_handles_cannot_resurrect_an_untraced_graph() -> Result<()> {
    for concurrent in [false, true] {
        let gc = GarbageCollector::with_config(
            Configuration::builder()
                .concurrent_publication(concurrent)
                .incremental_step_size(1)
                .max_pause_time_us(0)
                .allocation_threshold(usize::MAX)
                .build(),
        );
        let child = Gc::try_new(&gc, 42_u64)?;
        let parent = Gc::try_new(&gc, crate::sync::RwLock::new(vec![child.into_rooted()]))?;
        let owner = Gc::try_new(&gc, 7_u64)?;
        let retained = GcWeak::new(&parent, ReferenceKind::Final, owner.as_ptr() as usize);
        let address = parent.as_ptr_i64();
        let handle = parent.as_handle_i64();
        // Deliberately retain an identity-only edge to probe logical retirement.
        let mut stale = parent.clone_gc();
        // SAFETY: The owning handle protects demotion; the stale identity is never dereferenced.
        unsafe { stale.unroot() };
        drop((parent, owner));
        gc.collect();
        at_reference_stage(&gc, 1, 1);
        assert!(gc.clone_edge_root(&stale).is_none());
        assert!(Gc::<crate::sync::RwLock<Vec<Gc<u64>>>>::from_handle_rooted(&gc, handle).is_err());
        for _ in 0..10_000 {
            if lock(&gc.domains[1].cycle).phase == Phase::Sweep {
                break;
            }
            gc.step();
        }
        assert_eq!(lock(&gc.domains[1].cycle).phase, Phase::Sweep);
        assert!(
            gc.address_for_identity(crate::ObjectId::from_i64(handle))
                .is_some()
        );
        assert!(Gc::<crate::sync::RwLock<Vec<Gc<u64>>>>::from_raw_rooted(&gc, address).is_err());
        assert!(gc.clone_edge_root(&stale).is_none());
        assert!(retained.upgrade().is_none());
        finish(&gc);
        assert_eq!(gc.statistics()?.live_objects, 0);
    }
    Ok(())
}

#[test]
fn identity_resolution_revalidates_the_metadata_identity() -> Result<()> {
    for concurrent in [false, true] {
        let gc = GarbageCollector::with_config(
            Configuration::builder()
                .concurrent_publication(concurrent)
                .build(),
        );
        let first = Gc::try_new(&gc, 7_u64)?;
        let replacement = Gc::try_new(&gc, 42_u64)?;
        let id = first.identity();
        // Model an address being reused after the identity-index lookup but before
        // descriptor validation. The expected ID must travel through that lookup.
        gc.set_identity_hint(
            id.to_i64().cast_unsigned(),
            Some(replacement.as_ptr() as usize),
        );
        assert!(Gc::<u64>::from_handle_rooted(&gc, id.to_i64()).is_err());
        assert_eq!(
            Gc::<u64>::from_handle_rooted(&gc, replacement.as_handle_i64())?.as_ref(),
            &42
        );
        gc.set_identity_hint(id.to_i64().cast_unsigned(), Some(first.as_ptr() as usize));
        drop((first, replacement));
        gc.stop()?;
        assert_eq!(gc.statistics()?.live_objects, 0);
    }
    Ok(())
}

#[test]
fn mark_epoch_rollover_resets_incrementally_without_aliasing() -> Result<()> {
    for concurrent in [false, true] {
        for index in 0..2 {
            let gc = GarbageCollector::with_config(
                Configuration::builder()
                    .concurrent_publication(concurrent)
                    .metadata_shards(16)
                    .incremental_step_size(1)
                    .max_pause_time_us(0)
                    .allocation_threshold(usize::MAX)
                    .full_collection_interval(usize::MAX)
                    .build(),
            );
            let live = Gc::try_new(&gc, 42_u64)?;
            let dead = Gc::try_new(&gc, 7_u64)?;
            let weak = GcWeak::new(&dead, ReferenceKind::Weak, 0);
            weak.slot.born_after[index].store(crate::metadata::EPOCH_STRIDE, Ordering::Release);
            // A stale first-epoch mark must not survive the wrap to that same epoch.
            gc.objects
                .lock(dead.as_ptr() as usize)
                .get(&(dead.as_ptr() as usize))
                .unwrap()
                .mark_black(index, crate::metadata::EPOCH_STRIDE);
            drop(dead);
            gc.domains[index]
                .epoch
                .store(u64::MAX - 3, Ordering::Release);
            if index == 0 {
                gc.collect_young();
            } else {
                gc.collect();
            }
            gc.step();
            assert_eq!(lock(&gc.domains[index].cycle).phase, Phase::ResetMarks);
            assert!(!gc.domains[index].active.load(Ordering::Acquire));
            assert!(gc.domains[index].requested.load(Ordering::Acquire));
            let during_reset = Gc::try_new(&gc, 11_u64)?;
            finish(&gc);
            assert_eq!(
                gc.domains[index].epoch.load(Ordering::Acquire),
                crate::metadata::EPOCH_STRIDE
            );
            assert!(weak.upgrade().is_none());
            assert_eq!(**live, 42);
            assert_eq!(**during_reset, 11);
            drop((live, during_reset));
            gc.stop()?;
            assert_eq!(gc.statistics()?.live_objects, 0);
        }
    }
    Ok(())
}

#[cfg(not(target_family = "wasm"))]
#[test]
fn reference_waiters_can_be_woken_by_another_helper_during_a_stalled_wake() {
    use std::future::Future;
    use std::sync::mpsc;
    use std::task::{Context, Wake, Waker};
    use std::time::Duration;

    struct Calls {
        count: AtomicUsize,
        entered: mpsc::Sender<()>,
        release: Mutex<mpsc::Receiver<()>>,
    }
    struct Callback(Arc<Calls>);
    impl Wake for Callback {
        fn wake(self: Arc<Self>) {
            if self.0.count.fetch_add(1, Ordering::SeqCst) == 0 {
                assert!(self.0.entered.send(()).is_ok());
                assert!(
                    lock(&self.0.release)
                        .recv_timeout(Duration::from_secs(10))
                        .is_ok()
                );
            }
        }
    }
    let gc = GarbageCollector::new();
    let (entered, parked) = mpsc::channel();
    let (release, resume) = mpsc::channel();
    let calls = Arc::new(Calls {
        count: AtomicUsize::new(0),
        entered,
        release: Mutex::new(resume),
    });
    let mut futures = Vec::new();
    for _ in 0..2 {
        let waker = Waker::from(Arc::new(Callback(calls.clone())));
        let mut future = Box::pin(gc.wait_for_pending_references());
        assert!(
            future
                .as_mut()
                .poll(&mut Context::from_waker(&waker))
                .is_pending()
        );
        futures.push(future);
    }
    gc.pending_notify.store(true, Ordering::Release);
    thread::scope(|scope| {
        scope.spawn(|| gc.notify_progress());
        parked.recv_timeout(Duration::from_secs(10)).unwrap();
        gc.notify_progress();
        let delivered = calls.count.load(Ordering::SeqCst);
        release.send(()).unwrap();
        assert_eq!(delivered, 2);
    });
}

#[test]
fn dropping_or_repolling_a_reference_wait_releases_its_executor_waker() {
    use std::future::Future;
    use std::task::{Context, Wake, Waker};
    struct Task(AtomicBool);
    impl Wake for Task {
        fn wake(self: Arc<Self>) {
            self.0.store(true, Ordering::Release);
        }
    }
    let gc = GarbageCollector::new();
    let mut future = Box::pin(gc.wait_for_pending_references());
    let mut poll = || {
        let task = Arc::new(Task(AtomicBool::new(false)));
        let weak = Arc::downgrade(&task);
        let waker = Waker::from(task);
        assert!(Future::poll(future.as_mut(), &mut Context::from_waker(&waker)).is_pending());
        weak
    };
    let previous = poll();
    let current = poll();
    assert!(previous.upgrade().is_none());
    assert!(!current.upgrade().unwrap().0.load(Ordering::Acquire));
    drop(future);
    assert!(current.upgrade().is_none());
}

#[cfg(all(not(target_family = "wasm"), not(miri)))]
thread_local! {
    static WEAK_ACTIVATION_PAUSE: std::cell::Cell<Option<ClaimPause>> = const { std::cell::Cell::new(None) };
}

#[cfg(all(not(target_family = "wasm"), not(miri)))]
pub(super) fn pause_before_host_weak_activation() {
    if let Some(pause) = WEAK_ACTIVATION_PAUSE.take() {
        pause.entered.send(()).expect("parked weak publisher");
        pause
            .resume
            .recv_timeout(std::time::Duration::from_secs(15))
            .expect("resume weak publisher");
    }
}

#[cfg(all(not(target_family = "wasm"), not(miri)))]
#[test]
fn suspended_host_weak_activation_allows_capped_allocation_and_full_tickets() -> Result<()> {
    use std::sync::mpsc;
    use std::time::Duration;
    let bytes = Gc::allocation_size(&913_u64)?;
    for counted in [false, true] {
        for kind in [ReferenceKind::Soft, ReferenceKind::Weak] {
            let gc = GarbageCollector::with_config(
                Configuration::builder()
                    .concurrent_publication(counted)
                    .allocation_threshold(usize::MAX)
                    .max_heap_bytes(2 * bytes)
                    .incremental_step_size(1)
                    .max_pause_time_us(0)
                    .build(),
            );
            let target = Gc::try_new(&gc, 913_u64)?;
            drop(Gc::try_new(&gc, 77_u64)?);
            gc.collect();
            at_reference_stage(&gc, 1, 1);
            let (entered, parked) = mpsc::channel();
            let (resume, release) = mpsc::channel();
            let (done, completed) = mpsc::channel();
            let weak = thread::scope(|scope| -> Result<_> {
                let root = &target;
                let publisher = scope.spawn(move || {
                    WEAK_ACTIVATION_PAUSE.set(Some(ClaimPause {
                        entered,
                        resume: release,
                    }));
                    GcWeak::new(root, kind, 0)
                });
                parked
                    .recv_timeout(Duration::from_secs(3))
                    .expect("parked before activation");
                let heap = &gc;
                let peer = scope.spawn(move || -> Result<()> {
                    use std::future::Future;
                    use std::task::{Context as TaskContext, Poll, Waker};
                    let mut request = std::pin::pin!(Gc::try_new_async(heap, 88_u64));
                    let mut context = TaskContext::from_waker(Waker::noop());
                    let extra = (0..1024)
                        .find_map(|_| match request.as_mut().poll(&mut context) {
                            Poll::Ready(result) => Some(result),
                            Poll::Pending => None,
                        })
                        .expect("capped allocation exceeded its poll budget")?;
                    drop(extra);
                    for _ in 0..2 {
                        heap.collect_full()?;
                    }
                    assert_eq!(heap.managed_bytes(), bytes);
                    done.send(()).expect("peer completed");
                    Ok(())
                });
                let progress = completed.recv_timeout(Duration::from_secs(3));
                resume.send(()).expect("resume");
                peer.join().expect("peer")?;
                assert!(progress.is_ok(), "publisher held collection open");
                Ok(publisher.join().expect("publisher"))
            })?;
            assert_eq!(weak.slot.birth_stamp(1).load(Ordering::Acquire), 0);
            assert_eq!(
                weak.upgrade().expect("exact retained target").identity(),
                target.identity()
            );
            drop(target);
            gc.collect_full()?;
            assert!(weak.upgrade().is_none());
            assert_eq!(gc.managed_bytes(), 0);
        }
    }
    Ok(())
}

#[test]
fn final_clone_retains_a_referent_after_the_source_cursor_wraps() -> Result<()> {
    for counted in [false, true] {
        for (kind, stage) in [
            (ReferenceKind::Final, 2),
            (ReferenceKind::PhantomRetained, 3),
        ] {
            for id in [1, usize::MAX] {
                let gc = GarbageCollector::with_config(
                    Configuration::builder()
                        .concurrent_publication(counted)
                        .incremental_step_size(1)
                        .max_pause_time_us(0)
                        .allocation_threshold(usize::MAX)
                        .build(),
                );
                let owner = Gc::try_new(&gc, 0_u64)?;
                let target = Gc::try_new(&gc, 913_u64)?;
                gc.next_weak_id.store(id, Ordering::Relaxed);
                let source = GcWeak::new(&target, kind, owner.as_ptr().addr());
                drop((target, owner));
                gc.collect();
                at_reference_stage(&gc, 1, stage);
                for _ in 0..64 {
                    if lock(&gc.domains[1].cycle).cursor == id {
                        break;
                    }
                    gc.step();
                }
                assert_eq!(lock(&gc.domains[1].cycle).cursor, id);
                assert!(!source.slot.activated.load(Ordering::Acquire));
                let clone = source.clone();
                finish(&gc);
                assert_eq!(
                    *clone
                        .upgrade()
                        .expect("host final clone retains the target"),
                    913
                );
                drop((source, clone));
                gc.collect_full()?;
                assert_eq!(gc.managed_bytes(), 0);
            }
        }
    }
    Ok(())
}

#[test]
fn final_clone_during_post_reference_mark_is_drained_before_phantom_discovery() -> Result<()> {
    for counted in [false, true] {
        let gc = GarbageCollector::with_config(
            Configuration::builder()
                .concurrent_publication(counted)
                .incremental_step_size(1)
                .max_pause_time_us(0)
                .allocation_threshold(usize::MAX)
                .build(),
        );
        let owner = Gc::try_new(&gc, 17_u64)?;
        let target = Gc::try_new(&gc, 913_u64)?;
        let source = GcWeak::new(&target, ReferenceKind::Final, owner.as_ptr().addr());
        let weak = GcWeak::new(&target, ReferenceKind::Weak, 0);
        let phantom = GcWeak::new(&target, ReferenceKind::Phantom, 0);
        drop((owner, target));
        gc.collect();
        at_reference_stage(&gc, 1, 2);
        assert!((0..128).any(|_| {
            gc.step();
            let cycle = lock(&gc.domains[1].cycle);
            cycle.phase == Phase::Mark
                && cycle.reference_stage == 3
                && gc.domains[1].weak_phase() == 2
        }));
        let clone = source.clone();
        finish(&gc);
        assert!(clone.slot.activated.load(Ordering::Acquire));
        assert_eq!(*clone.upgrade().expect("late final clone"), 913);
        assert!(weak.upgrade().is_none());
        // Final discovery precedes the phantom decision at stage three.
        assert!(phantom.upgrade().is_some());
        drop((source, clone, phantom, weak));
        gc.collect_full()?;
        assert_eq!(gc.managed_bytes(), 0);
    }
    Ok(())
}

#[test]
fn parked_late_decision_allows_the_other_wrapped_slot_to_activate() -> Result<()> {
    for counted in [false, true] {
        let gc = GarbageCollector::with_config(
            Configuration::builder()
                .concurrent_publication(counted)
                .incremental_step_size(1)
                .max_pause_time_us(0)
                .allocation_threshold(usize::MAX)
                .build(),
        );
        let owner = Gc::try_new(&gc, 17_u64)?;
        let first = Gc::try_new(&gc, 19_u64)?;
        let second = Gc::try_new(&gc, 23_u64)?;
        gc.next_weak_id.store(usize::MAX - 1, Ordering::Relaxed);
        let a = GcWeak::new(&first, ReferenceKind::Final, owner.as_ptr().addr());
        let b = GcWeak::new(&second, ReferenceKind::Final, owner.as_ptr().addr());
        drop((owner, first, second));
        gc.collect();
        at_reference_stage(&gc, 1, 2);
        assert!((0..64).any(|_| {
            gc.step();
            lock(&gc.domains[1].cycle).cursor == usize::MAX
        }));
        let a = a.clone();
        let b = b.clone();
        let held = a.slot.decision();
        assert!((0..32).any(|_| {
            gc.step();
            b.slot.activated.load(Ordering::Acquire)
        }));
        assert!(!a.slot.activated.load(Ordering::Acquire));
        drop(held);
        finish(&gc);
        assert_eq!(*a.upgrade().expect("first target"), 19);
        assert_eq!(*b.upgrade().expect("second target"), 23);
    }
    Ok(())
}

#[cfg(all(not(target_family = "wasm"), not(miri)))]
#[test]
fn retaining_reference_shades_exact_graph_with_parked_target_shard() -> Result<()> {
    use std::sync::mpsc;
    use std::time::Duration;
    for counted in [false, true] {
        for (kind, stage) in [
            (ReferenceKind::Final, 2),
            (ReferenceKind::PhantomRetained, 3),
        ] {
            let gc = GarbageCollector::with_config(
                Configuration::builder()
                    .concurrent_publication(counted)
                    .incremental_step_size(1)
                    .max_pause_time_us(0)
                    .allocation_threshold(usize::MAX)
                    .build(),
            );
            let child = Gc::try_new(&gc, 17_u64)?;
            let child_id = child.as_handle_i64();
            let parent = Gc::try_new(&gc, vec![child.into_rooted()])?;
            let metadata = gc.objects.lookup(parent.as_ptr().addr()).unwrap();
            let retained = GcWeak::new(&parent, kind, 0);
            drop(parent);
            gc.collect();
            at_reference_stage(&gc, 1, stage);
            let epoch = gc.domains[1].epoch.load(Ordering::Acquire);
            let held = gc.objects.lock(metadata.ptr().0);
            let (done, finished) = mpsc::channel();
            let progress = thread::scope(|scope| {
                let heap = &gc;
                scope.spawn(move || {
                    heap.step();
                    done.send(()).unwrap();
                });
                let result = finished.recv_timeout(Duration::from_secs(2));
                drop(held);
                result
            });
            assert!(progress.is_ok(), "retaining shade waited on target shard");
            assert!(metadata.marked(1, epoch));
            finish(&gc);
            assert_eq!(gc.statistics()?.live_objects, 2);
            assert_eq!(*Gc::<u64>::from_handle_rooted(&gc, child_id)?, 17);
            retained.clear();
            gc.collect_full()?;
            assert_eq!(gc.managed_bytes(), 0);
        }
    }
    Ok(())
}

#[test]
fn retaining_decision_cannot_mark_a_replacement_at_the_target_address() -> Result<()> {
    for counted in [false, true] {
        for kind in [ReferenceKind::Final, ReferenceKind::PhantomRetained] {
            let gc = GarbageCollector::with_config(
                Configuration::builder()
                    .concurrent_publication(counted)
                    .allocation_threshold(usize::MAX)
                    .build(),
            );
            let first = Gc::try_new(&gc, 1_u64)?;
            let stale = GcWeak::new(&first, kind, 0);
            let replacement = Gc::try_new(&gc, 2_u64)?;
            // Present a reused address while retaining the original slot identity.
            stale
                .slot
                .target
                .store(replacement.as_ptr().addr(), Ordering::Release);
            drop((first, replacement));
            gc.collect_full()?;
            assert!(stale.upgrade().is_none());
            assert_eq!(gc.managed_bytes(), 0);
        }
    }
    Ok(())
}

#[cfg(all(not(target_family = "wasm"), not(miri)))]
#[test]
fn queue_delivery_completes_while_live_owner_shard_is_parked() -> Result<()> {
    use std::sync::mpsc;
    use std::time::Duration;
    for counted in [false, true] {
        for kind in [ReferenceKind::Weak, ReferenceKind::Phantom] {
            let gc = GarbageCollector::with_config(
                Configuration::builder()
                    .concurrent_publication(counted)
                    .incremental_step_size(1)
                    .max_pause_time_us(0)
                    .allocation_threshold(usize::MAX)
                    .build(),
            );
            let owner = Gc::try_new(&gc, 21_u64)?;
            let target = Gc::try_new(&gc, 13_u64)?;
            let reference = GcWeak::new(&target, kind, owner.as_ptr().addr());
            drop(target);
            gc.collect();
            at_reference_stage(&gc, 1, 4);
            assert!(reference.slot.pending.load(Ordering::Acquire));
            let held = gc.objects.lock(owner.as_ptr().addr());
            let (done, finished) = mpsc::channel();
            let progress = thread::scope(|scope| {
                let heap = &gc;
                scope.spawn(move || {
                    heap.step();
                    done.send(()).unwrap();
                });
                let result = finished.recv_timeout(Duration::from_secs(2));
                drop(held);
                result
            });
            assert!(progress.is_ok(), "queue delivery waited for owner shard");
            assert!(gc.has_pending_references());
            let delivered = gc
                .take_pending_reference::<u64>()?
                .expect("queued live owner");
            assert_eq!(delivered.identity(), owner.identity());
            assert!(gc.take_pending_reference::<u64>()?.is_none());
            finish(&gc);
            assert!(reference.upgrade().is_none());
        }
    }
    Ok(())
}
