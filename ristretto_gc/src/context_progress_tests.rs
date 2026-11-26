//! Context contention cannot retain collector coordination or stop unrelated cycles.
#![expect(clippy::panic_in_result_fn, clippy::unwrap_in_result)]
use super::*;
use crate::{Configuration, Gc, GcWeak, ReferenceKind, Result, RootStorage};
use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};

#[test]
fn final_barrier_owner_cleanup_allows_other_collectors_to_finish_cycles() -> Result<()> {
    for concurrent in [false, true] {
        for independent in [false, true] {
            for hold_registry in [false, true] {
                let gc = GarbageCollector::with_config(
                    Configuration::builder()
                        .concurrent_publication(concurrent)
                        .independent_phases(independent)
                        .barrier_buffer_size(4)
                        .incremental_step_size(1)
                        .max_pause_time_us(0)
                        .allocation_threshold(usize::MAX)
                        .build(),
                );
                assert_eq!(gc.registered_workers(), 0);
                let context = gc.register_context();
                let owner = Arc::downgrade(&context.0);
                let root = Gc::try_new(&gc, 42_u64)?;
                gc.collect();
                gc.step();
                context.with(|| gc.remember(&root));
                assert_eq!(gc.contexts.pending.load(Ordering::Acquire), 1);
                drop(context);
                assert_eq!(
                    owner.strong_count(),
                    1,
                    "only the ready registry owns the context"
                );
                let registry = hold_registry.then(|| lock(&gc.contexts.states));
                let statistics = (!hold_registry).then(|| lock(&gc.contexts.retired_roots));
                let worker_gc = gc.clone();
                let (finished, completion) = mpsc::channel();
                let worker = thread::spawn(move || {
                    finished
                        .send(worker_gc.step())
                        .expect("barrier cleanup complete");
                });
                let started = Instant::now();
                while owner.strong_count() != 0 && started.elapsed() < Duration::from_secs(1) {
                    thread::yield_now();
                }
                let entered = owner.strong_count() == 0;
                let waiting = matches!(completion.try_recv(), Err(mpsc::TryRecvError::Empty));
                let pending = gc.contexts.pending.load(Ordering::Acquire);
                let other_gc = gc.clone();
                let (finished, result) = mpsc::channel();
                let other = thread::spawn(move || {
                    for _ in 0..3 {
                        let garbage = Gc::new(&other_gc, 99_u64);
                        let weak = GcWeak::new(&garbage, ReferenceKind::Weak, 0);
                        drop(garbage);
                        other_gc.collect_full().expect("unrelated full cycle");
                        assert!(weak.upgrade().is_none());
                    }
                    finished.send(()).expect("collections complete");
                });
                let progress = result.recv_timeout(Duration::from_secs(1));
                println!(
                    "held barrier cleanup: counted={concurrent} independent={independent} registry={hold_registry}; entered={entered} waiting={waiting} pending={pending} progress={progress:?}"
                );
                drop((registry, statistics));
                worker.join().expect("barrier cleanup worker");
                other.join().expect("unrelated collector");
                assert!(entered && waiting);
                assert_eq!(pending, 0);
                assert_eq!(progress, Ok(()));
                assert_eq!(gc.contexts.statistics().0, 0);
                assert_eq!(**root, 42);
                drop(root);
                gc.stop()?;
                assert_eq!(gc.managed_bytes(), 0);
            }
        }
    }
    Ok(())
}

#[test]
fn barrier_claims_yield_on_held_buffers_and_keep_other_contexts_draining() -> Result<()> {
    for concurrent in [false, true] {
        for independent in [false, true] {
            for hold_ready in [false, true] {
                let gc = GarbageCollector::with_config(
                    Configuration::builder()
                        .concurrent_publication(concurrent)
                        .independent_phases(independent)
                        .root_storage(RootStorage::Contexts)
                        .barrier_buffer_size(8)
                        .incremental_step_size(1)
                        .max_pause_time_us(0)
                        .allocation_threshold(usize::MAX)
                        .build(),
                );
                assert_eq!(gc.registered_workers(), 0);
                let first = gc.register_context();
                let second = gc.register_context();
                let a =
                    first.with(|| Gc::try_new(&gc, vec![Gc::new(&gc, 41_u64).into_rooted()]))?;
                let b =
                    second.with(|| Gc::try_new(&gc, vec![Gc::new(&gc, 42_u64).into_rooted()]))?;
                let weak_a = GcWeak::new(&a, ReferenceKind::Weak, 0);
                let weak_b = GcWeak::new(&b, ReferenceKind::Weak, 0);
                gc.collect();
                gc.step(); // Start the snapshot before deleting either root.
                first.with(|| drop(a));
                second.with(|| drop(b));
                assert_eq!(gc.contexts.pending.load(Ordering::Acquire), 2);
                let buffer = (!hold_ready).then(|| lock(&first.0.buffer));
                let ready = hold_ready.then(|| lock(&gc.contexts.ready));
                let worker_gc = gc.clone();
                let (finished, completion) = mpsc::channel();
                let worker = thread::spawn(move || {
                    for _ in 0..128 {
                        worker_gc.step();
                    }
                    finished.send(()).expect("barrier attempts complete");
                });
                let progress = completion.recv_timeout(Duration::from_secs(1));
                let pending = gc.contexts.pending.load(Ordering::Acquire);
                let traced = gc.statistics()?.objects_marked;
                println!(
                    "held barrier claim: counted={concurrent} independent={independent} ready={hold_ready}; progress={progress:?} pending={pending} traced={traced}"
                );
                drop((buffer, ready));
                worker.join().expect("barrier collector");
                assert_eq!(progress, Ok(()));
                assert_eq!(pending, if hold_ready { 2 } else { 1 });
                assert_eq!(
                    traced,
                    if hold_ready { 0 } else { 2 },
                    "available parent/child tracing must also get turns"
                );
                for _ in 0..10000 {
                    if gc.step() {
                        break;
                    }
                }
                assert!(gc.step());
                assert_eq!(gc.contexts.pending.load(Ordering::Acquire), 0);
                let a = weak_a.upgrade().expect("first deletion graph retained");
                let b = weak_b.upgrade().expect("second deletion graph retained");
                assert_eq!((*a[0], *b[0]), (41, 42));
                drop((a, b, first, second));
                gc.stop()?;
                assert!(weak_a.upgrade().is_none() && weak_b.upgrade().is_none());
                assert_eq!(gc.managed_bytes(), 0);
            }
        }
    }
    Ok(())
}

