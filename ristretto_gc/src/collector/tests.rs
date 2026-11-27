#![expect(clippy::panic_in_result_fn)]
use super::*;

#[cfg(all(not(target_family = "wasm"), not(miri)))]
#[test]
fn abandoning_a_contended_regional_return_releases_its_range_once() -> Result<()> {
    struct CountDrop(Arc<AtomicUsize>);
    // SAFETY: the host counter owns no managed edges.
    unsafe impl Trace for CountDrop {
        fn trace(&self, _: &GarbageCollector) {}
    }
    impl Drop for CountDrop {
        fn drop(&mut self) {
            self.0.fetch_add(1, Ordering::Relaxed);
        }
    }
    for heap_gone in [false, true] {
        let drops = Arc::new(AtomicUsize::new(0));
        let gc = GarbageCollector::with_config(
            Configuration::builder()
                .local_allocation_buffers(true)
                .max_heap_bytes(8)
                .region_shards(1)
                .allocation_threshold(usize::MAX)
                .build(),
        );
        let heap = gc.descriptor_regions.clone().expect("descriptor heap");
        let root = gc
            .register_context()
            .with(|| Gc::try_new(&gc, CountDrop(drops.clone())))?;
        assert_eq!(heap.statistics().occupied_bytes, 8);
        let meta = gc
            .metadata_for_address(root.as_ptr().addr())
            .expect("live allocation");
        let lease = ObjectMetadata::borrow_callback(&meta, false).map_err(|_| {
            crate::Error::CollectionPhaseError("expected a live descriptor".to_owned())
        })?;
        drop(root);
        gc.collect_full()?;
        drop(lease);
        heap.with_parked_registry(|| {
            assert!(matches!(
                gc.reap_retired(false, None),
                Some(RetirementStep::Contended)
            ));
        });
        assert_eq!(gc.managed_bytes(), 8);
        assert_eq!(drops.load(Ordering::Acquire), 1);
        let job = gc
            .retired
            .load()
            .as_ref()
            .and_then(|queue| queue.pop())
            .expect("queued return");
        assert!(matches!(job.stage, ReclaimStage::Return(_)));
        let weak = gc.weak();
        let live = if heap_gone {
            drop(gc);
            None
        } else {
            Some(gc)
        };
        assert_eq!(weak.upgrade().is_none(), heap_gone);
        drop(job);
        if let Some(heap) = &live {
            assert_eq!(heap.managed_bytes(), 0);
        }
        assert_eq!(heap.statistics().occupied_bytes, 0);
        while !heap.trim(1) {}
        assert_eq!(heap.statistics().shared_committed_bytes, 0);
        assert_eq!(drops.load(Ordering::Acquire), 1);
    }
    Ok(())
}

#[cfg(not(target_family = "wasm"))]
#[test]
fn a_space_offer_lost_to_another_allocator_does_not_spend_recovery_cycles() -> Result<()> {
    use std::future::Future;
    use std::task::{Context, Waker};
    let gc = GarbageCollector::with_config(
        Configuration::builder()
            .max_heap_bytes(8)
            .allocation_threshold(usize::MAX)
            .build(),
    );
    // Model an active worker while the fixture drives completion deterministically.
    let worker = crate::recovery::WorkerRegistration::new(&gc);
    gc.reserve_allocation(8)?;
    let mut allocation = std::pin::pin!(gc.reserve_bytes_async(8));
    let mut context = Context::from_waker(Waker::noop());
    for offer in [false, true, false, true, true] {
        for _ in 0..8 {
            assert!(allocation.as_mut().poll(&mut context).is_pending());
            if gc.recovery.waiter_count() == 1 {
                break;
            }
        }
        assert_eq!(gc.recovery.waiter_count(), 1);
        if offer {
            gc.cancel_allocation(8);
            gc.recovery.notify_progress(&gc); // Enough headroom was offered to this waiter.
            assert_eq!(gc.recovery.waiter_count(), 0);
            gc.reserve_allocation(8)?; // Another allocator wins before the waiter is polled.
        }
        gc.collect_full()?;
        assert!(allocation.as_mut().poll(&mut context).is_pending());
    }
    gc.cancel_allocation(8);
    let std::task::Poll::Ready(reservation) = allocation.as_mut().poll(&mut context) else {
        panic!("available credit must be reserved on the next poll");
    };
    drop(reservation?);
    drop(worker);
    assert_eq!(gc.managed_bytes(), 0);
    assert_eq!(gc.recovery.waiter_count(), 0);
    Ok(())
}

pub(super) fn prepare_full_mark(gc: &GarbageCollector) {
    gc.collect();
    for _ in 0..1000 {
        gc.step();
        if gc.domains[1].trace_ready.load(Ordering::Acquire) {
            return;
        }
    }
    assert!(
        gc.domains[1].trace_ready.load(Ordering::Acquire),
        "fixture did not reach marking"
    );
}

#[test]
fn a_trace_started_before_remembered_enrollment_keeps_later_membership() -> Result<()> {
    struct TwoSteps;
    // SAFETY: the fixture has no outgoing references.
    unsafe impl Trace for TwoSteps {
        fn trace(&self, _: &GarbageCollector) {}
        fn trace_step(&self, _: &GarbageCollector, offset: usize, _: usize) -> TraceProgress {
            if offset == 0 {
                TraceProgress::Continue(1)
            } else {
                TraceProgress::Complete
            }
        }
    }

    for counted in [false, true] {
        let gc = GarbageCollector::with_config(
            Configuration::builder()
                .concurrent_publication(counted)
                .independent_marking(true)
                .incremental_step_size(1)
                .max_pause_time_us(0)
                .allocation_threshold(usize::MAX)
                .build(),
        );
        let owner = Gc::try_new(&gc, TwoSteps)?;
        prepare_full_mark(&gc);
        let address = owner.as_ptr().addr();
        let context = Context::enter(&gc, 1);
        let mut tasks = crate::work::Claim::default();
        assert!(!gc.remembered.may_prune());
        assert_eq!(gc.claim_mark_batch(&mut tasks, &mut 1), 1);
        gc.trace_mark_batch(&mut tasks, None, &mut WorkCounters::default());
        gc.remembered.lock(address).insert(
            address,
            gc.identity_for_address(address).expect("member identity"),
        );
        assert!(gc.remembered.may_prune());
        assert_eq!(gc.claim_mark_batch(&mut tasks, &mut 1), 1);
        gc.trace_mark_batch(&mut tasks, None, &mut WorkCounters::default());
        assert!(gc.remembered.lock(address).contains(address));
        drop(context);
        drop(owner);
        gc.collect_full()?;
        assert_eq!(gc.managed_bytes(), 0);
    }
    Ok(())
}

