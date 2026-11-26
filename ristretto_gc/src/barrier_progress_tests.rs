//! Required-record reservations outlive claims without retaining global guards.
#![expect(clippy::panic_in_result_fn)]
#![cfg_attr(not(target_family = "wasm"), expect(clippy::unwrap_in_result))]
use super::*;

#[cfg(all(not(target_family = "wasm"), not(miri)))]
#[test]
fn young_start_retraces_a_dirty_owner_enrolled_after_the_remembered_scan() -> Result<()> {
    use std::sync::mpsc;
    use std::time::Duration;
    for counted in [false, true] {
        for independent in [false, true] {
            let gc = GarbageCollector::with_config(
                Configuration::builder()
                    .concurrent_publication(counted)
                    .independent_phases(independent)
                    .barrier_buffer_size(4)
                    .promotion_age(1)
                    .incremental_step_size(1)
                    .max_pause_time_us(0)
                    .allocation_threshold(usize::MAX)
                    .full_collection_interval(usize::MAX)
                    .build(),
            );
            let context = gc.register_context();
            let parent = Gc::try_new(&gc, crate::cell::RwLock::new(None::<Gc<u64>>))?;
            gc.collect_full()?;
            let address = parent.as_ptr() as usize;
            assert!(!gc.is_young_allocation(address));
            let child = Gc::try_new(&gc, 42_u64)?;
            let weak = crate::GcWeak::new(&child, crate::ReferenceKind::Weak, 0);
            context.with(|| *parent.write() = Some(child.into_rooted()));
            // Require the buffered notification below, not an entry left by
            // promotion or the immediate idle mutation barrier.
            assert!(gc.remembered.lock(address).remove(address));
            gc.collect();
            assert!(!gc.step());
            context.with(|| gc.remember(&parent));
            assert_eq!(gc.contexts.pending.load(Ordering::Acquire), 1);
            assert!(!gc.domains[0].active());
            let (entered, observed) = mpsc::channel();
            let (resume, resumed) = mpsc::channel();
            *lock(&gc.barrier_prepare_pause) = Some(ClaimPause {
                entered,
                resume: resumed,
            });
            let worker_gc = gc.clone();
            let state = context.0.clone();
            let worker = thread::spawn(move || worker_gc.try_apply_buffered_record(&state));
            observed
                .recv_timeout(Duration::from_secs(10))
                .expect("inactive preparation");
            gc.collect_young();
            for _ in 0..1000 {
                gc.next_domain.store(0, Ordering::Release);
                gc.coordinated_step(false);
                if lock(&gc.domains[0].cycle).phase == Phase::Remembered {
                    break;
                }
            }
            for _ in 0..32 {
                gc.next_domain.store(0, Ordering::Release);
                gc.coordinated_step(false);
            }
            let phase = lock(&gc.domains[0].cycle).phase;
            let active = gc.domains[0].active();
            let pending = gc.contexts.pending.load(Ordering::Acquire);
            resume.send(()).expect("resume insertion");
            let applied = worker.join().expect("prepared worker");
            assert!(applied && active);
            assert_eq!(phase, Phase::Mark);
            assert_eq!(pending, 1);
            assert_eq!(gc.contexts.pending.load(Ordering::Acquire), 0);
            assert!(gc.remembered.lock(address).contains(address));
            let epoch = gc.domains[0].epoch.load(Ordering::Acquire);
            assert_eq!(
                gc.objects
                    .lock(address)
                    .get(&address)
                    .expect("old owner")
                    .retention(0, epoch),
                Some(Retention::Strong)
            );
            for _ in 0..10000 {
                // Keep full tracing from independently retaining the child.
                gc.next_domain.store(0, Ordering::Release);
                gc.coordinated_step(false);
                if gc.statistics()?.young_collections == 1 {
                    break;
                }
            }
            assert_eq!(gc.statistics()?.young_collections, 1);
            let recovered = weak.upgrade().expect("remembered child");
            assert_eq!(*recovered, 42);
            println!(
                "young start after inactive preparation: counted={counted} independent={independent}; scan_passed=true child_retained=true"
            );
            while !gc.step() {}
            drop((parent, recovered, context));
            gc.stop()?;
            assert_eq!(gc.managed_bytes(), 0);
        }
    }
    Ok(())
}