#[test]
fn held_registry_allows_full_cycles_without_skipping_context_roots() -> Result<()> {
    for concurrent in [false, true] {
        for independent in [false, true] {
            let gc = GarbageCollector::with_config(
                Configuration::builder()
                    .concurrent_publication(concurrent)
                    .independent_phases(independent)
                    .root_storage(RootStorage::Contexts)
                    .incremental_step_size(1)
                    .max_pause_time_us(0)
                    .allocation_threshold(usize::MAX)
                    .build(),
            );
            assert_eq!(gc.registered_workers(), 0);
            let context = gc.register_context();
            let root =
                context.with(|| Gc::try_new(&gc, vec![Gc::new(&gc, 42_u64).into_rooted()]))?;
            let weak = GcWeak::new(&root, ReferenceKind::Weak, 0);
            gc.collect();
            let registry = lock(&gc.contexts.states);
            let worker_gc = gc.clone();
            let (finished, completion) = mpsc::channel();
            let worker = thread::spawn(move || {
                for _ in 0..3 {
                    let garbage = Gc::new(&worker_gc, 99_u64);
                    let weak = GcWeak::new(&garbage, ReferenceKind::Weak, 0);
                    drop(garbage);
                    worker_gc.collect_full().expect("full cycle");
                    assert!(weak.upgrade().is_none());
                }
                finished.send(()).expect("full cycles complete");
            });
            let progress = completion.recv_timeout(Duration::from_secs(1));
            println!(
                "held context registry: counted={concurrent} independent={independent}; progress={progress:?}"
            );
            drop(registry);
            worker.join().expect("root scanner");
            assert_eq!(progress, Ok(()));
            gc.collect_full()?;
            let recovered = weak.upgrade().expect("context graph survived");
            assert_eq!(*recovered[0], 42);
            drop((recovered, root, context));
            gc.stop()?;
            assert!(weak.upgrade().is_none());
            assert_eq!(gc.managed_bytes(), 0);
        }
    }
    Ok(())
}

#[test]
fn final_context_cleanup_can_block_while_other_workers_complete_full_cycles() -> Result<()> {
    for concurrent in [false, true] {
        for independent in [false, true] {
            for hold_registry in [false, true] {
                let gc = GarbageCollector::with_config(
                    Configuration::builder()
                        .concurrent_publication(concurrent)
                        .independent_phases(independent)
                        .root_storage(RootStorage::Contexts)
                        .incremental_step_size(1)
                        .max_pause_time_us(0)
                        .allocation_threshold(usize::MAX)
                        .build(),
                );
                assert_eq!(gc.registered_workers(), 0);
                let context = gc.register_context();
                let owner = Arc::downgrade(&context.0);
                let root = context.with(|| Gc::try_new(&gc, 42_u64))?;
                assert_eq!(owner.strong_count(), 2);
                gc.collect();
                for _ in 0..1000 {
                    gc.step();
                    if owner.strong_count() == 3 {
                        break;
                    }
                }
                assert_eq!(owner.strong_count(), 3, "cursor owns the active context");
                drop((root, context));
                assert_eq!(owner.strong_count(), 1);
                let registry = hold_registry.then(|| lock(&gc.contexts.states));
                let statistics = (!hold_registry).then(|| lock(&gc.contexts.retired_roots));
                let worker_gc = gc.clone();
                let worker_owner = owner.clone();
                let (finished, completion) = mpsc::channel();
                let worker = thread::spawn(move || {
                    for _ in 0..1000 {
                        worker_gc.step();
                        if worker_owner.strong_count() == 0 {
                            break;
                        }
                    }
                    finished.send(()).expect("context cleanup complete");
                });
                let started = Instant::now();
                while owner.strong_count() != 0 && started.elapsed() < Duration::from_secs(1) {
                    thread::yield_now();
                }
                let entered = owner.strong_count() == 0;
                let cleanup_waiting =
                    matches!(completion.try_recv(), Err(mpsc::TryRecvError::Empty));
                let other_gc = gc.clone();
                let (progressed, progress) = mpsc::channel();
                let other = thread::spawn(move || {
                    for _ in 0..3 {
                        let root = Gc::new(&other_gc, 99_u64);
                        let weak = GcWeak::new(&root, ReferenceKind::Weak, 0);
                        drop(root);
                        other_gc.collect_full().expect("unrelated full cycle");
                        assert!(weak.upgrade().is_none());
                    }
                    progressed.send(()).expect("full cycles complete");
                });
                let collected = progress.recv_timeout(Duration::from_secs(1));
                println!(
                    "held context cleanup: counted={concurrent} independent={independent} registry={hold_registry}; entered={entered} cleanup_waiting={cleanup_waiting} progress={collected:?}"
                );
                drop((registry, statistics));
                worker.join().expect("cleanup worker");
                other.join().expect("unrelated collector");
                assert!(entered && cleanup_waiting);
                assert_eq!(collected, Ok(()));
                assert_eq!(gc.contexts.statistics().0, 0);
                assert_eq!(gc.contexts.statistics().1.resident_pages, 0);
                gc.stop()?;
                assert_eq!(gc.root_statistics().resident_pages, 0);
                assert_eq!(gc.managed_bytes(), 0);
            }
        }
    }
    Ok(())
}