#[test]
fn publication_after_all_sweep_shards_survives_and_retires_once() -> Result<()> {
    struct Counted(Arc<AtomicUsize>);
    // SAFETY: this fixture contains no outgoing GC references.
    unsafe impl Trace for Counted {
        fn trace(&self, _: &GarbageCollector) {}
    }
    impl Drop for Counted {
        fn drop(&mut self) {
            self.0.fetch_add(1, Ordering::Relaxed);
        }
    }
    for (shards, independent, index) in [(1, false), (16, false), (1, true), (16, true)]
        .into_iter()
        .flat_map(|(shards, independent)| (0..2).map(move |index| (shards, independent, index)))
    {
        let gc = GarbageCollector::with_config(
            Configuration::builder()
                .metadata_shards(shards)
                .independent_marking(independent)
                .incremental_step_size(1)
                .max_pause_time_us(0)
                .full_collection_interval(usize::MAX)
                .allocation_threshold(usize::MAX)
                .build(),
        );
        let drops = Arc::new(AtomicUsize::new(0));
        let old = Gc::new(&gc, Counted(drops.clone()));
        let old_id = old.identity();
        drop(old);
        if index == 0 {
            gc.collect_young();
        } else {
            gc.collect();
        }
        for _ in 0..1000 {
            gc.step();
            if lock(&gc.domain_at(index).cycle).phase == Phase::Compact {
                break;
            }
        }
        assert_eq!(lock(&gc.domain_at(index).cycle).phase, Phase::Compact);
        assert_eq!(drops.load(Ordering::Relaxed), 1);
        assert!(gc.address_for_identity(old_id).is_none());
        // Every shard's sweep cursor has passed. Publication must mark this
        // allocation black even if the allocator reuses a retired address.
        let late = Gc::new(&gc, Counted(drops.clone()));
        let late_id = late.identity();
        while !gc.step() {}
        assert!(gc.address_for_identity(late_id).is_some());
        assert_eq!(gc.statistics()?.live_objects, 1);
        assert_eq!(drops.load(Ordering::Relaxed), 1);
        drop(late);
        gc.stop()?;
        assert!(gc.address_for_identity(late_id).is_none());
        assert_eq!(gc.statistics()?.live_objects, 0);
        assert_eq!(drops.load(Ordering::Relaxed), 2);
    }
    Ok(())
}

#[test]
fn independent_marking_needs_neither_coordinator_nor_cycle_lock() -> Result<()> {
    let gc = GarbageCollector::with_config(
        Configuration::builder()
            .metadata_shards(16)
            .independent_marking(true)
            .diagnostics(true)
            .incremental_step_size(1)
            .max_pause_time_us(0)
            .allocation_threshold(usize::MAX)
            .build(),
    );
    let roots: Vec<_> = (0..16).map(|_| Gc::new(&gc, 42_u64)).collect();
    prepare_full_mark(&gc);
    let coordinator = lock(&gc.coordinator);
    let cycle = lock(&gc.domains[1].cycle);
    for _ in 0..16 {
        assert!(!gc.step());
    }
    assert_eq!(gc.diagnostics().independent_mark_steps, 16);
    assert_eq!(gc.statistics()?.objects_marked, 16);
    assert_eq!(gc.in_flight(), 0);
    assert!(!gc.step()); // Empty queues fall back without blocking on the held coordinator.
    drop((cycle, coordinator));
    while !gc.step() {}
    assert_eq!(gc.statistics()?.full_collections, 1);
    drop(roots);
    gc.stop()?;
    assert_eq!(gc.statistics()?.live_objects, 0);
    Ok(())
}

#[test]
fn a_departed_worker_leaves_its_continuation_available_to_callers() -> Result<()> {
    struct Resumable(Mutex<Vec<usize>>);
    // SAFETY: this fixture contains no outgoing GC references.
    unsafe impl Trace for Resumable {
        fn trace_work(&self) -> usize {
            1
        }
        fn trace(&self, _: &GarbageCollector) {}
        fn trace_step(&self, _: &GarbageCollector, offset: usize, _: usize) -> TraceProgress {
            lock(&self.0).push(offset);
            if offset == 0 {
                TraceProgress::Continue(1)
            } else {
                TraceProgress::Complete
            }
        }
    }
    for independent in [false, true] {
        let gc = GarbageCollector::with_config(
            Configuration::builder()
                .work_stealing(true)
                .independent_marking(independent)
                .threads(4)
                .diagnostics(true)
                .incremental_step_size(1)
                .max_pause_time_us(0)
                .allocation_threshold(usize::MAX)
                .build(),
        );
        let root = Gc::new(&gc, Resumable(Mutex::new(Vec::new())));
        {
            let _worker = WorkerContext::enter(&gc, 3);
            prepare_full_mark(&gc);
            assert!(!gc.step());
        }
        assert_eq!(*lock(&root.0), [0]);
        assert_eq!(gc.diagnostics().queued_mark_tasks, [0, 1]);
        assert_eq!(gc.diagnostics().in_flight_work, [0, 0]);
        // No thread remains assigned to that queue. A cooperative caller must
        // steal its continuation before the generation can close.
        while !gc.step() {}
        assert_eq!(*lock(&root.0), [0, 1]);
        assert_eq!(gc.statistics()?.objects_marked, 1);
        if !cfg!(target_family = "wasm") {
            assert_eq!(gc.diagnostics().mark_stolen, 1);
        }
        drop(root);
        gc.stop()?;
        let diagnostics = gc.diagnostics();
        assert_eq!(diagnostics.queued_mark_tasks, [0, 0]);
        assert_eq!(diagnostics.in_flight_work, [0, 0]);
        assert_eq!(diagnostics.mark_queue_capacity, [0, 0]);
        assert_eq!(gc.statistics()?.live_objects, 0);
    }
    Ok(())
}