#[cfg(all(not(target_family = "wasm"), not(miri)))]
#[test]
#[expect(
    clippy::too_many_lines,
    reason = "forced barrier stall and delayed storage release"
)]
fn busy_barrier_indexes_leave_the_worker_available_for_ready_reclamation() -> Result<()> {
    use std::sync::mpsc;
    use std::time::Duration;
    struct Retired(Arc<AtomicUsize>);
    // SAFETY: this fixture contains no outgoing GC references.
    unsafe impl Trace for Retired {
        fn trace(&self, _: &GarbageCollector) {}
    }
    impl Drop for Retired {
        fn drop(&mut self) {
            self.0.fetch_add(1, Ordering::Release);
        }
    }
    for counted in [false, true] {
        for independent in [false, true] {
            for remembered in [false, true] {
                let gc = GarbageCollector::with_config(
                    Configuration::builder()
                        .concurrent_publication(counted)
                        .independent_phases(independent)
                        .diagnostics(true)
                        .root_storage(crate::RootStorage::Contexts)
                        .barrier_buffer_size(4)
                        .incremental_step_size(1)
                        .max_pause_time_us(0)
                        .allocation_threshold(usize::MAX)
                        .build(),
                );
                assert_eq!(gc.registered_workers(), 0);
                let drops = Arc::new(AtomicUsize::new(0));
                let old = Gc::try_new(&gc, Retired(drops.clone()))?;
                let address = old.as_ptr() as usize;
                let metadata = gc
                    .objects
                    .lock(address)
                    .get(&address)
                    .expect("retired metadata")
                    .clone();
                let lease = ObjectMetadata::lease(&metadata).expect("storage lease");
                drop(old);
                gc.collect_full()?;
                assert_eq!(gc.retired_bytes.load(Ordering::Acquire), metadata.size());
                assert_eq!(drops.load(Ordering::Acquire), 0);
                let initial_ticket = gc.completed_full_ticket();

                let context = gc.register_context();
                let mut root = Some(
                    context.with(|| Gc::try_new(&gc, vec![Gc::new(&gc, 42_u64).into_rooted()]))?,
                );
                let parent = root.as_ref().expect("parent root");
                let address = parent.as_ptr() as usize;
                let weak = crate::GcWeak::new(parent, crate::ReferenceKind::Weak, 0);
                gc.collect();
                gc.step();
                context.with(|| {
                    if remembered {
                        gc.remember(root.as_ref().expect("dirty parent"));
                    } else {
                        drop(root.take());
                    }
                });
                assert_eq!(gc.contexts.pending.load(Ordering::Acquire), 1);
                let (held, entered) = mpsc::channel();
                let (release, released) = mpsc::channel();
                let holder_gc = gc.clone();
                let holder = thread::spawn(move || {
                    let pause = || {
                        held.send(()).expect("index held");
                        released
                            .recv_timeout(Duration::from_secs(10))
                            .expect("release index");
                    };
                    if remembered {
                        let _index = holder_gc.remembered.lock(address);
                        pause();
                    } else {
                        let _index = holder_gc.objects.lock(address);
                        pause();
                    }
                });
                entered
                    .recv_timeout(Duration::from_secs(10))
                    .expect("held index");
                let (first, attempted) = mpsc::channel();
                let (resume, resumed) = mpsc::channel();
                let (finished, completion) = mpsc::channel();
                let worker_gc = gc.clone();
                let worker_drops = drops.clone();
                let worker = thread::spawn(move || {
                    first.send(worker_gc.step()).expect("first step");
                    resumed
                        .recv_timeout(Duration::from_secs(10))
                        .expect("released lease");
                    for _ in 0..1000 {
                        worker_gc.step();
                        if worker_drops.load(Ordering::Acquire) != 0 {
                            break;
                        }
                    }
                    finished.send(()).expect("reclamation attempt");
                });
                let yielded = attempted.recv_timeout(Duration::from_secs(1));
                drop(lease);
                resume.send(()).expect("resume worker");
                let progressed = completion.recv_timeout(Duration::from_secs(1));
                let reclaimed = drops.load(Ordering::Acquire);
                let pending = gc.contexts.pending.load(Ordering::Acquire);
                let diagnostics = gc.diagnostics();
                let ticket = gc.completed_full_ticket();
                let active = gc
                    .publication_clock
                    .as_ref()
                    .map_or(0, |clock| clock.active());
                println!(
                    "busy barrier index: counted={counted} independent={independent} remembered={remembered}; yielded={yielded:?} progressed={progressed:?} reclaimed={reclaimed} pending={pending} ticket={ticket} initial_ticket={initial_ticket} active={active}"
                );
                release.send(()).expect("release index holder");
                holder.join().expect("index holder");
                worker.join().expect("collector worker");
                assert_eq!(yielded, Ok(false));
                assert_eq!(progressed, Ok(()));
                assert!(diagnostics.barrier_drain.count > 0);
                assert_eq!(diagnostics.barrier_flushed, 1);
                assert_eq!(
                    (reclaimed, pending, ticket, active),
                    (1, 0, initial_ticket, 0)
                );
                for _ in 0..10000 {
                    if gc.step() {
                        break;
                    }
                }
                assert!(gc.step());
                let recovered = weak.upgrade().expect("barrier graph retained");
                assert_eq!(*recovered[0], 42);
                drop((root, recovered, context, metadata));
                gc.stop()?;
                assert_eq!(drops.load(Ordering::Acquire), 1);
                assert_eq!(gc.managed_bytes(), 0);
            }
        }
    }
    Ok(())
}

#[test]
fn optional_barrier_certification_skips_busy_metadata_after_required_work() -> Result<()> {
    for concurrent in [false, true] {
        let gc = GarbageCollector::with_config(
            Configuration::builder()
                .concurrent_publication(concurrent)
                .barrier_buffer_size(1)
                .incremental_step_size(1)
                .allocation_threshold(usize::MAX)
                .build(),
        );
        let context = gc.register_context();
        let root = Gc::try_new(&gc, 42_u64)?;
        let address = root.as_ptr() as usize;
        // This synthetic stale epoch has no required shading. It isolates the
        // optional certificate lookup from the necessary metadata lookup.
        let record = crate::context::Record {
            address,
            identity: root.identity(),
            epochs: [Some(4), None],
            dirty: false,
        };
        let admission = gc.allocation_guard();
        assert_eq!(
            context.0.push(&gc.contexts, record, 1),
            crate::context::Push::Added
        );
        drop(admission);
        let objects = gc.objects.lock(address);
        assert_eq!(gc.drain_barrier_step(false), Some(true));
        assert_eq!(gc.contexts.pending.load(Ordering::Acquire), 0);
        assert!(!context.0.satisfies(&gc.contexts, record));
        drop(objects);
        gc.collect_full()?;
        assert_eq!(**root, 42);
        drop((root, context));
        gc.stop()?;
        assert_eq!(gc.managed_bytes(), 0);
    }
    Ok(())
}

