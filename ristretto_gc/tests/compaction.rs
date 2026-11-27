//! Relocation, pinning, epoch overlap and worker termination invariants.
#![expect(clippy::panic_in_result_fn, clippy::indexing_slicing)]
#![expect(
    clippy::fn_params_excessive_bools,
    reason = "independent configuration axes in the compaction matrix"
)]
use ristretto_gc::region::RegionHeap;
use ristretto_gc::sync::RwLock;
use ristretto_gc::{Configuration, GarbageCollector, Gc, HeapBuffer, Result, Trace, TraceProgress};
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

struct Payload(HeapBuffer<u64>);
// SAFETY: this fixture contains no outgoing GC references.
#[expect(unsafe_code, reason = "Trace requires an explicit ownership contract")]
unsafe impl Trace for Payload {
    fn trace(&self, _: &GarbageCollector) {}
    fn prepare(&mut self, heap: &RegionHeap) -> Result<()> {
        self.0.prepare(heap)
    }
    fn relocation_region(&self, heap: &RegionHeap) -> Option<u64> {
        self.0.relocation_region(heap)
    }
    fn compact_step(&mut self, heap: &RegionHeap, bytes: usize) -> TraceProgress {
        self.0.compact_step(heap, bytes)
    }
    fn external_size(&self) -> usize {
        size_of_val(self.0.as_ref())
    }
}

#[test]
fn direct_initialization_drops_only_the_initialized_prefix() -> Result<()> {
    struct Counted(Arc<AtomicUsize>);
    impl Drop for Counted {
        fn drop(&mut self) {
            self.0.fetch_add(1, Ordering::Relaxed);
        }
    }
    let heap = RegionHeap::new(512, 1024, 0);
    for backing in [None, Some(heap.as_ref())] {
        let drops = Arc::new(AtomicUsize::new(0));
        let result = HeapBuffer::try_with_fn(backing, 16, |index| {
            if index == 7 {
                return Err(ristretto_gc::Error::OutOfMemory);
            }
            Ok(Counted(drops.clone()))
        });
        assert!(result.is_err());
        assert_eq!(drops.load(Ordering::Relaxed), 7);
        assert_eq!(heap.statistics().occupied_bytes, 0);
        #[cfg(panic = "unwind")]
        {
            let panic = std::panic::catch_unwind(|| {
                let _ = HeapBuffer::try_with_fn(backing, 16, |index| {
                    assert!(index != 3, "initializer failure");
                    Ok(Counted(drops.clone()))
                });
            });
            assert!(panic.is_err());
            assert_eq!(drops.load(Ordering::Relaxed), 10);
        }
        assert_eq!(heap.statistics().occupied_bytes, 0);
    }
    let mut values = HeapBuffer::try_with_fn(Some(&heap), 16, |index| Ok(index as u64))?;
    let address = values.as_ptr();
    values.prepare(&heap)?;
    assert_eq!(values.as_ptr(), address);
    assert_eq!(heap.statistics().bytes_staged, 0);
    Ok(())
}

#[test]
fn transferring_a_buffer_between_heaps_reaccounts_its_storage() -> Result<()> {
    let source = RegionHeap::new(128, 256, 0);
    let target = RegionHeap::new(128, 128, 0);
    let hole = HeapBuffer::try_with_fn(Some(&source), 16, |_| Ok(0_u64))?;
    let mut buffer = HeapBuffer::try_with_fn(Some(&source), 16, |index| Ok(index as u64))?;
    drop(hole);
    assert!(matches!(
        buffer.compact_step(&source, 8),
        TraceProgress::Continue(_)
    ));
    buffer.prepare(&target)?;
    assert_eq!(source.statistics().occupied_bytes, 0);
    assert_eq!(target.statistics().occupied_bytes, 128);
    assert_eq!(buffer.compact_step(&target, 128), TraceProgress::Complete);
    assert_eq!(source.statistics().occupied_bytes, 0);
    assert_eq!(target.statistics().occupied_bytes, 128);
    assert_eq!(buffer[15], 15);
    drop(buffer);
    assert_eq!(target.statistics().occupied_bytes, 0);
    Ok(())
}

