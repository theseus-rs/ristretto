//! Bounded recovery, wakeups, cancellation, and unpublished construction ownership.
#![expect(clippy::panic_in_result_fn, clippy::panic)]
use ristretto_gc::{Configuration, Error, GarbageCollector, Gc, HeapBuffer, Result, Trace};
use std::future::Future;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::task::{Context, Poll, Wake, Waker};

#[derive(Default)]
struct Wakes(AtomicUsize);
impl Wake for Wakes {
    fn wake(self: Arc<Self>) {
        self.wake_by_ref();
    }
    fn wake_by_ref(self: &Arc<Self>) {
        self.0.fetch_add(1, Ordering::Release);
    }
}

fn collector(compact: bool, limit: usize) -> Arc<GarbageCollector> {
    GarbageCollector::with_config(
        Configuration::builder()
            .compact_heap(compact)
            .independent_phases(true)
            .max_heap_bytes(limit)
            .diagnostics(true)
            .allocation_threshold(usize::MAX)
            .incremental_step_size(1)
            .max_pause_time_us(0)
            .build(),
    )
}

fn finish<F: Future>(gc: &GarbageCollector, future: F) -> F::Output {
    let mut future = std::pin::pin!(future);
    let notifications = Arc::new(Wakes::default());
    let waker = Waker::from(notifications.clone());
    let mut context = Context::from_waker(&waker);
    for _ in 0..100_000 {
        let steps = gc.diagnostics().caller_steps.count;
        let result = future.as_mut().poll(&mut context);
        assert!(
            gc.diagnostics().caller_steps.count <= steps + 1,
            "a poll ran more than one GC batch"
        );
        if let Poll::Ready(value) = result {
            return value;
        }
        assert!(
            notifications.0.swap(0, Ordering::AcqRel) > 0,
            "cooperative progress lost its wakeup"
        );
    }
    panic!("bounded test fixture did not finish")
}

struct Counted(Arc<AtomicUsize>);
// SAFETY: this fixture contains no outgoing GC references.
#[expect(unsafe_code, reason = "Trace requires an explicit ownership contract")]
unsafe impl Trace for Counted {
    fn trace(&self, _: &GarbageCollector) {}
    fn external_size(&self) -> usize {
        192
    }
}
impl Drop for Counted {
    fn drop(&mut self) {
        self.0.fetch_add(1, Ordering::Relaxed);
    }
}

#[test]
fn one_attempt_preserves_input_and_async_cancellation_drops_it_once() -> Result<()> {
    let gc = collector(false, 256);
    let _live = Gc::try_new(&gc, vec![0_u8; 160])?;
    let before = gc.managed_bytes();
    let drops = Arc::new(AtomicUsize::new(0));
    let Err((Error::OutOfMemory, data)) = Gc::try_new_once(&gc, Counted(drops.clone())) else {
        panic!("expected an unpublished retry value");
    };
    assert_eq!(gc.statistics()?.collections_started, 0);
    let mut future = Box::pin(Gc::try_new_async(&gc, data));
    let waker = Waker::from(Arc::new(Wakes::default()));
    assert!(
        future
            .as_mut()
            .poll(&mut Context::from_waker(&waker))
            .is_pending()
    );
    assert_eq!(drops.load(Ordering::Relaxed), 0);
    drop(future);
    assert_eq!(drops.load(Ordering::Relaxed), 1);
    assert_eq!(gc.managed_bytes(), before);
    assert_eq!(gc.diagnostics().allocation_waiters, 0);
    assert_eq!(gc.diagnostics().allocation_recovery.count, 0);
    Ok(())
}

#[test]
fn exhaustion_terminates_after_two_async_cycles_without_inline_collection() -> Result<()> {
    let gc = collector(false, 256);
    let live = Gc::try_new(&gc, vec![0_u8; 160])?;
    let drops = Arc::new(AtomicUsize::new(0));
    assert!(matches!(
        finish(&gc, Gc::try_new_async(&gc, Counted(drops.clone()))),
        Err(Error::OutOfMemory)
    ));
    assert_eq!(drops.load(Ordering::Relaxed), 1);
    assert_eq!(gc.diagnostics().async_allocation_recovery.count, 2);
    assert_eq!(gc.diagnostics().allocation_recovery.count, 0);
    assert_eq!(gc.diagnostics().full_collection_calls, 0);
    drop(live);
    let root = finish(&gc, Gc::try_new_async(&gc, 42_u64))?;
    assert_eq!(**root, 42);
    finish(&gc, gc.collect_full_async())?;
    assert_eq!(gc.statistics()?.live_objects, 1);
    Ok(())
}

#[test]
fn unsatisfiable_size_and_managed_guards_do_not_start_recovery() -> Result<()> {
    let gc = collector(false, 256);
    assert!(matches!(
        finish(&gc, gc.make_room_async(257)),
        Err(Error::OutOfMemory)
    ));
    let root = Gc::try_new(&gc, ristretto_gc::sync::RwLock::new(7_u64))?;
    let _guard = root.read();
    assert!(matches!(
        Gc::try_new(&gc, vec![0_u8; 256 - size_of::<Vec<u8>>()]),
        Err(Error::OutOfMemory)
    ));
    gc.make_room(1)?;
    assert!(matches!(gc.make_room(256), Err(Error::OutOfMemory)));
    assert!(matches!(
        finish(&gc, gc.collect_full_async()),
        Err(Error::CollectionPhaseError(_))
    ));
    assert_eq!(gc.statistics()?.collections_started, 0);
    Ok(())
}

