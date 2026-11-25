//! Required-record reservations outlive claims without retaining global guards.
#![expect(clippy::panic_in_result_fn)]
#![cfg_attr(not(target_family = "wasm"), expect(clippy::unwrap_in_result))]
use super::*;

#[cfg(all(not(target_family = "wasm"), not(miri)))]
#[test]
fn young_start_cannot_pass_a_dirty_owner_prepared_while_inactive() -> Result<()> {
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
            assert_eq!(phase, Phase::Remembered);
            assert_eq!(pending, 1);
            assert_eq!(gc.contexts.pending.load(Ordering::Acquire), 0);
            assert!(gc.remembered.lock(address).contains(&address));
            let epoch = gc.domains[0].epoch.load(Ordering::Acquire);
            assert_eq!(
                gc.objects
                    .lock(address)
                    .get(&address)
                    .expect("old owner")
                    .retention(0, epoch),
                None
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
                "young start after inactive preparation: counted={counted} independent={independent}; scan_waited=true child_retained=true"
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
                assert_eq!(diagnostics.barrier_flushed, 0);
                assert_eq!(
                    (reclaimed, pending, ticket, active),
                    (1, 1, initial_ticket, 0)
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
                assert_eq!(claimed, hold_queue);
                assert_eq!(buffered, !hold_queue);
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
                assert_eq!((pending, ticket), if hold_queue { (0, 1) } else { (1, 0) });
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
fn mutator_overflow_leaves_busy_required_records_for_collector_drain() -> Result<()> {
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
                assert_eq!(still_pending, Some(record));
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
                    "mutator overflow: counted={counted} independent={independent} remembered={hold_remembered}; pending_preserved=true direct_fallback=true"
                );
            }
        }
    }
    Ok(())
}