#[cfg(all(not(target_family = "wasm"), not(miri)))]
#[test]
#[expect(
    clippy::too_many_lines,
    reason = "forced application schedule and ordered cleanup"
)]
fn barrier_application_retains_pending_or_publishes_around_a_busy_queue() -> Result<()> {
    use std::sync::mpsc;
    use std::time::Duration;
    for concurrent in [false, true] {
        for independent in [false, true] {
            for (hold_queue, worker_queues) in [(false, 0), (true, 0), (false, 2), (true, 2)] {
                let gc = GarbageCollector::with_config(
                    Configuration::builder()
                        .threads(worker_queues)
                        .work_stealing(worker_queues != 0)
                        .concurrent_publication(concurrent)
                        .independent_phases(independent)
                        .root_storage(crate::RootStorage::Contexts)
                        .barrier_buffer_size(4)
                        .incremental_step_size(1)
                        .max_pause_time_us(0)
                        .allocation_threshold(usize::MAX)
                        .build(),
                );
                assert_eq!(gc.registered_workers(), 0);
                let context = gc.register_context();
                let root =
                    context.with(|| Gc::try_new(&gc, vec![Gc::new(&gc, 42_u64).into_rooted()]))?;
                let address = root.as_ptr() as usize;
                let weak = crate::GcWeak::new(&root, crate::ReferenceKind::Weak, 0);
                let garbage = Gc::try_new(&gc, 99_u64)?;
                let garbage_weak = crate::GcWeak::new(&garbage, crate::ReferenceKind::Weak, 0);
                drop(garbage);
                gc.collect();
                gc.step();
                context.with(|| drop(root));
                assert_eq!(gc.contexts.pending.load(Ordering::Acquire), 1);
                let (entered, target_held) = mpsc::channel();
                let (release, released) = mpsc::channel();
                let holder_gc = gc.clone();
                let holder = thread::spawn(move || {
                    let pause = || {
                        entered.send(()).expect("target lock held");
                        released
                            .recv_timeout(Duration::from_secs(10))
                            .expect("release target lock");
                    };
                    if hold_queue {
                        holder_gc.domains[1].mark_queue.with_held_injector(pause);
                    } else {
                        let _objects = holder_gc.objects.lock(address);
                        pause();
                    }
                });
                target_held
                    .recv_timeout(Duration::from_secs(10))
                    .expect("held target");
                let (claimed, claim) = mpsc::channel();
                *lock(&gc.barrier_claim_observer) = Some(claimed);
                let worker_gc = gc.clone();
                let (finished, completion) = mpsc::channel();
                let worker = thread::spawn(move || {
                    finished.send(worker_gc.step()).expect("drain complete");
                });
                let drained = completion.recv_timeout(Duration::from_secs(1)).is_ok();
                let claimed = claim.try_recv().is_ok();
                let stalled =
                    !drained && matches!(completion.try_recv(), Err(mpsc::TryRecvError::Empty));
                let buffered = context.0.try_peek().is_some();
                let coordinator_free = gc.coordinator.try_lock().is_ok();
                let publication_free = gc.publication.try_write().is_some();
                let active = gc
                    .publication_clock
                    .as_ref()
                    .map_or(0, |clock| clock.active());
                let fallback = hold_queue.then(|| gc.domains[1].mark_queue.fallback_pending());
                let queued = hold_queue.then(|| gc.domains[1].mark_queue.queued_sample());
                let observer_gc = gc.clone();
                let (yielded, result) = mpsc::channel();
                let observer = thread::spawn(move || {
                    if hold_queue {
                        for _ in 0..10000 {
                            if observer_gc.step() {
                                break;
                            }
                        }
                        assert!(
                            observer_gc.step(),
                            "fallback work did not finish the snapshot"
                        );
                    } else {
                        for _ in 0..128 {
                            assert!(!observer_gc.step());
                        }
                    }
                    yielded.send(()).expect("other collector yields");
                });
                let progress = result.recv_timeout(Duration::from_secs(1));
                let pending = gc.contexts.pending.load(Ordering::Acquire);
                let ticket = gc.completed_full_ticket();
                let reclaimed = hold_queue.then(|| garbage_weak.upgrade().is_none());
                println!(
                    "held barrier application: counted={concurrent} independent={independent} queue={hold_queue} worker_queues={worker_queues}; claimed={claimed} stalled={stalled} drained={drained} buffered={buffered} coordinator_free={coordinator_free} publication_free={publication_free} active={active} progress={progress:?} pending={pending} ticket={ticket} reclaimed={reclaimed:?} fallback={fallback:?} queued={queued:?}"
                );
                release.send(()).expect("release application");
                holder.join().expect("target holder");
                worker.join().expect("barrier drain");
                observer.join().expect("competing collector");
                assert!(coordinator_free && publication_free);
                assert!(
                    claimed,
                    "clean edge remained unclaimed behind the target shard"
                );
                assert!(!buffered);
                assert_eq!(reclaimed, hold_queue.then_some(true));
                assert!(drained);
                assert!(!stalled);
                assert_eq!(active, 0);
                assert_eq!(
                    fallback,
                    hold_queue.then_some(usize::from(worker_queues == 0))
                );
                assert_eq!(queued, hold_queue.then_some(Some(1)));
                assert_eq!(progress, Ok(()));
                assert_eq!((pending, ticket), if hold_queue { (0, 1) } else { (0, 0) });
                for _ in 0..10000 {
                    if gc.step() {
                        break;
                    }
                }
                assert!(gc.step());
                let recovered = weak.upgrade().expect("claimed deletion graph retained");
                assert_eq!(*recovered[0], 42);
                drop((recovered, context));
                gc.stop()?;
                assert!(weak.upgrade().is_none());
                assert_eq!(gc.managed_bytes(), 0);
            }
        }
    }
    Ok(())
}