#[test]
fn held_allocation_cache_allows_full_cycles_and_keeps_unused_credit_charged() -> Result<()> {
    for concurrent in [false, true] {
        for independent in [false, true] {
            let gc = GarbageCollector::with_config(
                Configuration::builder()
                    .concurrent_publication(concurrent)
                    .independent_phases(independent)
                    .root_storage(RootStorage::Contexts)
                    .local_allocation_buffers(true)
                    .incremental_step_size(1)
                    .max_pause_time_us(0)
                    .allocation_threshold(usize::MAX)
                    .build(),
            );
            assert_eq!(gc.registered_workers(), 0);
            let context = gc.register_context();
            let root = context.with(|| Gc::try_new(&gc, 42_u64))?;
            let before = gc.managed_bytes();
            let cache = lock(&context.0.allocation);
            let credit = context.0.allocation_credit.load(Ordering::Acquire);
            let retained = cache.retained_bytes();
            let counters = (cache.hits, cache.refills);
            assert!(credit > 0);
            assert!(retained > 0);
            let worker_gc = gc.clone();
            let (finished, completion) = mpsc::channel();
            let worker = thread::spawn(move || {
                for _ in 0..3 {
                    let garbage = Gc::new(&worker_gc, 99_u64);
                    let weak = GcWeak::new(&garbage, ReferenceKind::Weak, 0);
                    drop(garbage);
                    worker_gc.collect_full().expect("full cycle");
                    assert!(weak.upgrade().is_none());
                }
                finished.send(()).expect("cycles complete");
            });
            let progress = completion.recv_timeout(Duration::from_secs(1));
            let charged = gc.managed_bytes();
            println!(
                "held context cache: counted={concurrent} independent={independent}; progress={progress:?} reserved_credit={credit} retained={retained} before={before} charged={charged}"
            );
            drop(cache);
            worker.join().expect("collector worker");
            assert_eq!(progress, Ok(()));
            assert_eq!(charged, before);
            gc.collect_full()?;
            assert_eq!(gc.managed_bytes(), before - credit);
            {
                let cache = lock(&context.0.allocation);
                assert_eq!(context.0.allocation_credit.load(Ordering::Acquire), 0);
                assert_eq!(cache.retained_bytes(), 0);
                assert_eq!((cache.hits, cache.refills), counters);
            }
            assert_eq!(**root, 42);
            drop((root, context));
            gc.stop()?;
            assert_eq!(gc.managed_bytes(), 0);
        }
    }
    Ok(())
}