#[test]
fn cancelling_initialization_releases_exactly_the_initialized_prefix() {
    for compact in [false, true] {
        let gc = collector(compact, 512 * 1024);
        let drops = Arc::new(AtomicUsize::new(0));
        let initialized = AtomicUsize::new(0);
        let mut future = Box::pin(HeapBuffer::try_with_fn_in_async(&gc, 20_000, |_| {
            initialized.fetch_add(1, Ordering::Relaxed);
            Ok(Counted(drops.clone()))
        }));
        let waker = Waker::from(Arc::new(Wakes::default()));
        assert!(
            future
                .as_mut()
                .poll(&mut Context::from_waker(&waker))
                .is_pending()
        );
        let count = initialized.load(Ordering::Relaxed);
        assert!(count > 0 && count < 20_000);
        assert_eq!(drops.load(Ordering::Relaxed), 0);
        if let Some(heap) = gc.region_heap() {
            assert_eq!(
                heap.statistics().occupied_bytes,
                20_000 * size_of::<Counted>()
            );
        }
        drop(future);
        assert_eq!(drops.load(Ordering::Relaxed), count);
        if let Some(heap) = gc.region_heap() {
            assert_eq!(heap.statistics().occupied_bytes, 0);
        }
        assert_eq!(gc.managed_bytes(), 0);
    }
}

#[test]
fn synchronous_buffer_recovers_logical_space_before_full_cycle_completion() -> Result<()> {
    let gc = collector(false, 256);
    drop(Gc::try_new(&gc, vec![0_u8; 160])?);
    let buffer = HeapBuffer::try_with_fn_in(&gc, 192, |_| Ok(7_u8))?;
    assert_eq!(buffer.as_ref(), &[7_u8; 192][..]);
    assert!(gc.diagnostics().allocation_recovery.count > 0);
    assert_eq!(gc.statistics()?.collections_completed, 0);
    drop(buffer);
    gc.stop()?;
    assert_eq!(gc.managed_bytes(), 0);
    Ok(())
}

#[test]
fn synchronous_buffer_recovers_physical_space_before_full_cycle_completion() -> Result<()> {
    let gc = GarbageCollector::with_config(
        Configuration::builder()
            .compact_heap(true)
            .region_layout(4096, 0)
            .max_heap_bytes(4096)
            .allocation_threshold(usize::MAX)
            .incremental_step_size(1)
            .max_pause_time_us(0)
            .diagnostics(true)
            .build(),
    );
    let heap = gc.region_heap().expect("compact heap");
    let pinned = HeapBuffer::try_with_fn(Some(heap), 3072, |_| Ok(1_u8))?;
    let doomed = HeapBuffer::try_with_fn(Some(heap), 1024, |_| Ok(2_u8))?;
    drop(Gc::try_new(&gc, doomed)?);
    let calls = AtomicUsize::new(0);
    let buffer = HeapBuffer::try_with_fn_in(&gc, 512, |_| {
        calls.fetch_add(1, Ordering::Relaxed);
        Ok(3_u8)
    })?;
    assert_eq!(calls.load(Ordering::Relaxed), 512);
    assert!(buffer.iter().all(|byte| *byte == 3));
    assert!(gc.diagnostics().allocation_recovery.count > 0);
    assert_eq!(gc.statistics()?.collections_completed, 0);
    drop((buffer, pinned));
    gc.stop()?;
    assert_eq!(heap.statistics().occupied_bytes, 0);
    Ok(())
}

#[test]
fn physical_retry_precedes_initializer_side_effects() -> Result<()> {
    let gc = collector(true, 256);
    let held = HeapBuffer::try_with_fn(gc.region_heap().map(AsRef::as_ref), 256, |_| Ok(1_u8))?;
    let calls = AtomicUsize::new(0);
    let mut future = Box::pin(HeapBuffer::try_with_fn_in_async(&gc, 128, |_| {
        calls.fetch_add(1, Ordering::Relaxed);
        Ok(9_u8)
    }));
    let waker = Waker::from(Arc::new(Wakes::default()));
    assert!(
        future
            .as_mut()
            .poll(&mut Context::from_waker(&waker))
            .is_pending()
    );
    assert_eq!(calls.load(Ordering::Relaxed), 0);
    drop(held);
    let buffer = finish(&gc, future)?;
    assert_eq!(calls.load(Ordering::Relaxed), 128);
    assert!(buffer.iter().all(|byte| *byte == 9));
    assert_eq!(gc.diagnostics().allocation_recovery.count, 0);
    Ok(())
}

