//! Independent bounded units and callbacks must not retain global coordination.
#![expect(clippy::panic_in_result_fn)]
#![cfg_attr(not(target_family = "wasm"), expect(clippy::unwrap_in_result))]
use super::*;

#[cfg(all(not(target_family = "wasm"), not(miri)))]
thread_local! {
    static STATISTICS_SNAPSHOT: std::cell::Cell<Option<ClaimPause>> = const { std::cell::Cell::new(None) };
    static AFTER_FULL_REQUEST: std::cell::Cell<Option<ClaimPause>> = const { std::cell::Cell::new(None) };
    static BEFORE_SWEEP_HANDOFF: std::cell::Cell<Option<ClaimPause>> = const { std::cell::Cell::new(None) };
    static BEFORE_SNAPSHOT_COMMIT: std::cell::Cell<Option<ClaimPause>> = const { std::cell::Cell::new(None) };
    static EPOCH_DESTRUCTION: std::cell::Cell<Option<ClaimPause>> = const { std::cell::Cell::new(None) };
    static AFTER_DELETION_SAMPLE: std::cell::Cell<Option<std::sync::mpsc::Sender<()>>> = const { std::cell::Cell::new(None) };
    static AFTER_ACTIVATION: std::cell::Cell<Option<ClaimPause>> = const { std::cell::Cell::new(None) };
}

#[cfg(all(not(target_family = "wasm"), not(miri)))]
pub(super) fn after_statistics_snapshot() {
    pause_once(&STATISTICS_SNAPSHOT);
}

#[cfg(all(not(target_family = "wasm"), not(miri)))]
pub(super) fn before_snapshot_commit() {
    pause_once(&BEFORE_SNAPSHOT_COMMIT);
}

#[cfg(all(not(target_family = "wasm"), not(miri)))]
pub(super) fn after_deletion_sample() {
    if let Some(resume) = AFTER_DELETION_SAMPLE.take() {
        resume.send(()).expect("resume snapshot");
    }
}

#[cfg(all(not(target_family = "wasm"), not(miri)))]
pub(super) fn after_snapshot_activation() {
    pause_once(&AFTER_ACTIVATION);
}

#[cfg(all(not(target_family = "wasm"), not(miri)))]
pub(super) fn after_full_request() {
    pause_once(&AFTER_FULL_REQUEST);
}

#[cfg(all(not(target_family = "wasm"), not(miri)))]
pub(super) fn before_sweep_handoff() {
    pause_once(&BEFORE_SWEEP_HANDOFF);
}

#[cfg(all(not(target_family = "wasm"), not(miri)))]
fn pause_once(point: &'static thread::LocalKey<std::cell::Cell<Option<ClaimPause>>>) {
    if let Some(pause) = point.take() {
        pause.entered.send(()).expect("collection observer");
        pause
            .resume
            .recv_timeout(std::time::Duration::from_secs(10))
            .expect("resume collection");
    }
}

#[cfg(all(not(target_family = "wasm"), not(miri)))]
pub(crate) fn before_epoch_destruction() {
    pause_once(&EPOCH_DESTRUCTION);
}

fn collector(concurrent: bool) -> Arc<GarbageCollector> {
    GarbageCollector::with_config(
        Configuration::builder()
            .concurrent_publication(concurrent)
            .independent_phases(true)
            .overlapping_generations(true)
            .diagnostics(true)
            .compact_heap(true)
            .cost_aware_relocation(true)
            .region_shards(16)
            .incremental_step_size(1)
            .max_pause_time_us(0)
            .allocation_threshold(usize::MAX)
            .full_collection_interval(usize::MAX)
            .promotion_age(100)
            .build(),
    )
}

#[test]
fn a_busy_sole_sweep_skips_the_duplicate_probe_and_preserves_other_domain_turns() -> Result<()> {
    for counted in [false, true] {
        let gc = collector(counted);
        drop(Gc::try_new(&gc, 57_u64)?);
        gc.collect();
        for _ in 0..10000 {
            if lock(&gc.domains[1].cycle).phase == Phase::Sweep {
                break;
            }
            gc.step();
        }
        let guard = lock(&gc.domains[1].cycle);
        assert_eq!(guard.phase, Phase::Sweep);
        let before = gc.next_domain.load(Ordering::Relaxed);
        assert!(gc.step_inner(false).is_none());
        assert_eq!(gc.next_domain.load(Ordering::Relaxed), before);
        gc.collect_young();
        gc.step();
        assert_ne!(gc.next_domain.load(Ordering::Relaxed), before);
        drop(guard);
        for _ in 0..10000 {
            if gc.step() {
                break;
            }
        }
        assert_eq!(gc.managed_bytes(), 0);
        assert!(!gc.pending());
    }
    Ok(())
}

#[test]
fn roots_reference_slots_and_sweep_entries_progress_with_the_coordinator_held() -> Result<()> {
    for concurrent in [false, true] {
        for phase in [
            Phase::Roots,
            Phase::References,
            Phase::Sweep,
            Phase::SelectRegions,
        ] {
            let gc = collector(concurrent);
            let root = Gc::try_new(&gc, 42_u64)?;
            let registration = gc.add_root(&root);
            let weak = crate::GcWeak::new(&root, crate::ReferenceKind::Weak, 0);
            gc.collect();
            for _ in 0..10000 {
                if lock(&gc.domains[1].cycle).phase == phase {
                    break;
                }
                gc.step();
            }
            assert_eq!(lock(&gc.domains[1].cycle).phase, phase);
            let before = gc.diagnostics().independent_phase_steps;
            let coordinator = lock(&gc.coordinator);
            assert!(
                gc.independent_phase_step(false)
                    .is_ok_and(|step| step.is_some())
            );
            assert_eq!(gc.diagnostics().independent_phase_steps, before + 1);
            drop(coordinator);
            while !gc.step() {}
            assert!(weak.upgrade().is_some());
            gc.remove_root_by_id(registration);
            drop(root);
            gc.stop()?;
            assert_eq!(gc.managed_bytes(), 0);
        }
    }
    Ok(())
}

#[cfg(not(target_family = "wasm"))]
mod native {
    use super::*;
    use std::sync::mpsc;
    use std::time::Duration;

