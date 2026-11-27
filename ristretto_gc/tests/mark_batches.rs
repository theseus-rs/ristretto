//! Bounded claims preserve resumable work and completion statistics on every host.
#![expect(clippy::panic_in_result_fn)]
use ristretto_gc::{
    Configuration, GarbageCollector, Gc, Result, RootStorage, Trace, TraceProgress,
};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};

struct Resumable {
    offsets: Arc<Mutex<Vec<usize>>>,
    busy: AtomicBool,
}
// SAFETY: this fixture contains no outgoing GC references.
#[expect(unsafe_code, reason = "Trace requires an explicit ownership contract")]
unsafe impl Trace for Resumable {
    fn trace_work(&self) -> usize {
        1
    }
    fn trace(&self, _: &GarbageCollector) {}
    fn trace_step(&self, _: &GarbageCollector, offset: usize, budget: usize) -> TraceProgress {
        assert_eq!(budget, 1);
        self.offsets
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .push(offset);
        if offset == 1 && self.busy.swap(false, Ordering::AcqRel) {
            TraceProgress::Busy
        } else if offset < 2 {
            TraceProgress::Continue(offset + 1)
        } else {
            TraceProgress::Complete
        }
    }
}

#[test]
fn optional_marking_preserves_the_inner_work_hint_and_retries() -> Result<()> {
    let gc = GarbageCollector::with_config(
        Configuration::builder()
            .incremental_step_size(64)
            .allocation_threshold(usize::MAX)
            .build(),
    );
    let offsets = Arc::new(Mutex::new(Vec::new()));
    let root = Gc::try_new(
        &gc,
        Some(Resumable {
            offsets: offsets.clone(),
            busy: AtomicBool::new(true),
        }),
    )?;
    gc.collect_full()?;
    assert_eq!(
        *offsets
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner),
        [0, 1, 1, 2]
    );
    drop(root);
    gc.collect_full()?;
    assert_eq!(gc.managed_bytes(), 0);
    Ok(())
}

#[test]
fn continuations_and_busy_tasks_preserve_offsets_and_step_limits() -> Result<()> {
    for independent in [false, true] {
        for storage in [RootStorage::Tree, RootStorage::Slots] {
            for budget in [1, 3, 64] {
                let gc = GarbageCollector::with_config(
                    Configuration::builder()
                        .root_storage(storage)
                        .metadata_shards(16)
                        .work_stealing(true)
                        .independent_marking(independent)
                        .mark_batch_size(8)
                        .diagnostics(true)
                        .incremental_step_size(budget)
                        .max_pause_time_us(0)
                        .allocation_threshold(usize::MAX)
                        .promotion_age(1)
                        .build(),
                );
                let offsets: Vec<_> = (0..17).map(|_| Arc::new(Mutex::new(Vec::new()))).collect();
                let roots: Vec<_> = offsets
                    .iter()
                    .map(|offsets| {
                        Gc::new(
                            &gc,
                            Resumable {
                                offsets: offsets.clone(),
                                busy: AtomicBool::new(true),
                            },
                        )
                    })
                    .collect();
                gc.collect();
                for _ in 0..1000 {
                    let before = gc.diagnostics().trace_tasks;
                    let done = gc.step();
                    assert!(gc.diagnostics().trace_tasks - before <= budget as u64);
                    if done {
                        break;
                    }
                }
                let stats = gc.statistics()?;
                assert_eq!(stats.full_collections, 1);
                assert_eq!(stats.objects_marked, 17);
                assert_eq!(stats.objects_promoted, 17);
                for offsets in &offsets {
                    assert_eq!(
                        *offsets
                            .lock()
                            .unwrap_or_else(std::sync::PoisonError::into_inner),
                        [0, 1, 1, 2]
                    );
                }
                assert_eq!(gc.diagnostics().trace_busy, 17);
                if budget > 1 {
                    assert!(gc.diagnostics().trace_deferred > 0);
                }
                let bytes = stats.bytes_managed;
                drop(roots);
                gc.collect_full()?;
                let stats = gc.statistics()?;
                assert_eq!(stats.objects_swept, 17);
                assert_eq!(stats.managed_bytes_freed, bytes);
                assert_eq!(stats.live_objects, 0);
            }
        }
    }
    Ok(())
}

struct Leaf(Arc<AtomicUsize>);
// SAFETY: this fixture contains no outgoing GC references.
#[expect(unsafe_code, reason = "Trace requires an explicit ownership contract")]
unsafe impl Trace for Leaf {
    fn trace_work(&self) -> usize {
        1
    }
    fn trace(&self, _: &GarbageCollector) {
        self.0.fetch_add(1, Ordering::Relaxed);
    }
}

#[test]
fn direct_configuration_clamps_claims_and_completion_flushes_counters() -> Result<()> {
    for configured in [0, 1, 8, usize::MAX] {
        let gc = GarbageCollector::with_config(Configuration {
            mark_batch_size: configured,
            diagnostics: true,
            incremental_step_size: 64,
            allocation_threshold: usize::MAX,
            max_pause_time_us: 0,
            ..Configuration::default()
        });
        let calls = Arc::new(AtomicUsize::new(0));
        let roots: Vec<_> = (0..16).map(|_| Gc::new(&gc, Leaf(calls.clone()))).collect();
        gc.collect_full()?;
        assert_eq!(gc.statistics()?.objects_marked, 16);
        assert_eq!(calls.load(Ordering::Relaxed), 16);
        assert_eq!(gc.diagnostics().trace_tasks, 16);
        assert_eq!(
            gc.diagnostics().mark_batches,
            if configured <= 1 { 16 } else { 2 }
        );
        drop(roots);
        gc.stop()?;
        assert_eq!(gc.statistics()?.objects_swept, 16);
    }
    Ok(())
}