#[cfg(all(not(target_family = "wasm"), not(miri)))]
#[test]
fn mutator_overflow_drains_clean_and_dirty_edges_with_busy_indexes() -> Result<()> {
    use std::sync::mpsc;
    use std::time::Duration;
    for counted in [false, true] {
        for independent in [false, true] {
            for hold_remembered in [false, true] {
                let gc = GarbageCollector::with_config(
                    Configuration::builder()
                        .concurrent_publication(counted)
                        .independent_phases(independent)
                        .metadata_shards(16)
                        .root_storage(crate::RootStorage::Contexts)
                        .barrier_buffer_size(1)
                        .diagnostics(true)
                        .incremental_step_size(1)
                        .max_pause_time_us(0)
                        .allocation_threshold(usize::MAX)
                        .build(),
                );
                let context = gc.register_context();
                let blocked = Gc::try_new(&gc, vec![Gc::new(&gc, 41_u64).into_rooted()])?;
                let address = blocked.as_ptr() as usize;
                let bucket = crate::index::address_bucket(address) & 15;
                let mut others = Vec::new();
                let mut target = None;
                for _ in 0..128 {
                    let candidate = Gc::try_new(&gc, vec![Gc::new(&gc, 42_u64).into_rooted()])?;
                    if crate::index::address_bucket(candidate.as_ptr() as usize) & 15 != bucket {
                        target = Some(candidate);
                        break;
                    }
                    others.push(candidate);
                }
                let target = target.expect("distinct publication shard");
                let weak = crate::GcWeak::new(&target, crate::ReferenceKind::Weak, 0);
                gc.collect();
                assert!(!gc.step());
                context.with(|| {
                    if hold_remembered {
                        gc.remember(&blocked);
                    } else {
                        gc.write_barrier(&blocked);
                    }
                });
                assert_eq!(gc.contexts.pending.load(Ordering::Acquire), 1);
                let record = context.0.try_peek().expect("required pending record");
                let next_address = target.as_ptr() as usize;
                let objects = (!hold_remembered).then(|| gc.objects.lock(address));
                let remembered = hold_remembered.then(|| gc.remembered.lock(address));
                let worker_context = context.clone();
                let (finished, completion) = mpsc::channel();
                let worker = thread::spawn(move || {
                    worker_context.with(|| drop(target));
                    finished.send(()).expect("overflow producer complete");
                });
                let progress = completion.recv_timeout(Duration::from_secs(2));
                let pending = gc.contexts.pending.load(Ordering::Acquire);
                let still_pending = context.0.try_peek();
                let active = gc
                    .publication_clock
                    .as_ref()
                    .map_or(0, |clock| clock.active());
                drop((objects, remembered));
                worker.join().expect("overflow producer");
                assert_eq!(
                    progress,
                    Ok(()),
                    "counted={counted} independent={independent} remembered={hold_remembered}"
                );
                assert_eq!(pending, 1);
                let buffered = still_pending.expect("one outstanding record");
                assert_eq!(buffered.address, next_address);
                assert!(!buffered.dirty);
                assert_eq!(buffered.epochs, record.epochs);
                if hold_remembered {
                    let mut scan = gc.remembered.dirty.cursor();
                    assert_eq!(
                        gc.remembered.dirty.next(&mut scan),
                        Some((record.identity.to_i64().cast_unsigned(), record.address))
                    );
                }
                assert_eq!(active, 0);
                while !gc.step() {}
                assert_eq!(gc.contexts.pending.load(Ordering::Acquire), 0);
                assert_eq!(gc.diagnostics().barrier_overflows, 1);
                let retained = weak
                    .upgrade()
                    .expect("direct overflow fallback retains target graph");
                assert_eq!(*retained[0], 42);
                assert_eq!(*blocked[0], 41);
                drop((retained, blocked, others, context));
                gc.stop()?;
                assert!(weak.upgrade().is_none());
                assert_eq!(gc.managed_bytes(), 0);
                println!(
                    "mutator overflow: counted={counted} independent={independent} remembered={hold_remembered}; buffered={still_pending:?}"
                );
            }
        }
    }
    Ok(())
}

#[cfg(all(not(target_family = "wasm"), not(miri)))]
thread_local! {
    static BEFORE_REMEMBER: std::cell::Cell<Option<ClaimPause>> = const { std::cell::Cell::new(None) };
}

#[cfg(all(not(target_family = "wasm"), not(miri)))]
pub(super) fn pause_before_remembering() {
    if let Some(pause) = BEFORE_REMEMBER.take() {
        pause.entered.send(()).expect("parked writer");
        pause
            .resume
            .recv_timeout(std::time::Duration::from_secs(10))
            .expect("release writer");
    }
}

#[cfg(all(not(target_family = "wasm"), not(miri)))]
#[test]
fn parked_writer_retains_incoming_root_through_two_young_cycles() -> Result<()> {
    use std::sync::mpsc;
    use std::time::Duration;

    for counted in [false, true] {
        let gc = GarbageCollector::with_config(
            Configuration::builder()
                .concurrent_publication(counted)
                .promotion_age(4)
                .barrier_buffer_size(0)
                .allocation_threshold(usize::MAX)
                .full_collection_interval(usize::MAX)
                .build(),
        );
        let owner = Gc::try_new(&gc, crate::cell::RwLock::new(None::<Gc<u64>>))?;
        for _ in 0..4 {
            gc.collect_full()?;
        }
        let address = owner.as_ptr() as usize;
        assert!(!gc.is_young_allocation(address));
        gc.collect_young();
        while !gc.step() {}
        assert!(!gc.remembered.lock(address).contains(address));
        let incoming = Gc::try_new(&gc, 42_u64)?.into_rooted();
        let incoming_address = incoming.as_ptr() as usize;
        let alias = incoming.clone();
        let weak = crate::GcWeak::new(&incoming, crate::ReferenceKind::Weak, 0);
        let garbage = Gc::try_new(&gc, 11_u64)?;
        let unreachable = crate::GcWeak::new(&garbage, crate::ReferenceKind::Weak, 0);
        drop(garbage);
        let (entered, parked) = mpsc::channel();
        let (resume, release) = mpsc::channel();
        thread::scope(|scope| -> Result<()> {
            let writer_owner = &owner;
            let writer = scope.spawn(move || {
                let mut guard = writer_owner.write();
                *guard = Some(incoming);
                BEFORE_REMEMBER.set(Some(ClaimPause {
                    entered,
                    resume: release,
                }));
                drop(guard);
            });
            parked
                .recv_timeout(Duration::from_secs(3))
                .expect("writer suspended before membership");
            drop(alias);
            let before = gc.statistics()?.young_collections;
            for _ in 0..2 {
                gc.collect_young();
                for _ in 0..4096 {
                    if gc.step() {
                        break;
                    }
                }
            }
            let completed = gc.statistics()?.young_collections;
            let child_alive = weak.upgrade().is_some();
            let reclaimed = unreachable.upgrade().is_none();
            resume.send(()).expect("release writer");
            writer.join().expect("writer");
            assert_eq!(completed, before + 2);
            assert!(reclaimed, "unrelated garbage survived both young cycles");
            assert!(
                child_alive,
                "incoming edge lost its last root before publication"
            );
            Ok(())
        })?;
        assert!(gc.remembered.lock(address).contains(address));
        assert!(gc.is_young_allocation(incoming_address));
        let before = gc.statistics()?.young_collections;
        gc.collect_young();
        for _ in 0..4096 {
            if gc.step() {
                break;
            }
        }
        assert_eq!(gc.statistics()?.young_collections, before + 1);
        assert!(weak.upgrade().is_some());
        assert_eq!(**owner.read().as_ref().expect("protected field"), 42);
        *owner.write() = None;
        gc.collect_full()?;
        assert!(weak.upgrade().is_none());
        drop(owner);
        gc.stop()?;
        assert_eq!(gc.managed_bytes(), 0);
    }
    Ok(())
}