#[test]
fn suspended_construction_retains_owning_edges_until_cancellation() -> Result<()> {
    struct Parent(Gc<u64>);
    // SAFETY: outgoing fields remain independently rooted and are included in tracing.
    #[expect(unsafe_code, reason = "Trace requires an explicit ownership contract")]
    unsafe impl Trace for Parent {
        fn trace(&self, gc: &GarbageCollector) {
            self.0.trace(gc);
        }
        fn external_size(&self) -> usize {
            256
        }
    }
    let gc = collector(false, 512);
    // Keep insufficient room for Parent on both 32- and 64-bit targets.
    let filler = Gc::try_new(&gc, vec![0_u8; 256])?;
    let child = Gc::try_new(&gc, 42_u64)?.into_rooted();
    let weak = ristretto_gc::GcWeak::new(&child, ristretto_gc::ReferenceKind::Weak, 0);
    let mut future = Box::pin(Gc::try_new_async(&gc, Parent(child)));
    let waker = Waker::from(Arc::new(Wakes::default()));
    assert!(
        future
            .as_mut()
            .poll(&mut Context::from_waker(&waker))
            .is_pending()
    );
    gc.collect_full()?;
    assert_eq!(weak.upgrade().as_deref(), Some(&42));
    drop(future);
    drop(filler);
    gc.collect_full()?;
    gc.collect_full()?;
    assert!(weak.upgrade().is_none());
    assert_eq!(gc.managed_bytes(), 0);
    Ok(())
}

#[cfg(panic = "unwind")]
#[test]
fn publication_unwind_refunds_credit_and_destroys_the_unpublished_value() -> Result<()> {
    struct Panics(Arc<AtomicUsize>);
    // SAFETY: this fixture contains no outgoing GC references.
    #[expect(unsafe_code, reason = "Trace requires an explicit ownership contract")]
    unsafe impl Trace for Panics {
        fn trace(&self, _: &GarbageCollector) {}
        fn trace_work(&self) -> usize {
            panic!("test metadata construction failure");
        }
    }
    impl Drop for Panics {
        fn drop(&mut self) {
            self.0.fetch_add(1, Ordering::Relaxed);
        }
    }
    for (asynchronous, storage) in [
        ristretto_gc::RootStorage::Tree,
        ristretto_gc::RootStorage::Slots,
    ]
    .into_iter()
    .flat_map(|storage| [false, true].map(|asynchronous| (asynchronous, storage)))
    {
        let gc = GarbageCollector::with_config(
            Configuration::builder()
                .max_heap_bytes(256)
                .allocation_threshold(usize::MAX)
                .root_storage(storage)
                .build(),
        );
        let drops = Arc::new(AtomicUsize::new(0));
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let data = Panics(drops.clone());
            if asynchronous {
                finish(&gc, Gc::try_new_async(&gc, data))
            } else {
                Gc::try_new(&gc, data)
            }
        }));
        assert!(result.is_err());
        assert_eq!(drops.load(Ordering::Relaxed), 1);
        assert_eq!(gc.managed_bytes(), 0);
        assert_eq!(gc.statistics()?.live_objects, 0);
        gc.collect_full()?;
    }
    Ok(())
}

#[test]
fn leaf_reclamation_uses_its_declared_work_budget() -> Result<()> {
    struct Leaf(Arc<AtomicUsize>);
    // SAFETY: this fixture contains no outgoing GC references.
    #[expect(unsafe_code, reason = "Trace requires an explicit ownership contract")]
    unsafe impl Trace for Leaf {
        fn trace(&self, _: &GarbageCollector) {}
        fn trace_work(&self) -> usize {
            1
        }
    }
    impl Drop for Leaf {
        fn drop(&mut self) {
            self.0.fetch_add(1, Ordering::Relaxed);
        }
    }
    let gc = GarbageCollector::with_config(
        Configuration::builder()
            .allocation_threshold(usize::MAX)
            .incremental_step_size(64)
            .max_pause_time_us(0)
            .build(),
    );
    let drops = Arc::new(AtomicUsize::new(0));
    for _ in 0..128 {
        drop(Gc::try_new(&gc, Leaf(drops.clone()))?);
    }
    gc.collect();
    while drops.load(Ordering::Relaxed) == 0 {
        gc.step();
    }
    let before = drops.load(Ordering::Relaxed);
    let registered = gc.statistics()?.live_objects;
    gc.step();
    let cleared = drops.load(Ordering::Relaxed) - before;
    let unregistered = registered - gc.statistics()?.live_objects;
    // A previous sweep batch can leave already unregistered leaves to clear.
    // This step pays one unit per new removal and one per leaf destruction.
    assert!(
        cleared >= 16,
        "leaf clearing wasted its work budget: {cleared}"
    );
    assert!(
        cleared + unregistered <= 64,
        "sweep exceeded its work budget"
    );
    gc.collect_full()?;
    assert_eq!(drops.load(Ordering::Relaxed), 128);
    Ok(())
}

#[test]
fn allocation_resumes_before_its_recovery_cycle_completes() -> Result<()> {
    for compact in [false, true] {
        let gc = collector(compact, 256);
        drop(Gc::try_new(&gc, vec![0_u8; 160])?);
        let value = vec![9_u8; 128];
        let size = Gc::allocation_size(&value)?;
        let root = finish(&gc, Gc::try_new_async(&gc, value))?;
        assert_eq!(gc.statistics()?.full_collections, 0);
        assert_eq!(gc.managed_bytes(), size);
        assert_eq!(root.first(), Some(&9));
        finish(&gc, gc.collect_full_async())?;
        assert_eq!(gc.statistics()?.live_objects, 1);
        drop(root);
        finish(&gc, gc.collect_full_async())?;
        assert_eq!(gc.managed_bytes(), 0);
    }
    Ok(())
}

