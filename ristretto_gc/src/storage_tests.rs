//! Forced lifetime tests use private leases: leases are not an embedding root API.
use super::*;

struct Tracked {
    drops: Arc<AtomicUsize>,
    clears: Arc<AtomicUsize>,
}
// SAFETY: this fixture contains no outgoing GC references.
unsafe impl Trace for Tracked {
    fn trace(&self, _gc: &GarbageCollector) {}
    fn trace_work(&self) -> usize {
        1
    }
    fn clear_step(&mut self, offset: usize, budget: usize) -> TraceProgress {
        assert_eq!(budget, 1);
        self.clears.fetch_add(1, Ordering::Relaxed);
        if offset == 3 {
            TraceProgress::Complete
        } else {
            TraceProgress::Continue(offset + 1)
        }
    }
}
impl Drop for Tracked {
    fn drop(&mut self) {
        self.drops.fetch_add(1, Ordering::Relaxed);
    }
}

fn heap() -> Arc<GarbageCollector> {
    GarbageCollector::with_config(
        Configuration::builder()
            .diagnostics(true)
            .incremental_step_size(1)
            .max_pause_time_us(0)
            .allocation_threshold(usize::MAX)
            .max_heap_bytes(4096)
            .build(),
    )
}

#[test]
fn a_stalled_storage_lease_does_not_retain_unrelated_garbage_or_allow_weak_upgrade() -> Result<()> {
    let gc = heap();
    let drops = Arc::new(AtomicUsize::new(0));
    let clears = Arc::new(AtomicUsize::new(0));
    let value = Gc::new(
        &gc,
        Tracked {
            drops: drops.clone(),
            clears: clears.clone(),
        },
    );
    let weak = crate::GcWeak::new(&value, crate::ReferenceKind::Weak, 0);
    let metadata = gc
        .objects
        .lock(value.as_ptr() as usize)
        .get(&(value.as_ptr() as usize))
        .unwrap()
        .clone();
    let first = ObjectMetadata::lease(&metadata).unwrap();
    let second = ObjectMetadata::lease(&metadata).unwrap();
    let bytes = metadata.size();
    drop(value);
    gc.collect_full()?;
    assert!(weak.upgrade().is_none());
    assert!(ObjectMetadata::lease(&metadata).is_none());
    assert_eq!(gc.managed_bytes(), bytes);
    assert_eq!(gc.diagnostics().retired_bytes, bytes);
    assert_eq!(gc.diagnostics().retired_objects, 1);
    assert_eq!(gc.diagnostics().retired_ready, 0);
    assert_eq!(clears.load(Ordering::Acquire), 0);
    for _ in 0..32 {
        drop(Gc::new(&gc, 42_u64));
        gc.collect_full()?;
        assert_eq!(gc.managed_bytes(), bytes);
    }
    drop(first);
    assert_eq!(gc.diagnostics().retired_ready, 0);
    drop(second);
    assert_eq!(gc.diagnostics().retired_ready, 1);
    for expected in 1..=4 {
        gc.step();
        assert_eq!(clears.load(Ordering::Acquire), expected);
    }
    assert_eq!(drops.load(Ordering::Acquire), 1);
    assert_eq!(gc.managed_bytes(), 0);
    assert_eq!(gc.diagnostics().retired_bytes, 0);
    assert_eq!(gc.diagnostics().retired_objects, 0);
    assert_eq!(gc.diagnostics().retired_ready, 0);
    drop(metadata);
    assert_eq!(drops.load(Ordering::Acquire), 1);
    Ok(())
}

