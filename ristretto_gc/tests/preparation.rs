//! Unpublished staging retains source ownership across suspension and cancellation.
#![expect(clippy::panic_in_result_fn)]
use ristretto_gc::region::RegionHeap;
use ristretto_gc::sync::RwLock;
use ristretto_gc::{
    Configuration, Error, GarbageCollector, Gc, HeapBuffer, Result, Trace, TraceProgress,
};
use std::future::Future;
use std::sync::{
    Arc,
    atomic::{AtomicUsize, Ordering},
};
use std::task::{Context, Poll, Waker};

#[repr(align(4096))]
struct Counted {
    drops: Arc<AtomicUsize>,
    value: usize,
}
// SAFETY: this fixture contains no outgoing GC references.
#[expect(unsafe_code, reason = "Trace requires an explicit ownership contract")]
unsafe impl Trace for Counted {
    fn trace(&self, _: &GarbageCollector) {}
}
impl Drop for Counted {
    fn drop(&mut self) {
        self.drops.fetch_add(1, Ordering::Relaxed);
    }
}

#[test]
fn cancellation_at_each_staging_boundary_preserves_exactly_once_destruction() -> Result<()> {
    for polls in 0..=2 {
        let gc = GarbageCollector::with_config(
            Configuration::builder()
                .compact_heap(true)
                .max_heap_bytes(512 * 1024)
                .allocation_threshold(usize::MAX)
                .build(),
        );
        let drops = Arc::new(AtomicUsize::new(0));
        let values: HeapBuffer<_> = (0..40)
            .map(|value| Counted {
                drops: drops.clone(),
                value,
            })
            .collect::<Vec<_>>()
            .into();
        let mut future = Box::pin(Gc::try_new_async(&gc, RwLock::new(values)));
        let waker = Waker::noop();
        let mut cx = Context::from_waker(waker);
        for index in 0..polls {
            assert!(future.as_mut().poll(&mut cx).is_pending());
            assert_eq!(
                gc.region_heap().unwrap().statistics().bytes_staged,
                (index + 1) * 64 * 1024
            );
            assert_eq!(
                gc.managed_bytes(),
                0,
                "partial preparation must stay unpublished"
            );
            assert_eq!(drops.load(Ordering::Relaxed), 0);
        }
        drop(future);
        assert_eq!(drops.load(Ordering::Relaxed), 40);
        assert_eq!(gc.region_heap().unwrap().statistics().occupied_bytes, 0);
        gc.stop()?;
    }
    Ok(())
}

#[test]
fn completed_staging_preserves_overalignment_and_all_elements() -> Result<()> {
    let gc = GarbageCollector::with_config(
        Configuration::builder()
            .compact_heap(true)
            .max_heap_bytes(512 * 1024)
            .allocation_threshold(usize::MAX)
            .build(),
    );
    let drops = Arc::new(AtomicUsize::new(0));
    let values: HeapBuffer<_> = (0..40)
        .map(|value| Counted {
            drops: drops.clone(),
            value,
        })
        .collect::<Vec<_>>()
        .into();
    let mut future = Box::pin(Gc::try_new_async(&gc, RwLock::new(values)));
    let waker = Waker::noop();
    let mut cx = Context::from_waker(waker);
    assert!(future.as_mut().poll(&mut cx).is_pending());
    assert!(future.as_mut().poll(&mut cx).is_pending());
    let Poll::Ready(root) = future.as_mut().poll(&mut cx) else {
        panic!("third batch completes")
    };
    let root = root?;
    assert_eq!(root.read().as_ptr() as usize % 4096, 0);
    assert!(root.read().iter().enumerate().all(|(i, v)| i == v.value));
    assert_eq!(
        gc.region_heap().unwrap().statistics().bytes_staged,
        40 * 4096
    );
    drop(future);
    drop(root);
    gc.stop()?;
    assert_eq!(drops.load(Ordering::Relaxed), 40);
    assert_eq!(gc.region_heap().unwrap().statistics().occupied_bytes, 0);
    Ok(())
}