#[cfg(all(not(target_family = "wasm"), not(miri)))]
#[test]
fn demoted_vector_edge_survives_a_young_cycle_before_owner_notification() -> Result<()> {
    struct Edges(Vec<Gc<u64>>, bool);

    // SAFETY: all strong edges delegate tracing and ownership to their vector.
    unsafe impl Trace for Edges {
        fn trace(&self, gc: &GarbageCollector) {
            self.0.trace(gc);
        }

        fn root(&mut self) {
            self.0.root();
        }

        unsafe fn unroot(&mut self, owner: crate::TraceOwner<'_>) {
            // SAFETY: this exclusive borrow and the actual owner cover all elements.
            unsafe { self.0.unroot(owner) };
            pause_before_remembering();
            assert!(!std::mem::take(&mut self.1), "interrupted demotion");
        }
    }

    for counted in [false, true] {
        for buffer in [0, 4] {
            let gc = GarbageCollector::with_config(
                Configuration::builder()
                    .concurrent_publication(counted)
                    .promotion_age(1)
                    .barrier_buffer_size(buffer)
                    .allocation_threshold(usize::MAX)
                    .full_collection_interval(usize::MAX)
                    .build(),
            );
            let parent = Gc::try_new(&gc, crate::cell::RwLock::new(Edges(vec![], false)))?;
            gc.collect_full()?;
            generation_tests::finish_young(&gc)?;
            let address = parent.as_ptr() as usize;
            assert!(!gc.is_young_allocation(address));
            assert!(!gc.remembered.lock(address).contains(address));
            let child = Gc::try_new(&gc, 42_u64)?.into_rooted();
            let id = child.identity();
            let weak = crate::GcWeak::new(&child, crate::ReferenceKind::Weak, 0);
            let garbage = Gc::try_new(&gc, 13_u64)?;
            let victim = crate::GcWeak::new(&garbage, crate::ReferenceKind::Weak, 0);
            drop(garbage);
            thread::scope(|scope| -> Result<()> {
                let (entered, parked) = std::sync::mpsc::channel();
                let (resume, release) = std::sync::mpsc::channel();
                let owner = &parent;
                let writer = scope.spawn(move || {
                    let mut edges = owner.write();
                    edges.0.push(child);
                    BEFORE_REMEMBER.set(Some(ClaimPause {
                        entered,
                        resume: release,
                    }));
                    drop(edges);
                });
                parked
                    .recv_timeout(std::time::Duration::from_secs(3))
                    .unwrap();
                let finished = generation_tests::finish_young(&gc);
                let alive = weak.upgrade().is_some();
                let reclaimed = victim.upgrade().is_none();
                resume.send(()).unwrap();
                writer.join().unwrap();
                finished?;
                assert!(
                    alive,
                    "lost child after demotion, before owner notification"
                );
                assert!(reclaimed, "a suspended store retained unrelated garbage");
                Ok(())
            })?;
            let witness = parent.read().0[0].rooted_clone();
            assert_eq!(witness.identity(), id);
            assert_eq!(*witness, 42);
            drop(witness);
            parent.write().0.clear();
            gc.collect_full()?;
            assert!(weak.upgrade().is_none());

            let next = Gc::try_new(&gc, 73_u64)?.into_rooted();
            let id = next.identity();
            let panic = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                let mut edges = parent.write();
                edges.0.push(next);
                edges.1 = true;
            }));
            assert!(panic.is_err());
            assert!(!crate::cell::has_guards());
            generation_tests::finish_young(&gc)?;
            assert_eq!(parent.read().0[0].rooted_clone().identity(), id);
        }
    }
    Ok(())
}

#[test]
fn prepared_field_update_keeps_new_membership_against_a_stale_prune() -> Result<()> {
    for counted in [false, true] {
        let gc = GarbageCollector::with_config(
            Configuration::builder()
                .concurrent_publication(counted)
                .promotion_age(1)
                .allocation_threshold(usize::MAX)
                .full_collection_interval(usize::MAX)
                .build(),
        );
        let owner = Gc::try_new(&gc, crate::cell::RwLock::new(None::<Gc<u64>>))?;
        gc.collect_full()?;
        let address = owner.as_ptr() as usize;
        assert!(!gc.is_young_allocation(address));
        let metadata = gc.objects.lookup(address).expect("live owner");
        let stale_version = metadata.mutation.load(Ordering::Acquire);
        let child = Gc::try_new(&gc, 19_u64)?.into_rooted();
        let weak = crate::GcWeak::new(&child, crate::ReferenceKind::Weak, 0);
        *owner.write() = Some(child);
        gc.prune_remembered(address, metadata.id, stale_version, false);
        assert!(gc.remembered.lock(address).contains(address));
        gc.collect_young();
        for _ in 0..4096 {
            if gc.step() {
                break;
            }
        }
        assert_eq!(gc.statistics()?.young_collections, 1);
        assert_eq!(*weak.upgrade().expect("strong field"), 19);
        *owner.write() = None;
        gc.collect_full()?;
        assert!(weak.upgrade().is_none());
    }
    Ok(())
}

#[cfg(all(not(target_family = "wasm"), not(miri)))]
#[test]
fn pinned_retired_owner_cannot_reinsert_its_remembered_address() -> Result<()> {
    use std::sync::mpsc;
    use std::time::Duration;
    for counted in [false, true] {
        let gc = GarbageCollector::with_config(
            Configuration::builder()
                .concurrent_publication(counted)
                .barrier_buffer_size(0)
                .allocation_threshold(usize::MAX)
                .build(),
        );
        let owner = Gc::try_new(&gc, 23_u64)?;
        let address = owner.as_ptr() as usize;
        let metadata = gc.objects.lookup(address).expect("visible owner");
        let lease = ObjectMetadata::lease(&metadata).expect("storage pin");
        let (entered, parked) = mpsc::channel();
        let (resume, release) = mpsc::channel();
        thread::scope(|scope| -> Result<()> {
            let heap = &gc;
            let writer = scope.spawn(move || {
                BEFORE_REMEMBER.set(Some(ClaimPause {
                    entered,
                    resume: release,
                }));
                heap.remember_before_unroot(address)
            });
            parked
                .recv_timeout(Duration::from_secs(3))
                .expect("publisher pending");
            drop(owner);
            gc.collect_full()?;
            assert!(gc.objects.lookup(address).is_none());
            resume.send(()).expect("resume");
            assert!(!writer.join().expect("publisher"));
            assert!(!gc.remembered.lock(address).contains(address));
            Ok(())
        })?;
        drop(lease);
        gc.stop()?;
        assert_eq!(gc.managed_bytes(), 0);
    }
    Ok(())
}