#[cfg(not(target_family = "wasm"))]
#[test]
#[expect(clippy::unwrap_in_result)]
#[expect(clippy::too_many_lines, reason = "one forced dequeue/closure schedule")]
fn a_claim_cannot_disappear_between_dequeue_and_reservation() -> Result<()> {
    struct Block {
        entered: std::sync::mpsc::Sender<()>,
        resume: Mutex<std::sync::mpsc::Receiver<()>>,
    }
    // SAFETY: this fixture contains no outgoing GC references.
    unsafe impl Trace for Block {
        fn trace_work(&self) -> usize {
            1
        }
        fn trace(&self, gc: &GarbageCollector) {
            assert_eq!(gc.in_flight(), 1);
            self.entered.send(()).expect("callback entered");
            lock(&self.resume)
                .recv_timeout(std::time::Duration::from_secs(10))
                .expect("resume");
        }
    }
    let gc = GarbageCollector::with_config(
        Configuration::builder()
            .metadata_shards(16)
            .independent_marking(true)
            .work_stealing(true)
            .threads(4)
            .diagnostics(true)
            .incremental_step_size(1)
            .max_pause_time_us(0)
            .allocation_threshold(usize::MAX)
            .build(),
    );
    let (entered, callback) = std::sync::mpsc::channel();
    let (resume, release) = std::sync::mpsc::channel();
    let root = Gc::new(
        &gc,
        Block {
            entered,
            resume: Mutex::new(release),
        },
    );
    {
        let _worker = WorkerContext::enter(&gc, 0);
        prepare_full_mark(&gc);
    }
    // Pause the actual claim after dequeue and before its in-flight reservation.
    // Claims now carry exact metadata and need no object-index lookup.
    let (entered, claimed) = std::sync::mpsc::channel();
    let (release_claim, resume_claim) = std::sync::mpsc::channel();
    *lock(&gc.mark_claim_pause) = Some(ClaimPause {
        entered,
        resume: resume_claim,
    });
    let marker = {
        let gc = gc.clone();
        thread::spawn(move || {
            let _worker = WorkerContext::enter(&gc, 1);
            // Force the independent path this test asserts. A general step
            // may miss a nonblocking steal and claim under the coordinator,
            // which safely excludes closure using a different lock.
            loop {
                if let Some(idle) = gc.independent_mark_step(false) {
                    break idle;
                }
                thread::yield_now();
            }
        })
    };
    claimed
        .recv_timeout(std::time::Duration::from_secs(10))
        .expect("claim entered");
    assert_eq!(gc.domains[1].mark_queue.try_is_empty(), Some(true));
    assert_eq!(gc.in_flight(), 0);
    let gate = gc.mark_claims.as_ref().expect("claim gate");
    assert!(
        gate.try_write().is_err(),
        "the unreserved claim must exclude closure"
    );
    let (finished, closure) = std::sync::mpsc::channel();
    let closer = {
        let gc = gc.clone();
        thread::spawn(move || {
            loop {
                // The observer below also probes the coordinator. Missing its
                // brief guard is allowed; retry until the closer actually runs.
                if let Some(idle) = gc.coordinated_step(true) {
                    finished.send(idle).expect("closure observer");
                    break Some(idle);
                }
                thread::yield_now();
            }
        })
    };
    let yielded = closure.recv_timeout(std::time::Duration::from_secs(1));
    let coordinator_free = gc.coordinator.try_lock().is_ok();
    let publication_free = gc.publication.try_write().is_some();
    release_claim.send(()).expect("release claim");
    callback
        .recv_timeout(std::time::Duration::from_secs(10))
        .expect("entered");
    assert_eq!(closer.join().expect("closer"), Some(false));
    assert_eq!(lock(&gc.domains[1].cycle).phase, Phase::Mark);
    assert_eq!(gc.domains[1].weak_phase(), 0);
    assert_eq!(gc.diagnostics().in_flight_work, [0, 1]);
    assert_eq!(gc.diagnostics().mark_stolen, 1);
    assert_eq!(gc.statistics()?.collections_completed, 0);
    resume.send(()).expect("release");
    marker.join().expect("marker");
    assert_eq!(yielded, Ok(false), "claim wait held global coordination");
    assert!(coordinator_free && publication_free);
    while !gc.step() {}
    assert_eq!(gc.statistics()?.objects_marked, 1);
    assert_eq!(gc.statistics()?.collections_completed, 1);
    drop(root);
    gc.stop()?;
    assert_eq!(gc.statistics()?.live_objects, 0);
    Ok(())
}

#[test]
fn retired_mark_work_cannot_trace_or_prune_a_reused_address() -> Result<()> {
    struct Probe(Arc<AtomicUsize>);
    // SAFETY: this fixture contains no outgoing GC references.
    unsafe impl Trace for Probe {
        fn trace(&self, _: &GarbageCollector) {
            panic!("retired task executed a callback");
        }
    }
    impl Drop for Probe {
        fn drop(&mut self) {
            self.0.fetch_add(1, Ordering::Relaxed);
        }
    }
    for concurrent in [false, true] {
        let gc = GarbageCollector::with_config(
            Configuration::builder()
                .concurrent_publication(concurrent)
                .incremental_step_size(1)
                .mark_batch_size(1)
                .max_pause_time_us(0)
                .allocation_threshold(usize::MAX)
                .build(),
        );
        let drops = Arc::new(AtomicUsize::new(0));
        let root = Gc::try_new(&gc, Probe(drops.clone()))?;
        let address = root.as_ptr() as usize;
        prepare_full_mark(&gc);
        drop(root);
        // Simulate retirement by another generation with a queued task left.
        let retired = gc
            .objects
            .replace_for_test(address, None)
            .expect("metadata");
        #[cfg(feature = "experimental-identity-index")]
        {
            let mut cursor = crate::identity_index::Removal::default();
            while gc
                .identities
                .remove_step(retired.id.to_i64().cast_unsigned(), address, &mut cursor)
                .is_none()
            {}
        }
        #[cfg(not(feature = "experimental-identity-index"))]
        gc.identities
            .remove(retired.id.to_i64().cast_unsigned(), address);
        gc.objects.set_young_for_test(address, false);
        retired.drop_object();
        assert_eq!(drops.load(Ordering::Relaxed), 1);
        assert!(ObjectMetadata::lease(&retired).is_none());

        let replacement = Gc::try_new(&gc, 42_u64)?;
        let replacement_address = replacement.as_ptr() as usize;
        let metadata = gc
            .objects
            .lock(replacement_address)
            .get(&replacement_address)
            .cloned()
            .expect("replacement");
        // Force the old address to resolve to a different type and identity,
        // regardless of the host allocator's reuse policy. Never dereference
        // this synthetic index alias; only the queued identity may be used.
        gc.objects.replace_for_test(address, Some(metadata.clone()));
        let context = Context::enter(&gc, 1);
        let mut tasks = crate::work::Claim::default();
        assert_eq!(gc.claim_mark_batch(&mut tasks, &mut 1), 1);
        let record = &tasks.items[0].as_ref().expect("claim").0;
        assert_eq!(record.metadata.id, retired.id);
        metadata
            .mutation
            .store(retired.mutation.load(Ordering::Acquire), Ordering::Release);
        gc.remembered.lock(address).insert(
            address,
            gc.identity_for_address(address).expect("member identity"),
        );
        let mut counters = WorkCounters::default();
        assert!(!gc.trace_mark_batch(&mut tasks, now(), &mut counters));
        assert_eq!(counters.marked, 1);
        assert!(gc.remembered.lock(address).contains(address));
        if address != replacement_address {
            gc.objects.replace_for_test(address, None);
        }
        gc.remembered.lock(address).remove(address);
        drop(context);
        while !gc.step() {}
        drop(replacement);
        gc.stop()?;
        assert_eq!(drops.load(Ordering::Relaxed), 1);
        assert_eq!(gc.statistics()?.live_objects, 0);
        assert_eq!(gc.managed_bytes(), 0);
    }
    Ok(())
}