#[test]
fn competing_allocations_share_progress_without_oversubscribing_the_heap() -> Result<()> {
    let gc = collector(false, 256);
    drop(Gc::try_new(&gc, vec![0_u8; 160])?);
    let mut first = Box::pin(Gc::try_new_async(&gc, vec![1_u8; 128]));
    let mut second = Box::pin(Gc::try_new_async(&gc, vec![2_u8; 128]));
    let notifications = Arc::new(Wakes::default());
    let waker = Waker::from(notifications);
    let mut context = Context::from_waker(&waker);
    assert!(first.as_mut().poll(&mut context).is_pending());
    assert!(second.as_mut().poll(&mut context).is_pending());
    let root = finish(&gc, first)?;
    assert!(matches!(finish(&gc, second), Err(Error::OutOfMemory)));
    assert_eq!(gc.managed_bytes(), Gc::allocation_size(&vec![1_u8; 128])?);
    assert_eq!(gc.diagnostics().allocation_waiters, 0);
    assert_eq!(gc.diagnostics().allocation_recovery.count, 0);
    drop(root);
    finish(&gc, gc.collect_full_async())?;
    assert_eq!(gc.managed_bytes(), 0);
    Ok(())
}

#[cfg(not(target_family = "wasm"))]
mod native {
    use super::*;
    use std::sync::{Condvar, Mutex};
    use std::time::{Duration, Instant};

    #[derive(Default)]
    struct Gate {
        entered: AtomicUsize,
        open: Mutex<bool>,
        wake: Condvar,
    }
    impl Gate {
        fn release(&self) {
            *self
                .open
                .lock()
                .unwrap_or_else(std::sync::PoisonError::into_inner) = true;
            self.wake.notify_all();
        }
    }
    struct Release(Arc<Gate>);
    impl Drop for Release {
        fn drop(&mut self) {
            self.0.release();
        }
    }
    impl Gate {
        fn wait(&self) {
            self.entered.fetch_add(1, Ordering::Release);
            let mut open = self
                .open
                .lock()
                .unwrap_or_else(std::sync::PoisonError::into_inner);
            while !*open {
                open = self
                    .wake
                    .wait(open)
                    .unwrap_or_else(std::sync::PoisonError::into_inner);
            }
        }
    }
    struct Slow(Arc<Gate>);
    // SAFETY: this fixture contains no outgoing GC references.
    #[expect(unsafe_code, reason = "Trace requires an explicit ownership contract")]
    unsafe impl Trace for Slow {
        fn trace(&self, _: &GarbageCollector) {
            self.0.wait();
        }
    }
    fn until(condition: impl Fn() -> bool) {
        let start = Instant::now();
        while !condition() {
            assert!(
                start.elapsed() < Duration::from_secs(10),
                "worker did not notify its waiter"
            );
            std::thread::yield_now();
        }
    }

    #[test]
    fn registering_a_sleeping_allocator_and_refunding_cannot_miss_each_other() {
        use loom::sync::Arc;
        use loom::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

        loom::model(|| {
            let used = Arc::new(AtomicUsize::new(1));
            let registered = Arc::new(AtomicBool::new(false));
            let refunder = {
                let used = used.clone();
                let registered = registered.clone();
                loom::thread::spawn(move || {
                    used.fetch_sub(1, Ordering::AcqRel);
                    registered.load(Ordering::Acquire)
                })
            };
            registered.store(true, Ordering::Release);
            // AllocationBudget::used_after_registration and release share the
            // byte counter's modification order, including on weak targets.
            let fits = used.fetch_or(0, Ordering::AcqRel) == 0;
            assert!(refunder.join().unwrap() || fits);
        });
    }

    struct Resume(std::thread::Thread);
    impl Wake for Resume {
        fn wake(self: Arc<Self>) {
            self.0.unpark();
        }
        fn wake_by_ref(self: &Arc<Self>) {
            self.0.unpark();
        }
    }
    fn run<F: Future>(future: F) -> F::Output {
        let mut future = std::pin::pin!(future);
        let waker = Waker::from(Arc::new(Resume(std::thread::current())));
        let mut context = Context::from_waker(&waker);
        let started = Instant::now();
        loop {
            if let Poll::Ready(result) = future.as_mut().poll(&mut context) {
                return result;
            }
            assert!(
                started.elapsed() < Duration::from_secs(10),
                "allocation lost its wakeup"
            );
            std::thread::park_timeout(Duration::from_millis(10));
        }
    }

    #[test]
    fn concurrent_native_allocators_reserve_reclaimed_space_without_lost_wakeups() -> Result<()> {
        allocator_stress(false, false)
    }

    #[test]
    fn counted_publication_allocators_reserve_reclaimed_space_without_lost_wakeups() -> Result<()> {
        allocator_stress(true, false)
    }

    #[test]
    fn overlapping_generations_reserve_reclaimed_space_without_lost_wakeups() -> Result<()> {
        allocator_stress(true, true)
    }