#[test]
fn mutation_version_saturation_disables_optional_pruning_for_that_owner() -> Result<()> {
    let gc = GarbageCollector::new();
    let owner = Gc::try_new(&gc, crate::cell::RwLock::new(None::<Gc<u64>>))?;
    let address = owner.as_ptr() as usize;
    let metadata = gc.objects.lookup(address).expect("visible owner");
    metadata.mutation.store(usize::MAX - 1, Ordering::Release);
    gc.remember(&owner);
    *owner.write() = Some(Gc::try_new(&gc, 75_u64)?.into_rooted());
    assert_eq!(metadata.mutation.load(Ordering::Acquire), usize::MAX);
    gc.prune_remembered(address, metadata.id, usize::MAX, false);
    assert!(gc.remembered.lock(address).contains(address));
    Ok(())
}

#[cfg(all(not(target_family = "wasm"), not(miri)))]
#[test]
fn ordinary_vector_write_enrolls_a_mature_owner_while_either_shard_is_parked() -> Result<()> {
    use std::sync::mpsc;
    use std::time::Duration;
    for counted in [false, true] {
        for object in [false, true] {
            let gc = GarbageCollector::with_config(
                Configuration::builder()
                    .concurrent_publication(counted)
                    .promotion_age(3)
                    .allocation_threshold(usize::MAX)
                    .full_collection_interval(usize::MAX)
                    .build(),
            );
            let owner = Gc::try_new(&gc, crate::cell::RwLock::new(Vec::<Gc<u64>>::new()))?;
            for _ in 0..3 {
                gc.collect_full()?;
            }
            let address = owner.as_ptr() as usize;
            assert!(!gc.is_young_allocation(address));
            generation_tests::finish_young(&gc)?;
            assert!(!gc.remembered.lock(address).contains(address));
            let child = Gc::try_new(&gc, 43_u64)?.into_rooted();
            let weak = crate::GcWeak::new(&child, crate::ReferenceKind::Weak, 0);
            let victim = Gc::try_new(&gc, 17_u64)?;
            let garbage = crate::GcWeak::new(&victim, crate::ReferenceKind::Weak, 0);
            drop(victim);
            thread::scope(|scope| {
                let (entered, parked) = mpsc::channel();
                let (go, wait) = mpsc::channel();
                let (done, observed) = mpsc::channel();
                let parent = &owner;
                let writer = scope.spawn(move || {
                    let mut value = parent.write();
                    value.push(child);
                    entered.send(()).unwrap();
                    wait.recv().unwrap();
                    drop(value);
                    done.send(()).unwrap();
                });
                parked.recv_timeout(Duration::from_secs(3)).unwrap();
                let objects = object.then(|| gc.objects.lock(address));
                let remembered = (!object).then(|| gc.remembered.lock(address));
                go.send(()).unwrap();
                let completed = observed.recv_timeout(Duration::from_secs(3)).is_ok();
                drop((objects, remembered));
                writer.join().unwrap();
                assert!(
                    completed,
                    "dirty publication waited for a parked owner shard"
                );
            });
            for _ in 0..2 {
                generation_tests::finish_young(&gc)?;
                assert_eq!(weak.upgrade().as_deref(), Some(&43));
            }
            assert!(gc.is_young_allocation(owner.read()[0].as_ptr() as usize));
            assert!(garbage.upgrade().is_none());
            drop(owner);
            gc.collect_full()?;
            assert!(weak.upgrade().is_none());
            assert_eq!(gc.managed_bytes(), 0);
        }
    }
    Ok(())
}

#[cfg(all(not(target_family = "wasm"), not(miri)))]
#[test]
fn prepared_field_store_completes_with_a_parked_metadata_writer() -> Result<()> {
    for (counted, buffer, object) in [false, true].into_iter().flat_map(|counted| {
        [(0, false), (0, true), (4, false), (4, true)]
            .map(|(buffer, object)| (counted, buffer, object))
    }) {
        let gc = GarbageCollector::with_config(
            Configuration::builder()
                .concurrent_publication(counted)
                .barrier_buffer_size(buffer)
                .promotion_age(1)
                .allocation_threshold(usize::MAX)
                .full_collection_interval(usize::MAX)
                .build(),
        );
        let owner = Gc::try_new(&gc, crate::cell::RwLock::new(None::<Gc<u64>>))?;
        gc.collect_full()?;
        generation_tests::finish_young(&gc)?;
        let address = owner.as_ptr().addr();
        assert!(!gc.remembered.lock(address).contains(address));
        let child = Gc::try_new(&gc, 71_u64)?.into_rooted();
        let id = child.identity();
        let weak = crate::GcWeak::new(&child, crate::ReferenceKind::Weak, 0);
        thread::scope(|scope| {
            let objects = object.then(|| gc.objects.lock(address));
            let remembered = (!object).then(|| gc.remembered.lock(address));
            let (done, finished) = std::sync::mpsc::channel();
            let parent = &owner;
            let writer = scope.spawn(move || {
                *parent.write() = Some(child);
                done.send(()).unwrap();
            });
            let completed = finished
                .recv_timeout(std::time::Duration::from_secs(3))
                .is_ok();
            drop((objects, remembered));
            writer.join().unwrap();
            assert!(
                completed,
                "a parked metadata writer blocked field publication"
            );
        });
        generation_tests::finish_young(&gc)?;
        let witness = owner.read().as_ref().unwrap().rooted_clone();
        assert_eq!(witness.identity(), id);
        assert_eq!(*witness, 71);
        drop(witness);
        *owner.write() = None;
        gc.collect_full()?;
        assert!(weak.upgrade().is_none());
    }
    Ok(())
}