#[test]
fn incremental_copy_restarts_modified_data_and_transfers_ownership_once() -> Result<()> {
    struct Counted(Arc<AtomicUsize>, usize);
    impl Drop for Counted {
        fn drop(&mut self) {
            self.0.fetch_add(1, Ordering::Relaxed);
        }
    }
    let drops = Arc::new(AtomicUsize::new(0));
    let size = 16 * size_of::<Counted>();
    let heap = RegionHeap::new(size, size * 3, 0);
    let make = || {
        (0..16)
            .map(|i| Counted(drops.clone(), i))
            .collect::<Vec<_>>()
    };
    let mut hole: HeapBuffer<_> = make().into();
    hole.prepare(&heap)?;
    let mut live: HeapBuffer<_> = make().into();
    live.prepare(&heap)?;
    let old = live.as_ptr();
    drop(hole);
    assert_eq!(drops.load(Ordering::Relaxed), 16);
    assert!(matches!(
        live.compact_step(&heap, size_of::<Counted>()),
        TraceProgress::Continue(_)
    ));
    live.get_mut(0).unwrap().1 = 99;
    for _ in 0..16 {
        live.compact_step(&heap, size_of::<Counted>());
    }
    assert_eq!(
        heap.statistics().bytes_recopied,
        size_of::<Counted>() as u64
    );
    assert_eq!(heap.statistics().bytes_staged, (size * 2) as u64);
    assert_ne!(old, live.as_ptr());
    assert_eq!(live[0].1, 99);
    assert_eq!(
        drops.load(Ordering::Relaxed),
        16,
        "copying must never drop moved elements"
    );
    assert!(heap.trim(8));
    assert_eq!(heap.statistics().regions, 1);
    let values = live.into_vec();
    assert_eq!(values[0].1, 99);
    drop(values);
    assert_eq!(drops.load(Ordering::Relaxed), 32);
    Ok(())
}

#[test]
fn failed_destination_keeps_payload_and_accounting_intact() -> Result<()> {
    let heap = RegionHeap::new(64, 128, 0);
    let mut a: HeapBuffer<_> = vec![7_u64; 8].into();
    a.prepare(&heap)?;
    let mut b: HeapBuffer<_> = vec![9_u64; 8].into();
    b.prepare(&heap)?;
    let original = b.as_ptr();
    assert_eq!(b.compact_step(&heap, 8), TraceProgress::Complete);
    assert_eq!(original, b.as_ptr());
    assert_eq!(&*b, &[9; 8]);
    let mut rejected: HeapBuffer<_> = vec![11_u64; 8].into();
    assert!(rejected.prepare(&heap).is_err());
    assert_eq!(&*rejected, &[11; 8]);
    assert_eq!(heap.statistics().occupied_bytes, 128);
    Ok(())
}

#[test]
fn relocation_recopies_only_dirty_chunks_across_bitmap_words() -> Result<()> {
    let size = 70 * 1024;
    let heap = RegionHeap::new(size, size * 2, 0);
    let hole = HeapBuffer::try_with_fn(Some(&heap), size, |_| Ok(0_u8))?;
    let mut live = HeapBuffer::try_with_fn(Some(&heap), size, |_| Ok(1_u8))?;
    let original = live.as_ptr();
    drop(hole);
    for _ in 0..67 {
        assert!(matches!(
            live.compact_step(&heap, 1024),
            TraceProgress::Continue(_)
        ));
    }
    *live.get_mut(0).unwrap() = 9;
    live.get_range_mut(63 * 1024..65 * 1024 + 1)
        .unwrap()
        .fill(7);
    for _ in 0..7 {
        live.compact_step(&heap, 1024);
    }
    assert_ne!(original, live.as_ptr());
    assert_eq!(heap.statistics().bytes_copied, 74 * 1024);
    assert_eq!(heap.statistics().bytes_recopied, 4 * 1024);
    for (index, value) in live.iter().enumerate() {
        let expected = if index == 0 {
            9
        } else if (63 * 1024..65 * 1024 + 1).contains(&index) {
            7
        } else {
            1
        };
        assert_eq!(*value, expected);
    }
    Ok(())
}