#[test]
fn mutator_root_deletion_bypasses_busy_empty_buffers_and_ready_registry() -> Result<()> {
    for concurrent in [false, true] {
        for independent in [false, true] {
            for storage in [RootStorage::Tree, RootStorage::Slots, RootStorage::Contexts] {
                for hold_ready in [false, true] {
                    let gc = GarbageCollector::with_config(
                        Configuration::builder()
                            .concurrent_publication(concurrent)
                            .independent_phases(independent)
                            .root_storage(storage)
                            .barrier_buffer_size(4)
                            .diagnostics(true)
                            .incremental_step_size(1)
                            .max_pause_time_us(0)
                            .allocation_threshold(usize::MAX)
                            .build(),
                    );
                    assert_eq!(gc.registered_workers(), 0);
                    let context = gc.register_context();
                    let parent = context
                        .with(|| Gc::try_new(&gc, vec![Gc::new(&gc, 42_u64).into_rooted()]))?;
                    let weak = GcWeak::new(&parent, ReferenceKind::Weak, 0);
                    let garbage = Gc::try_new(&gc, 99_u64)?;
                    let garbage_weak = GcWeak::new(&garbage, ReferenceKind::Weak, 0);
                    drop(garbage);
                    gc.collect();
                    assert!(!gc.step());
                    assert_eq!(gc.contexts.pending.load(Ordering::Acquire), 0);
                    let buffer = (!hold_ready).then(|| lock(&context.0.buffer));
                    let ready = hold_ready.then(|| lock(&gc.contexts.ready));
                    let worker_gc = gc.clone();
                    let worker_context = context.clone();
                    let (finished, completion) = mpsc::channel();
                    let worker = thread::spawn(move || {
                        worker_context.with(|| drop(parent));
                        while !worker_gc.step() {}
                        let retained = weak
                            .upgrade()
                            .expect("deletion barrier retains parent graph");
                        assert_eq!(*retained[0], 42);
                        assert!(garbage_weak.upgrade().is_none());
                        drop(retained);
                        for _ in 0..2 {
                            worker_gc.collect_full().expect("unrelated full collection");
                        }
                        assert!(weak.upgrade().is_none());
                        finished
                            .send(())
                            .expect("root deletion and reclamation complete");
                    });
                    let progress = completion.recv_timeout(Duration::from_secs(2));
                    let pending = gc.contexts.pending.load(Ordering::Acquire);
                    let admissions = gc.diagnostics().active_publications;
                    drop((buffer, ready));
                    worker.join().expect("mutator and collector");
                    assert_eq!(
                        progress,
                        Ok(()),
                        "counted={concurrent} independent={independent} storage={storage:?} ready={hold_ready}"
                    );
                    assert_eq!(pending, 0, "busy insertion must not strand a reservation");
                    assert_eq!(admissions, 0);
                    drop(context);
                    gc.stop()?;
                    assert_eq!(gc.managed_bytes(), 0);
                    println!(
                        "mutator root deletion: counted={concurrent} independent={independent} storage={storage:?} ready={hold_ready}; cycles=3 pending=0"
                    );
                }
            }
        }
    }
    Ok(())
}

#[test]
fn mutator_dirty_notification_bypasses_busy_buffer_registration() -> Result<()> {
    for concurrent in [false, true] {
        for independent in [false, true] {
            for hold_ready in [false, true] {
                let gc = GarbageCollector::with_config(
                    Configuration::builder()
                        .concurrent_publication(concurrent)
                        .independent_phases(independent)
                        .root_storage(RootStorage::Contexts)
                        .barrier_buffer_size(4)
                        .promotion_age(2)
                        .full_collection_interval(usize::MAX)
                        .incremental_step_size(1)
                        .max_pause_time_us(0)
                        .allocation_threshold(usize::MAX)
                        .build(),
                );
                let context = gc.register_context();
                let parent = Gc::try_new(&gc, crate::cell::RwLock::new(None::<Gc<u64>>))?;
                gc.collect_full()?;
                gc.collect_full()?;
                let child = Gc::try_new(&gc, 42_u64)?.into_rooted();
                let weak = GcWeak::new(&child, ReferenceKind::Weak, 0);
                gc.collect_young();
                assert!(!gc.step());
                let buffer = (!hold_ready).then(|| lock(&context.0.buffer));
                let ready = hold_ready.then(|| lock(&gc.contexts.ready));
                let worker_gc = gc.clone();
                let worker_context = context.clone();
                let (finished, completion) = mpsc::channel();
                let worker = thread::spawn(move || {
                    worker_context.with(|| *parent.write() = Some(child));
                    while !worker_gc.step() {}
                    worker_gc.collect_young();
                    while !worker_gc.step() {}
                    let child = weak
                        .upgrade()
                        .expect("old-to-young edge survives both snapshots");
                    assert_eq!(*child, 42);
                    assert_eq!(**parent.read().as_ref().expect("stored child"), 42);
                    finished.send(parent).expect("dirty notification complete");
                });
                let progress = completion.recv_timeout(Duration::from_secs(2));
                let pending = gc.contexts.pending.load(Ordering::Acquire);
                drop((buffer, ready));
                worker.join().expect("mutator and young collector");
                assert!(
                    progress.is_ok(),
                    "counted={concurrent} independent={independent} ready={hold_ready}"
                );
                assert_eq!(pending, 0);
                drop((progress, context));
                gc.stop()?;
                assert_eq!(gc.managed_bytes(), 0);
                println!(
                    "mutator dirty notification: counted={concurrent} independent={independent} ready={hold_ready}; young_cycles=2 pending=0"
                );
            }
        }
    }
    Ok(())
}

