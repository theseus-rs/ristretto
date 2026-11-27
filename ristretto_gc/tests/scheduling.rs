//! Predictive requests are advisory; roots and bounded recovery keep their contracts.
#![expect(clippy::panic_in_result_fn)]
use ristretto_gc::{Configuration, Error, GarbageCollector, Gc, HeapBuffer, Result};
use std::future::Future;
use std::task::{Context, Poll, Waker};

#[test]
fn prediction_is_opt_in_and_ignored_for_unbounded_or_zero_heaps() {
    assert!(!Configuration::default().predictive_collection);
    for limit in [0, usize::MAX] {
        let gc = GarbageCollector::with_config(
            Configuration::builder()
                .predictive_collection(true)
                .max_heap_bytes(limit)
                .build(),
        );
        gc.record_allocation(16);
        assert!(!gc.scheduling_statistics().enabled);
        assert_eq!(gc.scheduling_statistics().samples, 0);
    }
}

#[test]
fn requests_before_exhaustion_without_running_collection_on_publication() -> Result<()> {
    for compact in [false, true] {
        let limit = 64 * 1024;
        let gc = GarbageCollector::with_config(
            Configuration::builder()
                .predictive_collection(true)
                .compact_heap(compact)
                .region_layout(4096, 256)
                .max_heap_bytes(limit)
                .allocation_threshold(usize::MAX)
                .max_pause_time_us(0)
                .build(),
        );
        let mut roots = Vec::new();
        for _ in 0..60 {
            let buffer = HeapBuffer::try_with_fn_in(&gc, 1024, |_| Ok(7_u8))?;
            roots.push(Gc::try_new(&gc, ristretto_gc::sync::RwLock::new(buffer))?);
            if gc.scheduling_statistics().requests != 0 {
                break;
            }
        }
        assert_eq!(gc.scheduling_statistics().requests, 1);
        assert!(gc.managed_bytes() < limit);
        assert_eq!(gc.statistics()?.collections_started, 0);
        for _ in 0..10_000 {
            if gc.step() {
                break;
            }
        }
        assert_eq!(gc.statistics()?.full_collections, 1);
        assert_eq!(gc.statistics()?.live_objects, roots.len());
        assert_eq!(gc.scheduling_statistics().survival_ppm, 1_000_000);
        assert_eq!(gc.scheduling_statistics().live_bytes, gc.managed_bytes());
        assert!(
            roots
                .iter()
                .all(|root| root.read().iter().all(|byte| *byte == 7))
        );
        for _ in 0..10 {
            assert!(gc.step());
        }
        assert_eq!(gc.statistics()?.full_collections, 1);
        drop(roots);
        gc.collect_full()?;
        assert_eq!(gc.managed_bytes(), 0);
        assert_eq!(
            gc.region_heap()
                .map_or(0, |heap| heap.statistics().occupied_bytes),
            0
        );
    }
    Ok(())
}

#[test]
fn prediction_preserves_bounded_async_failure_for_an_exhausted_live_set() -> Result<()> {
    let gc = GarbageCollector::with_config(
        Configuration::builder()
            .predictive_collection(true)
            .diagnostics(true)
            .max_heap_bytes(512)
            .allocation_threshold(usize::MAX)
            .incremental_step_size(1)
            .max_pause_time_us(0)
            .build(),
    );
    let root = Gc::try_new(&gc, vec![7_u8; 380])?;
    assert_eq!(gc.scheduling_statistics().requests, 1);
    let mut future = std::pin::pin!(gc.make_room_async(256));
    let mut context = Context::from_waker(Waker::noop());
    for _ in 0..10_000 {
        let before = gc.diagnostics().caller_steps.count;
        let result = future.as_mut().poll(&mut context);
        assert!(gc.diagnostics().caller_steps.count <= before + 1);
        if let Poll::Ready(result) = result {
            assert!(matches!(result, Err(Error::OutOfMemory)));
            assert_eq!(gc.diagnostics().allocation_recovery.count, 0);
            assert_eq!(gc.diagnostics().async_allocation_recovery.count, 2);
            assert_eq!(gc.diagnostics().allocation_waiters, 0);
            assert!(root.iter().all(|byte| *byte == 7));
            return Ok(());
        }
    }
    panic!("exhausted live set must terminate");
}