    #[test]
    fn completed_cleanup_storage_is_reusable_only_after_the_last_observer_leaves() -> Result<()> {
        let _ = SPARE_FULL_CLEANUP.take();
        let first = collector(true);
        let second = collector(false);
        let (ticket, original) = first.request_full_cleanup();
        let identity = Arc::as_ptr(&original);
        let witness = original.clone();
        first.wait_full_ticket(ticket);
        FullCleanup::recycle(original);
        assert_eq!(witness.load(Ordering::Acquire), 0);
        let (ticket, other) = second.request_full_cleanup();
        assert_ne!(Arc::as_ptr(&other), identity);
        second.wait_full_ticket(ticket);
        FullCleanup::recycle(witness);
        let (ticket, reused) = first.request_full_cleanup();
        assert_eq!(Arc::as_ptr(&reused), identity);
        assert_eq!(reused.load(Ordering::Acquire), 1);
        assert_eq!(other.load(Ordering::Acquire), 0);
        first.wait_full_ticket(ticket);
        assert_eq!(reused.load(Ordering::Acquire), 0);
        first.stop()?;
        second.stop()?;
        Ok(())
    }

    #[cfg(not(miri))]
    #[test]
    fn a_failed_full_start_preserves_both_cleanup_groups() -> Result<()> {
        let gc = GarbageCollector::with_config(
            Configuration::builder()
                .concurrent_publication(true)
                .incremental_step_size(1)
                .max_pause_time_us(0)
                .diagnostics(true)
                .allocation_threshold(usize::MAX)
                .build(),
        );
        let (_, first) = gc.request_full_cleanup();
        let (_, joined) = gc.request_full_cleanup();
        assert!(Arc::ptr_eq(&first, &joined));
        let (before, observed) = mpsc::channel();
        let (release, resume) = mpsc::channel();
        let heap = gc.clone();
        let starter = thread::spawn(move || {
            BEFORE_SNAPSHOT_COMMIT.set(Some(ClaimPause {
                entered: before,
                resume,
            }));
            heap.step();
        });
        observed
            .recv_timeout(Duration::from_secs(5))
            .expect("captured original group");
        let (new_ticket, later) = gc.request_full_cleanup();
        assert!(!Arc::ptr_eq(&first, &later));
        // Change the real counted publication clock after observe and before commit.
        drop(gc.phase_clock.enter());
        release.send(()).expect("attempt stale transition");
        starter.join().expect("starter");
        assert_eq!(gc.statistics()?.collections_started, 0);
        assert!(gc.diagnostics().publication_retries > 0);
        for _ in 0..16384 {
            if first.load(Ordering::Acquire) == 0 {
                break;
            }
            gc.step();
        }
        assert_eq!(first.load(Ordering::Acquire), 0);
        assert!(gc.completed_full_ticket() >= new_ticket);
        assert_eq!(
            later.load(Ordering::Acquire),
            1,
            "the numerical ticket covered two requests"
        );
        for _ in 0..16384 {
            if later.load(Ordering::Acquire) == 0 {
                break;
            }
            gc.step();
        }
        assert_eq!(
            later.load(Ordering::Acquire),
            0,
            "later group lost its full request"
        );
        assert_eq!(gc.statistics()?.full_collections, 2);
        Ok(())
    }

    #[cfg(not(miri))]
    #[test]
    fn explicit_full_publishes_its_cleanup_request_while_the_cycle_lock_is_parked() -> Result<()> {
        let gc = collector(true);
        let cycle = lock(&gc.domains[1].cycle);
        let (issued, observed) = mpsc::channel();
        let (start, resume) = mpsc::channel();
        let heap = gc.clone();
        let owner = thread::spawn(move || {
            AFTER_FULL_REQUEST.set(Some(ClaimPause {
                entered: issued,
                resume,
            }));
            heap.collect_full()
        });
        let published = observed.recv_timeout(Duration::from_secs(1));
        let request_visible = gc.domains[1].requested.load(Ordering::Acquire);
        drop(cycle);
        start.send(()).expect("resume explicit caller");
        owner.join().expect("requesting thread")?;
        assert!(
            published.is_ok() && request_visible,
            "parked cycle blocked request publication"
        );
        Ok(())
    }

    #[cfg(not(miri))]
    #[test]
    fn synchronous_full_waits_for_its_claimed_cleanup_while_another_full_finishes() -> Result<()> {
        for counted in [false, true] {
            for independent in [false, true] {
                let gc = GarbageCollector::with_config(
                    Configuration::builder()
                        .concurrent_publication(counted)
                        .independent_phases(independent)
                        .incremental_step_size(1)
                        .max_pause_time_us(0)
                        .allocation_threshold(usize::MAX)
                        .build(),
                );
                let drops = Arc::new(AtomicUsize::new(0));
                let (_, unused) = mpsc::channel();
                drop(Gc::try_new(
                    &gc,
                    HeldClear {
                        entered: None,
                        release: Mutex::new(unused),
                        drops: drops.clone(),
                    },
                )?);
                let (issued, observed) = mpsc::channel();
                let (start, resume) = mpsc::channel();
                let (done, result) = mpsc::channel();
                let owner = {
                    let heap = gc.clone();
                    let drops = drops.clone();
                    thread::spawn(move || {
                        AFTER_FULL_REQUEST.set(Some(ClaimPause {
                            entered: issued,
                            resume,
                        }));
                        let collected = heap.collect_full();
                        done.send(drops.load(Ordering::Acquire))
                            .expect("owner result");
                        collected
                    })
                };
                observed
                    .recv_timeout(Duration::from_secs(5))
                    .expect("owner issued ticket");
                let ticket = gc.domains[1].request_ticket.load(Ordering::Acquire);
                let (claimed, held) = mpsc::channel();
                let (release, resume) = mpsc::channel();
                let sweeper = {
                    let heap = gc.clone();
                    let drops = drops.clone();
                    thread::spawn(move || {
                        BEFORE_SWEEP_HANDOFF.set(Some(ClaimPause {
                            entered: claimed,
                            resume,
                        }));
                        for _ in 0..65536 {
                            heap.step();
                            if drops.load(Ordering::Acquire) != 0 {
                                return;
                            }
                        }
                        panic!("sweeper did not destroy the test object");
                    })
                };
                held.recv_timeout(Duration::from_secs(5))
                    .expect("private sweep task");
                for _ in 0..65536 {
                    if gc.completed_full_ticket() >= ticket {
                        break;
                    }
                    gc.step();
                }
                assert_eq!(gc.completed_full_ticket(), ticket);
                gc.collect_full()?;
                assert!(gc.completed_full_ticket() > ticket);
                assert_eq!(drops.load(Ordering::Acquire), 0);
                start.send(()).expect("resume owner");
                let early = result.recv_timeout(Duration::from_millis(75));
                release.send(()).expect("publish claimed cleanup");
                let count = early.or_else(|_| result.recv_timeout(Duration::from_secs(5)));
                owner.join().expect("owner")?;
                sweeper.join().expect("sweeper");
                assert_eq!(
                    count.expect("successful collection"),
                    1,
                    "owning full returned before private sweep cleanup"
                );
            }
        }
        Ok(())
    }