#[cfg(not(target_family = "wasm"))]
#[test]
#[expect(clippy::unwrap_in_result)]
fn a_reserved_batch_prevents_termination_in_both_generations() -> Result<()> {
    for independent in [false, true] {
        struct Leaf;
        // SAFETY: this fixture contains no outgoing GC references.
        unsafe impl Trace for Leaf {
            fn trace_work(&self) -> usize {
                1
            }
            fn trace(&self, _: &GarbageCollector) {}
        }
        struct Block {
            entered: std::sync::mpsc::Sender<()>,
            resume: Mutex<std::sync::mpsc::Receiver<()>>,
        }
        // SAFETY: this fixture contains no outgoing GC references.
        unsafe impl Trace for Block {
            fn trace_work(&self) -> usize {
                1
            }
            fn trace(&self, gc: &GarbageCollector) {
                if gc.domain_index() == 1 {
                    assert!(gc.in_flight() >= 8);
                }
                self.entered.send(()).expect("entered");
                lock(&self.resume)
                    .recv_timeout(std::time::Duration::from_secs(10))
                    .expect("resume");
            }
        }
        let gc = GarbageCollector::with_config(
            Configuration::builder()
                .metadata_shards(16)
                .work_stealing(true)
                .independent_marking(independent)
                .diagnostics(true)
                .mark_batch_size(8)
                .incremental_step_size(64)
                .max_pause_time_us(0)
                .allocation_threshold(usize::MAX)
                .full_collection_interval(usize::MAX)
                .build(),
        );
        let (entered, ready) = std::sync::mpsc::channel();
        let (resume, released) = std::sync::mpsc::channel();
        let blocked = Gc::new(
            &gc,
            Block {
                entered,
                resume: Mutex::new(released),
            },
        );
        let roots: Vec<_> = (0..15).map(|_| Gc::new(&gc, Leaf)).collect();
        drop(Gc::new(&gc, 99_u64));
        gc.collect();
        let worker = {
            let gc = gc.clone();
            thread::spawn(move || gc.step())
        };
        ready
            .recv_timeout(std::time::Duration::from_secs(10))
            .expect("first callback");
        for _ in 0..16 {
            assert!(!gc.step());
        }
        assert_eq!(gc.domains[1].mark_queue.try_is_empty(), Some(true));
        assert_eq!(gc.in_flight(), 8);
        assert_eq!(gc.statistics()?.objects_marked, 8);
        assert_eq!(gc.statistics()?.collections_completed, 0);
        assert_eq!(gc.statistics()?.objects_swept, 0);
        // A second generation may scan other tasks but cannot retire either snapshot.
        gc.collect_young();
        for _ in 0..16 {
            assert!(!gc.step());
        }
        assert_eq!(gc.statistics()?.collections_completed, 0);
        assert_eq!(gc.statistics()?.objects_swept, 0);
        // Both generations will execute Block once; release both before draining.
        resume.send(()).expect("release full");
        resume.send(()).expect("release young");
        assert!(!worker.join().expect("worker"));
        while !gc.step() {}
        assert_eq!(gc.in_flight(), 0);
        assert_eq!(gc.statistics()?.collections_completed, 2);
        if independent {
            assert!(gc.diagnostics().independent_mark_steps > 0);
        }
        assert_eq!(gc.statistics()?.objects_marked, 32);
        drop((roots, blocked));
        gc.stop()?;
        assert_eq!(gc.statistics()?.live_objects, 0);
    }
    Ok(())
}

#[test]
fn idle_steps_repair_attached_contexts_while_another_context_writer_is_parked() -> Result<()> {
    let gc = GarbageCollector::with_config(
        Configuration::builder()
            .root_storage(crate::RootStorage::Contexts)
            .allocation_threshold(usize::MAX)
            .build(),
    );
    let parked = gc.register_context();
    let peer = gc.register_context();
    let reserve = || {
        let keep: Vec<_> = (0..crate::roots::SLOTS)
            .map(|_| gc.reserve_owned_root(0x1000))
            .collect();
        (keep, gc.reserve_owned_root(0x1000))
    };
    let (first, cancel_first) = parked.with(reserve);
    let (second, cancel_second) = peer.with(reserve);
    parked.0.roots.with_all_writers_held(|| {
        drop(cancel_first);
        peer.0.roots.with_all_writers_held(|| drop(cancel_second));
        assert!(parked.0.roots.repair_pending() && peer.0.roots.repair_pending());
        for _ in 0..256 {
            gc.step();
        }
        assert!(!peer.0.roots.repair_pending(), "idle peer never repaired");
        assert!(parked.0.roots.repair_pending());
        assert_eq!(peer.0.roots.statistics().pages, 1);
    });
    for _ in 0..256 {
        if gc.step() {
            break;
        }
    }
    assert!(!parked.0.roots.repair_pending());
    assert_eq!(parked.0.roots.statistics().pages, 1);
    assert_eq!(gc.statistics()?.collections_completed, 0);
    drop((first, second, parked, peer));
    gc.stop()
}

#[test]
fn removing_an_escaped_typed_root_services_its_context_without_a_collection() -> Result<()> {
    let gc = GarbageCollector::with_config(
        Configuration::builder()
            .root_storage(crate::RootStorage::Contexts)
            .allocation_threshold(usize::MAX)
            .build(),
    );
    let owner = Gc::try_new(&gc, 13_u64)?;
    let context = gc.register_context();
    let mut aliases = context.with(|| {
        (0..=crate::roots::SLOTS)
            .map(|_| Gc::<u64>::from_handle_rooted(&gc, owner.as_handle_i64()))
            .collect::<Result<Vec<_>>>()
    })?;
    let last = aliases.pop().unwrap();
    context.0.roots.with_all_writers_held(|| drop(last));
    assert!(context.0.roots.repair_pending());
    assert!((0..256).any(|_| gc.step()));
    assert!(!context.0.roots.repair_pending() && !gc.contexts.repairs.pending());
    assert_eq!(context.0.roots.statistics().pages, 1);
    assert_eq!(gc.statistics()?.collections_completed, 0);
    assert!(
        aliases
            .iter()
            .all(|edge| edge.identity() == owner.identity())
    );
    drop((aliases, owner, context));
    gc.stop()
}

#[cfg(not(target_family = "wasm"))]
#[test]
fn a_new_worker_pool_finds_context_repairs_published_between_generations() -> Result<()> {
    let gc = GarbageCollector::with_config(
        Configuration::builder()
            .root_storage(crate::RootStorage::Contexts)
            .build(),
    );
    gc.start();
    let old_pool = gc.worker_signals.load_full().unwrap();
    gc.stop()?;
    let context = gc.register_context();
    let reserve = || context.with(|| gc.reserve_owned_root(0x1000));
    let keep: Vec<_> = (0..crate::roots::SLOTS).map(|_| reserve()).collect();
    let cancel = reserve();
    context.0.roots.with_all_writers_held(|| drop(cancel));
    assert!(gc.contexts.repairs.pending());
    let completed = gc.statistics()?.collections_completed;
    gc.start();
    old_pool.wake();
    let started = std::time::Instant::now();
    while gc.contexts.repairs.pending() {
        assert!(started.elapsed() < std::time::Duration::from_secs(5));
        thread::yield_now();
    }
    assert_eq!(context.0.roots.statistics().pages, 1);
    assert_eq!(gc.statistics()?.collections_completed, completed);
    drop(keep);
    gc.stop()
}

