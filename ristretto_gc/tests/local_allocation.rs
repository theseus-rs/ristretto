//! Cache reservations remain bounded and independent of root/OS-thread ownership.
#![expect(clippy::panic_in_result_fn)]
use ristretto_gc::{Configuration, GarbageCollector, Gc, HeapBuffer, Result, RootStorage, Trace};
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

fn collector(compact: bool, limit: usize) -> Arc<GarbageCollector> {
    GarbageCollector::with_config(
        Configuration::builder()
            .local_allocation_buffers(true)
            .compact_heap(compact)
            .diagnostics(true)
            .max_heap_bytes(limit)
            .allocation_threshold(usize::MAX)
            .root_storage(RootStorage::Contexts)
            .barrier_buffer_size(2)
            .metadata_shards(16)
            .independent_marking(true)
            .work_stealing(true)
            .mark_batch_size(8)
            .threads(4)
            .incremental_step_size(32)
            .max_pause_time_us(0)
            .build(),
    )
}

struct Bytes(HeapBuffer<u8>);
// SAFETY: this fixture contains no outgoing GC references.
#[expect(unsafe_code, reason = "Trace requires an explicit ownership contract")]
unsafe impl Trace for Bytes {
    fn trace(&self, _: &GarbageCollector) {}
    fn external_size(&self) -> usize {
        self.0.len()
    }
}

fn released(gc: &GarbageCollector) -> Result<()> {
    gc.stop()?;
    let stats = gc.diagnostics();
    assert_eq!(gc.managed_bytes(), 0);
    assert_eq!(gc.statistics()?.live_objects, 0);
    assert_eq!(stats.cached_allocation_bytes, 0);
    assert_eq!(stats.cached_allocation_credit, 0);
    assert_eq!(stats.descriptor_occupied_bytes, 0);
    assert_eq!(stats.descriptor_committed_bytes, 0);
    if let Some(heap) = gc.region_heap() {
        assert_eq!(heap.statistics().occupied_bytes, 0);
    }
    Ok(())
}

#[test]
fn default_and_foreign_contexts_do_not_cache() -> Result<()> {
    assert!(!Configuration::default().local_allocation_buffers);
    let first = collector(true, 1024 * 1024);
    let second = collector(true, 1024 * 1024);
    let context = first.register_context();
    let root = context.with(|| Gc::try_new(&second, 7_u64))?;
    assert_eq!(first.diagnostics().cached_allocation_bytes, 0);
    assert_eq!(second.diagnostics().cached_allocation_bytes, 0);
    assert_eq!(**root, 7);
    drop(root);
    released(&second)?;
    released(&first)
}

#[test]
fn last_context_handle_returns_credit_even_with_escaped_roots() -> Result<()> {
    for compact in [false, true] {
        let gc = collector(compact, 1024 * 1024);
        let context = gc.register_context();
        let clone = context.clone();
        let roots = context.with(|| {
            (0..128_u64)
                .map(|n| Gc::try_new(&gc, n))
                .collect::<Result<Vec<_>>>()
        })?;
        let cache = gc.diagnostics();
        assert!(cache.local_allocation_hits > 100);
        assert!(cache.local_allocation_refills > 0);
        assert!(cache.cached_allocation_bytes <= 32 * 1024);
        assert!(cache.cached_allocation_credit <= 4096);
        let live = 128 * Gc::allocation_size(&0_u64)?;
        assert_eq!(gc.managed_bytes(), live + cache.cached_allocation_credit);
        drop(context);
        assert!(gc.diagnostics().cached_allocation_bytes > 0);
        drop(clone);
        assert_eq!(gc.diagnostics().cached_allocation_bytes, 0);
        assert_eq!(gc.diagnostics().cached_allocation_credit, 0);
        assert_eq!(gc.managed_bytes(), live);
        gc.collect_full()?;
        for (n, root) in roots.iter().enumerate() {
            assert_eq!(***root, n as u64);
        }
        drop(roots);
        released(&gc)?;
    }
    Ok(())
}

#[test]
fn tiny_heaps_fall_back_to_exact_ranges_and_credit() -> Result<()> {
    for limit in [512, 1024, 4096, 16 * 1024] {
        let gc = collector(true, limit);
        let context = gc.register_context();
        let root = context.with(|| -> Result<_> {
            let bytes = HeapBuffer::try_with_fn_in(&gc, 128, |index| {
                u8::try_from(index).map_err(|_| ristretto_gc::Error::OutOfMemory)
            })?;
            Gc::try_new(&gc, Bytes(bytes))
        })?;
        assert!(gc.managed_bytes() <= limit);
        assert!(
            gc.region_heap()
                .expect("compact")
                .statistics()
                .committed_bytes
                <= limit
        );
        assert_eq!(root.0.as_ref(), &(0..128_u8).collect::<Vec<_>>());
        drop((root, context));
        released(&gc)?;
    }
    Ok(())
}