#[test]
fn the_last_lease_keeps_storage_alive_after_collector_teardown() -> Result<()> {
    let gc = heap();
    let drops = Arc::new(AtomicUsize::new(0));
    let clears = Arc::new(AtomicUsize::new(0));
    let value = Gc::new(
        &gc,
        Tracked {
            drops: drops.clone(),
            clears,
        },
    );
    let metadata = gc
        .objects
        .lock(value.as_ptr() as usize)
        .get(&(value.as_ptr() as usize))
        .unwrap()
        .clone();
    let lease = ObjectMetadata::lease(&metadata).unwrap();
    drop(value);
    gc.collect_full()?;
    drop(metadata);
    drop(gc);
    assert_eq!(drops.load(Ordering::Acquire), 0);
    drop(lease);
    assert_eq!(drops.load(Ordering::Acquire), 1);
    Ok(())
}

#[cfg(panic = "unwind")]
#[test]
fn a_clear_unwind_releases_swept_and_deferred_storage_with_snapshots_alive() -> Result<()> {
    struct FailingClear(Arc<AtomicUsize>, Option<Arc<AtomicUsize>>);
    // SAFETY: both counters are host-owned, with no outgoing managed references.
    unsafe impl Trace for FailingClear {
        fn trace(&self, _: &GarbageCollector) {}
        fn clear_step(&mut self, _: usize, _: usize) -> TraceProgress {
            let count = self.1.take().expect("first clear attempt");
            count.fetch_add(1, Ordering::Relaxed);
            std::panic::panic_any("clear callback failed after modifying the payload");
        }
    }
    impl Drop for FailingClear {
        fn drop(&mut self) {
            assert!(self.1.is_none());
            self.0.fetch_add(1, Ordering::Relaxed);
        }
    }
    for leased in [false, true] {
        let gc = heap();
        let drops = Arc::new(AtomicUsize::new(0));
        let clears = Arc::new(AtomicUsize::new(0));
        let value = Gc::try_new(&gc, FailingClear(drops.clone(), Some(clears.clone())))?;
        let descriptor = gc
            .metadata_for_address(value.as_ptr() as usize)
            .expect("descriptor");
        let lease = leased.then(|| ObjectMetadata::lease(&descriptor).expect("lease"));
        drop(value);
        if leased {
            gc.collect_full()?;
            drop(lease);
        }
        let panic = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| gc.collect_full()));
        assert!(panic.is_err());
        let diagnostics = gc.diagnostics();
        assert_eq!(diagnostics.retired_ready, 0);
        assert_eq!(diagnostics.retired_bytes, 0);
        assert_eq!(gc.managed_bytes(), 0);
        assert_eq!(clears.load(Ordering::Relaxed), 1);
        assert_eq!(drops.load(Ordering::Relaxed), 1);
        drop(Gc::try_new(&gc, 19_u64)?);
        gc.stop()?;
        drop(descriptor);
        assert_eq!(drops.load(Ordering::Relaxed), 1);
        assert_eq!(gc.managed_bytes(), 0);
    }
    Ok(())
}

#[cfg(not(target_family = "wasm"))]
#[test]
fn parked_retirement_reader_allows_other_ready_and_unrelated_storage_to_be_reclaimed() -> Result<()>
{
    use std::sync::mpsc;
    use std::time::Duration;
    let gc = heap();
    let root = Gc::try_new(&gc, 13_u64)?;
    let descriptor = gc
        .metadata_for_address(root.as_ptr() as usize)
        .expect("descriptor");
    let lease = ObjectMetadata::lease(&descriptor).expect("lease");
    drop(root);
    gc.collect_full()?;
    drop(lease);
    drop(Gc::try_new(&gc, 17_u64)?);

    let (entered, parked) = mpsc::channel();
    let (resume, release) = mpsc::channel::<()>();
    let reader_gc = gc.clone();
    let reader = thread::spawn(move || {
        reader_gc
            .retired
            .load_full()
            .expect("ready storage")
            .test_pop_after_head(|| {
                entered.send(()).expect("observer");
                let _ = release.recv();
            })
    });
    let timeout = Duration::from_secs(if cfg!(miri) { 120 } else { 10 });
    parked
        .recv_timeout(timeout)
        .expect("reader parked at queue head");
    let (finished, result) = mpsc::channel();
    let worker_gc = gc.clone();
    let worker = thread::spawn(move || {
        worker_gc.collect_full().expect("collect");
        finished.send(worker_gc.managed_bytes()).expect("observer");
    });
    let freed = result.recv_timeout(timeout);
    drop(resume);
    assert!(reader.join().expect("reader").is_none());
    worker.join().expect("worker");
    assert_eq!(freed.expect("collection while queue reader is parked"), 0);
    assert_eq!(gc.diagnostics().retired_ready, 0);
    assert_eq!(gc.diagnostics().retired_objects, 0);
    gc.stop()?;
    Ok(())
}