#[test]
fn a_shared_context_spends_prepaid_bytes_while_its_range_cache_is_parked() -> Result<()> {
    use std::future::Future;
    use std::task::{Context, Poll, Waker};
    for counted in [false, true] {
        for independent in [false, true] {
            let gc = GarbageCollector::with_config(
                Configuration::builder()
                    .concurrent_publication(counted)
                    .independent_phases(independent)
                    .root_storage(RootStorage::Contexts)
                    .max_heap_bytes(crate::allocation::CREDIT)
                    .local_allocation_buffers(true)
                    .allocation_threshold(usize::MAX)
                    .build(),
            );
            let context = gc.register_context();
            let first = context.with(|| Gc::try_new(&gc, 41u64))?;
            assert_eq!(gc.managed_bytes(), crate::allocation::CREDIT);
            let range_cache = lock(&context.0.allocation);
            let peer_context = context.clone();
            let heap = gc.clone();
            let (send, done) = mpsc::channel();
            let peer = thread::spawn(move || {
                let result = peer_context.with(|| -> Result<_> {
                    heap.make_room(8)?;
                    let mut room = std::pin::pin!(heap.make_room_async(8));
                    assert!(matches!(
                        room.as_mut().poll(&mut Context::from_waker(Waker::noop())),
                        Poll::Ready(Ok(()))
                    ));
                    Gc::try_new(&heap, 42u64)
                });
                let _ = send.send(result);
            });
            let result = done.recv_timeout(Duration::from_secs(2));
            drop(range_cache);
            peer.join().expect("shared-context peer");
            let second = result.expect("peer allocated before range-cache release")?;
            assert_eq!((**first, **second), (41, 42));
            assert_eq!(gc.managed_bytes(), crate::allocation::CREDIT);
            drop((first, second, context));
            gc.stop()?;
            assert_eq!(gc.managed_bytes(), 0);
        }
    }
    Ok(())
}

#[test]
fn competing_allocators_spend_each_prepaid_byte_once() -> Result<()> {
    use std::sync::Barrier;
    let gc = GarbageCollector::with_config(
        Configuration::builder()
            .max_heap_bytes(crate::allocation::CREDIT)
            .root_storage(RootStorage::Contexts)
            .local_allocation_buffers(true)
            .allocation_threshold(usize::MAX)
            .build(),
    );
    let context = gc.register_context();
    let pinned = context.with(|| {
        [1024, 1024, 1024, 992]
            .map(|bytes| gc.reserve_bytes(bytes))
            .into_iter()
            .collect::<Result<Vec<_>>>()
    })?;
    assert_eq!(context.0.allocation_credit.load(Ordering::Acquire), 32);
    let range_cache = lock(&context.0.allocation);
    let start = Arc::new(Barrier::new(8));
    let (finished, completion) = mpsc::channel();
    let mut peers = Vec::new();
    for value in 0..8_u64 {
        let (scope, heap, start, finished) =
            (context.clone(), gc.clone(), start.clone(), finished.clone());
        peers.push(thread::spawn(move || {
            start.wait();
            let root = scope.with(|| Gc::try_new(&heap, value));
            let _ = finished.send((value, root));
        }));
    }
    drop(finished);
    let deadline = Instant::now() + Duration::from_secs(3);
    let replies = (0..8)
        .map(|_| completion.recv_timeout(deadline.saturating_duration_since(Instant::now())))
        .collect::<std::result::Result<Vec<_>, _>>();
    drop(range_cache);
    for peer in peers {
        peer.join().expect("allocating peer");
    }
    let mut roots = Vec::new();
    for (value, result) in replies.expect("all peers finish with the range cache parked") {
        match result {
            Ok(root) => {
                assert_eq!(**root, value);
                assert!(
                    roots
                        .iter()
                        .all(|prior: &crate::GcRootGuard<u64>| prior.identity() != root.identity())
                );
                roots.push(root);
            }
            Err(crate::Error::OutOfMemory) => {}
            Err(error) => return Err(error),
        }
    }
    assert_eq!(roots.len(), 4);
    assert_eq!(context.0.allocation_credit.load(Ordering::Acquire), 0);
    assert_eq!(gc.managed_bytes(), crate::allocation::CREDIT);
    drop((roots, pinned, context));
    gc.stop()?;
    assert_eq!(gc.managed_bytes(), 0);
    Ok(())
}

#[test]
fn a_failed_partial_reservation_returns_its_credit_to_the_heap() -> Result<()> {
    let gc = GarbageCollector::with_config(
        Configuration::builder()
            .max_heap_bytes(crate::allocation::CREDIT)
            .local_allocation_buffers(true)
            .allocation_threshold(usize::MAX)
            .build(),
    );
    let context = gc.register_context();
    let pinned = context.with(|| {
        [1024, 1024, 1024, 1020]
            .map(|bytes| gc.reserve_bytes(bytes))
            .into_iter()
            .collect::<Result<Vec<_>>>()
    })?;
    assert_eq!(context.0.allocation_credit.load(Ordering::Acquire), 4);
    let cache = lock(&context.0.allocation);
    let failed = context.with(|| gc.reserve_bytes(8));
    assert!(matches!(failed, Err(crate::Error::OutOfMemory)));
    assert_eq!(context.0.allocation_credit.load(Ordering::Acquire), 0);
    assert_eq!(gc.managed_bytes(), crate::allocation::CREDIT - 4);
    let root = Gc::try_new(&gc, 7_u32)?;
    assert_eq!(**root, 7);
    assert_eq!(gc.managed_bytes(), crate::allocation::CREDIT);
    drop(cache);
    drop((root, pinned, context));
    gc.stop()?;
    assert_eq!(gc.managed_bytes(), 0);
    Ok(())
}