#[cfg(not(target_family = "wasm"))]
#[test]
fn cancelled_root_reservation_wakes_an_idle_worker_to_repair_pages() -> Result<()> {
    let gc = GarbageCollector::with_config(
        Configuration::builder()
            .root_storage(crate::RootStorage::Slots)
            .build(),
    );
    let roots = gc.scanned_slots().expect("paged root fixture");
    // Fill the cached page so cancelling the next reservation releases a page.
    let reservations: Vec<_> = (0..crate::roots::SLOTS)
        .map(|_| gc.reserve_owned_root(0x1000))
        .collect();
    let cancelled = gc.reserve_owned_root(0x1000);
    gc.start();
    let signals = gc.worker_signals.load_full().expect("worker pool");
    let wait_until = |condition: &dyn Fn() -> bool| {
        let start = std::time::Instant::now();
        while !condition() {
            assert!(start.elapsed() < std::time::Duration::from_secs(5));
            thread::yield_now();
        }
    };
    wait_until(&|| {
        signals
            .workers
            .first()
            .is_some_and(|worker| worker.waiting.load(Ordering::Acquire))
    });
    roots.with_all_writers_held(|| {
        drop(cancelled);
        assert!(roots.repair_pending());
    });
    wait_until(&|| !roots.repair_pending());
    assert_eq!(roots.statistics().pages, 1);
    drop(reservations);
    gc.stop()
}

#[cfg(not(target_family = "wasm"))]
#[test]
fn idle_worker_does_not_block_on_a_busy_coordinator() -> Result<()> {
    let gc = GarbageCollector::with_config(Configuration::builder().diagnostics(true).build());
    let coordinator = lock(&gc.coordinator);
    gc.start();
    gc.wake();
    let started = std::time::Instant::now();
    while gc.diagnostics().worker_backoffs == 0
        && started.elapsed() < std::time::Duration::from_secs(1)
    {
        thread::sleep(std::time::Duration::from_millis(1));
    }
    let backed_off = gc.diagnostics().worker_backoffs != 0;
    drop(coordinator);
    gc.stop()?;
    assert!(
        backed_off,
        "idle handoff must back off without blocking on the coordinator"
    );
    Ok(())
}

#[test]
fn root_published_after_slot_scan_is_shaded_in_each_generation() -> Result<()> {
    for full in [false, true] {
        let gc = GarbageCollector::with_config(
            Configuration::builder()
                .root_storage(crate::RootStorage::Slots)
                .incremental_step_size(1)
                .max_pause_time_us(0)
                .allocation_threshold(usize::MAX)
                .build(),
        );
        // SAFETY: the fixture drives only root enumeration before rooting this allocation.
        let value = unsafe { Gc::with_collector_unrooted(&gc, 7_u64) };
        if full {
            gc.collect();
        } else {
            gc.collect_young();
        }
        let index = usize::from(full);
        for _ in 0..100 {
            gc.step();
            if lock(&gc.domain_at(index).cycle).phase == Phase::Roots {
                break;
            }
        }
        assert_eq!(lock(&gc.domain_at(index).cycle).phase, Phase::Roots);
        let root = gc.create_root_guard(value);
        while !gc.step() {}
        assert_eq!(**root, 7);
        assert_eq!(gc.statistics()?.live_objects, 1);
        drop(root);
        gc.stop()?;
        assert_eq!(gc.statistics()?.live_objects, 0);
        assert_eq!(gc.root_statistics().pages, 0);
    }
    Ok(())
}

#[test]
fn context_records_cover_both_snapshots_and_late_dirty_owners() -> Result<()> {
    for prepared in [false, true] {
        let gc = GarbageCollector::with_config(
            Configuration::builder()
                .root_storage(crate::RootStorage::Contexts)
                .barrier_buffer_size(64)
                .diagnostics(true)
                .incremental_step_size(1)
                .max_pause_time_us(0)
                .allocation_threshold(usize::MAX)
                .full_collection_interval(usize::MAX)
                .build(),
        );
        let context = gc.register_context();
        let parent = context
            .with(|| Gc::try_new(&gc, crate::sync::RwLock::new(Vec::<Gc<u64>>::new())))?
            .into_rooted();
        gc.collect_full()?;
        gc.collect_full()?;
        assert!(!gc.is_young_allocation(parent.as_ptr() as usize));
        gc.collect_young();
        gc.collect();
        for _ in 0..1000 {
            gc.step();
            if lock(&gc.domains[0].cycle).phase == Phase::Mark {
                break;
            }
        }
        assert_eq!(lock(&gc.domains[0].cycle).phase, Phase::Mark);
        assert!(gc.domains.iter().all(Domain::active));
        let child = context.with(|| Gc::try_new(&gc, 42_u64))?.into_rooted();
        let weak = crate::GcWeak::new(&child, crate::ReferenceKind::Weak, 0);
        let address = parent.as_ptr() as usize;
        let version = gc
            .objects
            .lock(address)
            .get(&address)
            .expect("parent")
            .mutation
            .load(Ordering::Acquire);
        context.with(|| {
            let mut writer = parent.write();
            if prepared {
                writer.push(child);
            } else {
                // SAFETY: this appends an owning root without moving existing traced edges.
                unsafe { writer.unrooted_mut() }.push(child);
            }
        });
        if prepared {
            assert!(gc.remembered.lock(address).contains(address));
        } else {
            assert!(gc.contexts.pending.load(Ordering::Acquire) > 0);
        }
        assert!(
            gc.objects
                .lock(address)
                .get(&address)
                .expect("parent")
                .mutation
                .load(Ordering::Acquire)
                > version
        );
        while !gc.step() {}
        assert_eq!(gc.contexts.pending.load(Ordering::Acquire), 0);
        gc.collect_young();
        while !gc.step() {}
        assert_eq!(weak.upgrade().as_deref(), Some(&42));
        context.with(|| parent.write().clear());
        drop(parent);
        gc.stop()?;
        assert!(weak.upgrade().is_none());
        assert_eq!(gc.statistics()?.live_objects, 0);
    }
    Ok(())
}