#[cfg(not(target_family = "wasm"))]
#[test]
fn last_lease_release_wakes_idle_workers_without_requesting_a_cycle() -> Result<()> {
    let gc = heap();
    let value = Gc::new(&gc, 42_u64);
    let metadata = gc
        .objects
        .lock(value.as_ptr() as usize)
        .get(&(value.as_ptr() as usize))
        .unwrap()
        .clone();
    let lease = ObjectMetadata::lease(&metadata).unwrap();
    drop(value);
    gc.collect_full()?;
    let collections = gc.statistics()?.collections_completed;
    gc.start();
    thread::spawn(move || drop(lease)).join().expect("borrower");
    let until = std::time::Instant::now() + std::time::Duration::from_secs(5);
    while gc.managed_bytes() != 0 && std::time::Instant::now() < until {
        thread::yield_now();
    }
    assert_eq!(gc.managed_bytes(), 0);
    assert_eq!(gc.statistics()?.collections_completed, collections);
    gc.stop()?;
    Ok(())
}

#[test]
fn a_soft_deadline_cannot_prevent_the_first_work_unit() -> Result<()> {
    let gc = GarbageCollector::with_config(
        Configuration::builder()
            .max_pause_time_us(1)
            .incremental_step_size(1)
            .build(),
    );
    drop(Gc::new(&gc, 42_u64));
    gc.collect();
    for _ in 0..128 {
        if gc.step() {
            break;
        }
    }
    assert_eq!(gc.statistics()?.live_objects, 0);
    assert_eq!(gc.statistics()?.full_collections, 1);
    Ok(())
}

#[test]
fn an_in_flight_destructor_alone_is_not_a_worker_claim() -> Result<()> {
    struct Probe {
        collector: Weak<GarbageCollector>,
        claim: Arc<std::sync::OnceLock<Option<bool>>>,
    }
    // SAFETY: both fields are host-owned and contain no managed references.
    unsafe impl Trace for Probe {
        fn trace(&self, _: &GarbageCollector) {}
    }
    impl Drop for Probe {
        fn drop(&mut self) {
            let gc = self.collector.upgrade().expect("active collecting caller");
            // The outer reaper owns this task until Drop returns. The cycle has
            // ended and the queue is empty, just as for a concurrent helper.
            self.claim.set(gc.step_inner(true)).expect("one destructor");
        }
    }
    let gc = heap();
    let claim = Arc::new(std::sync::OnceLock::new());
    let root = Gc::try_new(
        &gc,
        Probe {
            collector: Arc::downgrade(&gc),
            claim: claim.clone(),
        },
    )?;
    let metadata = gc
        .metadata_for_address(root.as_ptr() as usize)
        .expect("descriptor");
    let lease = ObjectMetadata::lease(&metadata).expect("storage lease");
    drop(root);
    gc.collect_full()?;
    assert_eq!(gc.diagnostics().retired_ready, 0);
    drop(lease);
    assert!(gc.step(), "the outer call reclaimed the last queued task");
    assert_eq!(
        claim.get(),
        Some(&None),
        "a callback cannot be claimed twice"
    );
    assert_eq!(
        gc.step_inner(true),
        Some(true),
        "quiescence remains observable"
    );
    assert_eq!(gc.managed_bytes(), 0);
    Ok(())
}