#[cfg(not(target_family = "wasm"))]
#[test]
fn concurrent_region_reuse_and_trim_preserve_budget_and_ranges() -> Result<()> {
    let heap = RegionHeap::new(4096, 256 * 1024, 0);
    std::thread::scope(|scope| {
        for thread in 0..4 {
            let heap = &heap;
            scope.spawn(move || {
                let mut buffers = std::collections::VecDeque::new();
                for index in 0..1000 {
                    let length = 1 + (index * 17 + thread * 11) % 64;
                    buffers.push_back(
                        HeapBuffer::try_with_fn(Some(heap), length, |_| Ok(index as u64)).unwrap(),
                    );
                    if buffers.len() > 16 {
                        let buffer = buffers.pop_front().unwrap();
                        assert!(buffer.iter().all(|value| *value == (index - 16) as u64));
                    }
                    if index % 31 == 0 {
                        heap.trim(2);
                        assert!(heap.statistics().committed_bytes <= 256 * 1024);
                    }
                }
            });
        }
    });
    assert_eq!(heap.statistics().occupied_bytes, 0);
    // Finish the partial pass left by mutators, then inspect all earlier regions
    // that may have become empty since their last inspection.
    while !heap.trim(8) {}
    while !heap.trim(8) {}
    assert_eq!(heap.statistics().committed_bytes, 0);
    // Reuse after removing every region must also update the availability index.
    let buffer = HeapBuffer::try_with_fn(Some(&heap), 512, |_| Ok(42_u64))?;
    assert!(buffer.iter().all(|value| *value == 42));
    Ok(())
}

#[test]
fn managed_guards_pin_payload_but_roots_allow_relocation() -> Result<()> {
    fn case(independent: bool, cost_aware: bool, concurrent: bool, overlap: bool) -> Result<()> {
        let gc = GarbageCollector::with_config(
            Configuration::builder()
                .concurrent_publication(concurrent)
                .overlapping_generations(overlap)
                .old_collection_interval(if overlap { 4 } else { 0 })
                .independent_phases(overlap)
                .region_shards(if overlap { 16 } else { 1 })
                .diagnostics(true)
                .independent_relocation(independent)
                .cost_aware_relocation(cost_aware)
                .metadata_shards(16)
                .work_stealing(true)
                .independent_marking(true)
                .compact_heap(true)
                .region_layout(512, 32)
                .allocation_threshold(usize::MAX)
                .build(),
        );
        let hole = Gc::try_new(&gc, RwLock::new(Payload(vec![1; 32].into())))?;
        let filler = Gc::try_new(&gc, RwLock::new(Payload(vec![2; 32].into())))?;
        let live = Gc::try_new(&gc, RwLock::new(Payload(vec![42; 32].into())))?;
        let id = live.identity();
        let guard = live.read();
        let old = guard.0.as_ptr();
        drop(hole);
        gc.collect_full()?;
        assert_eq!(old, guard.0.as_ptr());
        assert_eq!(guard.0[17], 42);
        drop(guard);
        // Keep the released referent sparse enough for cost-aware selection.
        // Packing other cold data beside a pinned payload may otherwise make its
        // region dense, in which case retaining its address is the better policy.
        drop(filler);
        for _ in 0..3 {
            gc.collect_full()?;
            if old != live.read().0.as_ptr() {
                break;
            }
        }
        assert_ne!(old, live.read().0.as_ptr());
        assert_eq!(
            gc.diagnostics().relocation_pin_deferral.count > 0,
            cost_aware
        );
        if cost_aware {
            assert!(
                gc.region_heap()
                    .expect("heap")
                    .statistics()
                    .pinned_region_skips
                    > 0
            );
        }
        assert_eq!(id, live.identity());
        assert_eq!(
            *Gc::<RwLock<Payload>>::from_handle_rooted(&gc, id.to_i64())?
                .read()
                .0,
            [42; 32]
        );
        Ok(())
    }
    for independent in [false, true] {
        for cost_aware in [false, true] {
            for (concurrent, overlap) in
                [(false, false), (true, false), (false, true), (true, true)]
            {
                case(independent, cost_aware, concurrent, overlap)?;
            }
        }
    }
    Ok(())
}