#[cfg(not(target_family = "wasm"))]
#[test]
fn remembered_pruning_does_not_wait_for_unrelated_publication() -> Result<()> {
    let gc = GarbageCollector::with_config(Configuration::builder().metadata_shards(16).build());
    let root = Gc::try_new(&gc, 42_u64)?;
    let address = root.as_ptr() as usize;
    let identity = root.identity();
    gc.remember(&root);
    let version = gc
        .objects
        .lock(address)
        .get(&address)
        .expect("root")
        .mutation
        .load(Ordering::Acquire);
    let publication = gc.allocation_guard();
    let (send, receive) = std::sync::mpsc::channel();
    let worker = {
        let gc = gc.clone();
        thread::spawn(move || {
            gc.prune_remembered(address, identity, version, false);
            send.send(()).expect("prune completion");
        })
    };
    let completed = receive
        .recv_timeout(std::time::Duration::from_secs(1))
        .is_ok();
    drop(publication); // let a failing legacy writer finish before joining
    worker
        .join()
        .unwrap_or_else(|error| std::panic::resume_unwind(error));
    assert!(
        completed,
        "pruning waited for the heap-wide publication writer"
    );
    assert!(!gc.remembered.lock(address).contains(address));
    // A later mutation must invalidate the old scan and remain remembered.
    gc.remember(&root);
    gc.prune_remembered(address, identity, version, false);
    assert!(gc.remembered.lock(address).contains(address));
    drop(root);
    gc.stop()?;
    Ok(())
}

#[cfg(all(not(target_family = "wasm"), not(miri)))]
#[test]
fn optional_pruning_defers_while_either_required_shard_is_parked() -> Result<()> {
    use std::sync::mpsc;
    let gc = GarbageCollector::new();
    let root = Gc::try_new(&gc, 42_u64)?;
    let address = root.as_ptr().addr();
    let id = root.identity();
    gc.remember(&root);
    let version = gc
        .objects
        .lookup(address)
        .unwrap()
        .mutation
        .load(Ordering::Acquire);
    for object_shard in [false, true] {
        let objects = object_shard.then(|| gc.objects.lock(address));
        let remembered = (!object_shard).then(|| gc.remembered.lock(address));
        let (done, result) = mpsc::channel();
        let completed = thread::scope(|scope| {
            let heap = &gc;
            scope.spawn(move || {
                heap.prune_remembered(address, id, version, false);
                done.send(()).unwrap();
            });
            let completed = result.recv_timeout(std::time::Duration::from_secs(2));
            drop((objects, remembered));
            completed
        });
        assert!(completed.is_ok(), "optional pruning waited on a shard");
        assert!(gc.remembered.lock(address).contains(address));
    }
    gc.prune_remembered(address, id, version, false);
    assert!(!gc.remembered.lock(address).contains(address));
    Ok(())
}

#[cfg(not(target_family = "wasm"))]
#[test]
fn remembered_barriers_progress_with_unrelated_generation_shards_held() -> Result<()> {
    let gc = GarbageCollector::with_config(Configuration::builder().metadata_shards(16).build());
    let root = Gc::try_new(&gc, 42_u64)?;
    let address = root.as_ptr() as usize;
    let other = crate::index::address_bucket(address) ^ 1;
    let objects = gc.objects.lock_bucket(other);
    let remembered = gc.remembered.lock_bucket(other);
    let (send, receive) = std::sync::mpsc::channel();
    let worker = {
        let gc = gc.clone();
        thread::spawn(move || {
            gc.remember(&root);
            send.send(root).expect("barrier completion");
        })
    };
    let completed = receive.recv_timeout(std::time::Duration::from_secs(1));
    drop((objects, remembered));
    worker
        .join()
        .unwrap_or_else(|error| std::panic::resume_unwind(error));
    let root = completed.map_err(|error| crate::Error::SyncError(error.to_string()))?;
    assert!(gc.remembered.lock(address).contains(address));
    assert!(gc.objects.contains_young(address));
    drop(root);
    gc.stop()?;
    assert_eq!(gc.statistics()?.live_objects, 0);
    Ok(())
}

#[cfg(not(target_family = "wasm"))]
#[test]
fn conditional_owner_construction_releases_weak_decision_admission() -> Result<()> {
    let gc = GarbageCollector::new();
    let child = Gc::try_new(&gc, 42_u64)?;
    let link = crate::GcWeak::new(&child, crate::ReferenceKind::Final, 0);
    let mut construction = crate::construction::Roots::new(&gc);
    let scope = construction.enter();
    assert!(gc.publication.try_write().is_some());
    link.set_owner(usize::MAX); // pending owner has no registered metadata yet
    assert!(gc.publication.try_write().is_some());
    drop(scope);
    assert!(gc.publication.try_write().is_some());
    drop(link);
    drop(child);
    gc.stop()?;
    Ok(())
}

#[cfg(not(target_family = "wasm"))]
#[test]
fn buffered_drain_does_not_wait_for_unrelated_publication() -> Result<()> {
    let gc = GarbageCollector::with_config(
        Configuration::builder()
            .barrier_buffer_size(4)
            .incremental_step_size(1)
            .max_pause_time_us(0)
            .allocation_threshold(usize::MAX)
            .build(),
    );
    let context = gc.register_context();
    let root = Gc::try_new(&gc, 42_u64)?;
    gc.collect();
    // Drive to tracing without allowing the one-unit step to close it.
    while lock(&gc.domains[1].cycle).phase != Phase::Mark {
        gc.step();
    }
    context.with(|| gc.write_barrier(&root));
    assert_eq!(gc.contexts.pending.load(Ordering::Acquire), 1);
    let publication = gc.allocation_guard();
    let (send, receive) = std::sync::mpsc::channel();
    let worker = {
        let gc = gc.clone();
        thread::spawn(move || {
            gc.step();
            send.send(()).expect("drain completion");
        })
    };
    let completed = receive
        .recv_timeout(std::time::Duration::from_secs(5))
        .is_ok();
    drop(publication);
    worker
        .join()
        .unwrap_or_else(|error| std::panic::resume_unwind(error));
    assert!(
        completed,
        "buffer draining waited for an exclusive publication gate"
    );
    assert_eq!(gc.contexts.pending.load(Ordering::Acquire), 0);
    drop(root);
    drop(context);
    gc.stop()?;
    Ok(())
}

#[cfg(not(target_family = "wasm"))]
#[test]
fn a_popped_context_record_excludes_closure_without_holding_publication() -> Result<()> {
    let gc = GarbageCollector::with_config(
        Configuration::builder()
            .barrier_buffer_size(4)
            .incremental_step_size(1)
            .max_pause_time_us(0)
            .allocation_threshold(usize::MAX)
            .build(),
    );
    let context = gc.register_context();
    let root = Gc::try_new(&gc, 42_u64)?;
    gc.collect();
    while lock(&gc.domains[1].cycle).phase != Phase::Mark {
        gc.step();
    }
    context.with(|| gc.write_barrier(&root));
    let objects = gc.objects.lock(root.as_ptr() as usize);
    let (finished, done) = std::sync::mpsc::channel();
    let (claimed, claim) = std::sync::mpsc::channel();
    let (release, resume) = std::sync::mpsc::channel();
    let flusher = thread::spawn(move || {
        barrier_progress_tests::park_next_claim(ClaimPause {
            entered: claimed,
            resume,
        });
        context.flush();
        finished.send(()).expect("clean record acknowledged");
    });
    let claimed = claim.recv_timeout(std::time::Duration::from_secs(10));
    let pending = gc.contexts.pending.load(Ordering::Acquire);
    let publication_free = gc.publication.try_write().is_some();
    gc.step();
    let active = gc.domains[1].active();
    let ticket = gc.completed_full_ticket();
    assert!(release.send(()).is_ok(), "resume flusher");
    let without_shard = done.recv_timeout(std::time::Duration::from_secs(2));
    drop(objects);
    flusher
        .join()
        .unwrap_or_else(|error| std::panic::resume_unwind(error));
    assert!(claimed.is_ok());
    assert_eq!(without_shard, Ok(()));
    assert_eq!(pending, 1);
    assert!(publication_free && active);
    assert_eq!(ticket, 0);
    assert_eq!(gc.contexts.pending.load(Ordering::Acquire), 0);
    gc.collect_full()?;
    assert_eq!(**root, 42);
    drop(root);
    gc.stop()?;
    Ok(())
}