#[test]
fn a_parked_range_cache_allows_an_independent_compacting_payload() -> Result<()> {
    let gc = GarbageCollector::with_config(
        Configuration::builder()
            .compact_heap(true)
            .local_allocation_buffers(true)
            .region_layout(1024, 128)
            .relocation_reserve_bytes(1024)
            .max_heap_bytes(64 * 1024)
            .allocation_threshold(usize::MAX)
            .build(),
    );
    let context = gc.register_context();
    let first = context.with(|| Gc::try_new(&gc, 41u64))?;
    let range_cache = lock(&context.0.allocation);
    let heap = gc.clone();
    let scope = context.clone();
    let (send, done) = mpsc::channel();
    let peer = thread::spawn(move || {
        let result = scope.with(|| crate::HeapBuffer::try_with_fn_in(&heap, 16, |i| Ok(i + 1)));
        let _ = send.send(result);
    });
    let result = done.recv_timeout(Duration::from_secs(2));
    drop(range_cache);
    peer.join().expect("compacting heap peer");
    let payload = result.expect("allocated before the range cache was released")?;
    assert_eq!((**first, payload[0], payload[15]), (41, 1, 16));
    drop((first, payload, context));
    gc.stop()?;
    assert_eq!(gc.managed_bytes(), 0);
    Ok(())
}

#[test]
fn trimming_prepaid_credit_races_shared_allocations_without_refunding_live_bytes() -> Result<()> {
    use std::sync::Barrier;
    // Allow a detached batch awaiting refund and a new batch alongside the live charges.
    let limit = 3 * crate::allocation::CREDIT;
    let gc = GarbageCollector::with_config(
        Configuration::builder()
            .max_heap_bytes(limit)
            .local_allocation_buffers(true)
            .allocation_threshold(usize::MAX)
            .build(),
    );
    let context = gc.register_context();
    let pinned = context.with(|| gc.reserve_bytes(512))?;
    let start = Arc::new(Barrier::new(5));
    let mutators = (0..4)
        .map(|_| {
            let (heap, scope, start) = (gc.clone(), context.clone(), start.clone());
            thread::spawn(move || -> Result<_> {
                start.wait();
                scope.with(|| {
                    for _ in 0..128 {
                        let reservation = heap.reserve_bytes(128)?;
                        assert!((640..=limit).contains(&heap.managed_bytes()));
                        drop(reservation);
                    }
                    Gc::try_new(&heap, 200u64)
                })
            })
        })
        .collect::<Vec<_>>();
    start.wait();
    for _ in 0..128 {
        context.0.try_release_allocations(&gc);
        assert!((512..=limit).contains(&gc.managed_bytes()));
    }
    let roots = mutators
        .into_iter()
        .map(|mutator| mutator.join().expect("reserving mutator"))
        .collect::<Result<Vec<_>>>()?;
    context.0.release_allocations(&gc);
    assert_eq!(gc.managed_bytes(), 512 + 4 * 8);
    assert!(roots.iter().all(|root| ***root == 200));
    drop((roots, pinned, context));
    gc.stop()?;
    assert_eq!(gc.managed_bytes(), 0);
    Ok(())
}

type AllocationPause = (mpsc::Sender<()>, mpsc::Receiver<()>);
thread_local! {
    static AFTER_CHARGE: RefCell<Option<AllocationPause>> = const { RefCell::new(None) };
    static BEFORE_EXACT: RefCell<Option<AllocationPause>> = const { RefCell::new(None) };
}

fn pause_allocation(hook: &'static thread::LocalKey<RefCell<Option<AllocationPause>>>) {
    if let Some((entered, resume)) = hook.take() {
        assert!(entered.send(()).is_ok(), "test driver disconnected");
        assert!(
            resume.recv_timeout(Duration::from_secs(8)).is_ok(),
            "test driver did not resume the allocator"
        );
    }
}

pub(super) fn before_exact_allocation() {
    pause_allocation(&BEFORE_EXACT);
}

#[test]
fn an_allocation_spends_credit_published_before_its_exact_fallback() -> Result<()> {
    let gc = GarbageCollector::with_config(
        Configuration::builder()
            .max_heap_bytes(crate::allocation::CREDIT)
            .local_allocation_buffers(true)
            .allocation_threshold(usize::MAX)
            .build(),
    );
    let context = gc.register_context();
    let cache = lock(&context.0.allocation);
    let (entered, fallback) = mpsc::channel();
    let (resume, permit) = mpsc::channel();
    let heap = gc.clone();
    let scope = context.clone();
    let peer = thread::spawn(move || {
        BEFORE_EXACT.set(Some((entered, permit)));
        scope.with(|| Gc::try_new(&heap, 42u64))
    });
    fallback
        .recv_timeout(Duration::from_secs(2))
        .expect("peer bypassed parked range cache");
    drop(cache);
    let first = context.with(|| Gc::try_new(&gc, 41u64))?;
    assert_eq!(
        context.0.allocation_credit.load(Ordering::Acquire),
        crate::allocation::CREDIT - 8
    );
    resume.send(()).expect("release peer");
    let second = peer.join().expect("peer allocator")?;
    assert_eq!((**first, **second), (41, 42));
    assert_eq!(gc.managed_bytes(), crate::allocation::CREDIT);
    drop((first, second, context));
    gc.stop()?;
    assert_eq!(gc.managed_bytes(), 0);
    Ok(())
}

pub(super) fn after_batch_charge() {
    pause_allocation(&AFTER_CHARGE);
}