    async fn allocate_buffer(
        gc: &Arc<GarbageCollector>,
        byte: u8,
    ) -> std::result::Result<(), (&'static str, Error)> {
        let buffer = HeapBuffer::try_with_fn_in_async(gc, 4096, |_| Ok(byte))
            .await
            .map_err(|error| ("buffer backing", error))?;
        let root = Gc::try_new_async(gc, ristretto_gc::sync::RwLock::new(buffer))
            .await
            .map_err(|error| ("managed object", error))?;
        assert_eq!(root.read().last(), Some(&byte));
        Ok(())
    }

    fn allocator_stress(concurrent_publication: bool, overlap: bool) -> Result<()> {
        for (compact, predictive, storage, independent) in [
            ristretto_gc::RootStorage::Tree,
            ristretto_gc::RootStorage::Slots,
            ristretto_gc::RootStorage::Contexts,
        ]
        .into_iter()
        .flat_map(|storage| {
            [(false, false), (true, false), (false, true), (true, true)]
                .map(|(compact, predictive)| (compact, predictive, storage))
        })
        .flat_map(|(compact, predictive, storage)| {
            [false, true].map(|independent| (compact, predictive, storage, independent))
        }) {
            let gc = GarbageCollector::with_config(
                Configuration::builder()
                    .concurrent_publication(concurrent_publication)
                    .overlapping_generations(overlap)
                    .old_collection_interval(if overlap { 4 } else { 0 })
                    .independent_phases(overlap)
                    .region_shards(if overlap { 16 } else { 1 })
                    .threads(4)
                    .predictive_collection(predictive)
                    .local_allocation_buffers(storage == ristretto_gc::RootStorage::Contexts)
                    .adaptive_workers(storage == ristretto_gc::RootStorage::Contexts)
                    .independent_relocation(storage == ristretto_gc::RootStorage::Contexts)
                    .metadata_shards(16)
                    .work_stealing(true)
                    .independent_marking(independent)
                    .mark_batch_size(8)
                    .root_storage(storage)
                    .barrier_buffer_size(if storage == ristretto_gc::RootStorage::Contexts {
                        4
                    } else {
                        0
                    })
                    .compact_heap(compact)
                    .cost_aware_relocation(compact)
                    .relocation_reserve_bytes(16 * 1024)
                    .diagnostics(true)
                    .max_heap_bytes(256 * 1024)
                    .allocation_threshold(usize::MAX)
                    .region_layout(16 * 1024, 4096)
                    .incremental_step_size(8)
                    .build(),
            );
            gc.start();
            std::thread::scope(|scope| -> Result<()> {
                let handles: Vec<_> = (0..4_u8)
                    .map(|byte| {
                        let gc = &gc;
                        scope.spawn(move || -> Result<()> {
                            let context = gc.register_context();
                            for iteration in 0_usize..500 {
                                if overlap && iteration.is_multiple_of(17) {
                                    gc.collect_old();
                                    gc.collect_young();
                                }
                                run(context.scope(allocate_buffer(gc, byte))).map_err(|(stage, error)| {
                                    eprintln!("{error:?} during {stage}: concurrent_publication={concurrent_publication}, overlap={overlap}, compact={compact}, predictive={predictive}, storage={storage:?}, independent={independent}, mutator={byte}, iteration={iteration}; logical={:?}; regions={:?}", gc.statistics(), gc.region_heap().map(|heap| heap.statistics()));
                                    error
                                })?;
                            }
                            Ok(())
                        })
                    })
                    .collect();
                for handle in handles {
                    handle
                        .join()
                        .unwrap_or_else(|error| std::panic::resume_unwind(error))?;
                }
                Ok(())
            })?;
            assert_eq!(gc.diagnostics().allocation_waiters, 0);
            assert_eq!(gc.diagnostics().allocation_recovery.count, 0);
            if predictive {
                // Physical pressure may keep a recovery cycle active throughout
                // every prediction sample. Prediction correctly coalesces with
                // that work instead of issuing a redundant request. Dedicated
                // scheduling tests check its idle-to-request threshold.
                assert!(gc.scheduling_statistics().samples > 0);
            }
            gc.stop()?;
            assert_eq!(gc.managed_bytes(), 0);
            assert_eq!(gc.diagnostics().pending_barrier_records, 0);
            assert_eq!(gc.diagnostics().execution_contexts, 0);
            assert_eq!(gc.root_statistics().pages, 0);
            if let Some(heap) = gc.region_heap() {
                assert_eq!(heap.statistics().occupied_bytes, 0);
                assert_eq!(heap.statistics().shared_committed_bytes, 0);
                assert!(heap.statistics().peak_shared_committed_bytes <= 256 * 1024);
            }
        }
        Ok(())
    }

    #[test]
    fn workers_complete_a_fresh_ticket_while_a_destructor_owns_its_storage() -> Result<()> {
        struct SlowDrop(Arc<Gate>);
        // SAFETY: this fixture contains no outgoing GC references.
        #[expect(unsafe_code, reason = "Trace requires an explicit ownership contract")]
        unsafe impl Trace for SlowDrop {
            fn trace(&self, _: &GarbageCollector) {}
        }
        impl Drop for SlowDrop {
            fn drop(&mut self) {
                self.0.wait();
            }
        }
        let gc = GarbageCollector::with_config(
            Configuration::builder()
                .threads(2)
                .diagnostics(true)
                .allocation_threshold(usize::MAX)
                .worker_yield_time_us(50)
                .build(),
        );
        let gate = Arc::new(Gate::default());
        let _release = Release(gate.clone());
        drop(Gc::try_new(&gc, SlowDrop(gate.clone()))?);
        gc.start();
        gc.collect();
        until(|| gate.entered.load(Ordering::Acquire) > 0);
        let charge = gc.managed_bytes();
        assert!(charge > 0);
        run(gc.collect_full_async())?;
        assert_eq!(gc.managed_bytes(), charge);
        gate.release();
        gc.stop()?;
        assert_eq!(gc.managed_bytes(), 0);
        Ok(())
    }

