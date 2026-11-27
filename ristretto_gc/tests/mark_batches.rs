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