#[test]
fn mutation_truncation_and_heap_transfer_cannot_commit_stale_preparation() -> Result<()> {
    let first = RegionHeap::new(512, 1024, 0);
    let second = RegionHeap::new(512, 1024, 0);
    let mut values = HeapBuffer::from(vec![1_u64, 2, 3, 4]);
    assert_eq!(values.prepare_step(&first, 8)?, TraceProgress::Continue(1));
    assert_eq!(first.statistics().occupied_bytes, 32);
    *values.get_mut(0).unwrap() = 9;
    assert_eq!(first.statistics().occupied_bytes, 0);
    assert_eq!(values.prepare_step(&first, 8)?, TraceProgress::Continue(1));
    values.truncate(3);
    assert_eq!(first.statistics().occupied_bytes, 0);
    assert_eq!(values.prepare_step(&first, 8)?, TraceProgress::Continue(1));
    assert_eq!(values.prepare_step(&second, 8)?, TraceProgress::Continue(1));
    assert_eq!(first.statistics().occupied_bytes, 0);
    values.prepare(&second)?;
    assert_eq!(&*values, &[9, 2, 3]);
    assert_eq!(values.into_vec(), vec![9, 2, 3]);
    assert_eq!(second.statistics().occupied_bytes, 0);
    Ok(())
}

#[test]
fn failed_transfer_keeps_the_source_and_managed_guards_prevent_async_preparation() -> Result<()> {
    let first = RegionHeap::new(512, 1024, 0);
    let full = RegionHeap::new(8, 8, 0);
    let mut values = HeapBuffer::from(vec![1_u64, 2, 3, 4]);
    assert_eq!(values.prepare_step(&first, 8)?, TraceProgress::Continue(1));
    assert!(matches!(
        values.prepare_step(&full, 8),
        Err(Error::OutOfMemory)
    ));
    assert_eq!(first.statistics().occupied_bytes, 0);
    assert_eq!(&*values, &[1, 2, 3, 4]);
    let gc = GarbageCollector::with_config(Configuration::builder().compact_heap(true).build());
    let pin = Gc::try_new(&gc, RwLock::new(0_u64))?;
    let _guard = pin.read();
    let mut future = Box::pin(Gc::try_new_async(&gc, RwLock::new(values)));
    let waker = Waker::noop();
    let mut cx = Context::from_waker(waker);
    assert!(matches!(
        future.as_mut().poll(&mut cx),
        Poll::Ready(Err(Error::CollectionPhaseError(_)))
    ));
    assert_eq!(gc.region_heap().unwrap().statistics().bytes_staged, 0);
    Ok(())
}

#[test]
#[cfg(panic = "unwind")]
fn a_panicking_suffix_destructor_cannot_leave_initialized_length_stale() -> Result<()> {
    struct PanicsOnce {
        index: usize,
        counts: Arc<[AtomicUsize]>,
    }
    impl Drop for PanicsOnce {
        fn drop(&mut self) {
            let previous = self
                .counts
                .get(self.index)
                .expect("fixture index")
                .fetch_add(1, Ordering::Relaxed);
            assert!(
                self.index != 2 || previous != 0,
                "suffix destructor panicked"
            );
        }
    }
    let heap = RegionHeap::new(1024, 2048, 0);
    for backing in [None, Some(heap.as_ref())] {
        let counts: Arc<[AtomicUsize]> = (0..8).map(|_| AtomicUsize::new(0)).collect();
        let mut buffer = HeapBuffer::try_with_fn(backing, 8, |index| {
            Ok(PanicsOnce {
                index,
                counts: counts.clone(),
            })
        })?;
        assert!(
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| buffer.truncate(2))).is_err()
        );
        assert_eq!(buffer.len(), 2);
        assert_eq!(buffer.get(1).expect("retained prefix").index, 1);
        assert!(
            counts
                .iter()
                .skip(2)
                .all(|count| count.load(Ordering::Relaxed) == 1)
        );
        drop(buffer);
        assert!(
            counts
                .iter()
                .all(|count| count.load(Ordering::Relaxed) == 1)
        );
        assert_eq!(heap.statistics().occupied_bytes, 0);
    }
    Ok(())
}