#[test]
fn root_and_weak_identifiers_skip_live_entries_after_wrap() -> Result<()> {
    let gc = GarbageCollector::new();
    let first = Gc::new(&gc, 1_u64);
    let first_weak = crate::GcWeak::new(&first, crate::ReferenceKind::Weak, 0);
    gc.next_root_id.store(usize::MAX, Ordering::Relaxed);
    gc.next_weak_id.store(usize::MAX, Ordering::Relaxed);
    let second = Gc::new(&gc, 2_u64);
    let third = Gc::new(&gc, 3_u64);
    let second_weak = crate::GcWeak::new(&second, crate::ReferenceKind::Weak, 0);
    let third_weak = crate::GcWeak::new(&third, crate::ReferenceKind::Weak, 0);
    gc.collect_full()?;
    assert_eq!(gc.statistics()?.live_objects, 3);
    drop((first, second, third));
    gc.collect_full()?;
    assert!(first_weak.upgrade().is_none());
    assert!(second_weak.upgrade().is_none());
    assert!(third_weak.upgrade().is_none());
    Ok(())
}

#[cfg(all(not(target_family = "wasm"), not(miri)))]
fn poll_while_flusher_is_suspended(
    gc: &Arc<GarbageCollector>,
    context: crate::ExecutionContext,
    debt: usize,
) -> (bool, bool, usize) {
    use std::future::Future;
    use std::sync::mpsc;
    use std::task::{Context as TaskContext, Poll, Waker};
    use std::time::Duration;

    let (entered, observed) = mpsc::channel();
    let (release, resume) = mpsc::channel();
    let flusher = thread::spawn(move || {
        if debt != 0 {
            barrier_progress_tests::park_next_claim(ClaimPause { entered, resume });
            context.flush();
        } else {
            context.flush();
            entered.send(()).expect("flush finished");
            resume
                .recv_timeout(Duration::from_secs(10))
                .expect("resume after flush");
        }
    });
    observed
        .recv_timeout(Duration::from_secs(3))
        .expect("flusher suspended");
    let ticket = gc.request_full_cycle();
    let mut peer = Box::pin(Gc::try_new_async(gc, 23u64));
    let mut task = TaskContext::from_waker(Waker::noop());
    let mut grant = None;
    for _ in 0..2048 {
        if grant.is_none()
            && let Poll::Ready(result) = peer.as_mut().poll(&mut task)
        {
            grant = Some(result);
        }
        gc.step();
        if grant.is_some() && gc.completed_full_ticket() >= ticket {
            break;
        }
    }
    let full = gc.completed_full_ticket() >= ticket;
    let acquired = grant
        .as_ref()
        .is_some_and(|x| x.as_ref().is_ok_and(|root| ***root == 23));
    let pending = gc.contexts.pending.load(Ordering::Acquire);
    release.send(()).expect("resume flusher");
    flusher.join().expect("flusher");
    drop(peer);
    drop(grant);
    (full, acquired, pending)
}

#[cfg(all(not(target_family = "wasm"), not(miri)))]
#[test]
#[expect(clippy::panic_in_result_fn, reason = "bounded peer progress schedule")]
fn fresh_full_and_capped_allocation_finish_while_clean_context_flusher_is_suspended() -> Result<()>
{
    for (counted, raw, interval, old_pass, promote) in [false, true].into_iter().flat_map(|c| {
        [
            (c, false, 0, false, false),
            (c, true, 0, false, false),
            (c, false, 4, false, false),
            (c, true, 4, false, false),
            (c, false, 4, true, false),
            (c, true, 4, true, false),
            (c, false, 4, true, true),
            (c, true, 4, true, true),
        ]
    }) {
        let gc = GarbageCollector::with_config(
            Configuration::builder()
                .barrier_buffer_size(4)
                .concurrent_publication(counted)
                .old_collection_interval(interval)
                .promotion_age(1)
                .max_heap_bytes(512)
                .incremental_step_size(1)
                .max_pause_time_us(0)
                .allocation_threshold(usize::MAX)
                .build(),
        );
        let child = Gc::try_new(&gc, 42u64)?.into_rooted();
        // Old-only reference processing would pre-mark a weak referent.
        // Delay its witness for the mature-white schedule.
        let weak = (!promote).then(|| crate::GcWeak::new(&child, crate::ReferenceKind::Weak, 0));
        let parent = Gc::try_new(&gc, vec![child])?;
        if promote {
            gc.collect_full()?;
        }
        drop(Gc::try_new(&gc, 99u64)?);
        let funding = gc.reserve_bytes(gc.allocation_headroom())?;
        let context = gc.register_context();
        if old_pass {
            gc.collect_old();
        } else {
            gc.collect();
        }
        for _ in 0..2048 {
            if lock(&gc.domains[1].cycle).phase == Phase::Mark {
                break;
            }
            gc.step();
        }
        assert_eq!(lock(&gc.domains[1].cycle).phase, Phase::Mark);
        assert_eq!(gc.old_collection_active(), old_pass);
        let target = parent.first().expect("child");
        if promote {
            let descriptor = gc
                .objects
                .lookup_identity(target.as_ptr().addr(), target.identity())
                .expect("exact child");
            assert!(!gc.is_young_metadata(&descriptor));
            assert!(!descriptor.marked(1, gc.domains[1].epoch.load(Ordering::Acquire)));
        }
        if raw {
            // SAFETY: the rooted parent retains this exact child across the barrier.
            let borrowed = unsafe { Gc::<u64>::from_raw_i64(target.as_ptr_i64())? };
            assert_eq!(borrowed.identity().to_i64(), 0);
            context.with(|| borrowed.write_barrier(&gc));
        } else {
            context.with(|| gc.write_barrier(target));
        }
        let debt = gc.contexts.pending.load(Ordering::Acquire);
        let (full, acquired, pending) = poll_while_flusher_is_suspended(&gc, context, debt);
        assert_eq!(
            (debt, pending, full, acquired),
            (0, 0, true, true),
            "counted={counted} raw={raw} interval={interval} old_pass={old_pass} promote={promote}"
        );
        let weak =
            weak.unwrap_or_else(|| crate::GcWeak::new(target, crate::ReferenceKind::Weak, 0));
        drop(funding);
        gc.collect_full()?;
        assert!(weak.refers_to(target));
        drop(parent);
        gc.collect_full()?;
        assert!(weak.upgrade().is_none());
        gc.stop()?;
        assert_eq!(gc.managed_bytes(), 0);
    }
    Ok(())
}