    #[test]
    fn a_space_waiter_and_a_full_waiter_can_share_a_cycle_ticket() -> Result<()> {
        let gc = collector(false, 256);
        let gate = Arc::new(Gate::default());
        let _release = Release(gate.clone());
        let root = Gc::try_new(&gc, Slow(gate.clone()))?;
        gc.start();
        let notifications = Arc::new(Wakes::default());
        let waker = Waker::from(notifications);
        let mut context = Context::from_waker(&waker);
        let mut full = Box::pin(gc.collect_full_async());
        assert!(full.as_mut().poll(&mut context).is_pending());
        until(|| gate.entered.load(Ordering::Acquire) > 0);
        let mut space = Box::pin(gc.make_room_async(256));
        assert!(space.as_mut().poll(&mut context).is_pending()); // recovery boundary
        assert!(space.as_mut().poll(&mut context).is_pending()); // waiter registration
        assert_eq!(gc.diagnostics().allocation_waiters, 2);
        drop(space);
        assert_eq!(gc.diagnostics().allocation_waiters, 1);
        gate.release();
        until(|| {
            gc.statistics()
                .is_ok_and(|stats| stats.full_collections >= 1)
        });
        assert!(matches!(
            full.as_mut().poll(&mut context),
            Poll::Ready(Ok(()))
        ));
        assert_eq!(
            gc.statistics()?.full_collections,
            1,
            "recovery should join the active cycle"
        );
        drop(full);
        drop(root);
        gc.stop()?;
        Ok(())
    }

    #[test]
    fn an_early_wakeup_supports_publication_or_cancellation_before_compaction_finishes()
    -> Result<()> {
        struct SlowCompact {
            mark: Arc<Gate>,
            compact: Arc<Gate>,
        }
        // SAFETY: this fixture contains no outgoing GC references.
        #[expect(unsafe_code, reason = "Trace requires an explicit ownership contract")]
        unsafe impl Trace for SlowCompact {
            fn trace(&self, _: &GarbageCollector) {
                self.mark.wait();
            }
            fn compact_guarded(
                &self,
                _: &ristretto_gc::region::RegionHeap,
                _: usize,
            ) -> ristretto_gc::TraceProgress {
                self.compact.wait();
                ristretto_gc::TraceProgress::Complete
            }
        }
        for cancel in [false, true] {
            let gc = collector(true, 256);
            let mark = Arc::new(Gate::default());
            let compact = Arc::new(Gate::default());
            let _release_mark = Release(mark.clone());
            let _release_compact = Release(compact.clone());
            let root = Gc::try_new(
                &gc,
                SlowCompact {
                    mark: mark.clone(),
                    compact: compact.clone(),
                },
            )?;
            drop(Gc::try_new(&gc, vec![0_u8; 160])?);
            let drops = Arc::new(AtomicUsize::new(0));
            gc.start();
            let notifications = Arc::new(Wakes::default());
            let waker = Waker::from(notifications.clone());
            let mut context = Context::from_waker(&waker);
            let mut future = Box::pin(Gc::try_new_async(&gc, Counted(drops.clone())));
            assert!(future.as_mut().poll(&mut context).is_pending());
            assert!(future.as_mut().poll(&mut context).is_pending());
            until(|| mark.entered.load(Ordering::Acquire) > 0);
            notifications.0.store(0, Ordering::Release);
            mark.release();
            until(|| compact.entered.load(Ordering::Acquire) > 0);
            // Refund delivery may run on the independent notifier. Keep the
            // compaction callback parked until the executor actually gets a wake.
            until(|| notifications.0.load(Ordering::Acquire) > 0);
            assert_eq!(gc.statistics()?.full_collections, 0);
            assert_eq!(gc.diagnostics().allocation_early_wakeups, 1);
            let value = if cancel {
                None
            } else {
                let Poll::Ready(result) = future.as_mut().poll(&mut context) else {
                    panic!("freed space must resume allocation before compaction ends");
                };
                Some(result?)
            };
            drop(future);
            assert_eq!(gc.diagnostics().allocation_waiters, 0);
            if value.is_none() {
                assert_eq!(drops.load(Ordering::Relaxed), 1);
            }
            assert!(gc.managed_bytes() <= 256);
            compact.release();
            drop(value);
            drop(root);
            gc.stop()?;
            assert_eq!(drops.load(Ordering::Relaxed), 1);
        }
        Ok(())
    }