#[test]
fn equal_size_fragmentation_releases_reclaimable_regions() -> Result<()> {
    fn case(independent: bool, cost_aware: bool, concurrent: bool, overlap: bool) -> Result<()> {
        let gc = GarbageCollector::with_config(
            Configuration::builder()
                .concurrent_publication(concurrent)
                .overlapping_generations(overlap)
                .old_collection_interval(if overlap { 4 } else { 0 })
                .independent_phases(overlap)
                .region_shards(if overlap { 16 } else { 1 })
                .independent_relocation(independent)
                .cost_aware_relocation(cost_aware)
                .metadata_shards(16)
                .work_stealing(true)
                .independent_marking(true)
                .compact_heap(true)
                .region_layout(4096, 128)
                .allocation_threshold(usize::MAX)
                .max_pause_time_us(0)
                .build(),
        );
        let mut roots = Vec::new();
        for i in 0..128 {
            roots.push(Some(Gc::try_new(
                &gc,
                RwLock::new(Payload(vec![i; 64].into())),
            )?));
        }
        let heap = gc.region_heap().unwrap();
        assert_eq!(heap.statistics().regions, 16);
        for index in (0..128).step_by(2) {
            roots[index] = None;
        }
        for _ in 0..4 {
            gc.collect_full()?;
        }
        assert_eq!(
            heap.statistics().regions,
            8,
            "all theoretically reclaimable equal-size regions must be released"
        );
        assert!(heap.statistics().relocations > 0);
        for (i, root) in roots.iter().enumerate() {
            if let Some(root) = root {
                assert!(root.read().0.iter().all(|value| *value == i as u64));
            }
        }
        drop(roots);
        gc.collect_full()?;
        assert_eq!(heap.statistics().committed_bytes, 0);
        assert_eq!(gc.managed_bytes(), 0);
        Ok(())
    }
    for independent in [false, true] {
        for cost_aware in [false, true] {
            for (concurrent, overlap) in
                [(false, false), (true, false), (false, true), (true, true)]
            {
                case(independent, cost_aware, concurrent, overlap)?;
            }
        }
    }
    Ok(())
}

#[test]
fn evacuation_transfers_destructors_before_releasing_source_backing() -> Result<()> {
    struct Counted(Arc<AtomicUsize>, usize);
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
    for independent in [false, true] {
        let gc = GarbageCollector::with_config(
            Configuration::builder()
                .compact_heap(true)
                .cost_aware_relocation(true)
                .relocation_reserve_bytes(1024)
                .max_heap_bytes(8192)
                .region_layout(1024, 128)
                .independent_relocation(independent)
                .incremental_step_size(8)
                .max_pause_time_us(0)
                .allocation_threshold(usize::MAX)
                .build(),
        );
        let drops = Arc::new(AtomicUsize::new(0));
        let heap = gc.region_heap().unwrap();
        let mut roots = Vec::new();
        for id in 0..6 {
            let values =
                HeapBuffer::try_with_fn(Some(heap), 32, |_| Ok(Counted(drops.clone(), id)))?;
            roots.push(Some(Gc::try_new(&gc, RwLock::new(values))?));
        }
        for index in (1..6).step_by(2) {
            roots[index] = None;
        }
        gc.collect_full()?;
        assert_eq!(drops.load(Ordering::Relaxed), 96);
        assert!(heap.statistics().evacuation_regions_released > 0);
        for (id, root) in roots.iter().enumerate() {
            if let Some(root) = root {
                assert!(root.read().iter().all(|value| value.1 == id));
            }
        }
        drop(roots);
        gc.collect_full()?;
        assert_eq!(drops.load(Ordering::Relaxed), 192);
        assert_eq!(heap.statistics().shared_committed_bytes, 0);
        assert_eq!(gc.managed_bytes(), 0);
    }
    Ok(())
}

struct LongTrace;
// SAFETY: this fixture contains no outgoing GC references.
#[expect(unsafe_code, reason = "Trace requires an explicit ownership contract")]
unsafe impl Trace for LongTrace {
    fn trace(&self, _: &GarbageCollector) {}
    fn trace_step(&self, _: &GarbageCollector, offset: usize, _: usize) -> TraceProgress {
        if offset == 10_000 {
            TraceProgress::Complete
        } else {
            TraceProgress::Continue(offset + 1)
        }
    }
}