#[cfg(all(not(target_family = "wasm"), not(miri)))]
#[test]
#[expect(clippy::panic_in_result_fn, reason = "bounded peer progress schedule")]
fn fresh_full_and_capped_allocation_finish_while_dirty_owner_flusher_is_suspended() -> Result<()> {
    let mut outcomes = Vec::new();
    for (counted, busy) in [false, true]
        .into_iter()
        .flat_map(|counted| [(counted, false), (counted, true)])
    {
        let gc = GarbageCollector::with_config(
            Configuration::builder()
                .barrier_buffer_size(4)
                .concurrent_publication(counted)
                .max_heap_bytes(512)
                .incremental_step_size(1)
                .max_pause_time_us(0)
                .allocation_threshold(usize::MAX)
                .build(),
        );
        let child = Gc::try_new(&gc, 42u64)?.into_rooted();
        let weak = crate::GcWeak::new(&child, crate::ReferenceKind::Weak, 0);
        let parent = Gc::try_new(&gc, crate::cell::RwLock::new(vec![child]))?;
        let inserted = Gc::try_new(&gc, 51_u64)?.into_rooted();
        let inserted_weak = crate::GcWeak::new(&inserted, crate::ReferenceKind::Weak, 0);
        drop(Gc::try_new(&gc, 99u64)?);
        let funding = gc.reserve_bytes(gc.allocation_headroom())?;
        let context = gc.register_context();
        gc.collect();
        for _ in 0..2048 {
            if lock(&gc.domains[1].cycle).phase == Phase::Mark {
                break;
            }
            gc.step();
        }
        assert_eq!(lock(&gc.domains[1].cycle).phase, Phase::Mark);
        let address = parent.as_ptr().addr();
        let held = busy.then(|| gc.remembered.lock(address));
        let metadata = gc
            .objects
            .lookup_identity(address, parent.identity())
            .expect("original parent");
        metadata.mutation.store(usize::MAX, Ordering::Release);
        context.with(|| parent.write().push(inserted));
        context.with(|| {
            let state = crate::context::current(&gc).expect("active mutation context");
            for _ in 0..crate::context::MAX_RECORDS {
                let Some(record) = state.try_peek() else {
                    if busy {
                        let mut cursor = gc.remembered.dirty.cursor();
                        assert!(
                            std::iter::from_fn(|| gc.remembered.dirty.next(&mut cursor))
                                .any(|entry| entry
                                    == (parent.identity().to_i64().cast_unsigned(), address))
                        );
                    } else {
                        assert!(gc.remembered.lock(address).contains(address));
                    }
                    return;
                };
                if record.dirty {
                    assert_eq!(record.identity, parent.identity());
                    return;
                }
                assert!(gc.flush_context_record(&state));
            }
            panic!("dirty owner not found");
        });
        let debt = gc.contexts.pending.load(Ordering::Acquire);
        let (full, acquired, pending) = poll_while_flusher_is_suspended(&gc, context, debt);
        outcomes.push((counted, busy, debt, pending, full, acquired));
        drop(held);
        drop(funding);
        gc.collect_full()?;
        {
            let edges = parent.read();
            assert!(weak.refers_to(&edges[0]) && inserted_weak.refers_to(&edges[1]));
        }
        drop(parent);
        gc.collect_full()?;
        assert!(weak.upgrade().is_none() && inserted_weak.upgrade().is_none());
        gc.stop()?;
        assert_eq!(gc.managed_bytes(), 0);
    }
    assert!(
        outcomes
            .iter()
            .all(|(_, _, debt, pending, full, acquired)| *debt == 0
                && *pending == 0
                && *full
                && *acquired),
        "{outcomes:?}"
    );
    Ok(())
}

#[test]
#[expect(
    clippy::panic_in_result_fn,
    reason = "verify exact graph retention across two full epochs"
)]
fn detached_clean_edge_preserves_its_graph_for_one_full_epoch() -> Result<()> {
    for counted in [false, true] {
        let gc = GarbageCollector::with_config(
            Configuration::builder()
                .concurrent_publication(counted)
                .barrier_buffer_size(4)
                .incremental_step_size(1)
                .max_pause_time_us(0)
                .allocation_threshold(usize::MAX)
                .build(),
        );
        let leaf = Gc::try_new(&gc, 59u64)?.into_rooted();
        let leaf_weak = crate::GcWeak::new(&leaf, crate::ReferenceKind::Weak, 0);
        let child = Gc::try_new(&gc, vec![leaf])?.into_rooted();
        let child_weak = crate::GcWeak::new(&child, crate::ReferenceKind::Weak, 0);
        let parent = Gc::try_new(&gc, vec![child])?;
        let edge = parent.first().expect("traced child");
        let id = edge.identity();
        let address = edge.as_ptr().addr();
        let descriptor = edge
            .traced_metadata()
            .expect("same-heap traced edge")
            .clone();
        drop(parent);
        // Only the captured edge can retain either weak witness in this epoch.
        let ticket = gc.request_full_cycle();
        for _ in 0..2048 {
            if lock(&gc.domains[1].cycle).phase == Phase::Mark {
                break;
            }
            gc.step();
        }
        assert_eq!(lock(&gc.domains[1].cycle).phase, Phase::Mark);
        let context = gc.register_context();
        let epoch = gc.domains[1]
            .mark_queue
            .birth_epoch()
            .expect("active full epoch");
        let before = epoch.proposal_count();
        context.with(|| {
            for _ in 0..4096 {
                gc.publish_edge(address, id, Some(&descriptor), false);
            }
        });
        let offered = epoch.proposal_count() - before;
        assert!(offered <= 4, "one white target added {offered} proposals");
        assert_eq!(gc.contexts.pending.load(Ordering::Acquire), 0);
        for _ in 0..2048 {
            if gc.completed_full_ticket() >= ticket {
                break;
            }
            gc.step();
        }
        assert!(gc.completed_full_ticket() >= ticket);
        let recovered = child_weak.upgrade().expect("original child graph");
        let leaf = recovered.first().expect("original leaf");
        assert!(leaf_weak.refers_to(leaf));
        assert_eq!(**leaf, 59);
        drop(recovered);
        gc.collect_full()?;
        assert!(child_weak.upgrade().is_none() && leaf_weak.upgrade().is_none());
        assert_eq!(gc.managed_bytes(), 0);
    }
    Ok(())
}
