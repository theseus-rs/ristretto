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
            let credit = cache.credit;
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
                assert_eq!(cache.credit, 0);
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