#[test]
fn young_finishes_during_full_tracing_and_full_waits_for_its_requested_cycle() -> Result<()> {
    fn case(independent: bool, cost_aware: bool, concurrent: bool, overlap: bool) -> Result<()> {
        let gc = GarbageCollector::with_config(
            Configuration::builder()
                .concurrent_publication(concurrent)
                .overlapping_generations(overlap)
                .independent_phases(overlap)
                .region_shards(if overlap { 16 } else { 1 })
                .independent_relocation(independent)
                .cost_aware_relocation(cost_aware)
                .metadata_shards(16)
                .work_stealing(true)
                .independent_marking(true)
                .allocation_threshold(usize::MAX)
                .incremental_step_size(1)
                .max_pause_time_us(0)
                .full_collection_interval(usize::MAX)
                .build(),
        );
        let _old = Gc::new(&gc, LongTrace);
        gc.collect_full()?;
        gc.collect_full()?;
        gc.collect_young();
        while !gc.step() {}
        let before = gc.statistics()?;
        gc.collect();
        for _ in 0..10 {
            gc.step();
        }
        drop(Gc::new(&gc, 12_u64));
        gc.collect_young();
        // The all-feature WASI build completes this young cycle on one-unit
        // poll 101. The full trace still has thousands of units outstanding.
        for _ in 0..101 {
            gc.step();
        }
        assert_eq!(
            gc.statistics()?.young_collections,
            before.young_collections + 1
        );
        assert_eq!(gc.statistics()?.full_collections, before.full_collections);
        gc.collect_full()?;
        assert!(
            gc.statistics()?.full_collections >= before.full_collections + 2,
            "collect_full must finish the request made after the active full cycle began"
        );
        Ok(())
    }
    for independent in [false, true] {
        for cost_aware in [false, true] {
            for (concurrent, overlap) in
                [(false, false), (true, false), (false, true), (true, true)]
            {
                case(independent, cost_aware, concurrent, overlap)?;
            }
        }
    }
    Ok(())
}

#[cfg(not(target_family = "wasm"))]
#[test]
fn native_workers_really_overlap_tracing_and_terminate() -> Result<()> {
    fn case(independent: bool, cost_aware: bool, concurrent: bool, overlap: bool) -> Result<()> {
        struct Work {
            active: Arc<AtomicUsize>,
            peak: Arc<AtomicUsize>,
            moving: Arc<AtomicUsize>,
            moving_peak: Arc<AtomicUsize>,
        }
        // SAFETY: this fixture contains no outgoing GC references.
        #[expect(unsafe_code, reason = "Trace requires an explicit ownership contract")]
        unsafe impl Trace for Work {
            fn compact_guarded(&self, _: &RegionHeap, _: usize) -> TraceProgress {
                let active = self.moving.fetch_add(1, Ordering::AcqRel) + 1;
                self.moving_peak.fetch_max(active, Ordering::AcqRel);
                let start = std::time::Instant::now();
                while self.moving_peak.load(Ordering::Acquire) < 2
                    && start.elapsed() < std::time::Duration::from_millis(100)
                {
                    std::thread::yield_now();
                }
                self.moving.fetch_sub(1, Ordering::AcqRel);
                TraceProgress::Complete
            }
            fn trace(&self, _: &GarbageCollector) {
                let active = self.active.fetch_add(1, Ordering::AcqRel) + 1;
                self.peak.fetch_max(active, Ordering::AcqRel);
                // A bounded rendezvous exposes overlap without relying on throughput timing.
                let start = std::time::Instant::now();
                while self.peak.load(Ordering::Acquire) < 2
                    && start.elapsed() < std::time::Duration::from_millis(100)
                {
                    std::thread::yield_now();
                }
                self.active.fetch_sub(1, Ordering::AcqRel);
            }
        }
        let gc = GarbageCollector::with_config(
            Configuration::builder()
                .concurrent_publication(concurrent)
                .overlapping_generations(overlap)
                .old_collection_interval(if overlap { 4 } else { 0 })
                .independent_phases(overlap)
                .region_shards(if overlap { 16 } else { 1 })
                .independent_relocation(independent)
                .cost_aware_relocation(cost_aware)
                .metadata_shards(16)
                .work_stealing(true)
                .independent_marking(true)
                .threads(4)
                .mark_batch_size(8)
                .compact_heap(true)
                .allocation_threshold(usize::MAX)
                .build(),
        );
        let active = Arc::new(AtomicUsize::new(0));
        let peak = Arc::new(AtomicUsize::new(0));
        let moving = Arc::new(AtomicUsize::new(0));
        let moving_peak = Arc::new(AtomicUsize::new(0));
        let roots: Vec<_> = (0..32)
            .map(|_| {
                Gc::new(
                    &gc,
                    Work {
                        active: active.clone(),
                        peak: peak.clone(),
                        moving: moving.clone(),
                        moving_peak: moving_peak.clone(),
                    },
                )
            })
            .collect();
        gc.start();
        gc.collect();
        let start = std::time::Instant::now();
        while gc.statistics()?.full_collections == 0 {
            assert!(start.elapsed() < std::time::Duration::from_secs(10));
            std::thread::yield_now();
        }
        assert!(peak.load(Ordering::Acquire) >= 2);
        assert!(moving_peak.load(Ordering::Acquire) >= 2);
        assert_eq!(moving.load(Ordering::Acquire), 0);
        assert_eq!(active.load(Ordering::Acquire), 0);
        drop(roots);
        gc.stop()?;
        assert_eq!(gc.statistics()?.live_objects, 0);
        Ok(())
    }
    for independent in [false, true] {
        for cost_aware in [false, true] {
            for (concurrent, overlap) in
                [(false, false), (true, false), (false, true), (true, true)]
            {
                case(independent, cost_aware, concurrent, overlap)?;
            }
        }
    }
    Ok(())
}