struct SplitField {
    edge: Mutex<Option<Gc<u64>>>,
    scans: AtomicUsize,
    split: AtomicBool,
    busy: AtomicBool,
}
// SAFETY: the fixture serializes edge installation and publishes its dirty
// owner before granting the collector any further steps.
unsafe impl Trace for SplitField {
    fn trace(&self, gc: &GarbageCollector) {
        if gc.domain_index() == 0 {
            self.scans.fetch_add(1, Ordering::Relaxed);
        }
        lock(&self.edge).trace(gc);
    }

    fn trace_step(&self, gc: &GarbageCollector, offset: usize, _: usize) -> TraceProgress {
        if offset == 0 {
            self.trace(gc);
            if self.split.load(Ordering::Acquire) {
                return TraceProgress::Continue(1);
            }
        } else if self.busy.swap(false, Ordering::AcqRel) {
            return TraceProgress::Busy;
        }
        TraceProgress::Complete
    }
}

#[test]
fn a_late_contended_dirty_owner_is_retraced_even_if_already_strong() -> Result<()> {
    for (counted, contended, buffer) in [(false, false), (true, false), (false, true), (true, true)]
        .into_iter()
        .flat_map(|(counted, contended)| [0, 4].map(|buffer| (counted, contended, buffer)))
    {
        check_late_owner_write(counted, contended, buffer, DirtyDelivery::Immediate)?;
    }
    Ok(())
}

#[test]
fn a_late_buffered_owner_write_survives_collector_draining() -> Result<()> {
    for counted in [false, true] {
        check_late_owner_write(counted, false, 4, DirtyDelivery::Collector)?;
    }
    Ok(())
}

#[test]
fn a_late_buffered_owner_write_survives_explicit_flushing() -> Result<()> {
    for counted in [false, true] {
        check_late_owner_write(counted, false, 4, DirtyDelivery::ExplicitFlush)?;
    }
    Ok(())
}

#[derive(Clone, Copy)]
enum DirtyDelivery {
    Immediate,
    Collector,
    ExplicitFlush,
}

fn check_late_owner_write(
    counted: bool,
    contended: bool,
    buffer: usize,
    delivery: DirtyDelivery,
) -> Result<()> {
    let gc = GarbageCollector::with_config(
        Configuration::builder()
            .concurrent_publication(counted)
            .barrier_buffer_size(buffer)
            .promotion_age(2)
            .incremental_step_size(1)
            .max_pause_time_us(0)
            .allocation_threshold(usize::MAX)
            .full_collection_interval(usize::MAX)
            .build(),
    );
    let owner = Gc::try_new(
        &gc,
        SplitField {
            edge: Mutex::default(),
            scans: AtomicUsize::new(0),
            split: AtomicBool::new(false),
            busy: AtomicBool::new(false),
        },
    )?;
    gc.collect_full()?;
    gc.collect_full()?;
    let address = owner.as_ptr() as usize;
    assert!(!gc.is_young_allocation(address));
    let meta = gc.objects.lookup(address).expect("registered owner");
    let mut child = Gc::try_new(&gc, 79_u64)?.into_rooted();
    let target = gc
        .objects
        .lookup(child.as_ptr() as usize)
        .expect("registered child");
    let weak = crate::GcWeak::new(&child, crate::ReferenceKind::Weak, 0);
    // SAFETY: the test drives this young cycle only through the owner's
    // first mark callback. It installs the edge before weak closure/sweep.
    unsafe { child.unroot() };
    gc.remember(&owner);
    let before = owner.scans.load(Ordering::Relaxed);
    gc.collect_young();
    assert!((0..4096).any(|_| {
        gc.step();
        owner.scans.load(Ordering::Relaxed) > before
    }));
    let epoch = gc.domains[0].epoch.load(Ordering::Acquire);
    assert_eq!(gc.domains[0].weak_phase(), 0);
    assert_eq!(meta.retention(0, epoch), Some(Retention::Strong));
    assert!(!target.marked(0, epoch));
    owner.split.store(true, Ordering::Release);
    owner.busy.store(true, Ordering::Release);
    meta.mutation.store(usize::MAX, Ordering::Release);
    {
        let _parked = gc.remembered.lock(address);
        for _ in 0..1024 {
            gc.remember(&owner);
        }
    }
    assert!((0..4096).any(|_| {
        gc.step();
        owner.scans.load(Ordering::Relaxed) == before + 2
    }));
    assert!(!target.marked(0, epoch));
    *lock(&owner.edge) = Some(child);
    let context = gc.register_context();
    match delivery {
        DirtyDelivery::Immediate => {
            let _parked = contended.then(|| gc.remembered.lock(address));
            for _ in 0..1024 {
                gc.remember(&owner);
            }
        }
        DirtyDelivery::Collector | DirtyDelivery::ExplicitFlush => {
            context.with(|| gc.remember(&owner));
            assert!(context.0.try_peek().expect("queued mutation").dirty);
            assert_eq!(gc.contexts.pending.load(Ordering::Acquire), 1);
            if matches!(delivery, DirtyDelivery::ExplicitFlush) {
                context.flush();
            } else {
                assert!(gc.try_apply_buffered_record(&context.0));
            }
            assert_eq!(gc.contexts.pending.load(Ordering::Acquire), 0);
        }
    }
    assert!((0..4096).any(|_| gc.step()));
    assert_eq!(weak.upgrade().as_deref(), Some(&79));
    assert_eq!(owner.scans.load(Ordering::Relaxed), before + 3);
    drop(owner);
    gc.collect_full()?;
    assert!(weak.upgrade().is_none());
    assert!(
        gc.remembered
            .dirty
            .next(&mut gc.remembered.dirty.cursor())
            .is_none()
    );
    let _admission = gc.allocation_guard();
    gc.remember_contended_owner(&meta, None);
    assert!(
        gc.remembered
            .dirty
            .next(&mut gc.remembered.dirty.cursor())
            .is_none()
    );
    Ok(())
}