    #[cfg(not(miri))]
    #[test]
    fn deleting_a_traced_edge_observes_the_committed_snapshot() -> Result<()> {
        struct Edge(Option<Gc<Vec<Gc<u64>>>>);
        // SAFETY: the single managed field forwards all tracing and rooting.
        unsafe impl Trace for Edge {
            fn root(&mut self) {
                self.0.root();
            }
            fn trace(&self, gc: &GarbageCollector) {
                self.0.trace(gc);
            }
            unsafe fn unroot(&mut self, owner: crate::TraceOwner<'_>) {
                // SAFETY: the exclusive owner controls the optional edge.
                unsafe { Trace::unroot(&mut self.0, owner) };
            }
            fn can_skip_unroot_after_write(&self) -> bool {
                self.0.is_none()
            }
        }
        let mut retained = Vec::new();
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
            let leaf = Gc::try_new(&gc, 17u64)?.into_rooted();
            let leaf_weak = crate::GcWeak::new(&leaf, crate::ReferenceKind::Weak, 0);
            let child = Gc::try_new(&gc, vec![leaf])?.into_rooted();
            let child_weak = crate::GcWeak::new(&child, crate::ReferenceKind::Weak, 0);
            let parent = Gc::try_new(&gc, crate::cell::RwLock::new(Edge(Some(child))))?;
            let context = gc.register_context();
            let ticket = gc.request_full_cycle();
            let (entered, parked) = mpsc::channel();
            let (release, resume) = mpsc::channel();
            let heap = gc.clone();
            let starter = thread::spawn(move || {
                AFTER_ACTIVATION.set(Some(ClaimPause { entered, resume }));
                heap.step();
            });
            parked
                .recv_timeout(Duration::from_secs(5))
                .expect("full snapshot activated");
            context.with(|| {
                let mut guard = parent.write();
                // SAFETY: the original edge is dropped in place while holding the owner guard.
                let edge = unsafe { guard.unrooted_mut() };
                assert!(
                    edge.0
                        .as_ref()
                        .is_some_and(|child| child.traced_metadata().is_some())
                );
                // Resume the starter only after the deletion samples the marking state.
                // The original publication lock can then admit the barrier.
                AFTER_DELETION_SAMPLE.set(Some(release));
                edge.0 = None;
            });
            starter.join().expect("snapshot starter");
            for _ in 0..2048 {
                if gc.completed_full_ticket() >= ticket {
                    break;
                }
                gc.step();
            }
            assert!(gc.completed_full_ticket() >= ticket);
            retained.push((
                counted,
                child_weak.upgrade().as_ref().is_some_and(|child| {
                    child
                        .first()
                        .is_some_and(|leaf| leaf_weak.refers_to(leaf) && **leaf == 17)
                }),
            ));
            gc.collect_full()?;
            assert!(child_weak.upgrade().is_none() && leaf_weak.upgrade().is_none());
            drop(parent);
            gc.stop()?;
            assert_eq!(gc.managed_bytes(), 0);
        }
        assert_eq!(retained, [(false, true), (true, true)]);
        Ok(())
    }

    #[cfg(not(miri))]
    #[test]
    fn a_request_after_snapshot_activation_waits_for_a_fresh_full_collection() -> Result<()> {
        use std::future::Future;
        use std::task::{Context as PollContext, Waker};

        for counted in [true, false] {
            for independent in [false, true] {
                let gc = GarbageCollector::with_config(
                    Configuration::builder()
                        .concurrent_publication(counted)
                        .independent_phases(independent)
                        .incremental_step_size(1)
                        .max_pause_time_us(0)
                        .allocation_threshold(usize::MAX)
                        .full_collection_interval(usize::MAX)
                        .build(),
                );
                let mut root = Some(Gc::try_new(&gc, 71_u64)?);
                let address = root.as_ref().unwrap().as_ptr().addr();
                let metadata = gc.objects.lookup(address).unwrap();
                let first = gc.request_full_cycle();
                let (entered, observed) = mpsc::channel();
                let (resume, resumed) = mpsc::channel();
                let heap = gc.clone();
                let starter = thread::spawn(move || {
                    AFTER_ACTIVATION.set(Some(ClaimPause {
                        entered,
                        resume: resumed,
                    }));
                    heap.step();
                });
                observed
                    .recv_timeout(Duration::from_secs(5))
                    .expect("first full snapshot");
                // Counted publication admits this deletion while the starter is parked.
                // The gated collector admits it once the starter releases the writer.
                if counted {
                    assert!(gc.domains[1].marking());
                    drop(root.take());
                }
                let mut request = Box::pin(gc.collect_full_async());
                let mut cx = PollContext::from_waker(Waker::noop());
                assert!(request.as_mut().poll(&mut cx).is_pending());
                let second = gc.domains[1].request_ticket.load(Ordering::Acquire);
                assert!(second > first);
                resume.send(()).expect("resume startup");
                starter.join().expect("starter");
                drop(root);
                for _ in 0..4096 {
                    if gc.completed_full_ticket() >= first {
                        break;
                    }
                    gc.step();
                }
                assert_eq!(
                    gc.completed_full_ticket(),
                    first,
                    "an active snapshot acknowledged a later request"
                );
                assert_eq!(gc.objects.lookup(address).unwrap().id, metadata.id);
                let mut finished = false;
                for _ in 0..4096 {
                    if let std::task::Poll::Ready(result) = request.as_mut().poll(&mut cx) {
                        result?;
                        finished = true;
                        break;
                    }
                }
                assert!(finished, "later explicit collection did not complete");
                assert_eq!(gc.completed_full_ticket(), second);
                assert!(gc.objects.lookup(address).is_none());
                assert_eq!(gc.managed_bytes(), 0);
            }
        }
        Ok(())
    }

    #[test]
    fn a_compact_ticket_finishes_while_the_trim_registry_is_parked() -> Result<()> {
        for counted in [false, true] {
            let gc = GarbageCollector::with_config(
                Configuration::builder()
                    .concurrent_publication(counted)
                    .compact_heap(true)
                    .region_shards(2)
                    .incremental_step_size(1)
                    .max_pause_time_us(0)
                    .allocation_threshold(usize::MAX)
                    .build(),
            );
            let root = Gc::try_new(&gc, 42_u64)?;
            gc.collect();
            for _ in 0..10_000 {
                if lock(&gc.domains[1].cycle).phase == Phase::Compact {
                    break;
                }
                gc.step();
            }
            assert_eq!(lock(&gc.domains[1].cycle).phase, Phase::Compact);
            thread::scope(|scope| {
                let (done, recv) = mpsc::channel();
                let peer = &gc;
                gc.region_heap()
                    .expect("payload arena")
                    .with_parked_registry(|| {
                        scope.spawn(move || {
                            for _ in 0..64 {
                                peer.coordinated_step(false);
                            }
                            done.send(()).expect("completed polls");
                        });
                        recv.recv_timeout(Duration::from_secs(3))
                            .expect("a trim poll waited for the parked region");
                        assert_eq!(lock(&gc.domains[1].cycle).phase, Phase::Idle);
                        let id = gc.add_root(&root);
                        gc.write_barrier(&root);
                        gc.remove_root_by_id(id);
                    });
            });
            gc.collect_full()?;
            assert_eq!(**root, 42);
            drop(root);
            gc.collect_full()?;
            assert_eq!(gc.managed_bytes(), 0);
        }
        Ok(())
    }

    #[cfg(not(miri))]
    #[test]
    fn paused_statistics_reader_allows_collection_and_capped_allocation() -> Result<()> {
        use std::future::Future;
        use std::task::{Context as PollContext, Poll, Waker};
        let size = Gc::allocation_size(&41_u64)?;
        let mut blocked = Vec::new();
        for counted in [false, true] {
            for diagnostics in [false, true] {
                let gc = GarbageCollector::with_config(
                    Configuration::builder()
                        .threads(0)
                        .concurrent_publication(counted)
                        .independent_phases(true)
                        .independent_marking(true)
                        .diagnostics(diagnostics)
                        .allocation_threshold(usize::MAX)
                        .max_heap_bytes(2 * size)
                        .incremental_step_size(2)
                        .build(),
                );
                let survivor = Gc::try_new(&gc, 41_u64)?;
                let donor = Gc::try_new(&gc, 7_u64)?;
                let weak = crate::GcWeak::new(&donor, crate::ReferenceKind::Weak, 0);
                drop(donor);
                let before = gc.statistics()?;
                assert_eq!(gc.managed_bytes(), 2 * size);
                let (entered, parked) = mpsc::channel();
                let (resume, resumed) = mpsc::channel();
                let (done, result) = mpsc::channel();
                let (while_parked, peer, held) = thread::scope(|scope| {
                    let heap = &gc;
                    let reader = scope.spawn(move || {
                        STATISTICS_SNAPSHOT.set(Some(ClaimPause {
                            entered,
                            resume: resumed,
                        }));
                        heap.statistics()
                    });
                    parked
                        .recv_timeout(Duration::from_secs(3))
                        .expect("statistics snapshot");
                    let collector = scope.spawn(move || {
                        let ticket = heap.request_full_cycle();
                        let mut allocation = Box::pin(Gc::try_new_async(heap, 23_u64));
                        let mut cx = PollContext::from_waker(Waker::noop());
                        let mut object = None;
                        for _ in 0..4096 {
                            if object.is_none()
                                && let Poll::Ready(result) = allocation.as_mut().poll(&mut cx)
                            {
                                object = Some(result.expect("refunded donor"));
                            }
                            if object.is_some() && heap.completed_full_ticket() >= ticket {
                                break;
                            }
                            heap.step();
                        }
                        let finished = heap.completed_full_ticket() >= ticket && object.is_some();
                        done.send(finished).expect("progress observer");
                        object
                    });
                    let while_parked = result.recv_timeout(Duration::from_secs(2)).unwrap_or(false);
                    resume.send(()).expect("resume snapshot reader");
                    (
                        while_parked,
                        collector.join().unwrap(),
                        reader.join().unwrap(),
                    )
                });
                if !while_parked {
                    blocked.push((counted, diagnostics));
                }
                let held = held?;
                assert_eq!(held.full_collections, before.full_collections);
                assert_eq!(held.managed_bytes_freed, before.managed_bytes_freed);
                let after = gc.statistics()?;
                assert!(after.full_collections > before.full_collections);
                assert_eq!(after.managed_bytes_freed - before.managed_bytes_freed, size);
                assert!(weak.upgrade().is_none());
                assert_eq!((**survivor, **peer.expect("replacement")), (41, 23));
            }
        }
        assert!(
            blocked.is_empty(),
            "statistics reader blocked collection or recovery: {blocked:?}"
        );
        Ok(())
    }

    #[test]
    fn phase_statistics_do_not_park_unrelated_root_and_edge_publication() -> Result<()> {
        for starting in [true, false] {
            let gc = GarbageCollector::with_config(
                Configuration::builder()
                    .incremental_step_size(1)
                    .allocation_threshold(usize::MAX)
                    .build(),
            );
            let root = Gc::try_new(&gc, 42_u64)?;
            gc.collect();
            if !starting {
                for _ in 0..10_000 {
                    if lock(&gc.domains[1].cycle).phase == Phase::Compact {
                        break;
                    }
                    gc.step();
                }
                assert_eq!(lock(&gc.domains[1].cycle).phase, Phase::Compact);
            }
            thread::scope(|scope| {
                let stats = gc.statistics.load();
                scope.spawn(|| gc.coordinated_step(false));
                let waiting = if starting {
                    &gc.domains[1].requested
                } else {
                    &gc.domains[1].active
                };
                let deadline = std::time::Instant::now() + Duration::from_secs(3);
                while waiting.load(Ordering::Acquire) && std::time::Instant::now() < deadline {
                    thread::yield_now();
                }
                let transitioned = !waiting.load(Ordering::Acquire);
                let (done, recv) = mpsc::channel();
                let (collector, target) = (&gc, &root);
                scope.spawn(move || {
                    let id = collector.add_root(target);
                    collector.write_barrier(target);
                    collector.remove_root_by_id(id);
                    done.send(()).unwrap();
                });
                let progressed = recv.recv_timeout(Duration::from_secs(2)).is_ok();
                drop(stats);
                assert!(transitioned);
                assert!(
                    progressed,
                    "publication stalled at a phase statistics update"
                );
            });
            gc.collect_full()?;
            assert_eq!(**root, 42);
        }
        Ok(())
    }

    #[test]
    fn an_admitted_publisher_does_not_park_collection_or_other_publishers() -> Result<()> {
        for independent in [false, true] {
            for phase in [
                Phase::Idle,
                Phase::Mark,
                Phase::References,
                Phase::Sweep,
                Phase::Compact,
            ] {
                held_publication_reader(independent, phase)?;
            }
        }
        Ok(())
    }

    fn held_publication_reader(independent: bool, phase: Phase) -> Result<()> {
        let gc = GarbageCollector::with_config(
            Configuration::builder()
                .independent_phases(independent)
                .diagnostics(true)
                .incremental_step_size(1)
                .max_pause_time_us(0)
                .allocation_threshold(usize::MAX)
                .build(),
        );
        let root = Gc::try_new(&gc, 42_u64)?;
        gc.collect();
        for _ in 0..1000 {
            if lock(&gc.domains[1].cycle).phase == phase
                && gc.domains[1].mark_queue.statistics().0 == 0
            {
                break;
            }
            gc.step();
        }
        assert_eq!(lock(&gc.domains[1].cycle).phase, phase);
        let admitted = gc.allocation_guard();
        let worker_gc = gc.clone();
        let (finished, observed) = mpsc::channel();
        let worker = thread::spawn(move || {
            finished.send(worker_gc.step()).expect("step observer");
        });
        let yielded = observed.recv_timeout(Duration::from_secs(1));
        let coordinator_free = gc.coordinator.try_lock().is_ok();
        let (notifier, publication) = mpsc::channel();
        let publisher_gc = gc.clone();
        let publisher_root = root.clone();
        let publisher = thread::spawn(move || {
            let id = publisher_gc.add_root(&publisher_root);
            publisher_gc.remove_root_by_id(id);
            notifier.send(()).expect("publication observer");
        });
        let progress = publication.recv_timeout(Duration::from_secs(1));
        println!(
            "held publication reader: independent={independent} phase={phase:?}; yielded={} coordinator_free={coordinator_free} published={}",
            yielded.is_ok(),
            progress.is_ok()
        );
        drop(admitted);
        worker.join().expect("collector worker");
        publisher.join().expect("root publisher");
        assert_eq!(yielded, Ok(false), "collection waited for its writer");
        assert!(coordinator_free);
        progress.expect("queued writer blocked another publisher");
        assert_eq!(lock(&gc.domains[1].cycle).phase, phase);
        assert_eq!(gc.diagnostics().publication_write_wait.count, 0);
        gc.collect_full()?;
        assert_eq!(**root, 42);
        drop(root);
        gc.stop()?;
        assert_eq!(gc.managed_bytes(), 0);
        Ok(())
    }

    #[test]
    fn held_mark_queues_yield_coordinator_and_publication_without_losing_work() -> Result<()> {
        for concurrent in [false, true] {
            for independent in [false, true] {
                for queues in [2, 0] {
                    for empty in [false, true] {
                        held_mark_queue(concurrent, independent, queues, empty, Phase::Mark)?;
                    }
                    for phase in [Phase::References, Phase::Sweep, Phase::Compact] {
                        held_mark_queue(concurrent, independent, queues, true, phase)?;
                    }
                }
            }
        }
        Ok(())
    }

    fn held_mark_queue(
        concurrent: bool,
        independent: bool,
        queues: usize,
        empty: bool,
        phase: Phase,
    ) -> Result<()> {
        let gc = GarbageCollector::with_config(
            Configuration::builder()
                .concurrent_publication(concurrent)
                .independent_phases(independent)
                .work_stealing(queues != 0)
                .threads(queues)
                .diagnostics(true)
                .incremental_step_size(1)
                .max_pause_time_us(0)
                .allocation_threshold(usize::MAX)
                .build(),
        );
        // with_config does not start workers. The test owns every collector step.
        assert_eq!(gc.registered_workers(), 0);
        let child = Gc::try_new(&gc, 42_u64)?.into_rooted();
        let weak = crate::GcWeak::new(&child, crate::ReferenceKind::Weak, 0);
        let root = Gc::try_new(&gc, vec![child])?;
        gc.collect();
        for _ in 0..1000 {
            let ready = {
                let cycle = lock(&gc.domains[1].cycle);
                cycle.phase == phase && (phase != Phase::References || cycle.cursor != 0)
            };
            if ready && (gc.domains[1].mark_queue.statistics().0 == 0) == empty {
                break;
            }
            gc.step();
        }
        assert_eq!(lock(&gc.domains[1].cycle).phase, phase);
        let reference_stage = lock(&gc.domains[1].cycle).reference_stage;
        assert_eq!(gc.domains[1].mark_queue.statistics().0 == 0, empty);
        let (entered, observed) = mpsc::channel();
        let (release, released) = mpsc::channel();
        let holder_gc = gc.clone();
        let holder = thread::spawn(move || {
            holder_gc.domains[1].mark_queue.with_held_injector(|| {
                entered.send(()).expect("queue held");
                released
                    .recv_timeout(Duration::from_secs(10))
                    .expect("release held queue");
            });
        });
        observed
            .recv_timeout(Duration::from_secs(10))
            .expect("held queue observed");
        let (finished, step_result) = mpsc::channel();
        let step_gc = gc.clone();
        let stepper = thread::spawn(move || {
            finished.send(step_gc.step()).expect("step observer");
        });
        let yielded = step_result.recv_timeout(Duration::from_secs(1));
        let coordinator_free = gc.coordinator.try_lock().is_ok();
        let publication_free = gc.publication.try_write().is_some();
        let active = gc
            .publication_clock
            .as_ref()
            .map_or(0, |clock| clock.active());

        let (published, publication_result) = mpsc::channel();
        let mutator_gc = gc.clone();
        let mutator_root = root.clone();
        let mutator = thread::spawn(move || {
            // This already-marked graph needs no new queue item. A held global
            // writer must not prevent its ordinary root registration/removal.
            let id = mutator_gc.add_root(&mutator_root);
            mutator_gc.remove_root_by_id(id);
            published.send(()).expect("publication observer");
        });
        let published = publication_result.recv_timeout(Duration::from_secs(1));
        println!(
            "held queue: counted={concurrent} independent={independent} worker_queues={queues} phase={phase:?} empty={empty}; yielded={} coordinator_free={coordinator_free} publication_free={publication_free} active={active} published={}",
            yielded.is_ok(),
            published.is_ok()
        );
        release.send(()).expect("release stalled queue");
        holder.join().expect("queue holder");
        stepper.join().expect("collector step");
        mutator.join().expect("root publisher");
        let completed = yielded.expect("collector step waited for mark queue");
        if !empty {
            assert!(!completed, "unclaimed work cannot certify completion");
        }
        assert!(coordinator_free && publication_free && active == 0);
        published.expect("queue contention held unrelated root publication");
        if !empty {
            assert_eq!(lock(&gc.domains[1].cycle).phase, phase);
            assert_eq!(lock(&gc.domains[1].cycle).reference_stage, reference_stage);
        }
        gc.collect_full()?;
        assert_eq!(weak.upgrade().as_deref(), Some(&42));
        drop(root);
        gc.stop()?;
        assert!(weak.upgrade().is_none());
        assert_eq!(gc.managed_bytes(), 0);
        Ok(())
    }

    #[test]
    fn coordinated_mark_returns_its_continuation_while_a_peer_holds_phase_locks() -> Result<()> {
        struct Paused {
            entered: mpsc::Sender<()>,
            release: Mutex<mpsc::Receiver<()>>,
            progress: TraceProgress,
            first: AtomicBool,
        }
        // SAFETY: this fixture has no outgoing edges or shutdown callbacks.
        unsafe impl Trace for Paused {
            fn trace(&self, _: &GarbageCollector) {}
            fn trace_step(&self, _: &GarbageCollector, _: usize, _: usize) -> TraceProgress {
                if self.first.swap(false, Ordering::AcqRel) {
                    self.entered.send(()).expect("callback entered");
                    lock(&self.release)
                        .recv_timeout(Duration::from_secs(5))
                        .expect("resume");
                    self.progress
                } else {
                    TraceProgress::Complete
                }
            }
        }
        for counted in [false, true] {
            for independent in [false, true] {
                for progress in [
                    TraceProgress::Complete,
                    TraceProgress::Busy,
                    TraceProgress::Continue(1),
                ] {
                    let gc = GarbageCollector::with_config(
                        Configuration::builder()
                            .concurrent_publication(counted)
                            .independent_phases(independent)
                            .incremental_step_size(1)
                            .allocation_threshold(usize::MAX)
                            .build(),
                    );
                    let (entered, parked) = mpsc::channel();
                    let (release, resume) = mpsc::channel();
                    let root = Gc::try_new(
                        &gc,
                        Paused {
                            entered,
                            release: Mutex::new(resume),
                            progress,
                            first: AtomicBool::new(true),
                        },
                    )?;
                    tests::prepare_full_mark(&gc);
                    thread::scope(|scope| {
                        let (done, observed) = mpsc::channel();
                        let heap = &gc;
                        let worker = scope.spawn(move || {
                            done.send(heap.coordinated_step(false)).expect("observer");
                        });
                        parked
                            .recv_timeout(Duration::from_secs(3))
                            .expect("first slice");
                        let coordinator = (!independent).then(|| lock(&gc.coordinator));
                        let cycle = independent.then(|| lock(&gc.domains[1].cycle));
                        release.send(()).expect("resume callback");
                        let finished = observed.recv_timeout(Duration::from_secs(1));
                        let flight = gc.in_flight();
                        let empty = gc.domains[1].mark_queue.try_is_empty();
                        drop((cycle, coordinator));
                        worker.join().expect("marker");
                        assert_eq!(finished, Ok(Some(false)));
                        assert_eq!(flight, 0);
                        assert_eq!(empty, Some(progress == TraceProgress::Complete));
                    });
                    while !gc.step() {}
                    assert_eq!(gc.statistics()?.objects_marked, 1);
                    drop(root);
                    gc.stop()?;
                    assert_eq!(gc.managed_bytes(), 0);
                }
            }
        }
        Ok(())
    }

    struct HeldClear {
        entered: Option<mpsc::Sender<Option<usize>>>,
        release: Mutex<mpsc::Receiver<()>>,
        drops: Arc<AtomicUsize>,
    }
    // SAFETY: this fixture contains no outgoing GC references.
    unsafe impl Trace for HeldClear {
        fn trace(&self, _: &GarbageCollector) {}
        fn clear_step(&mut self, _: usize, _: usize) -> TraceProgress {
            if let Some(entered) = self.entered.take() {
                entered
                    .send(WORKER.get().map(|(_, index)| index))
                    .expect("callback entered");
                lock(&self.release)
                    .recv_timeout(Duration::from_secs(10))
                    .expect("release callback");
            }
            TraceProgress::Complete
        }
    }
    impl Drop for HeldClear {
        fn drop(&mut self) {
            self.drops.fetch_add(1, Ordering::Relaxed);
        }
    }

    #[test]
    fn independent_root_read_advances_past_a_parked_object_shard() -> Result<()> {
        for concurrent in [false, true] {
            let gc = collector(concurrent);
            let first = Gc::try_new(&gc, 7_u64)?;
            let root = Gc::try_new(&gc, vec![Gc::new(&gc, 42_u64).into_rooted()])?;
            let weak = crate::GcWeak::new(&root, crate::ReferenceKind::Weak, 0);
            gc.collect();
            gc.step(); // Only the earlier scalar root has been consumed.
            assert_eq!(lock(&gc.domains[1].cycle).phase, Phase::Roots);
            let cursor = lock(&gc.domains[1].cycle).cursor;
            let objects = gc.objects.lock(root.as_ptr() as usize);
            let worker_gc = gc.clone();
            let (finished, result) = mpsc::channel();
            let worker = thread::spawn(move || {
                finished
                    .send(worker_gc.independent_phase_step(false).ok().flatten())
                    .expect("root observer");
            });
            let yielded = result.recv_timeout(Duration::from_secs(1));
            let coordinator_free = gc.coordinator.try_lock().is_ok();
            let admission_free = gc.publication_clock.as_ref().map_or_else(
                || gc.publication.try_write().is_some(),
                |clock| clock.active() == 0,
            );
            drop(objects);
            worker.join().expect("root scan");
            assert_eq!(yielded, Ok(Some(false)));
            assert!(coordinator_free && admission_free);
            assert!(lock(&gc.domains[1].cycle).cursor > cursor);
            gc.collect_full()?;
            let recovered = weak.upgrade().expect("scanned graph survived");
            assert_eq!(*recovered[0], 42);
            drop((recovered, first, root));
            gc.stop()?;
            assert!(weak.upgrade().is_none());
            assert_eq!(gc.managed_bytes(), 0);
        }
        Ok(())
    }

    #[test]
    fn registered_workers_back_off_on_a_held_phase_transition() -> Result<()> {
        let gc = GarbageCollector::with_config(
            Configuration::builder()
                .threads(2)
                .diagnostics(true)
                .worker_yield_time_us(50)
                .allocation_threshold(usize::MAX)
                .build(),
        );
        let coordinator = lock(&gc.coordinator);
        gc.start();
        let before = gc.diagnostics();
        let ticket = gc.request_full_cycle();
        let deadline = std::time::Instant::now() + Duration::from_secs(3);
        loop {
            let after = gc.diagnostics();
            if after.coordinator_misses > before.coordinator_misses
                && after.worker_backoffs > before.worker_backoffs
            {
                break;
            }
            assert!(
                std::time::Instant::now() < deadline,
                "worker did not back off after a failed claim"
            );
            thread::yield_now();
        }
        assert!(gc.completed_full_ticket() < ticket);
        drop(coordinator);
        gc.stop()?;
        assert!(gc.completed_full_ticket() >= ticket);
        Ok(())
    }

    #[test]
    fn adaptive_peer_completes_a_fresh_ticket_while_worker_zero_is_parked() -> Result<()> {
        struct ParkOnWake(mpsc::Sender<Option<usize>>, Mutex<mpsc::Receiver<()>>);
        impl std::task::Wake for ParkOnWake {
            fn wake(self: Arc<Self>) {
                self.0
                    .send(WORKER.get().map(|(_, index)| index))
                    .expect("ticket observer");
                lock(&self.1)
                    .recv_timeout(Duration::from_secs(10))
                    .expect("resume worker");
            }
        }
        for counted in [false, true] {
            let gc = GarbageCollector::with_config(
                Configuration::builder()
                    .threads(2)
                    .adaptive_workers(true)
                    .diagnostics(true)
                    .concurrent_publication(counted)
                    .incremental_step_size(1)
                    .allocation_threshold(usize::MAX)
                    .build(),
            );
            let donor = Gc::try_new(&gc, 37_u64)?;
            let (entered, parked) = mpsc::channel();
            let (release, resume) = mpsc::channel();
            let waker = std::task::Waker::from(Arc::new(ParkOnWake(entered, Mutex::new(resume))));
            let mut first = std::pin::pin!(crate::recovery::CollectionWait::new(&gc, 1));
            let callback_worker = gc.pacing.as_ref().unwrap().with_sampling_frozen(|| {
                gc.start();
                assert!(
                    Future::poll(first.as_mut(), &mut std::task::Context::from_waker(&waker))
                        .is_pending()
                );
                assert_eq!(gc.request_full_cycle(), 1);
                parked.recv_timeout(Duration::from_secs(3))
            });
            drop(donor);
            let ticket = gc.request_full_cycle();
            let start = std::time::Instant::now();
            while gc.completed_full_ticket() < ticket && start.elapsed() < Duration::from_secs(3) {
                thread::sleep(Duration::from_millis(1));
            }
            let completed = gc.completed_full_ticket() >= ticket;
            let refunded = gc.managed_bytes() == 0;
            release.send(()).expect("resume callback");
            gc.stop()?;
            assert_eq!(callback_worker.unwrap(), Some(0));
            assert!(completed, "standby failed to complete the full collection");
            assert!(refunded, "standby failed to release the independent donor");
            assert_eq!(gc.managed_bytes(), 0);
        }
        Ok(())
    }

    #[test]
    fn a_blocked_full_clear_allows_young_cycles_and_coordinator_acquisition() -> Result<()> {
        for concurrent in [false, true] {
            let gc = collector(concurrent);
            let drops = Arc::new(AtomicUsize::new(0));
            let (entered_tx, entered_rx) = mpsc::channel();
            let (release_tx, release_rx) = mpsc::channel();
            drop(Gc::try_new(
                &gc,
                HeldClear {
                    entered: Some(entered_tx),
                    release: Mutex::new(release_rx),
                    drops: drops.clone(),
                },
            )?);
            gc.collect();
            for _ in 0..10000 {
                if !lock(&gc.domains[1].cycle).reclaim.is_empty() {
                    break;
                }
                gc.step();
            }
            assert!(!lock(&gc.domains[1].cycle).reclaim.is_empty());
            let worker_gc = gc.clone();
            let worker = thread::spawn(move || worker_gc.step());
            entered_rx
                .recv_timeout(Duration::from_secs(10))
                .expect("held clearing");
            assert!(gc.coordinator.try_lock().is_ok());
            for _ in 0..3 {
                let root = Gc::try_new(&gc, 42_u64)?;
                let weak = crate::GcWeak::new(&root, crate::ReferenceKind::Weak, 0);
                drop(root);
                generation_tests::finish_young(&gc)?;
                assert!(weak.upgrade().is_none());
            }
            assert_eq!(drops.load(Ordering::Relaxed), 0);
            assert!(gc.statistics()?.full_collections > 0);
            assert!(gc.diagnostics().retired_bytes > 0);
            release_tx.send(()).expect("release");
            worker.join().expect("collector callback");
            gc.stop()?;
            assert_eq!(drops.load(Ordering::Relaxed), 1);
            assert_eq!(gc.managed_bytes(), 0);
        }
        Ok(())
    }

    #[test]
    fn a_blocked_retired_clear_does_not_hold_other_ready_storage() -> Result<()> {
        for concurrent in [false, true] {
            for independent in [false, true] {
                let gc = GarbageCollector::with_config(
                    Configuration::builder()
                        .concurrent_publication(concurrent)
                        .independent_phases(independent)
                        .diagnostics(true)
                        .allocation_threshold(usize::MAX)
                        .build(),
                );
                let drops = Arc::new(AtomicUsize::new(0));
                let (entered_tx, entered_rx) = mpsc::channel();
                let (release_tx, release_rx) = mpsc::channel();
                let root = Gc::try_new(
                    &gc,
                    HeldClear {
                        entered: Some(entered_tx),
                        release: Mutex::new(release_rx),
                        drops: drops.clone(),
                    },
                )?;
                let other = Gc::try_new(&gc, 7_u64)?;
                let metadata = gc
                    .metadata_for_address(root.as_ptr() as usize)
                    .expect("metadata");
                let other_metadata = gc
                    .metadata_for_address(other.as_ptr() as usize)
                    .expect("other metadata");
                let lease = ObjectMetadata::lease(&metadata).expect("storage lease");
                let other_lease = ObjectMetadata::lease(&other_metadata).expect("other lease");
                drop((root, other));
                gc.collect_full()?;
                assert_eq!(gc.diagnostics().retired_objects, 2);
                drop(lease);
                let worker_gc = gc.clone();
                let worker = thread::spawn(move || worker_gc.reap_retired(false, None));
                entered_rx
                    .recv_timeout(Duration::from_secs(10))
                    .expect("held retired callback");
                assert!(gc.coordinator.try_lock().is_ok());
                drop(other_lease);
                assert!(gc.reap_retired(false, None).is_some());
                assert_eq!(gc.diagnostics().retired_bytes, metadata.size());
                // A stalled retired callback must allow new full cycles as well.
                for _ in 0..3 {
                    drop(Gc::try_new(&gc, 11_u64)?);
                    gc.collect_full()?;
                    assert_eq!(gc.managed_bytes(), metadata.size());
                }
                release_tx.send(()).expect("release");
                worker.join().expect("retirement callback");
                gc.stop()?;
                assert_eq!(drops.load(Ordering::Relaxed), 1);
                assert_eq!(gc.managed_bytes(), 0);
                assert_eq!(gc.diagnostics().retired_bytes, 0);
            }
        }
        Ok(())
    }
}

