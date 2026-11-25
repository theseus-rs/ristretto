//! A type with no trace actions needs no payload lock to complete marking.
#![expect(clippy::panic_in_result_fn)]
#![expect(
    clippy::panic,
    reason = "fail if a forbidden trace hook or unbounded collection occurs"
)]
use ristretto_gc::cell::RwLock;
use ristretto_gc::{Configuration, GarbageCollector, Gc, HeapBuffer, Result, Trace};
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

fn finish(gc: &GarbageCollector) {
    for _ in 0..20_000 {
        if gc.step() {
            return;
        }
    }
    panic!("collection did not finish while a leaf payload was locked");
}

#[test]
fn held_leaf_payload_allows_old_young_and_full_reclamation() -> Result<()> {
    for concurrent in [false, true] {
        for independent in [false, true] {
            let gc = GarbageCollector::with_config(
                Configuration::builder()
                    .concurrent_publication(concurrent)
                    .independent_phases(independent)
                    .independent_marking(independent)
                    .independent_relocation(independent)
                    .overlapping_generations(true)
                    .old_collection_interval(usize::MAX)
                    .full_collection_interval(usize::MAX)
                    .allocation_threshold(usize::MAX)
                    .promotion_age(4)
                    .compact_heap(true)
                    .region_layout(4096, 64)
                    .incremental_step_size(8)
                    .max_pause_time_us(0)
                    .diagnostics(true)
                    .build(),
            );
            let root = Gc::try_new(&gc, RwLock::new(HeapBuffer::from(vec![7_u64; 64])))?;
            let mut guard = root.write();
            let pointer = guard.as_ptr();
            let bytes = gc.managed_bytes();
            // Old handoff encounters a still-young leaf. Holding the write lock
            // must not delay its scan or the subsequent precise full passes.
            gc.collect_old();
            finish(&gc);
            assert_eq!(gc.statistics()?.old_collections, 1);
            gc.collect_young();
            finish(&gc);
            for cycle in 1..=3 {
                for _ in 0..32 {
                    drop(Gc::try_new(&gc, 42_u64)?);
                }
                gc.collect();
                finish(&gc);
                assert_eq!(gc.statistics()?.full_collections, cycle);
                assert_eq!(gc.statistics()?.live_objects, 1);
                assert_eq!(gc.managed_bytes(), bytes);
                assert_eq!(guard.as_ptr(), pointer, "locked storage must not relocate");
                assert!(guard.iter().all(|value| *value == 7));
            }
            assert_eq!(gc.diagnostics().trace_busy, 0);
            guard.fill(9);
            drop(guard);
            gc.collect_full()?;
            assert!(root.read().iter().all(|value| *value == 9));
            drop(root);
            gc.stop()?;
            assert_eq!(gc.managed_bytes(), 0);
            assert_eq!(gc.statistics()?.live_objects, 0);
        }
    }
    Ok(())
}

#[test]
fn locked_standard_and_portable_leaves_do_not_delay_marking() -> Result<()> {
    let gc = GarbageCollector::with_config(
        Configuration::builder()
            .allocation_threshold(usize::MAX)
            .build(),
    );
    let mutex = Gc::try_new(&gc, std::sync::Mutex::new(1_u64))?;
    let rwlock = Gc::try_new(&gc, std::sync::RwLock::new(2_u64))?;
    let portable = Gc::try_new(&gc, ristretto_gc::sync::Mutex::new(3_u64))?;
    let a = mutex
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    let b = rwlock
        .write()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    let c = portable.lock();
    // Exercise both direct/container tracing and the descriptor's mark path.
    (**mutex).trace(&gc);
    (**rwlock).trace(&gc);
    (**portable).trace(&gc);
    gc.collect();
    finish(&gc);
    assert_eq!(gc.statistics()?.full_collections, 1);
    assert_eq!((*a, *b, *c), (1, 2, 3));
    drop((a, b, c));
    drop((mutex, rwlock, portable));
    gc.stop()?;
    assert_eq!(gc.managed_bytes(), 0);
    Ok(())
}

struct Dropped(Arc<AtomicUsize>);
// SAFETY: this fixture contains no outgoing GC references.
#[expect(unsafe_code, reason = "Trace requires an explicit ownership contract")]
unsafe impl Trace for Dropped {
    fn may_contain_gc_references() -> bool {
        false
    }
    fn trace(&self, _: &GarbageCollector) {
        panic!("declared leaves require no trace actions");
    }
}
impl Drop for Dropped {
    fn drop(&mut self) {
        self.0.fetch_add(1, Ordering::Relaxed);
    }
}

#[test]
fn omitted_leaf_tracing_preserves_bounded_element_destruction() -> Result<()> {
    for independent in [false, true] {
        let gc = GarbageCollector::with_config(
            Configuration::builder()
                .independent_phases(independent)
                .independent_marking(independent)
                .incremental_step_size(1)
                .max_pause_time_us(0)
                .allocation_threshold(usize::MAX)
                .build(),
        );
        let drops = Arc::new(AtomicUsize::new(0));
        let root = Gc::try_new(
            &gc,
            RwLock::new(
                (0..97)
                    .map(|_| Dropped(drops.clone()))
                    .collect::<HeapBuffer<_>>(),
            ),
        )?;
        gc.collect_full()?;
        assert_eq!(drops.load(Ordering::Relaxed), 0);
        drop(root);
        gc.collect();
        for _ in 0..20_000 {
            let before = drops.load(Ordering::Relaxed);
            let done = gc.step();
            assert!(drops.load(Ordering::Relaxed) - before <= 1);
            if done {
                break;
            }
        }
        assert_eq!(drops.load(Ordering::Relaxed), 97);
        assert_eq!(gc.managed_bytes(), 0);
        gc.stop()?;
        assert_eq!(drops.load(Ordering::Relaxed), 97);
    }
    Ok(())
}