#[repr(align(256))]
struct Aligned(u64);
// SAFETY: this fixture contains no outgoing GC references.
#[expect(unsafe_code, reason = "Trace requires an explicit ownership contract")]
unsafe impl Trace for Aligned {
    fn trace(&self, _: &GarbageCollector) {}
}

#[test]
fn aligned_and_large_requests_keep_their_separate_path() -> Result<()> {
    let gc = collector(true, 1024 * 1024);
    let context = gc.register_context();
    let (aligned, bytes) = context.with(|| -> Result<_> {
        let aligned = Gc::try_new(&gc, Aligned(7))?;
        let bytes = HeapBuffer::try_with_fn_in(&gc, 64 * 1024, |_| Ok(9_u8))?;
        Ok((aligned, bytes))
    })?;
    assert_eq!(std::ptr::from_ref(&**aligned) as usize % 256, 0);
    assert_eq!(aligned.0, 7);
    assert!(bytes.iter().all(|byte| *byte == 9));
    assert_eq!(gc.diagnostics().local_allocation_refills, 0);
    drop((aligned, bytes, context));
    released(&gc)
}

struct Counted(Arc<AtomicUsize>);
// SAFETY: this fixture contains no outgoing GC references.
#[expect(unsafe_code, reason = "Trace requires an explicit ownership contract")]
unsafe impl Trace for Counted {
    fn trace(&self, _: &GarbageCollector) {}
}
impl Drop for Counted {
    fn drop(&mut self) {
        self.0.fetch_add(1, Ordering::Relaxed);
    }
}

#[test]
fn repeated_refill_and_reclamation_destroy_each_descriptor_once() -> Result<()> {
    for compact in [false, true] {
        let gc = collector(compact, 1024 * 1024);
        let context = gc.register_context();
        let drops = Arc::new(AtomicUsize::new(0));
        for round in 0..4 {
            context.with(|| -> Result<()> {
                for _ in 0..600 {
                    drop(Gc::try_new(&gc, Counted(drops.clone()))?);
                }
                Ok(())
            })?;
            gc.collect_full()?;
            gc.collect_full()?;
            assert_eq!(drops.load(Ordering::Relaxed), (round + 1) * 600);
            assert_eq!(gc.diagnostics().cached_allocation_credit, 0);
        }
        drop(context);
        released(&gc)?;
    }
    Ok(())
}

#[cfg(not(target_family = "wasm"))]
#[test]
#[expect(clippy::unwrap_in_result, reason = "worker panics fail the fixture")]
fn context_migration_and_concurrent_reservations_stay_within_budget() -> Result<()> {
    for compact in [false, true] {
        let gc = collector(compact, 4 * 1024 * 1024);
        gc.start();
        let context = gc.register_context();
        let drops = Arc::new(AtomicUsize::new(0));
        let mut workers = Vec::new();
        for _ in 0..4 {
            let gc = gc.clone();
            let context = context.clone();
            let drops = drops.clone();
            workers.push(std::thread::spawn(move || {
                context.with(|| -> Result<()> {
                    for _ in 0..500 {
                        let root = Gc::try_new(&gc, Counted(drops.clone()))?;
                        assert!(gc.managed_bytes() <= 4 * 1024 * 1024);
                        std::thread::yield_now();
                        drop(root);
                    }
                    Ok(())
                })
            }));
        }
        for worker in workers {
            worker.join().expect("mutator")?;
        }
        drop(context);
        gc.collect_full()?;
        gc.collect_full()?;
        assert_eq!(drops.load(Ordering::Relaxed), 2000);
        released(&gc)?;
    }
    Ok(())
}

#[cfg(panic = "unwind")]
#[test]
fn constructor_unwind_refunds_storage_and_drops_once() -> Result<()> {
    struct Fails {
        drops: Arc<AtomicUsize>,
        attaching: bool,
    }
    // SAFETY: this fixture contains no outgoing GC references.
    #[expect(unsafe_code, reason = "Trace requires an explicit ownership contract")]
    unsafe impl Trace for Fails {
        fn trace(&self, _: &GarbageCollector) {}
        unsafe fn attach(&mut self, _owner: ristretto_gc::TraceOwner<'_>) {
            assert!(!self.attaching, "attachment failure");
        }
        fn trace_work(&self) -> usize {
            panic!("metadata construction failure")
        }
    }
    impl Drop for Fails {
        fn drop(&mut self) {
            self.drops.fetch_add(1, Ordering::Relaxed);
        }
    }
    for compact in [false, true] {
        for attaching in [false, true] {
            let gc = collector(compact, 1024 * 1024);
            let context = gc.register_context();
            let drops = Arc::new(AtomicUsize::new(0));
            let failed = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                context.with(|| {
                    Gc::try_new(
                        &gc,
                        Fails {
                            drops: drops.clone(),
                            attaching,
                        },
                    )
                })
            }));
            assert!(failed.is_err());
            assert_eq!(drops.load(Ordering::Relaxed), 1);
            assert_eq!(gc.root_statistics().slot_roots, 0);
            drop(context);
            released(&gc)?;
            assert_eq!(drops.load(Ordering::Relaxed), 1);
        }
    }
    Ok(())
}