struct ParkTrace(mpsc::Sender<()>, Mutex<Option<mpsc::Receiver<()>>>);
// SAFETY: this callback owns channels and contains no managed edges.
#[expect(unsafe_code, reason = "Trace requires an explicit ownership contract")]
unsafe impl crate::Trace for ParkTrace {
    fn trace(&self, _: &GarbageCollector) {
        if let Some(release) = lock(&self.1).take() {
            let _ = self.0.send(());
            let _ = release.recv_timeout(Duration::from_secs(10));
        }
    }
}
struct AllocationNotice(mpsc::Sender<()>);
impl std::task::Wake for AllocationNotice {
    fn wake(self: Arc<Self>) {
        let _ = self.0.send(());
    }
}

#[test]
fn a_shared_context_refill_wakes_allocators_while_the_collector_is_parked() -> Result<()> {
    use std::task::{Context, Poll, Waker};

    for (advisory, steal) in [(false, false), (false, true), (true, false), (true, true)] {
        let (entered, parked) = mpsc::channel();
        let (resume_worker, release) = mpsc::channel();
        let trace = ParkTrace(entered, Mutex::new(Some(release)));
        let cap = 2 * crate::allocation::CREDIT + Gc::allocation_size(&trace)?;
        let gc = GarbageCollector::with_config(
            Configuration::builder()
                .threads(1)
                .max_heap_bytes(cap)
                .local_allocation_buffers(true)
                .allocation_threshold(usize::MAX)
                .build(),
        );
        let owner = Gc::try_new(&gc, trace)?;
        let independent = gc.reserve_exact_bytes(crate::allocation::CREDIT)?;
        gc.start();
        gc.collect();
        parked
            .recv_timeout(Duration::from_secs(2))
            .expect("collector parked in Trace");
        let scope = gc.register_context();
        let heap = gc.clone();
        let refill_context = scope.clone();
        let (entered, charged) = mpsc::channel();
        let (resume_refill, release) = mpsc::channel();
        let refiller = thread::spawn(move || {
            AFTER_CHARGE.set(Some((entered, release)));
            refill_context.with(|| Gc::try_new(&heap, 17u64))
        });
        charged
            .recv_timeout(Duration::from_secs(2))
            .expect("refiller charged the last global bytes");
        assert_eq!(gc.managed_bytes(), cap);
        let (send, notifications) = mpsc::channel();
        let waker = Waker::from(Arc::new(AllocationNotice(send)));
        let mut cx = Context::from_waker(&waker);
        let mut waiter = Box::pin(scope.scope(async {
            if advisory {
                gc.make_room_async(8).await?;
                Gc::try_new(&gc, 42u64)
            } else {
                Gc::try_new_async(&gc, 42u64).await
            }
        }));
        assert!(waiter.as_mut().poll(&mut cx).is_pending());
        notifications
            .recv_timeout(Duration::from_secs(2))
            .expect("initial cooperative yield");
        assert!(waiter.as_mut().poll(&mut cx).is_pending());
        assert!(
            notifications.try_recv().is_err(),
            "waiting on an actual refund"
        );
        let mut thief = None;
        let mut later = None;
        let first = if steal {
            gc.recovery.with_parked_waiter_index(|| -> Result<_> {
                resume_refill.send(()).expect("resume refiller");
                let first = refiller.join().expect("refill thread")?;
                thief = Some(gc.reserve_exact_bytes(crate::allocation::CREDIT - 8)?);
                assert_eq!(gc.allocation_headroom(), 0);
                // Neither the first refund nor the second can dispatch until
                // this closure returns. The same waiter still owns its interest.
                drop(independent);
                later = Some(scope.with(|| gc.reserve_bytes(8))?);
                assert_eq!(gc.allocation_headroom(), crate::allocation::CREDIT - 8);
                Ok(first)
            })?
        } else {
            resume_refill.send(()).expect("resume refiller");
            let first = refiller.join().expect("refill thread")?;
            // Retain the independent charge until the waiter has consumed the
            // first refund. There is no other source of space in this schedule.
            thief = Some(independent);
            first
        };
        let woke = notifications.recv_timeout(Duration::from_secs(2));
        let result = woke.map(|()| waiter.as_mut().poll(&mut cx));
        let ticket = gc.completed_full_ticket();
        drop(waiter);
        drop(resume_worker);
        let Poll::Ready(second) = result.expect("refill must wake the waiting allocator") else {
            panic!("allocation must complete on its refund wake");
        };
        let second = second?;
        assert_eq!((**first, **second, ticket), (17, 42, 0));
        drop((first, second, scope, owner, thief, later));
        gc.stop()?;
        assert_eq!(gc.managed_bytes(), 0);
    }
    Ok(())
}