    #[test]
    fn native_waiters_suspend_migrate_cancel_and_wait_for_the_requested_cycle() -> Result<()> {
        let gc = collector(false, 4096);
        let gate = Arc::new(Gate::default());
        let _release = Release(gate.clone());
        let root = Gc::try_new(&gc, Slow(gate.clone()))?;
        gc.start();
        let old = Arc::new(Wakes::default());
        let old_waker = Waker::from(old.clone());
        let mut cancelled = Box::pin(gc.collect_full_async());
        assert!(
            cancelled
                .as_mut()
                .poll(&mut Context::from_waker(&old_waker))
                .is_pending()
        );
        until(|| gate.entered.load(Ordering::Acquire) != 0);
        assert_eq!(
            old.0.load(Ordering::Acquire),
            0,
            "native waiting must not self-wake"
        );
        let mut waiting = Box::pin(gc.collect_full_async());
        assert!(
            waiting
                .as_mut()
                .poll(&mut Context::from_waker(&old_waker))
                .is_pending()
        );
        assert_eq!(gc.diagnostics().allocation_waiters, 2);
        drop(cancelled);
        assert_eq!(gc.diagnostics().allocation_waiters, 1);
        let new = Arc::new(Wakes::default());
        let new_waker = Waker::from(new.clone());
        let pending = std::thread::scope(|scope| {
            scope
                .spawn(|| {
                    waiting
                        .as_mut()
                        .poll(&mut Context::from_waker(&new_waker))
                        .is_pending()
                })
                .join()
                .unwrap_or_else(|error| std::panic::resume_unwind(error))
        });
        assert!(pending);
        gate.release();
        until(|| new.0.load(Ordering::Acquire) != 0);
        assert!(matches!(
            waiting.as_mut().poll(&mut Context::from_waker(&new_waker)),
            Poll::Ready(Ok(()))
        ));
        assert_eq!(old.0.load(Ordering::Acquire), 0);
        assert_eq!(gc.statistics()?.full_collections, 2);
        assert_eq!(gc.diagnostics().allocation_waiters, 0);
        assert_eq!(gc.diagnostics().caller_steps.count, 0);
        assert_eq!(gc.diagnostics().async_assists, 0);
        drop(waiting);
        drop(root);
        gc.stop()?;
        Ok(())
    }

    #[test]
    fn stopping_workers_notifies_pending_async_collection() -> Result<()> {
        let gc = collector(false, 4096);
        gc.start();
        let notifications = Arc::new(Wakes::default());
        let waker = Waker::from(notifications.clone());
        let mut future = Box::pin(gc.collect_full_async());
        let first = future.as_mut().poll(&mut Context::from_waker(&waker));
        gc.stop()?;
        if first.is_pending() {
            assert!(notifications.0.load(Ordering::Acquire) > 0);
            assert!(matches!(
                future.as_mut().poll(&mut Context::from_waker(&waker)),
                Poll::Ready(Ok(()))
            ));
        }
        assert_eq!(gc.diagnostics().allocation_waiters, 0);
        Ok(())
    }

    #[test]
    fn completion_wakers_can_reenter_the_collector() -> Result<()> {
        struct Reenter {
            gc: Arc<GarbageCollector>,
            calls: AtomicUsize,
        }
        impl Wake for Reenter {
            fn wake(self: Arc<Self>) {
                assert!(self.gc.collect_full().is_ok());
                self.calls.fetch_add(1, Ordering::Release);
            }
        }
        let gc = collector(false, 4096);
        let gate = Arc::new(Gate::default());
        let _release = Release(gate.clone());
        let root = Gc::try_new(&gc, Slow(gate.clone()))?;
        gc.start();
        let notification = Arc::new(Reenter {
            gc: gc.clone(),
            calls: AtomicUsize::new(0),
        });
        let waker = Waker::from(notification.clone());
        let mut future = Box::pin(gc.collect_full_async());
        assert!(
            future
                .as_mut()
                .poll(&mut Context::from_waker(&waker))
                .is_pending()
        );
        until(|| gate.entered.load(Ordering::Acquire) > 0);
        gate.release();
        until(|| notification.calls.load(Ordering::Acquire) > 0);
        assert!(matches!(
            future.as_mut().poll(&mut Context::from_waker(&waker)),
            Poll::Ready(Ok(()))
        ));
        drop(future);
        drop(root);
        gc.stop()?;
        Ok(())
    }

    #[cfg(panic = "unwind")]
    #[test]
    fn a_worker_unwind_wakes_waiters_with_an_error() -> Result<()> {
        struct Panics;
        // SAFETY: this fixture contains no outgoing GC references.
        #[expect(unsafe_code, reason = "Trace requires an explicit ownership contract")]
        unsafe impl Trace for Panics {
            fn trace(&self, _: &GarbageCollector) {
                panic!("test worker failure");
            }
        }
        let gc = GarbageCollector::with_config(
            Configuration::builder()
                .threads(2)
                .diagnostics(true)
                .allocation_threshold(usize::MAX)
                .incremental_step_size(1)
                .build(),
        );
        let _root = Gc::try_new(&gc, Panics)?;
        gc.start();
        let notifications = Arc::new(Wakes::default());
        let waker = Waker::from(notifications.clone());
        let mut future = Box::pin(gc.collect_full_async());
        let first = future.as_mut().poll(&mut Context::from_waker(&waker));
        if first.is_pending() {
            until(|| notifications.0.load(Ordering::Acquire) > 0);
            assert!(matches!(
                future.as_mut().poll(&mut Context::from_waker(&waker)),
                Poll::Ready(Err(Error::SyncError(_)))
            ));
        } else {
            assert!(matches!(first, Poll::Ready(Err(Error::SyncError(_)))));
        }
        assert_eq!(gc.diagnostics().allocation_waiters, 0);
        drop(future);
        assert!(gc.stop().is_err());
        Ok(())
    }
}