#[test]
fn an_idle_step_can_finish_during_publication_and_preserves_the_next_request() -> Result<()> {
    for counted in [false, true] {
        for kind in [
            CollectionKind::Young,
            CollectionKind::Full,
            CollectionKind::Old,
        ] {
            let gc = GarbageCollector::with_config(
                Configuration::builder()
                    .concurrent_publication(counted)
                    .old_collection_interval(usize::MAX)
                    .full_collection_interval(usize::MAX)
                    .incremental_step_size(1)
                    .build(),
            );
            let publisher = gc.allocation_guard();
            assert!(
                gc.step(),
                "idle with a publisher, {kind:?}, counted={counted}"
            );
            match kind {
                CollectionKind::Young => gc.collect_young(),
                CollectionKind::Full => gc.collect(),
                CollectionKind::Old => gc.collect_old(),
            }
            assert!(!gc.step());
            assert_eq!(gc.statistics()?.collections_completed, 0);
            drop(publisher);
            for _ in 0..2048 {
                if gc.step() {
                    break;
                }
            }
            let stats = gc.statistics()?;
            assert_eq!(
                stats.collections_completed, 1,
                "{kind:?}, counted={counted}"
            );
            assert_eq!(
                stats.young_collections,
                usize::from(kind == CollectionKind::Young)
            );
            assert_eq!(
                stats.full_collections,
                usize::from(kind == CollectionKind::Full)
            );
            assert_eq!(
                stats.old_collections,
                usize::from(kind == CollectionKind::Old)
            );
        }
    }
    Ok(())
}

