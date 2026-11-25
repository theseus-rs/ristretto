use super::*;

struct Probe(AtomicUsize);
// SAFETY: this fixture contains no outgoing GC references.
unsafe impl Trace for Probe {
    fn trace(&self, _: &GarbageCollector) {}
    fn external_size(&self) -> usize {
        8192
    }
    fn compact_guarded(&self, _: &crate::region::RegionHeap, bytes: usize) -> TraceProgress {
        assert_ne!(bytes, 0, "three chunks cannot exhaust the retry limit");
        let previous = self.0.fetch_add(1, Ordering::AcqRel);
        if previous < 2 {
            TraceProgress::Continue(previous + 1)
        } else {
            TraceProgress::Complete
        }
    }
}

fn collector(step: usize) -> Arc<GarbageCollector> {
    GarbageCollector::with_config(
        Configuration::builder()
            .independent_relocation(true)
            .work_stealing(true)
            .threads(4)
            .compact_heap(true)
            .diagnostics(true)
            .incremental_step_size(step)
            .max_pause_time_us(0)
            .allocation_threshold(usize::MAX)
            .build(),
    )
}

fn prepare(gc: &GarbageCollector) {
    gc.collect();
    for _ in 0..1024 {
        gc.coordinated_step(false);
        if gc.domains[1].relocations.as_ref().unwrap().len() != 0 {
            return;
        }
    }
    panic!("relocation task was not enumerated");
}

#[test]
fn relocation_continuations_progress_with_a_held_coordinator_and_abandoned_worker_queue()
-> Result<()> {
    let gc = collector(1);
    let root = Gc::new(&gc, Probe(AtomicUsize::new(0)));
    {
        let _worker = WorkerContext::enter(&gc, 2);
        prepare(&gc);
    }
    {
        let _coordinator = lock(&gc.coordinator);
        assert!(!gc.step());
        assert_eq!(root.0.load(Ordering::Acquire), 1);
        assert!(!gc.step());
        assert_eq!(root.0.load(Ordering::Acquire), 2);
        assert!(!gc.step());
        assert_eq!(root.0.load(Ordering::Acquire), 3);
    }
    assert_eq!(gc.diagnostics().independent_relocation_steps, 3);
    assert_eq!(gc.diagnostics().in_flight_work, [0, 0]);
    assert_eq!(gc.diagnostics().queued_relocation_tasks, [0, 0]);
    if !cfg!(target_family = "wasm") {
        assert!(gc.diagnostics().relocation_stolen > 0);
    }
    while !gc.step() {}
    drop(root);
    gc.stop()?;
    assert_eq!(gc.diagnostics().relocation_queue_capacity, [0, 0]);
    assert_eq!(gc.managed_bytes(), 0);
    Ok(())
}

#[test]
fn independent_batches_obey_the_work_budget() -> Result<()> {
    let gc = collector(2);
    let root = Gc::new(&gc, Probe(AtomicUsize::new(0)));
    prepare(&gc);
    {
        let _coordinator = lock(&gc.coordinator);
        assert!(!gc.step());
        assert_eq!(root.0.load(Ordering::Acquire), 2);
        assert_eq!(gc.diagnostics().queued_relocation_tasks, [0, 1]);
        assert_eq!(gc.diagnostics().in_flight_work, [0, 0]);
        assert!(!gc.step());
        assert_eq!(root.0.load(Ordering::Acquire), 3);
    }
    assert_eq!(gc.diagnostics().independent_relocation_steps, 2);
    assert_eq!(gc.diagnostics().relocation_tasks, 3);
    assert_eq!(gc.diagnostics().queued_relocation_tasks, [0, 0]);
    while !gc.step() {}
    drop(root);
    gc.stop()?;
    assert_eq!(gc.managed_bytes(), 0);
    Ok(())
}

#[test]
fn overflow_and_stealing_preserve_each_relocation_continuation() -> Result<()> {
    let gc = collector(1);
    let root = Gc::new(&gc, 42_u64);
    let metadata = gc
        .objects
        .lock(root.as_ptr() as usize)
        .get(&(root.as_ptr() as usize))
        .unwrap()
        .clone();
    let queues = crate::relocation::Queues::new(4);
    for attempts in 0..300 {
        queues.push(
            1,
            crate::relocation::Task {
                metadata: metadata.clone(),
                attempts,
            },
        );
    }
    assert_eq!(queues.len(), 300);
    let mut visited = Vec::new();
    while queues.len() != 0 {
        let (task, stolen, _) = queues.claim(2);
        assert_eq!(stolen, 1);
        visited.push(task.unwrap().attempts);
    }
    visited.sort_unstable();
    assert_eq!(visited, (0..300).collect::<Vec<_>>());
    queues.trim_empty();
    assert_eq!(queues.capacity(), 0);
    drop(root);
    gc.stop()?;
    Ok(())
}

#[cfg(not(target_family = "wasm"))]
#[test]
#[expect(clippy::panic_in_result_fn, clippy::unwrap_in_result)]
fn a_held_relocation_claim_yields_the_coordinator_before_completion() -> Result<()> {
    use std::sync::mpsc;
    use std::time::Duration;
    for concurrent in [false, true] {
        let gc = GarbageCollector::with_config(
            Configuration::builder()
                .concurrent_publication(concurrent)
                .independent_relocation(true)
                .work_stealing(true)
                .threads(2)
                .compact_heap(true)
                .incremental_step_size(1)
                .max_pause_time_us(0)
                .allocation_threshold(usize::MAX)
                .build(),
        );
        let root = Gc::new(&gc, Probe(AtomicUsize::new(0)));
        prepare(&gc);
        // Finish enumeration while its queued callback prevents closure, then
        // drain the callback without running a coordinated terminal check.
        for _ in 0..1000 {
            if lock(&gc.domains[1].cycle).relocation_scan_done {
                break;
            }
            gc.coordinated_step(false);
        }
        assert!(lock(&gc.domains[1].cycle).relocation_scan_done);
        let queues = gc.domains[1]
            .relocations
            .as_ref()
            .expect("relocation queues");
        for _ in 0..1000 {
            if queues.len() == 0 {
                break;
            }
            gc.independent_relocation_step(false);
        }
        assert_eq!(queues.len(), 0);
        assert_eq!(root.0.load(Ordering::Acquire), 3);
        assert_eq!(lock(&gc.domains[1].cycle).phase, Phase::Compact);
        let claims = gc
            .relocation_claims
            .as_ref()
            .expect("relocation claim gate");
        let held = claims.read().expect("unpoisoned claim reader");
        let worker_gc = gc.clone();
        let (finished, observed) = mpsc::channel();
        let worker = thread::spawn(move || {
            finished
                .send(worker_gc.coordinated_step(false))
                .expect("terminal observer");
        });
        let yielded = observed.recv_timeout(Duration::from_secs(1));
        let coordinator_free = gc.coordinator.try_lock().is_ok();
        println!(
            "held relocation claim: counted={concurrent}; yielded={} coordinator_free={coordinator_free}",
            yielded.is_ok()
        );
        drop(held);
        worker.join().expect("terminal worker");
        assert_eq!(yielded, Ok(Some(false)), "terminal claim waited");
        assert!(coordinator_free);
        assert_eq!(lock(&gc.domains[1].cycle).phase, Phase::Compact);
        gc.collect_full()?;
        drop(root);
        gc.stop()?;
        assert_eq!(gc.managed_bytes(), 0);
    }
    Ok(())
}