#[test]
fn a_stale_overflow_identity_does_not_retain_an_address_replacement() -> Result<()> {
    for counted in [false, true] {
        let gc = GarbageCollector::with_config(
            Configuration::builder()
                .concurrent_publication(counted)
                .allocation_threshold(usize::MAX)
                .build(),
        );
        let old = Gc::try_new(&gc, 3_u64)?;
        let id = old.identity().to_i64().cast_unsigned();
        drop(old);
        gc.collect_full()?;
        let replacement = Gc::try_new(&gc, 5_u64)?;
        assert_ne!(id, replacement.identity().to_i64().cast_unsigned());
        let weak = crate::GcWeak::new(&replacement, crate::ReferenceKind::Weak, 0);
        gc.remembered
            .dirty
            .publish(id, replacement.as_ptr() as usize, 1);
        drop(replacement);
        generation_tests::finish_young(&gc)?;
        assert!(weak.upgrade().is_none());
        assert!(gc.remembered.dirty.try_remove(id, usize::MAX));
        assert_eq!(gc.managed_bytes(), 0);
    }
    Ok(())
}

#[test]
fn a_clean_unreachable_spilled_owner_is_reclaimed_by_the_second_young_collection() -> Result<()> {
    for counted in [false, true] {
        let gc = GarbageCollector::with_config(
            Configuration::builder()
                .concurrent_publication(counted)
                .promotion_age(4)
                .allocation_threshold(usize::MAX)
                .full_collection_interval(usize::MAX)
                .build(),
        );
        let owner = Gc::try_new(&gc, 53_u64)?;
        let address = owner.as_ptr() as usize;
        let weak = crate::GcWeak::new(&owner, crate::ReferenceKind::Weak, 0);
        {
            let _parked = gc.remembered.lock(address);
            gc.remember(&owner);
        }
        drop(owner);
        generation_tests::finish_young(&gc)?;
        generation_tests::finish_young(&gc)?;
        assert!(weak.upgrade().is_none());
        assert_eq!(gc.managed_bytes(), 0);
    }
    Ok(())
}

#[cfg(not(target_family = "wasm"))]
thread_local! {
    static AFTER_CLAIM: std::cell::Cell<Option<ClaimPause>> = const { std::cell::Cell::new(None) };
}

#[cfg(not(target_family = "wasm"))]
pub(super) fn park_next_claim(pause: ClaimPause) {
    AFTER_CLAIM.set(Some(pause));
}

#[cfg(not(target_family = "wasm"))]
pub(super) fn pause_after_buffer_claim() {
    if let Some(pause) = AFTER_CLAIM.take() {
        pause.entered.send(()).expect("flusher owns record");
        pause
            .resume
            .recv_timeout(std::time::Duration::from_secs(10))
            .expect("resume flusher");
    }
}

#[test]
fn dirty_owner_debt_can_drain_while_the_young_owner_shard_is_parked() -> Result<()> {
    for counted in [false, true] {
        let gc = GarbageCollector::with_config(
            Configuration::builder()
                .concurrent_publication(counted)
                .barrier_buffer_size(4)
                .incremental_step_size(1)
                .max_pause_time_us(0)
                .allocation_threshold(usize::MAX)
                .full_collection_interval(usize::MAX)
                .build(),
        );
        let context = gc.register_context();
        let owner = Gc::try_new(&gc, vec![Gc::try_new(&gc, 41u64)?.into_rooted()])?;
        let address = owner.as_ptr().addr();
        gc.collect_young();
        gc.step();
        assert!(gc.domains[0].active());
        context.with(|| gc.remember(&owner));
        assert!(context.0.try_peek().expect("pending owner").dirty);
        let shard = gc.objects.lock(address);
        assert!(
            gc.try_apply_buffered_record(&context.0),
            "dirty owner waited for object shard"
        );
        assert_eq!(gc.contexts.pending.load(Ordering::Acquire), 0);
        assert!(gc.remembered.lock(address).contains(address));
        drop(shard);
        gc.collect_full()?;
        assert_eq!(**owner.first().expect("live child"), 41);
    }
    Ok(())
}

#[cfg(all(not(target_family = "wasm"), not(miri)))]
fn young_progress_with_remembered_publisher(counted: bool, parked: bool) -> Result<()> {
    use std::sync::mpsc;
    use std::time::Duration;

    let gc = GarbageCollector::with_config(
        Configuration::builder()
            .concurrent_publication(counted)
            .metadata_shards(1)
            .promotion_age(1)
            .incremental_step_size(1)
            .max_pause_time_us(0)
            .allocation_threshold(usize::MAX)
            .full_collection_interval(usize::MAX)
            .build(),
    );
    let owner = Gc::try_new(&gc, crate::cell::RwLock::new(None::<Gc<u64>>))?;
    gc.collect_full()?;
    let address = owner.as_ptr().addr();
    assert!(!gc.is_young_allocation(address));
    let child = Gc::try_new(&gc, 67_u64)?;
    let live = crate::GcWeak::new(&child, crate::ReferenceKind::Weak, 0);
    *owner.write() = Some(child.into_rooted());
    assert!(gc.remembered.lock(address).contains(address));
    let garbage = Gc::try_new(&gc, 91_u64)?;
    let dead = crate::GcWeak::new(&garbage, crate::ReferenceKind::Weak, 0);
    drop(garbage);
    let before = gc.statistics()?.young_collections;
    let shard = parked.then(|| gc.remembered.lock(address));
    let (finished, result) = mpsc::channel();
    let worker_gc = gc.clone();
    let worker = thread::spawn(move || {
        worker_gc.collect_young();
        let completed = (0..4096).any(|_| worker_gc.step());
        finished.send(completed).expect("observing helper");
    });
    let completed = result.recv_timeout(Duration::from_secs(5));
    drop(shard);
    worker.join().expect("young collector helper");
    assert_eq!(completed, Ok(true), "parked={parked}, counted={counted}");
    assert_eq!(gc.statistics()?.young_collections, before + 1);
    assert_eq!(live.upgrade().as_deref(), Some(&67));
    assert!(dead.upgrade().is_none());
    Ok(())
}

#[cfg(all(not(target_family = "wasm"), not(miri)))]
#[test]
fn young_collection_progress_with_parked_legacy_remembered_publisher() -> Result<()> {
    young_progress_with_remembered_publisher(false, false)?;
    young_progress_with_remembered_publisher(false, true)
}

#[cfg(all(not(target_family = "wasm"), not(miri)))]
#[test]
fn young_collection_progress_with_parked_counted_remembered_publisher() -> Result<()> {
    young_progress_with_remembered_publisher(true, false)?;
    young_progress_with_remembered_publisher(true, true)
}