#[test]
fn an_old_snapshot_keeps_a_removed_young_graph_valid() -> Result<()> {
    fn case(independent: bool, cost_aware: bool, concurrent: bool, overlap: bool) -> Result<()> {
        use ristretto_gc::{GcWeak, ReferenceKind};
        let gc = GarbageCollector::with_config(
            Configuration::builder()
                .concurrent_publication(concurrent)
                .overlapping_generations(overlap)
                .independent_phases(overlap)
                .region_shards(if overlap { 16 } else { 1 })
                .independent_relocation(independent)
                .cost_aware_relocation(cost_aware)
                .metadata_shards(16)
                .work_stealing(true)
                .independent_marking(true)
                .allocation_threshold(usize::MAX)
                .incremental_step_size(1)
                .max_pause_time_us(0)
                .build(),
        );
        let child = Gc::new(&gc, 99_u64).into_rooted();
        let weak = GcWeak::new(&child, ReferenceKind::Weak, 0);
        let parent = Gc::new(&gc, RwLock::new(vec![child]));
        gc.collect();
        gc.step(); // The root is shaded, but its outgoing edge has not been scanned.
        drop(parent);
        gc.collect_young();
        for _ in 0..1000 {
            if gc.step() {
                break;
            }
        }
        assert_eq!(weak.upgrade().as_deref(), Some(&99));
        gc.collect_full()?;
        assert!(weak.upgrade().is_none());
        Ok(())
    }
    for independent in [false, true] {
        for cost_aware in [false, true] {
            for (concurrent, overlap) in
                [(false, false), (true, false), (false, true), (true, true)]
            {
                case(independent, cost_aware, concurrent, overlap)?;
            }
        }
    }
    Ok(())
}

#[test]
fn relocation_preserves_overalignment_after_odd_sized_allocations() -> Result<()> {
    #[repr(align(256))]
    struct Aligned(u64);
    let heap = RegionHeap::new(1024, 4096, 0);
    let mut odd: HeapBuffer<_> = vec![1_u8; 3].into();
    odd.prepare(&heap)?;
    let mut hole: HeapBuffer<_> = vec![Aligned(1), Aligned(2)].into();
    hole.prepare(&heap)?;
    let mut live: HeapBuffer<_> = vec![Aligned(42), Aligned(99)].into();
    live.prepare(&heap)?;
    assert_eq!(heap.statistics().regions, 2);
    let before = live.as_ptr();
    assert_eq!(before as usize % 256, 0);
    drop(hole);
    assert!(matches!(
        live.compact_step(&heap, 256),
        TraceProgress::Continue(_)
    ));
    assert_eq!(live.compact_step(&heap, 256), TraceProgress::Complete);
    assert_ne!(before, live.as_ptr());
    assert_eq!(live.as_ptr() as usize % 256, 0);
    assert_eq!((live[0].0, live[1].0), (42, 99));
    assert_eq!(&*odd, &[1, 1, 1]);
    drop((odd, live));
    assert!(heap.trim(4));
    assert_eq!(heap.statistics().committed_bytes, 0);
    Ok(())
}