#[cfg(all(not(target_family = "wasm"), not(miri)))]
#[test]
fn detached_birth_epoch_destruction_allows_the_next_full_collection() -> Result<()> {
    use std::sync::mpsc;
    use std::time::Duration;

    let mut outcomes = Vec::new();
    for (counted, abort_start) in [(false, false), (true, false), (true, true)] {
        let gc = collector(counted);
        gc.collect();
        let (entered, parked) = mpsc::channel();
        let (release, resume) = mpsc::channel();
        let (attempt, starting) = mpsc::channel();
        let (retry, retry_rx) = mpsc::channel();
        let accessible = thread::scope(|scope| {
            let heap = &gc;
            let first = scope.spawn(move || {
                if abort_start {
                    BEFORE_SNAPSHOT_COMMIT.set(Some(ClaimPause {
                        entered: attempt,
                        resume: retry_rx,
                    }));
                }
                EPOCH_DESTRUCTION.set(Some(ClaimPause { entered, resume }));
                for _ in 0..10_000 {
                    if heap.step() {
                        break;
                    }
                }
            });
            if abort_start {
                starting
                    .recv_timeout(Duration::from_secs(5))
                    .expect("opening the epoch");
                drop(gc.phase_clock.enter());
                retry.send(()).expect("attempt stale phase stamp");
            }
            parked
                .recv_timeout(Duration::from_secs(5))
                .expect("last epoch owner released");
            let accessible = (
                gc.publication.try_read().is_some(),
                gc.coordinator.try_lock().is_ok(),
                gc.domains[1].cycle.try_lock().is_ok(),
            );
            if accessible == (true, true, true) {
                let before = gc.statistics()?.full_collections;
                let root = Gc::try_new(&gc, 37_u64)?;
                gc.collect_full()?;
                assert!(gc.statistics()?.full_collections > before);
                assert_eq!(**root, 37);
            }
            release.send(()).expect("finish epoch destruction");
            first.join().expect("first collector step");
            Ok::<_, crate::Error>(accessible)
        })?;
        outcomes.push(accessible);
        gc.collect_full()?;
        assert_eq!(gc.managed_bytes(), 0);
    }
    assert_eq!(
        outcomes,
        [(true, true, true); 3],
        "(publication, coordinator, full cursor) at final epoch release"
    );
    Ok(())
}