#[test]
fn space_observers_keep_later_refills_global_until_the_last_cancellation() -> Result<()> {
    let batch = crate::allocation::CREDIT;
    let gc = GarbageCollector::with_config(
        Configuration::builder()
            .max_heap_bytes(2 * batch)
            .local_allocation_buffers(true)
            .allocation_threshold(usize::MAX)
            .build(),
    );
    let scope = gc.register_context();
    let independent = gc.reserve_exact_bytes(batch)?;
    let prepaid = scope.with(|| {
        [1024, 1024, 1024, 1020]
            .into_iter()
            .map(|bytes| gc.reserve_bytes(bytes))
            .collect::<Result<Vec<_>>>()
    })?;
    assert_eq!(scope.0.available_credit(), 4);
    let first = scope.with(|| gc.observe_space(8))?;
    assert_eq!(scope.0.available_credit(), 0);
    let stolen = gc.reserve_exact_bytes(4)?;
    assert!(!first.was_offered());
    let second = scope.with(|| gc.observe_space(8))?;
    thread::scope(|threads| {
        threads.spawn(move || drop(first));
    });
    scope.0.try_release_allocations(&gc);
    drop(independent);
    assert!(second.was_offered());
    let live = scope.with(|| Gc::try_new(&gc, 42u64))?;
    assert_eq!(scope.0.available_credit(), 0);
    assert_eq!(gc.managed_bytes(), batch + 8);
    drop((second, prepaid, stolen));
    assert_eq!(gc.managed_bytes(), 8);
    let next = scope.with(|| Gc::try_new(&gc, 41u64))?;
    assert_eq!(scope.0.available_credit(), batch - 8);
    assert_eq!(gc.managed_bytes(), batch + 8);
    assert_eq!((**live, **next), (42, 41));
    drop((live, next, scope));
    gc.stop()?;
    assert_eq!(gc.managed_bytes(), 0);
    Ok(())
}

#[test]
fn a_pending_allocation_rechecks_credit_when_polled_in_another_context() -> Result<()> {
    use std::task::{Context, Poll, Waker};
    let batch = crate::allocation::CREDIT;
    for advisory in [false, true] {
        let gc = GarbageCollector::with_config(
            Configuration::builder()
                .max_heap_bytes(batch)
                .local_allocation_buffers(true)
                .allocation_threshold(usize::MAX)
                .build(),
        );
        let original = gc.register_context();
        let destination = gc.register_context();
        let first = destination.with(|| Gc::try_new(&gc, 41_u64))?;
        assert_eq!(destination.0.available_credit(), batch - 8);
        // A registered, idle worker makes the future wait for real capacity.
        let worker = crate::recovery::WorkerRegistration::new(&gc);
        let mut future = Box::pin(async {
            if advisory {
                gc.make_room_async(8).await?;
                Gc::try_new(&gc, 42_u64)
            } else {
                Gc::try_new_async(&gc, 42_u64).await
            }
        });
        let mut cx = Context::from_waker(Waker::noop());
        assert!(original.with(|| future.as_mut().poll(&mut cx)).is_pending());
        assert!(original.with(|| future.as_mut().poll(&mut cx)).is_pending());
        assert_eq!(gc.recovery.waiter_count(), 1);
        let migrated = destination.with(|| future.as_mut().poll(&mut cx));
        drop(future);
        drop(worker);
        let Poll::Ready(root) = migrated else {
            panic!("spendable credit in the new context must allow progress");
        };
        let root = root?;
        assert_eq!((**root, **first), (42, 41));
        assert_eq!(gc.recovery.waiter_count(), 0);
        drop((root, first, original, destination));
        gc.stop()?;
        assert_eq!(gc.managed_bytes(), 0);
    }
    Ok(())
}

#[test]
fn pending_interest_tracks_attach_detach_and_cancel_without_a_refund() -> Result<()> {
    use std::task::{Context, Waker};
    let batch = crate::allocation::CREDIT;
    let gc = GarbageCollector::with_config(
        Configuration::builder()
            .max_heap_bytes(batch)
            .local_allocation_buffers(true)
            .allocation_threshold(usize::MAX)
            .build(),
    );
    let full = gc.reserve_exact_bytes(batch)?;
    let worker = crate::recovery::WorkerRegistration::new(&gc);
    let a = gc.register_context();
    let b = gc.register_context();
    let mut future = Box::pin(Gc::try_new_async(&gc, 42u64));
    let mut cx = Context::from_waker(Waker::noop());
    assert!(a.with(|| future.as_mut().poll(&mut cx)).is_pending());
    assert!(a.with(|| future.as_mut().poll(&mut cx)).is_pending());
    let counts = || {
        [
            a.0.allocation_credit.load(Ordering::Acquire),
            b.0.allocation_credit.load(Ordering::Acquire),
        ]
    };
    assert_eq!(counts(), [OBSERVER_UNIT, 0]);
    assert!(b.with(|| future.as_mut().poll(&mut cx)).is_pending());
    assert_eq!(counts(), [0, OBSERVER_UNIT]);
    assert!(future.as_mut().poll(&mut cx).is_pending());
    assert_eq!(counts(), [0, 0]);
    assert!(b.with(|| future.as_mut().poll(&mut cx)).is_pending());
    assert_eq!(counts(), [0, OBSERVER_UNIT]);
    drop(future);
    assert_eq!(counts(), [0, 0]);
    assert_eq!(gc.recovery.waiter_count(), 0);
    assert_eq!(gc.managed_bytes(), batch);
    drop((full, worker, a, b));
    gc.stop()?;
    assert_eq!(gc.managed_bytes(), 0);
    Ok(())
}