#[test]
fn a_partially_cleared_buffer_donates_only_its_destroyed_suffix() -> Result<()> {
    use ristretto_gc::TraceProgress;
    #[repr(align(64))]
    struct Counted(Arc<AtomicUsize>);
    // SAFETY: Counted shares a host-side counter and contains no managed edges.
    #[expect(unsafe_code, reason = "the test element has no managed edges")]
    unsafe impl Trace for Counted {
        fn trace(&self, _: &GarbageCollector) {}
    }
    impl Drop for Counted {
        fn drop(&mut self) {
            self.0.fetch_add(1, Ordering::Relaxed);
        }
    }
    let size = 256 * size_of::<Counted>();
    let gc = GarbageCollector::with_config(
        Configuration::builder()
            .compact_heap(true)
            .max_heap_bytes(size)
            .region_layout(size, 0)
            .allocation_threshold(usize::MAX)
            .incremental_step_size(1)
            .build(),
    );
    let heap = gc.region_heap().expect("regional storage");
    let drops = Arc::new(AtomicUsize::new(0));
    let mut donor = HeapBuffer::try_with_fn(Some(heap), 256, |_| Ok(Counted(drops.clone())))?;
    let base = donor.as_ptr().addr();
    let initialized = AtomicUsize::new(0);
    let mut request = Box::pin(HeapBuffer::try_with_fn_in_async(&gc, size / 2, |_| {
        initialized.fetch_add(1, Ordering::Relaxed);
        Ok(73_u8)
    }));
    assert!(
        request
            .as_mut()
            .poll(&mut Context::from_waker(Waker::noop()))
            .is_pending()
    );
    for step in 1..=2 {
        assert_eq!(donor.clear_step(0, 64), TraceProgress::Continue(0));
        assert_eq!(drops.load(Ordering::Relaxed), step * 64);
        assert_eq!(donor.as_ptr().addr(), base);
        assert_eq!(heap.statistics().occupied_bytes, size - step * 4096);
        gc.collect_full()?;
        if step == 1 {
            assert!(
                request
                    .as_mut()
                    .poll(&mut Context::from_waker(Waker::noop()))
                    .is_pending()
            );
            assert_eq!(initialized.load(Ordering::Relaxed), 0);
        }
    }
    let received = finish(&gc, request)?;
    assert_eq!(received.as_ptr().addr(), base + size / 2);
    assert_eq!(initialized.load(Ordering::Relaxed), size / 2);
    assert!(received.iter().all(|value| *value == 73));
    assert_eq!(donor.len(), 128);
    drop(donor);
    assert_eq!(drops.load(Ordering::Relaxed), 256);
    assert_eq!(heap.statistics().occupied_bytes, size / 2);
    drop(received);
    assert_eq!(heap.statistics().occupied_bytes, 0);
    Ok(())
}

#[test]
fn reclaimed_physical_ranges_stolen_by_a_competitor_do_not_exhaust_recovery() -> Result<()> {
    let gc = GarbageCollector::with_config(
        Configuration::builder()
            .compact_heap(true)
            .max_heap_bytes(4096)
            .region_layout(4096, 0)
            .allocation_threshold(usize::MAX)
            .incremental_step_size(1)
            .max_pause_time_us(0)
            .build(),
    );
    let heap = gc.region_heap().expect("compact storage");
    let mut competing = Some(HeapBuffer::try_with_fn(Some(heap), 4096, |_| Ok(1_u8))?);
    let initialized = AtomicUsize::new(0);
    let mut pending = Box::pin(HeapBuffer::try_with_fn_in_async(&gc, 4096, |_| {
        initialized.fetch_add(1, Ordering::Relaxed);
        Ok(2_u8)
    }));
    let waker = Waker::from(Arc::new(Wakes::default()));
    let mut context = Context::from_waker(&waker);
    assert!(pending.as_mut().poll(&mut context).is_pending());
    // Return capacity and consume it before the suspended allocator can retry.
    // Unpublished buffers isolate physical pressure from logical reservations.
    for _ in 0..1024 {
        drop(competing.take());
        competing = Some(HeapBuffer::try_with_fn(Some(heap), 4096, |_| Ok(1_u8))?);
        assert!(pending.as_mut().poll(&mut context).is_pending());
        if gc.statistics()?.full_collections >= 3 {
            break;
        }
    }
    assert!(gc.statistics()?.full_collections >= 3);
    // Stop reclamation while retaining the competing live range: failure must
    // still terminate once two cycles make no physical progress.
    assert!(matches!(finish(&gc, pending), Err(Error::OutOfMemory)));
    assert_eq!(initialized.load(Ordering::Relaxed), 0);
    drop(competing);
    gc.stop()?;
    assert_eq!(heap.statistics().occupied_bytes, 0);
    Ok(())
}