#[cfg(not(target_family = "wasm"))]
#[test]
fn time_limit_returns_unexecuted_tasks() -> Result<()> {
    struct SlowFirst(Arc<AtomicBool>);
    // SAFETY: this fixture contains no outgoing GC references.
    #[expect(unsafe_code, reason = "Trace requires an explicit ownership contract")]
    unsafe impl Trace for SlowFirst {
        fn trace_work(&self) -> usize {
            1
        }
        fn trace(&self, _: &GarbageCollector) {
            if self.0.swap(false, Ordering::AcqRel) {
                std::thread::sleep(std::time::Duration::from_millis(20));
            }
        }
    }
    let gc = GarbageCollector::with_config(
        Configuration::builder()
            .mark_batch_size(8)
            .diagnostics(true)
            .incremental_step_size(64)
            .max_pause_time_us(5_000)
            .allocation_threshold(usize::MAX)
            .build(),
    );
    let slow = Arc::new(AtomicBool::new(true));
    let roots: Vec<_> = (0..16)
        .map(|_| Gc::new(&gc, SlowFirst(slow.clone())))
        .collect();
    gc.collect_full()?;
    assert!(gc.diagnostics().trace_deferred > 0);
    assert_eq!(gc.diagnostics().trace_busy, 0);
    assert_eq!(gc.diagnostics().trace_tasks, 16);
    assert_eq!(gc.statistics()?.objects_marked, 16);
    drop(roots);
    gc.stop()?;
    assert_eq!(gc.statistics()?.live_objects, 0);
    Ok(())
}

#[test]
fn optional_vector_visits_only_the_budgeted_edges_in_each_collection_step() -> Result<()> {
    struct Visit(Arc<AtomicUsize>);
    // SAFETY: this counter records visits and contains no managed edges.
    #[expect(unsafe_code, reason = "Trace requires an explicit ownership contract")]
    unsafe impl Trace for Visit {
        fn trace(&self, _: &GarbageCollector) {
            self.0.fetch_add(1, Ordering::Relaxed);
        }
    }

    let gc = GarbageCollector::with_config(
        Configuration::builder()
            .incremental_step_size(1)
            .max_pause_time_us(0)
            .allocation_threshold(usize::MAX)
            .build(),
    );
    let visits = Arc::new(AtomicUsize::new(0));
    let leaf = Gc::try_new(&gc, 73_u64)?.into_rooted();
    let weak = ristretto_gc::GcWeak::new(&leaf, ristretto_gc::ReferenceKind::Weak, 0);
    let len = if cfg!(miri) { 32 } else { 4096 };
    let edges: Vec<_> = (0..len)
        .map(|i| (Visit(visits.clone()), (i + 1 == len).then(|| leaf.clone())))
        .collect();
    let root = Gc::try_new(&gc, Some(edges))?;
    drop(leaf);
    visits.store(0, Ordering::Relaxed);
    gc.collect();
    let mut max_visits = 0;
    for _ in 0..(len * 8 + 512) {
        let before = visits.load(Ordering::Relaxed);
        let done = gc.step();
        max_visits = max_visits.max(visits.load(Ordering::Relaxed) - before);
        if done {
            break;
        }
    }
    assert_eq!(gc.statistics()?.full_collections, 1);
    assert_eq!(visits.load(Ordering::Relaxed), len);
    assert_eq!(
        max_visits, 1,
        "optional edges visited in one budget-one step"
    );
    assert_eq!(weak.upgrade().map(|r| *r), Some(73));
    drop(root);
    gc.collect_full()?;
    assert!(weak.is_cleared());
    assert_eq!(gc.managed_bytes(), 0);
    Ok(())
}

#[cfg(all(not(target_family = "wasm"), not(miri)))]
#[test]
fn optional_locked_payload_returns_to_the_collector_while_the_mutator_is_parked() -> Result<()> {
    use std::sync::mpsc;
    use std::time::Duration;

    let gc = GarbageCollector::with_config(
        Configuration::builder()
            .incremental_step_size(1)
            .max_pause_time_us(0)
            .allocation_threshold(usize::MAX)
            .build(),
    );
    let target = Gc::try_new(&gc, 89_u64)?.into_rooted();
    let weak = ristretto_gc::GcWeak::new(&target, ristretto_gc::ReferenceKind::Weak, 0);
    let root = Gc::try_new(&gc, Some(Mutex::new(vec![target])))?;
    let locked = root
        .as_ref()
        .as_ref()
        .expect("present mutex")
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    gc.collect();
    let (done, finished) = mpsc::channel();
    let returned = std::thread::scope(|scope| {
        scope.spawn(|| {
            for _ in 0..2048 {
                gc.step();
            }
            done.send(()).unwrap();
        });
        let returned = finished.recv_timeout(Duration::from_secs(3)).is_ok();
        drop(locked);
        returned
    });
    assert!(
        returned,
        "a collection step blocked on the mutator's payload lock"
    );
    gc.collect_full()?;
    assert_eq!(weak.upgrade().map(|r| *r), Some(89));
    drop(root);
    gc.collect_full()?;
    // A standard mutex keeps its child independently rooted until the container
    // is cleared. The following cycle can then reclaim that child.
    gc.collect_full()?;
    assert!(weak.is_cleared());
    assert_eq!(gc.managed_bytes(), 0);
    Ok(())
}
